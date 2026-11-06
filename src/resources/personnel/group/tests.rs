#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn list_path_filters_by_name_substring() {
    assert_eq!(list_path(None).unwrap(), "/groups");
    assert_eq!(list_path(Some("IT Ops")).unwrap(), "/groups?q=IT%20Ops");
}

#[test]
fn list_path_rejects_a_name_over_191_chars() {
    assert!(list_path(Some(&"x".repeat(191))).is_ok());
    let err = list_path(Some(&"x".repeat(192))).unwrap_err().to_string();
    assert!(err.contains("--name"), "{err}");
}
