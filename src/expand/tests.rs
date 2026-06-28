#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn no_expand_returns_path_unchanged() {
    assert_eq!(append_expand("/assets", &[]), "/assets");
    assert_eq!(append_expand("/assets?foo=bar", &[]), "/assets?foo=bar");
}

#[test]
fn single_expand_uses_question_mark() {
    let result = append_expand("/assets", &["asset".to_string()]);
    assert!(result.contains("expand%5B%5D=asset"), "got: {result}");
    assert!(result.starts_with("/assets?"));
}

#[test]
fn multiple_expand_uses_ampersands() {
    let result = append_expand("/assets", &["asset".to_string(), "complianceChecks".to_string()]);
    assert!(result.contains("expand%5B%5D=asset"), "got: {result}");
    assert!(result.contains("expand%5B%5D=complianceChecks"), "got: {result}");
    // Two expand params: only one ? and one &
    assert_eq!(result.matches('?').count(), 1);
    assert_eq!(result.matches('&').count(), 1);
}

#[test]
fn existing_query_uses_ampersand_separator() {
    let result = append_expand("/assets?size=50", &["asset".to_string()]);
    assert!(result.starts_with("/assets?size=50&"), "got: {result}");
    assert!(result.contains("expand%5B%5D=asset"), "got: {result}");
}

#[test]
fn expand_value_is_percent_encoded() {
    // A value containing special chars must be encoded.
    let result = append_expand("/assets", &["a b&c".to_string()]);
    assert!(!result.contains("a b&c"), "raw special chars must be encoded: {result}");
    assert!(result.contains("a%20b"), "got: {result}");
}
