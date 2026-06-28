#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::FrameworkAction;

#[test]
fn example_only_for_create_with_flag() {
    let create_example = FrameworkAction::Create {
        workspace_id: Some("w1".to_string()),
        name: None,
        short_name: None,
        description: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = FrameworkAction::Create {
        workspace_id: Some("w1".to_string()),
        name: Some("SOC2".to_string()),
        short_name: None,
        description: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = FrameworkAction::List {
        workspace_id: "w1".to_string(),
    };
    assert!(example_if_requested(&list).is_none());
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({});
    set_opt(&mut body, "name", Some("SOC 2"));
    set_opt(&mut body, "shortName", None);
    assert_eq!(body["name"], json!("SOC 2"));
    assert!(body.get("shortName").is_none());
}
