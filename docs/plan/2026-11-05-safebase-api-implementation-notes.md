# Implementation Notes: SafeBase REST API in drata-cli

Design doc: `docs/plan/2026-11-05-safebase-api.md`

## Phase 1: Backend seam

### Design decisions
- `DrataClient` stores the auth header pair once, as `auth: (HeaderName, HeaderValue)` built by `Backend::auth_header` in `new` (`src/client.rs:DrataClient::new`). The key is checked as a header value at construction, the error never echoes the key, and `send_inner` / `send_multipart` both read the one stored pair, so no other path names an auth header. The value is marked sensitive
- SafeBase same-origin redirects take the origin from the redirect chain's first URL (`attempt.previous().first()`), not a captured base URL (`src/backend.rs:same_origin_redirects`). Every request goes to `base_url`, so production behavior is the same, and it stays correct after `with_base_url` points the client at wiremock with no `Client` rebuild. Otherwise harvested as-is from atlassian-cli (`MAX_REDIRECTS = 10`, `>=` check, error text)
- A broken SafeBase envelope is a new typed `EnvelopeError { status, body, formatted }`, not an `ApiError` (`src/client.rs:check_envelope`). An `ApiError` with status 200 would lie, and `try_get`'s 404 match stays untouched. Checked in both `send_inner` and `send_multipart`. On `ok: true` the whole response comes back; callers read `result`
- Drata region constants and the region resolver moved from `client.rs` to `src/backend.rs` (`DRATA_*_BASE_URL`, `drata_base_url`): the backend owns its base URL. `Backend::Drata.base_url(None)` is `us`, the profile default
- Both spec JSON strings are `pub(crate)` consts in `src/spec.rs`; `Backend::spec` holds the two `OnceLock`s and uses `get_or_init`, replacing the old `set` then re-`get`
- `Backend` implements `Display` ("Drata" | "SafeBase") for error text; Phase 2's backend-named `WriteGuardError` can use it
- `operation_tags` takes a `Backend` too, beside the two functions the doc names, so no spec walker quietly assumes Drata

### Deviations
- The doc sketches `spec() -> &'static SpecIndex`. No `SpecIndex` type exists; the cache has always held the parsed `serde_json::Value`. Implemented as `Backend::spec(self) -> Result<&'static Value>`: same effect, correct seam (parsing an embedded spec can fail, hence `Result`)
- `DrataClient::new` is `new(backend, api_key, region: Option<&str>, allow_writes)`; the doc does not spell the signature out. Every existing caller passes `Backend::Drata` and `Some(region)`
- `delete_with_body` is gone; `delete(path, body: Option<Value>)` covers both, per "delete() gains an optional body". Its one caller (`src/resources/vendor/security/agent.rs`) passes `Some(body)`, every other caller `None`. No shim
- `test_base_url_for_region` moved out of `src/client/tests.rs` with the function it tests: it is `drata_base_url_resolves_each_region` in `src/backend/tests.rs`, same assertions plus the `None` default
- `example_for_operation`, `example_for_schema` and `enum_values` stay Drata-only (they read `Backend::Drata.spec()`). Phase 3 owns `--example` from the SafeBase spec and adds the backend argument there. The 3.1 type-list handling in `build_skeleton` is tested directly against the SafeBase spec

### Tradeoffs
- A SafeBase `204 No Content` still maps to `Value::Null`, not an envelope error. The doc keeps "204 -> null unchanged" beside the envelope rule, SafeBase documents no 204, and a 204 has no body to check. See open questions
- `type: [T, "null"]` reads as the first non-null member, `["null"]` alone as null, over emitting `null` for every nullable field: a skeleton is a fill-in template, so it shows the usable type
- SafeBase client tests live in `tests/client.rs` beside the Drata ones over a new file: same seam, same helpers, well under the line cap

### Open questions
- Should a SafeBase 204 fail the envelope check (fail closed) over returning null? Phase 0 saw none
- The SafeBase spec has one op with no `operationId` (`PATCH /accounts/{id}`). Harmless here; Phase 3's coverage test must not assert non-empty ids for SafeBase

## Phase 2: Credentials, lazy clients, `drata safebase` skeleton

### Design decisions
- Saves always write the nested shape (`src/config.rs:Credentials::save`). A flat profile is read as a `drata` block and is never written back flat: one write shape, and the `.pre-safebase` backup covers rollback
- The shape the file was read in is a private `Shape` (`Empty | Legacy | Flat | Nested`) set by `Credentials::parse`; `save` copies the on-disk file to `credentials.json.pre-safebase` (0600, atomic) only when it is `Legacy` or `Flat` (`src/config.rs:back_up`). A read never saves; only the legacy single-key `migrated` path does, as before
- The legacy migration now writes nested, so it takes the backup too. `Credentials::load` marks the migrated set `Nested` after persisting it, so the login that follows in the same process does not overwrite the legacy backup with the nested file
- Profiles parse one by one from raw JSON (`ProfileOnDisk::into_profile`) so every error names its profile: flat + nested is an error, and so is a profile with neither (the doc's "a profile needs at least one")
- `Config` gains `backend` and `region: Option<String>` (`None` for SafeBase); `Config::load(cli, backend)` resolves only that backend's credential. `lib.rs:command_backend` maps `Commands::Safebase` to SafeBase and everything else to Drata; `main` resolves that one credential, so no command needs the other backend's key. `run` builds the client from `config.backend`
- `AuthDiagnostic` holds a `CredentialStatus` per backend (`drata`, `safebase`). The SafeBase status resolves from `SAFEBASE_API_KEY` and the profile only; `--api-key` / `--allow-writes` / `--region` feed the Drata status, as the doc says
- `whoami` / `auth` print the Drata lines first, byte for byte as before, then an indented `safebase:` block (`  key:` / `  source:` / `  writes:`), so the only unindented `writes:` line stays Drata's for vendor-eval (`src/resources/auth.rs:render_diagnostic`, pinned by `whoami_drata_lines_keep_their_wording_and_lead`)
- `--region` scoping is one helper, `config::check_region_scope`, called from `Config::load(.., Safebase)` and from `run_auth`, so `drata safebase login --region eu` fails before anything is written
- `config::remove_block` returns a `Removal` (`NotFound | Block | Profile`) and saves only when it removed something: a no-op logout does not rewrite (or back up) the file
- `WriteGuardError` carries `backend: Backend`; its first line reads `write blocked: the active {backend} credential is not write-enabled.` The Drata text is unchanged
- Global `--api-key`, `--region`, `--allow-writes` help lines now state their backend scope, since "Drata API key" stopped being true. The README and `after_help` stay Phase 7's
- The config test fixture `Env` clears the key env vars on drop, while it still holds the lock

### Deviations
- `run(cli, config, confirm)` keeps its signature; the lazy step sits in `main`, which calls `Config::load(&cli, command_backend(&cli.command))`. Same effect, correct seam: `tests/logging.rs` keeps driving `run` with a fixture `Config`
- `save_profile` / `remove_profile` are replaced by `save_drata`, `save_safebase` and `remove_block(name, backend)`, and `Profile` is now the two-block struct (the old flat fields live in `DrataBlock`). No shim
- `drata logout` output changes from `Removed profile "x".` to `Removed the Drata credential from profile "x".` (or `... and profile "x".` when it was the last block), because logout no longer drops the whole profile. `drata login` output is unchanged
- `tests/commands.rs` `UNCURATED` gains `safebase`: the command table proves Drata ops only. SafeBase coverage arrives with Phases 3 and 5
- Existing tests changed only for the new shapes: `Config` fixtures (`backend`, `region: Some(..)`), `diag.drata.*`, `test_remove_profile` asserting `Removal` values

### Tradeoffs
- Always writing nested vs leaving Drata-only profiles flat so an old binary keeps reading the file without an `mv`: one write shape is simpler to reason about, and the backup is the rollback the doc names
- An existing `.pre-safebase` is overwritten when a flat file is saved again (only reachable after a manual restore) vs refusing the save: the file about to be rewritten is the newest flat state, the one a rollback wants
- Unknown keys inside a block are ignored, as the old flat `Profile` did, vs `deny_unknown_fields`: an older binary keeps reading a file a newer one wrote
- `drata auth` (not `whoami`) prints SafeBase setup help whenever no SafeBase key is found, including for Drata-only users: discoverability over two extra lines

### Open questions
- `drata auth` now shows SafeBase setup help to every Drata-only user. Keep it, or show it only when the profile has a `safebase` block or `SAFEBASE_API_KEY` is set?
- The legacy single-key migration now also leaves a `.pre-safebase` copy (the legacy file). Fine, or should that path skip the backup?
