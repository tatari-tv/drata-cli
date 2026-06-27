#![allow(clippy::unwrap_used)]
use crate::cli::WorkspaceAction;

// Workspace module only has a `list` action.
#[test]
fn workspace_action_list_compiles() {
    let action = WorkspaceAction::List;
    // Use the value to avoid an unused-variable warning.
    assert!(matches!(action, WorkspaceAction::List));
}
