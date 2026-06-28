#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::VendorAction;

#[test]
fn vendor_name_reads_name() {
    let v = json!({"id": 1, "name": "Okta"});
    assert_eq!(vendor_name(&v), "Okta");
}

#[test]
fn vendor_name_defaults_empty() {
    let v = json!({"id": 1});
    assert_eq!(vendor_name(&v), "");
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({"name": "x"});
    set_opt(&mut body, "category", Some("SECURITY"));
    set_opt(&mut body, "risk", None);
    assert_eq!(body["category"], json!("SECURITY"));
    assert!(body.get("risk").is_none());
}

#[test]
fn example_only_for_create_with_flag() {
    let create_example = VendorAction::Create {
        name: None,
        category: None,
        risk: None,
        status: None,
        url: None,
        notes: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = VendorAction::Create {
        name: Some("x".to_string()),
        category: None,
        risk: None,
        status: None,
        url: None,
        notes: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = VendorAction::List {
        patterns: vec![],
        all: false,
        expand: vec![],
    };
    assert!(example_if_requested(&list).is_none());
}

#[test]
fn example_skeleton_is_valid_json() {
    let parsed: Value = serde_json::from_str(VENDOR_CREATE_EXAMPLE).unwrap();
    assert_eq!(parsed["name"], json!("Example Vendor"));
}
