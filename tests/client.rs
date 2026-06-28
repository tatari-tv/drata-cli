//! Wiremock integration tests for `DrataClient`: auth header, cursor
//! pagination + hardening, 204 handling, error parsing, and the write guardrail.

use drata_cli::client::{ApiError, DrataClient, WriteGuardError};
use reqwest::StatusCode;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn ro_client(server: &MockServer) -> DrataClient {
    DrataClient::new("test-key".to_string(), "us", false)
        .expect("client builds")
        .with_base_url(server.uri())
}

async fn rw_client(server: &MockServer) -> DrataClient {
    DrataClient::new("test-key".to_string(), "us", true)
        .expect("client builds")
        .with_base_url(server.uri())
}

#[tokio::test]
async fn get_sends_bearer_auth() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/vendors/1"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": 1})))
        .expect(1)
        .mount(&server)
        .await;

    let client = ro_client(&server).await;
    let resp = client.get("/vendors/1").await.expect("get ok");
    assert_eq!(resp["id"], 1);
}

#[tokio::test]
async fn try_get_returns_none_on_404() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/vendors/999"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({"message": "not found"})))
        .mount(&server)
        .await;

    let client = ro_client(&server).await;
    let resp = client.try_get("/vendors/999").await.expect("try_get ok");
    assert!(resp.is_none());
}

#[tokio::test]
async fn error_response_parses_message_and_details() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/vendors/1"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({"message": "Bad", "errors": ["name required"]})))
        .mount(&server)
        .await;

    let client = ro_client(&server).await;
    let err = client.get("/vendors/1").await.expect_err("expected error");
    let api = err.downcast_ref::<ApiError>().expect("ApiError");
    assert_eq!(api.status, StatusCode::BAD_REQUEST);
    assert!(api.formatted.contains("Bad"));
    assert!(api.formatted.contains("name required"));
}

#[tokio::test]
async fn delete_204_returns_null() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/vendors/1"))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = rw_client(&server).await;
    let result = client.delete("/vendors/1").await.expect("delete ok");
    assert_eq!(result, serde_json::Value::Null);
}

#[tokio::test]
async fn cursor_pagination_drains_all_pages() {
    let server = MockServer::start().await;
    // Page 1: no cursor param, returns a non-null cursor.
    Mock::given(method("GET"))
        .and(path("/vendors"))
        .and(query_param("size", "50"))
        .and(query_param("cursor", "abc"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{"id": 3, "name": "C"}],
            "pagination": {"cursor": null}
        })))
        .mount(&server)
        .await;
    // First call has no cursor; respond with cursor=abc to fetch page 2.
    Mock::given(method("GET"))
        .and(path("/vendors"))
        .and(query_param("size", "50"))
        .and(wiremock::matchers::query_param_is_missing("cursor"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{"id": 1, "name": "A"}, {"id": 2, "name": "B"}],
            "pagination": {"cursor": "abc"}
        })))
        .mount(&server)
        .await;

    let client = ro_client(&server).await;
    let all = client.get_all("/vendors").await.expect("pagination ok");
    assert_eq!(all.len(), 3, "should drain both pages");
    assert_eq!(all[0]["name"], "A");
    assert_eq!(all[2]["name"], "C");
}

#[tokio::test]
async fn cursor_pagination_aborts_on_repeated_cursor() {
    let server = MockServer::start().await;
    // Every page returns the same non-null cursor: no forward progress. The
    // hardening must now return Err (fail-closed) instead of partial data.
    Mock::given(method("GET"))
        .and(path("/vendors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{"id": 1, "name": "A"}],
            "pagination": {"cursor": "stuck"}
        })))
        .mount(&server)
        .await;

    let client = ro_client(&server).await;
    let result = client.get_all("/vendors").await;
    assert!(result.is_err(), "repeated cursor must return Err, not partial data");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("repeated cursor"),
        "error should describe the problem: {msg}"
    );
}

#[tokio::test]
async fn write_guard_blocks_non_get_on_readonly_client() {
    let server = MockServer::start().await;
    // No mock mounted: the request must never reach the server.
    let client = ro_client(&server).await;
    let err = client
        .post("/vendors", json!({"name": "x"}))
        .await
        .expect_err("write must be blocked");
    assert!(
        err.downcast_ref::<WriteGuardError>().is_some(),
        "expected WriteGuardError, got: {err}"
    );
}

#[tokio::test]
async fn write_allowed_on_write_enabled_client() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/vendors"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"id": 5, "name": "x"})))
        .expect(1)
        .mount(&server)
        .await;

    let client = rw_client(&server).await;
    let resp = client.post("/vendors", json!({"name": "x"})).await.expect("write ok");
    assert_eq!(resp["id"], 5);
}
