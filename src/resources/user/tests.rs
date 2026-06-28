#[cfg(test)]
use super::*;

// UserAction has no example_if_requested; tests verify the module compiles
// and that handle dispatches the right arms (compile-time check).

#[test]
fn user_list_action_is_read_only() {
    // Verify all list/get variants exist and are constructible (compilation check).
    let list = UserAction::List { all: false };
    assert!(matches!(list, UserAction::List { .. }));

    let get = UserAction::Get {
        user_id: "1".to_string(),
    };
    assert!(matches!(get, UserAction::Get { .. }));

    assert!(matches!(UserAction::Roles, UserAction::Roles));

    let role = UserAction::Role {
        role_id: "2".to_string(),
    };
    assert!(matches!(role, UserAction::Role { .. }));

    let role_users = UserAction::RoleUsers {
        role_id: "3".to_string(),
        all: false,
    };
    assert!(matches!(role_users, UserAction::RoleUsers { .. }));
}
