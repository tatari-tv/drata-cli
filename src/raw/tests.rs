#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn build_path_no_query_is_unchanged() {
    assert_eq!(build_path("/vendors", &[]).unwrap(), "/vendors");
}

#[test]
fn build_path_appends_query_with_question_mark() {
    let q = vec!["status=ACTIVE".to_string()];
    assert_eq!(build_path("/vendors", &q).unwrap(), "/vendors?status=ACTIVE");
}

#[test]
fn build_path_joins_multiple_with_ampersand() {
    let q = vec!["a=1".to_string(), "b=2".to_string()];
    assert_eq!(build_path("/v", &q).unwrap(), "/v?a=1&b=2");
}

#[test]
fn build_path_uses_ampersand_when_path_has_query() {
    let q = vec!["b=2".to_string()];
    assert_eq!(build_path("/v?a=1", &q).unwrap(), "/v?a=1&b=2");
}

#[test]
fn build_path_percent_encodes_values() {
    let q = vec!["name=a b&c".to_string()];
    // space -> %20, & -> %26 (the value's & must not be read as a separator).
    assert_eq!(build_path("/v", &q).unwrap(), "/v?name=a%20b%26c");
}

#[test]
fn build_path_rejects_entry_without_equals() {
    let q = vec!["bogus".to_string()];
    let err = build_path("/v", &q).unwrap_err();
    assert!(err.to_string().contains("expected key=value"));
}

#[test]
fn example_skeleton_known_op_returns_json() {
    let out = example_skeleton("post", "/vendors").unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(parsed.get("name").is_some());
}

#[test]
fn example_skeleton_unknown_op_errors() {
    let err = example_skeleton("post", "/no/such/path").unwrap_err();
    assert!(err.to_string().contains("no operation"));
}

#[test]
fn example_skeleton_get_has_no_body() {
    let err = example_skeleton("get", "/vendors").unwrap_err();
    assert!(err.to_string().contains("no JSON request body"));
}

#[test]
fn example_if_requested_none_without_flag() {
    let args = RawArgs {
        method: "post".into(),
        path: "/vendors".into(),
        query: vec![],
        data: None,
        file: vec![],
        file_field: None,
        field: vec![],
        example: false,
    };
    assert!(example_if_requested(&args).is_none());
}

#[test]
fn example_if_requested_some_with_flag() {
    let args = RawArgs {
        method: "post".into(),
        path: "/vendors".into(),
        query: vec![],
        data: None,
        file: vec![],
        file_field: None,
        field: vec![],
        example: true,
    };
    let out = example_if_requested(&args).unwrap().unwrap();
    assert!(out.contains("name"));
}
