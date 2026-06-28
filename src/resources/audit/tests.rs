#[cfg(test)]
use super::*;

#[test]
fn audit_actions_are_constructible() {
    let list = AuditAction::List {
        workspace_id: "ws1".to_string(),
        all: false,
    };
    assert!(matches!(list, AuditAction::List { .. }));

    let get = AuditAction::Get {
        workspace_id: "ws1".to_string(),
        audit_id: "a1".to_string(),
    };
    assert!(matches!(get, AuditAction::Get { .. }));
}
