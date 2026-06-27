#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{PolicyAction, PolicySourceType};

#[test]
fn example_only_for_create_with_flag() {
    let create_example = PolicyAction::Create {
        name: None,
        owner_id: None,
        source_type: None,
        file: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = PolicyAction::Create {
        name: Some("p".to_string()),
        owner_id: None,
        source_type: None,
        file: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = PolicyAction::List {
        all: false,
        expand: vec![],
    };
    assert!(example_if_requested(&list).is_none());
}

#[test]
fn source_type_str_roundtrips() {
    assert_eq!(source_type_str(&PolicySourceType::Uploaded), "UPLOADED");
    assert_eq!(source_type_str(&PolicySourceType::External), "EXTERNAL");
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({});
    set_opt(&mut body, "name", Some("Pol"));
    set_opt(&mut body, "description", None);
    assert_eq!(body["name"], json!("Pol"));
    assert!(body.get("description").is_none());
}
