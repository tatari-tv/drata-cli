#![allow(clippy::unwrap_used)]
use super::*;
use reqwest::StatusCode;
use serde_json::json;

fn rows_of(codes: &[u64]) -> Value {
    json!({ "data": codes.iter().map(|c| json!({"statusCode": c, "outcome": "X", "data": {}})).collect::<Vec<_>>() })
}

#[test]
fn status_code_failed_is_400_and_above() {
    assert!(!status_code_failed(&json!({"statusCode": 201})));
    assert!(!status_code_failed(&json!({"statusCode": 200})));
    assert!(status_code_failed(&json!({"statusCode": 400})));
    assert!(status_code_failed(&json!({"statusCode": 409})));
    assert!(status_code_failed(&json!({"statusCode": 500})));
}

#[test]
fn a_row_without_a_status_code_counts_as_failed() {
    assert!(status_code_failed(&json!({"outcome": "EXCLUDED"})));
}

#[test]
fn check_passes_when_every_row_succeeded() {
    assert!(check(&rows_of(&[201, 201]), status_code_failed, None).is_ok());
}

#[test]
fn check_fails_and_counts_failed_rows() {
    let err = check(&rows_of(&[201, 422, 409]), status_code_failed, None).unwrap_err();
    assert_eq!(err.to_string(), "2 of 3 items failed");
}

#[test]
fn check_fails_on_a_non_2xx_even_with_no_failed_row() {
    let err = check(&rows_of(&[201]), status_code_failed, Some(500)).unwrap_err();
    assert_eq!(err.to_string(), "HTTP 500: 0 of 1 items failed");
}

#[test]
fn a_body_without_rows_is_an_error() {
    let err = check(&json!({"id": 1}), status_code_failed, None).unwrap_err();
    assert!(err.to_string().contains("expected per-item results"), "{err}");
}

#[test]
fn error_rows_are_read_from_an_api_error_body() {
    let body = rows_of(&[422]).to_string();
    let err: eyre::Report = ApiError {
        status: StatusCode::UNPROCESSABLE_ENTITY,
        formatted: "API error 422".to_string(),
        body,
    }
    .into();
    let (status, value) = rows_of_error(&err).expect("rows");
    assert_eq!(status, 422);
    assert_eq!(rows(&value).unwrap().len(), 1);
}

#[test]
fn a_plain_api_error_has_no_rows() {
    let err: eyre::Report = ApiError {
        status: StatusCode::BAD_REQUEST,
        formatted: "API error 400".to_string(),
        body: json!({"message": "codebase test"}).to_string(),
    }
    .into();
    assert!(rows_of_error(&err).is_none());
}
