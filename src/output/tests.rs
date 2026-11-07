#![allow(clippy::unwrap_used)]
use super::*;
use serde_json::json;

#[test]
fn test_print_value_json_format() {
    print_value(&json!({"key": "val"}), &OutputFormat::Json);
}

#[test]
fn test_print_value_table_format() {
    print_value(&json!({"key": "val"}), &OutputFormat::Table);
}

#[test]
fn test_print_value_table_known_shape() {
    print_value(
        &json!({"data": [{"id": 1, "name": "Okta", "status": "ACTIVE"}]}),
        &OutputFormat::Table,
    );
}

#[test]
fn pager_command_respects_env() {
    let (cmd, args) = pager_command_from(Some("bat --style=plain".to_string()));
    assert_eq!(cmd, "bat");
    assert_eq!(args, vec!["--style=plain"]);
}

#[test]
fn pager_command_defaults_to_less() {
    let (cmd, args) = pager_command_from(None);
    assert_eq!(cmd, "less");
    assert!(args.contains(&"-F".to_string()));
    assert!(args.contains(&"-X".to_string()));
}
