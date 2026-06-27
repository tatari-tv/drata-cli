#![allow(clippy::unwrap_used)]
use crate::cli::CompanyAction;

// Company module only has a `get` action; no helpers to test directly.
// Verify the action enum compiles with its single variant.
#[test]
fn company_action_get_compiles() {
    let action = CompanyAction::Get;
    // Use the value to avoid an unused-variable warning.
    assert!(matches!(action, CompanyAction::Get));
}
