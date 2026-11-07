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

## Phase 3: `drata safebase raw`

### Design decisions
- `SafebaseAction::Raw(RawArgs)` reuses the existing `RawArgs` and `raw::handle`, no fork (`src/cli/safebase.rs`, `src/lib.rs:dispatch`). `handle` already takes the backend from `Config`/client, so the only change inside it is the spec-validation lookup, which now reads `config.backend` instead of `Backend::Drata` (`src/raw.rs:handle`). The confirm prompt and write guard are the same code paths as `drata raw`
- `--example` takes the backend from the caller: `raw::example_if_requested(backend, args)`; `lib.rs:example_if_requested` passes `Backend::Drata` for `Commands::Raw` and `Backend::Safebase` for `safebase raw`. It runs before config load, so no key is needed
- `spec::example_for_operation`, `example_for_schema`, `enum_values` gained a leading `backend: Backend` argument, as Phase 1 deferred. Every Drata caller passes `Backend::Drata` explicitly; no default shim
- The "no operation" error names the spec (`in the SafeBase spec`), so a Drata path typed under `safebase raw --example` says which spec was searched
- `tests/coverage.rs` gained a SafeBase test with `EXPECTED_SAFEBASE_OPERATIONS = 47`, `raw_reachable` accepts PATCH, and no `operationId` is required (`PATCH /accounts/{id}` has none). A second test pins the `POST /questionnaires` multipart field names (`qnr`, `accountId`, `productId`, `dueDate`, `notes`) to the committed spec
- Wiremock tests live in a new `tests/safebase.rs`, driving `raw::handle` from parsed argv

### Deviations
- `--example PATCH '/document/{id}'` prints `{}`, not a populated object. `build_skeleton` emits required fields (and fields with an example/default) only, and every `PATCH /document/{id}` field is optional. That is the shared Drata behavior and still satisfies "prints a JSON object, exit 0"; I did not change the skeleton rule for SafeBase. The skeleton tests use `POST /documents/upload`, which has required fields
- `raw` multipart stays POST and PUT only, as for `drata raw`: no SafeBase op takes a multipart PATCH, so none was added
- `run_auth` gets an unreachable `SafebaseAction::Raw(_)` error arm, because the `Commands::Safebase` match there is now non-exhaustive in spirit; `is_auth_command` already excludes it

### Tradeoffs
- Reusing `RawArgs` unchanged over a SafeBase-specific struct: identical flags are the requirement, so one struct keeps the two namespaces from drifting. The `--help` text for the flags still reads "under the API base", which is true for both
- A required-fields-only `{}` skeleton vs listing optional fields for SafeBase: changing it would fork `build_skeleton`'s contract for one backend

### Open questions
- Should `--example` skeletons include optional fields (commented or as nulls) so `PATCH /document/{id}` shows its fields? It applies to Drata too, so it is a separate change

## Phase 4: Page-number pagination

### Design decisions
- One descriptor type, `PagedOp`, with three consts `ACCOUNTS`, `DOCUMENTS`, `KB_SEARCH` (`src/client/pages.rs`). Fields: path, page param + first page (1 or 0), size param + page size (1000 / 100 / 1000, Phase 0's maximums), JSON pointer to the items, `Stop::TotalPages(ptr)` or `Stop::Count(ptr)`, `max_pages` and a `cap_source` string for the error
- Every page is validated by a pure `PagedOp::check` before any of its items reach the sink, so the rules are unit-testable without a server and a bad page under NDJSON streaming writes none of its own items; earlier pages' lines stay written
- Two entries mirror the cursor path: `DrataClient::get_all_pages(op, path)` (collect, the list default) and `stream_all_pages(op, path, writer)` (NDJSON, the `--all` mode), both over one loop, `drain_numbered_pages`. Phase 5 calls these the way `vendor list` calls `get_all` / `stream_all`
- Errors are a typed `PageError` enum (`BadField`, `EmptyPage`, `RepeatedPage`, `CountExceeded`, `PastLastPage`, `PageCap`). Each message names the path and the request's `param=value` (`pageNumber=2`, `page=1`), so zero-based `/documents` reports the number actually sent. Tests downcast to the variant as well as checking the text
- A request failure (HTTP error, envelope error) mid-listing is wrapped with `<path> failed at <param>=<n> after <k> items`; the inner `ApiError` / `EnvelopeError` still downcasts
- `/accounts` checks `totalPages` against its 500 cap on every page, so `totalPages: 501` fails on page 1 in one request instead of after 500 pages of work. The loop also refuses to request page index `>= max_pages`, which is the hard cap for `/documents` (count-based, no total pages to check up front) and `/kb/search`. `DOCUMENTS` and `KB_SEARCH` use `client.rs`'s `MAX_PAGES` (child module reads the private const)
- `totalPages: 0` or `count: 0` with an empty first page is empty success. Items on a page past `totalPages` (including `totalPages: 0` with items) is `PastLastPage`; items past `count` is `CountExceeded`
- The loop refuses a non-SafeBase client and a `path` whose route differs from `op.path`: a descriptor applied to the wrong op fails loudly
- Pages are paced with the client's existing `page_interval` (150 ms), same as the cursor loop
- Break-to-prove: each guard (spec-bound cap, loop cap, off-by-one on the cap, empty page, repeated page, count exceeded, past last page, bad stop field, page label on a mid-stream failure) was disabled once and its test failed

### Deviations
- The doc's "page 501" rule is enforced from the server's reported `totalPages` (> 500 fails on the first page) and by the loop cap; no request for page 501 is ever sent. Same effect, earlier
- No `limit` variant (Drata has `get_all_limited`): no Phase 5 SafeBase verb takes `--limit`
- No CLI commands in this phase, per the phase scope; the `--all` NDJSON mode is the `stream_all_pages` entry Phase 5 wires up

### Tradeoffs
- Pacing SafeBase pages at 150 ms vs no pacing: SafeBase documents no limit and Phase 0 saw no rate-limit headers, but pacing cannot trip an unknown limit and costs little next to request latency. Tests set the interval to zero
- Comparing a page's items to the previous page's (whole-`Value` equality) vs tracking every seen item id: matches the doc's rule exactly, needs no per-op id field, and a server echoing an older page still ends at the stop rule or the cap
- Hard cap tested with a copy of `DOCUMENTS` capped at 2 pages, not 10,000 wiremock requests; a separate test pins `DOCUMENTS` / `KB_SEARCH` to `MAX_PAGES` and `ACCOUNTS` to 500

### Open questions
- None.
