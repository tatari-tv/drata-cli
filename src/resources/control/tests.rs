#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::ControlAction;

#[test]
fn example_only_for_create_with_flag() {
    let create_example = ControlAction::Create {
        workspace_id: Some("w1".to_string()),
        name: None,
        description: None,
        code: None,
        question: None,
        activity: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = ControlAction::Create {
        workspace_id: Some("w1".to_string()),
        name: Some("ctrl".to_string()),
        description: None,
        code: None,
        question: None,
        activity: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = ControlAction::List {
        workspace_id: "w1".to_string(),
        all: false,
        expand: vec![],
    };
    assert!(example_if_requested(&list).is_none());
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({});
    set_opt(&mut body, "name", Some("My Control"));
    set_opt(&mut body, "description", None);
    assert_eq!(body["name"], json!("My Control"));
    assert!(body.get("description").is_none());
}
