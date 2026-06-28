#[cfg(test)]
use super::*;

#[test]
fn example_not_requested_for_list() {
    let action = RegisterAction::List;
    assert!(example_if_requested(&action).is_none());
}

#[test]
fn example_not_requested_for_get() {
    let action = RegisterAction::Get {
        register_id: "1".to_string(),
    };
    assert!(example_if_requested(&action).is_none());
}

#[test]
fn example_requested_for_create_with_example_flag() {
    let action = RegisterAction::Create {
        name: None,
        description: None,
        owner_ids: vec![],
        workspace_ids: vec![],
        example: true,
    };
    let result = example_if_requested(&action);
    assert!(result.is_some(), "expected Some for create --example");
    let inner = result.expect("Some").expect("skeleton should not error");
    assert!(!inner.is_empty(), "skeleton should not be empty");
}

#[test]
fn example_not_requested_when_example_false() {
    let action = RegisterAction::Create {
        name: Some("test".to_string()),
        description: None,
        owner_ids: vec![],
        workspace_ids: vec![],
        example: false,
    };
    assert!(example_if_requested(&action).is_none());
}
