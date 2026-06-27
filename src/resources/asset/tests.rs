#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{AssetAction, AssetType};

#[test]
fn example_only_for_create_with_flag() {
    let create_example = AssetAction::Create {
        name: None,
        description: None,
        asset_type: None,
        notes: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = AssetAction::Create {
        name: Some("a".to_string()),
        description: None,
        asset_type: None,
        notes: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = AssetAction::List;
    assert!(example_if_requested(&list).is_none());
}

#[test]
fn asset_type_str_roundtrips() {
    assert_eq!(asset_type_str(&AssetType::Physical), "PHYSICAL");
    assert_eq!(asset_type_str(&AssetType::Virtual), "VIRTUAL");
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({});
    set_opt(&mut body, "name", Some("Laptop"));
    set_opt(&mut body, "notes", None);
    assert_eq!(body["name"], json!("Laptop"));
    assert!(body.get("notes").is_none());
}
