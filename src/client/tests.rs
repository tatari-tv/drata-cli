#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn test_new_succeeds() {
    let client = DrataClient::new("test-key".to_string(), "us", false);
    assert!(client.is_ok());
}

#[test]
fn test_with_base_url() {
    let client = DrataClient::new("test-key".to_string(), "us", false)
        .unwrap()
        .with_base_url("https://custom.example.com".to_string());
    assert_eq!(client.base_url, "https://custom.example.com");
}

#[test]
fn test_base_url_for_region() {
    assert_eq!(base_url_for_region("us"), US_BASE_URL);
    assert_eq!(base_url_for_region("eu"), EU_BASE_URL);
    assert_eq!(base_url_for_region("EU"), EU_BASE_URL);
    assert_eq!(base_url_for_region("apac"), APAC_BASE_URL);
    // Unknown region falls back to US.
    assert_eq!(base_url_for_region("mars"), US_BASE_URL);
}

#[test]
fn test_allow_writes_reflects_construction() {
    let ro = DrataClient::new("k".to_string(), "us", false).unwrap();
    assert!(!ro.allow_writes());
    let rw = DrataClient::new("k".to_string(), "us", true).unwrap();
    assert!(rw.allow_writes());
}

#[test]
fn test_encode_query_escapes_reserved() {
    assert_eq!(encode_query("a b"), "a%20b");
    assert_eq!(encode_query("a&b=c"), "a%26b%3Dc");
    // Plain ASCII passes through untouched.
    assert_eq!(encode_query("simple"), "simple");
}

#[test]
fn test_format_api_error_structured() {
    let body = r#"{"message":"Invalid Input","errors":["name is required"]}"#;
    let url = "https://public-api.drata.com/public/v2/vendors";
    let msg = format_api_error(&Method::POST, url, StatusCode::BAD_REQUEST, body);
    assert!(msg.contains("400"));
    assert!(msg.contains("Invalid Input"));
    assert!(msg.contains("name is required"));
    // The failing request is named so the user knows what broke.
    assert!(msg.contains("POST"));
    assert!(msg.contains(url));
}

#[test]
fn test_format_api_error_nested_error_message() {
    let body = r#"{"error":{"message":"forbidden"}}"#;
    let msg = format_api_error(
        &Method::GET,
        "https://public-api.drata.com/public/v2/vendors/1",
        StatusCode::FORBIDDEN,
        body,
    );
    assert!(msg.contains("forbidden"));
    assert!(!msg.contains("Unknown error"));
}

#[test]
fn test_format_api_error_plain_text() {
    let msg = format_api_error(
        &Method::GET,
        "https://public-api.drata.com/public/v2/vendors/999",
        StatusCode::NOT_FOUND,
        "not found",
    );
    assert!(msg.contains("404"));
    assert!(msg.contains("not found"));
}

/// Valid JSON whose error shape we don't recognize: the raw body MUST survive
/// instead of being replaced with "Unknown error", and the URL must appear.
#[test]
fn test_format_api_error_no_message_surfaces_body_and_url() {
    let body = r#"{"unexpected":[{"field":"name","detail":"too long"}]}"#;
    let url = "https://public-api.drata.com/public/v2/vendors";
    let msg = format_api_error(&Method::POST, url, StatusCode::BAD_REQUEST, body);
    assert!(!msg.contains("Unknown error"), "body must not be hidden: {msg}");
    assert!(msg.contains("too long"), "raw body must survive: {msg}");
    assert!(msg.contains(url));
    assert!(msg.contains("POST"));
}
