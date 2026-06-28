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
