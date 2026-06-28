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
