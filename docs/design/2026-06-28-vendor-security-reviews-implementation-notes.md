## Phase 1: clap surface + dispatch wiring

### Design decisions
- Removed explicit `VendorSecurityReviewAction` import from `vendor.rs` - the type is only needed by `security::handle` which imports it from `crate::cli` directly; `vendor.rs` dispatches via the enum arm without naming the inner type - `src/resources/vendor.rs` - avoids a clippy `-D unused-imports` error while keeping dispatch ergonomic
- Added serialization helper fns (`status_str`, `type_str`, `action_str`) in `security.rs` Phase 1 rather than waiting for Phase 2 - `src/resources/vendor/security.rs` - they are needed by the stub debug log entries, and pinning them in tests now validates the spec mapping before any HTTP code lands
- Stubs use `eyre::bail!` with a "not yet implemented (Phase 2)" message rather than `todo!()` - `src/resources/vendor/security.rs:handle` - keeps the binary runnable and gives a clear error to anyone who accidentally exercises the stub path; `todo!()` would panic with a less useful message

### Deviations
- Design doc's clap snippet shows `Get { vendor_id, security_review_id, ... }` with fields inline and no doc comments; added doc comments (`/// Vendor ID`, `/// Security review ID`) to all positional args for consistency with the existing `VendorQuestionnaireAction` style in the same file.
- `mod security;` placed after `#[cfg(test)] mod tests;` at the bottom of `vendor.rs` (not at the top with the `use` imports) to follow the existing module's own pattern where `mod tests` is at the bottom; `security` is a sibling submodule so same placement applies.

### Tradeoffs
- Stubs bail with an error vs. returning `Ok(())` silently - chosen because a silent no-op would be confusing in CI or interactive use; an error makes the incomplete state obvious
- Test file location `src/resources/vendor/security/tests.rs` (subdir of security) vs. inline `#[cfg(test)] mod tests` block - subdir chosen per repo rules (`rust.md`: "Tests live in their own files, NEVER as `#[cfg(test)] mod tests { ... }` blocks at the bottom of a source file")

### Open questions
- None.

## Phase 3: Multipart handlers

### Design decisions
- Factored each multipart body into a pure builder fn (`build_create_with_file_form`, `build_files_form`) that returns a `Multipart` with no I/O - `src/resources/vendor/security.rs` - lets the unit tests assert file-part name/count and scalar field shapes without wiremock or a live server (the design's "pure unit, no wiremock" testing strategy); the async handlers are then a thin confirm-gate + build + `post_multipart` + print.
- `create-with-file` stringifies `requester_user_id: Option<u64>` via `id.to_string()` for the multipart text field (`add_opt_field` takes `Option<impl Into<String>>`) - `src/resources/vendor/security.rs:build_create_with_file_form` - multipart fields are text; the JSON `create` path keeps it numeric, this path serializes it as a decimal string.
- `--status`/`--type` translate to the same long body keys (`securityReviewStatus`/`securityReviewType`) in the multipart form as in JSON `create`, matching the create DTO; a unit test asserts the flag names never leak through as field names.
- Q1b 415 path implemented as `map_with_file_415`, applied only to `create-with-file` via `.map_err(...)` - `src/resources/vendor/security.rs` - it `downcast_ref::<ApiError>()`s and rewrites only `415 Unsupported Media Type` into an actionable message naming the endpoint and the `drata raw POST <path> --data '<json>'` fallback; all other statuses (and non-`ApiError` errors) pass through unchanged (asserted by a 400-passthrough test). This reuses the existing typed `ApiError.status` rather than string-matching the error text (per rust.md "typed values at seams").
- Reused the existing confirm gate (`confirm("POST", &path)?`) and the client write guardrail (fail-closed in `send_multipart` without `--allow-writes`) for all three new verbs - no new credential handling.
- Per logging.md, each handler emits a `debug!` entry log (fn + params, file paths and counts only - never file contents or the key) and a `debug!` exit log on success; file size/contents are never logged here (the client logs byte lengths at TRACE in `send_multipart`).

### Deviations
- None. The three verbs, their required/optional sets, and the multipart field names match the design's API Design table and Phase 3 scope exactly.

### Tradeoffs
- Pure builder fns + thin async handlers vs. building the `Multipart` inline in each handler - chose the split so the part shapes are unit-testable (the only way to assert field/file-part names without a wire test); marginal extra indirection, but it is the design's stated testing strategy.
- `map_with_file_415` lives in `security.rs` (verb-specific) rather than in the client - chosen because the "fall back to `raw --data <json>`" guidance is specific to the `create-with-file` JSON-vs-multipart ambiguity (Q1b), not a generic client concern; the questionnaire endpoints are confirmed multipart (Q1a) and do not get this treatment.

### Open questions
- Q1b remains UNVERIFIED LIVE. `create-with-file` (POST /vendors/{id}/security-reviews/with-file) is sent as `multipart/form-data` with a single `file` part per the design's decision. The available Drata credential is read-only, so this mutating path could not be probed against the real endpoint in this phase; if the endpoint actually requires `application/json` with a base64 `file` field, the request returns `415` and the new `map_with_file_415` error path points the user at the `raw` JSON fallback. This should be confirmed with a live write key when one exists (it is the least-critical verb: `create` + a separate upload achieves the same result).

## Phase 2: Read + JSON-create handlers

### Design decisions
- Added `--example` flag to `Update` variant in `src/cli.rs` and a corresponding `SECURITY_REVIEW_UPDATE_EXAMPLE` constant in `security.rs` - the design doc's API Design table says "For JSON-body verbs that have many optional fields (`create`, `update`), follow the existing pattern: ... `--example` to print a skeleton." The Phase 1 stub did not include `example` on `Update`, so it was added here along with dispatch handling in `example_if_requested`.
- Implemented `set_opt_str` as a `pub(crate)` helper in `security.rs` rather than importing `vendor::set_opt` - `src/resources/vendor/security.rs:set_opt_str` - the existing `set_opt` in `vendor.rs` is private (`fn set_opt`) and not accessible from the sibling `security` module; a local helper keeps the module self-contained and mirrors the pattern each resource module follows.
- Filter query params (`status`, `type`) are appended manually before `append_expand` rather than using a query builder - `src/resources/vendor/security.rs:list` - matches the existing inline `format!` pattern used by `expand.rs` and other handlers; no query-builder crate is in the dependency tree.
- `list --all` returns `stream_all`'s `total` count at DEBUG exit; `list` (buffered) logs the item count - follows the pattern in `vendor::list` where the exit outcome is count/status at DEBUG per `logging.md`.
- Body for `run-action` is `{"action": action_str(action)}` built inline, not via `set_opt_str`, because the key is always present (not optional) - `src/resources/vendor/security.rs:run_action`.
- `questionnaires` uses `get_all` (cursor-paginated) rather than a bare `get` - the path (`/security-reviews/{srId}/security-questionnaires`) is a list endpoint matching the same Drata v2 cursor-paginated shape as other list endpoints; `get_all` is the correct choice.

### Deviations
- `Update` gained an `--example` flag that was not present in Phase 1's stub; this is additive and spec-conformant (the API Design table explicitly includes `update` in the "follow the existing pattern ... --example" sentence). The dispatch in `handle` and `example_if_requested` was updated accordingly.

### Tradeoffs
- `set_opt_str` duplicated locally vs. re-export from `vendor` - chosen local because `vendor::set_opt` is private; a re-export or visibility change would widen scope beyond what's needed. The helper is tiny (3 lines) and the pattern is identical in each resource module.
- Manual query param construction vs. a URL builder - manual chosen to match existing codebase style; a builder crate would be an unneeded dependency for this use case.
- `questionnaires` buffers all items vs. streaming - buffering chosen because: (a) the parent function returns `json!({ "data": items })` matching every other list handler; (b) security-review questionnaire counts are small in practice (per design doc); (c) `--all` streaming is only applied where the design doc explicitly calls it out (the `list` verb).

### Open questions
- None.

## Phase 4: Output + polish

### Design decisions
- `render_security_reviews` sniffs on `reviewDeadlineAt` - `src/output/table.rs:pick_renderer` - this field is present on every security-review response DTO (`VendorSecurityReviewResponsePublicV2Dto` and compact variant) and does not appear in any other Drata response shape in the spec; it is therefore the minimal, unambiguous discriminator. Using `status`+`type` alone would risk false-positive matches against other resources that carry `status` (policies, vendors, devices all do); `reviewDeadlineAt` is unique.
- Table columns chosen as `ID | TITLE | STATUS | TYPE | DECISION | DEADLINE` - `src/output/table.rs:render_security_reviews` - covers the most operator-relevant fields at a glance: identity (id, title), current state (status, decision), resource type (type), and timeline (deadline). Fields `note`, `userId`, `requesterUserId`, `socReviewForm`, and `_links` are detail fields better served by JSON output.
- Module doc comment on `security.rs` expanded to a full verb-map table and enum-serialization reference section - `src/resources/vendor/security.rs` - the per-phase implementation notes that the prior comment contained (Phase 2/3 references) were replaced with a stable, reader-facing API map that will remain accurate post-implementation.
- Three renderer tests added: list shape (two rows, all columns), single-get shape (one-row wrap), and null-field behavior (nullable `title`/`decision` must render as empty string not the word "null") - `src/output/table/tests.rs` - the null test protects against a regression if `str_field` behavior changes; JSON null is not the same as an absent key and both already return empty via `as_str()` returning `None`.
- `docs/ts-vs-rust-comparison.md` section 2.5 rewritten from "gap" to "closed" with the full verb list and coverage table row updated (TS's 5 verbs vs. Rust's 10-verb superset); section 5 recommendation 1 struck through and marked Done.

### Deviations
- None. All Phase 4 deliverables (renderer, sniff branch, module doc, tests, ts-vs-rust update, design doc status flip, notes append) match the design doc's Phase 4 scope.

### Tradeoffs
- `reviewDeadlineAt` as sole sniff key vs. `reviewDeadlineAt` + `decision` combined - single key chosen because the compound check adds no discrimination benefit (both are unique to the security-review DTO); simpler sniff, same correctness.
- Six columns (ID/TITLE/STATUS/TYPE/DECISION/DEADLINE) vs. a minimal three-column (ID/STATUS/DEADLINE) - six chosen to match the depth of the existing vendor renderer (6 cols) and to give operators enough context to act without running a `get`; the shrink-to-width engine handles narrow terminals.

### Open questions
- None.
