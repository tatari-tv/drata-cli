//! Wiremock integration tests for `DrataClient`: auth header, cursor
//! pagination + hardening, 204 handling, error parsing, the write guardrail,
//! and the SafeBase backend (auth header, envelope, redirect policy).

use drata_cli::backend::Backend;
use drata_cli::client::{ApiError, DrataClient, EnvelopeError, Multipart, WriteGuardError};
use reqwest::StatusCode;
use serde_json::json;
use wiremock::matchers::{any, body_json, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn ro_client(server: &MockServer) -> DrataClient {
    DrataClient::new(Backend::Drata, "test-key".to_string(), Some("us"), false)
        .expect("client builds")
        .with_base_url(server.uri())
}

async fn rw_client(server: &MockServer) -> DrataClient {
    DrataClient::new(Backend::Drata, "test-key".to_string(), Some("us"), true)
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
    let result = client.delete("/vendors/1", None).await.expect("delete ok");
    assert_eq!(result, serde_json::Value::Null);
}

#[tokio::test]
async fn cursor_pagination_drains_all_pages() {
    let server = MockServer::start().await;
    // Page 1: no cursor param, returns a non-null cursor.
    Mock::given(method("GET"))
        .and(path("/vendors"))
        .and(query_param("size", "500"))
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
        .and(query_param("size", "500"))
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

#[tokio::test]
async fn delete_sends_an_optional_body() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/vendors/1/security-reviews/2/assessment-runs/current"))
        .and(body_json(json!({})))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;

    let client = rw_client(&server).await;
    let result = client
        .delete("/vendors/1/security-reviews/2/assessment-runs/current", Some(json!({})))
        .await
        .expect("delete ok");
    assert_eq!(result, serde_json::Value::Null);
}

#[tokio::test]
async fn patch_sends_method_and_body() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/vendors/1"))
        .and(body_json(json!({"name": "y"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": 1, "name": "y"})))
        .expect(1)
        .mount(&server)
        .await;

    let client = rw_client(&server).await;
    let resp = client
        .patch("/vendors/1", json!({"name": "y"}))
        .await
        .expect("patch ok");
    assert_eq!(resp["name"], "y");
}

#[tokio::test]
async fn safebase_write_guard_sends_no_request() {
    let server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
        .expect(0)
        .mount(&server)
        .await;
    let client = DrataClient::new(Backend::Safebase, "sb-read-only".to_string(), None, false)
        .expect("client builds")
        .with_base_url(server.uri());

    let err = client
        .patch("/document/1", json!({"name": "x"}))
        .await
        .expect_err("a read-only SafeBase key must not PATCH");
    let guard = err.downcast_ref::<WriteGuardError>().expect("WriteGuardError");
    assert_eq!(guard.backend, Backend::Safebase);
    assert!(
        err.to_string()
            .starts_with("write blocked: the active SafeBase credential is not write-enabled."),
        "{err}"
    );
    assert!(server.received_requests().await.unwrap_or_default().is_empty());
}

#[tokio::test]
async fn drata_write_guard_names_drata() {
    let server = MockServer::start().await;
    let err = ro_client(&server)
        .await
        .delete("/vendors/1", None)
        .await
        .expect_err("blocked");
    assert!(
        err.to_string()
            .starts_with("write blocked: the active Drata credential is not write-enabled."),
        "{err}"
    );
}

#[tokio::test]
async fn patch_is_write_guarded() {
    let server = MockServer::start().await;
    let client = ro_client(&server).await;
    let err = client.patch("/vendors/1", json!({})).await.expect_err("blocked");
    assert!(err.downcast_ref::<WriteGuardError>().is_some(), "{err}");
    assert!(server.received_requests().await.unwrap_or_default().is_empty());
}

// ---------------------------------------------------------------------------
// SafeBase backend
// ---------------------------------------------------------------------------

const SB_KEY: &str = "sb-test-key";

fn safebase_client(server: &MockServer, allow_writes: bool) -> DrataClient {
    DrataClient::new(Backend::Safebase, SB_KEY.to_string(), None, allow_writes)
        .expect("client builds")
        .with_base_url(server.uri())
}

/// Every request the server saw carried `x-sb-api-key: <key>` and no
/// `Authorization` header.
async fn assert_only_sb_key(server: &MockServer) {
    let requests = server.received_requests().await.expect("recording on");
    assert!(!requests.is_empty(), "the server saw no request");
    for req in requests {
        let sb = req.headers.get("x-sb-api-key").and_then(|v| v.to_str().ok());
        assert_eq!(sb, Some(SB_KEY), "{} {} lacks x-sb-api-key", req.method, req.url);
        assert!(
            req.headers.get("authorization").is_none(),
            "{} {} carried an Authorization header",
            req.method,
            req.url
        );
    }
}

#[tokio::test]
async fn safebase_client_sends_sb_key_and_no_authorization() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/document/1"))
        .and(header("x-sb-api-key", SB_KEY))
        .and(body_json(json!({"name": "n"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
        .expect(1)
        .mount(&server)
        .await;

    let client = safebase_client(&server, true);
    let resp = client
        .patch("/document/1", json!({"name": "n"}))
        .await
        .expect("patch ok");
    assert_eq!(resp, json!({"ok": true}));
    assert_only_sb_key(&server).await;
}

#[tokio::test]
async fn safebase_multipart_sends_sb_key_and_no_authorization() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/questionnaires"))
        .and(header("x-sb-api-key", SB_KEY))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true, "result": {"id": "q1"}})))
        .expect(1)
        .mount(&server)
        .await;

    let file = tempfile::Builder::new().suffix(".csv").tempfile().expect("tempfile");
    std::fs::write(file.path(), "question\n").expect("write fixture");
    let mut form = Multipart::single("qnr", file.path());
    form.add_field("accountId", "a1");

    let client = safebase_client(&server, true);
    let resp = client
        .post_multipart("/questionnaires", &form)
        .await
        .expect("upload ok");
    assert_eq!(resp["result"]["id"], "q1");
    assert_only_sb_key(&server).await;
}

#[tokio::test]
async fn safebase_cross_origin_redirect_is_refused() {
    let elsewhere = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
        .expect(0)
        .mount(&elsewhere)
        .await;

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/organizations"))
        .respond_with(
            ResponseTemplate::new(302).insert_header("Location", format!("{}/organizations", elsewhere.uri())),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = safebase_client(&server, false);
    let err = client
        .get("/organizations")
        .await
        .expect_err("redirect must be refused");
    let chain = format!("{err:#}");
    assert!(chain.contains("HTTP request failed"), "{chain}");
    let reached = elsewhere.received_requests().await.expect("recording on");
    assert!(
        reached.is_empty(),
        "the redirect target saw {} request(s)",
        reached.len()
    );
}

#[tokio::test]
async fn safebase_same_origin_redirect_is_followed_with_the_key() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/old"))
        .respond_with(ResponseTemplate::new(302).insert_header("Location", format!("{}/organizations", server.uri())))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/organizations"))
        .and(header("x-sb-api-key", SB_KEY))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true, "result": {"name": "Tatari"}})))
        .expect(1)
        .mount(&server)
        .await;

    let client = safebase_client(&server, false);
    let resp = client.get("/old").await.expect("same-origin redirect followed");
    assert_eq!(resp["result"]["name"], "Tatari");
}

#[tokio::test]
async fn safebase_ok_false_is_an_envelope_error_carrying_the_body() {
    let server = MockServer::start().await;
    let body = json!({"ok": false, "errorCode": "conflict", "errors": ["already approved"]});
    Mock::given(method("GET"))
        .and(path("/requests"))
        .respond_with(ResponseTemplate::new(200).set_body_json(body.clone()))
        .mount(&server)
        .await;

    let client = safebase_client(&server, false);
    let err = client.get("/requests").await.expect_err("ok:false is an error");
    let env = err.downcast_ref::<EnvelopeError>().expect("EnvelopeError");
    assert_eq!(env.status, StatusCode::OK);
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&env.body).unwrap_or_default(),
        body
    );
    assert!(env.formatted.contains("`ok` is false"), "{}", env.formatted);
    assert!(env.formatted.contains("already approved"), "{}", env.formatted);
}

#[tokio::test]
async fn safebase_missing_ok_is_an_envelope_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/organizations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": "true", "result": {}})))
        .mount(&server)
        .await;

    let client = safebase_client(&server, false);
    let err = client
        .get("/organizations")
        .await
        .expect_err("non-boolean ok is an error");
    let env = err.downcast_ref::<EnvelopeError>().expect("EnvelopeError");
    assert!(env.formatted.contains("no boolean `ok`"), "{}", env.formatted);
}

#[tokio::test]
async fn safebase_multipart_checks_the_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/questionnaires"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"result": {"id": "q1"}})))
        .mount(&server)
        .await;

    let file = tempfile::Builder::new().suffix(".csv").tempfile().expect("tempfile");
    let form = Multipart::single("qnr", file.path());
    let client = safebase_client(&server, true);
    let err = client
        .post_multipart("/questionnaires", &form)
        .await
        .expect_err("no ok");
    assert!(err.downcast_ref::<EnvelopeError>().is_some(), "{err}");
}

#[tokio::test]
async fn drata_body_without_ok_is_not_envelope_checked() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/vendors/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": 1})))
        .mount(&server)
        .await;

    let client = ro_client(&server).await;
    assert_eq!(client.get("/vendors/1").await.expect("get ok")["id"], 1);
}

#[test]
fn safebase_client_rejects_a_region() {
    let err = DrataClient::new(Backend::Safebase, SB_KEY.to_string(), Some("us"), false)
        .err()
        .expect("region must be rejected");
    assert!(err.to_string().contains("Drata only"), "{err}");
}
