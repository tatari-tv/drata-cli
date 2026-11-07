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
