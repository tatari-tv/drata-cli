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
    // SAFETY: test-only mutation of process env.
    unsafe { std::env::set_var("PAGER", "bat --style=plain") };
    let (cmd, args) = pager_command();
    assert_eq!(cmd, "bat");
    assert_eq!(args, vec!["--style=plain"]);
    unsafe { std::env::remove_var("PAGER") };
}

#[test]
fn pager_command_defaults_to_less() {
    // SAFETY: test-only mutation of process env.
    unsafe { std::env::remove_var("PAGER") };
    let (cmd, args) = pager_command();
    assert_eq!(cmd, "less");
    assert!(args.contains(&"-F".to_string()));
    assert!(args.contains(&"-X".to_string()));
}
