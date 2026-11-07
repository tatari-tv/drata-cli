#![allow(clippy::unwrap_used)]
use super::*;
use crate::backend::Backend;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_new_succeeds() {
    let client = DrataClient::new(Backend::Drata, "test-key".to_string(), Some("us"), false);
    assert!(client.is_ok());
}

#[test]
fn test_with_base_url() {
    let client = DrataClient::new(Backend::Drata, "test-key".to_string(), Some("us"), false)
        .unwrap()
        .with_base_url("https://custom.example.com".to_string());
    assert_eq!(client.base_url, "https://custom.example.com");
}

#[test]
fn test_new_rejects_unknown_region() {
    let result = DrataClient::new(Backend::Drata, "test-key".to_string(), Some("mars"), false);
    assert!(result.is_err(), "unknown region should return Err");
    if let Err(e) = result {
        let msg = e.to_string();
        assert!(msg.contains("mars"), "error should name the bad region: {msg}");
        assert!(msg.contains("us, eu, apac"), "error should list valid regions: {msg}");
    }
}

#[test]
fn test_allow_writes_reflects_construction() {
    let ro = DrataClient::new(Backend::Drata, "k".to_string(), Some("us"), false).unwrap();
    assert!(!ro.allow_writes());
    let rw = DrataClient::new(Backend::Drata, "k".to_string(), Some("us"), true).unwrap();
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
fn encode_path_segment_keeps_one_segment() {
    assert_eq!(encode_path_segment("name:MFA check"), "name:MFA%20check");
    assert_eq!(encode_path_segment("a/b?c#d%"), "a%2Fb%3Fc%23d%25");
    assert_eq!(encode_path_segment("42"), "42");
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

// ---------------------------------------------------------------------------
// Phase 4: NDJSON streaming, multipart upload
// ---------------------------------------------------------------------------

use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn rw_client(server: &MockServer) -> DrataClient {
    DrataClient::new(Backend::Drata, "test-key".to_string(), Some("us"), true)
        .unwrap()
        .with_base_url(server.uri())
}

async fn ro_client_for_stream(server: &MockServer) -> DrataClient {
    DrataClient::new(Backend::Drata, "test-key".to_string(), Some("us"), false)
        .unwrap()
        .with_base_url(server.uri())
}

#[tokio::test]
async fn stream_all_writes_ndjson_lines() {
    let server = MockServer::start().await;
    // Two-page response.
    Mock::given(method("GET"))
        .and(path("/vendors"))
        .and(query_param("size", PAGINATION_SIZE.to_string()))
        .and(query_param_is_missing("cursor"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{"id": 1, "name": "A"}, {"id": 2, "name": "B"}],
            "pagination": {"cursor": "next"}
        })))
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/vendors"))
        .and(query_param("size", PAGINATION_SIZE.to_string()))
        .and(query_param("cursor", "next"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{"id": 3, "name": "C"}],
            "pagination": {"cursor": null}
        })))
        .mount(&server)
        .await;

    let client = ro_client_for_stream(&server).await;
    let mut out: Vec<u8> = Vec::new();
    let count = client.stream_all("/vendors", &mut out).await.unwrap();
    assert_eq!(count, 3, "should stream 3 items");

    let text = String::from_utf8(out).unwrap();
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 3, "each item should be on its own line");

    // Each line is valid JSON
    for line in &lines {
        let parsed: serde_json::Value = serde_json::from_str(line).unwrap();
        assert!(parsed["id"].is_number(), "each line has an id: {line}");
    }

    // Lines are in order
    let ids: Vec<i64> = lines
        .iter()
        .map(|l| {
            serde_json::from_str::<serde_json::Value>(l).unwrap()["id"]
                .as_i64()
                .unwrap()
        })
        .collect();
    assert_eq!(ids, vec![1, 2, 3]);
}

#[tokio::test]
async fn stream_all_aborts_on_repeated_cursor() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/items"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": [{"id": 1}],
            "pagination": {"cursor": "stuck"}
        })))
        .mount(&server)
        .await;

    let client = ro_client_for_stream(&server).await;
    let mut out: Vec<u8> = Vec::new();
    // A repeated cursor now returns Err (fail-closed) instead of partial data.
    let result = client.stream_all("/items", &mut out).await;
    assert!(result.is_err(), "repeated cursor should return Err, not partial data");
    let msg = result.unwrap_err().to_string();
    assert!(
        msg.contains("repeated cursor"),
        "error should mention repeated cursor: {msg}"
    );
}

#[tokio::test]
async fn post_multipart_sends_file() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/vendors/1/documents"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({"id": 10, "fileName": "test.txt"})))
        .expect(1)
        .mount(&server)
        .await;

    let client = rw_client(&server).await;

    // Create a temp file to upload.
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "hello drata").unwrap();
    let path = tmp.path().to_path_buf();

    let form = Multipart::single("file", path);
    let result = client.post_multipart("/vendors/1/documents", &form).await.unwrap();
    assert_eq!(result["id"].as_u64(), Some(10));
}

#[tokio::test]
async fn post_multipart_blocked_on_readonly_client() {
    let server = MockServer::start().await;
    let client = DrataClient::new(Backend::Drata, "test-key".to_string(), Some("us"), false)
        .unwrap()
        .with_base_url(server.uri());

    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "data").unwrap();

    let form = Multipart::single("file", tmp.path());
    let err = client.post_multipart("/vendors/1/documents", &form).await.unwrap_err();
    let downcast = err.downcast_ref::<WriteGuardError>();
    assert!(downcast.is_some(), "expected WriteGuardError, got: {err}");
}

#[test]
fn multipart_builder_collects_files_and_fields() {
    let mut form = Multipart::single("file", "/tmp/a.pdf");
    form.add_file("files", "/tmp/b.pdf")
        .add_field("type", "ANTIVIRUS_EVIDENCE")
        .add_opt_field("description", Some("d"))
        .add_opt_field("skip", None::<String>);

    assert_eq!(form.files.len(), 2);
    assert_eq!(form.files[0].field, "file");
    assert_eq!(form.files[1].field, "files");
    // The None field is dropped; only `type` and `description` survive.
    assert_eq!(form.fields.len(), 2);
    assert!(
        form.fields
            .iter()
            .any(|(k, v)| k == "type" && v == "ANTIVIRUS_EVIDENCE")
    );
    assert!(form.fields.iter().any(|(k, v)| k == "description" && v == "d"));
    assert!(!form.is_empty());
}

fn cooldown_429() -> ResponseTemplate {
    // Retry-After: 0 keeps a retrying client fast; the count is what matters.
    ResponseTemplate::new(429)
        .insert_header("Retry-After", "0")
        .set_body_json(serde_json::json!({"message": "test is cooling down"}))
}

#[tokio::test]
async fn retry_never_sends_once() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/retest"))
        .respond_with(cooldown_429())
        .mount(&server)
        .await;

    let client = rw_client(&server).await;
    let err = client
        .send_inner(Method::POST, "/retest", Some(serde_json::json!({})), Retry429::Never)
        .await
        .unwrap_err();

    assert_eq!(server.received_requests().await.unwrap().len(), 1);
    let api = err.downcast_ref::<ApiError>().expect("429 surfaces as ApiError");
    assert_eq!(api.status, StatusCode::TOO_MANY_REQUESTS);
    assert!(err.to_string().contains("test is cooling down"), "{err}");
}

#[tokio::test]
async fn post_once_sends_one_bodiless_post_on_429() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/retest"))
        .respond_with(cooldown_429())
        .mount(&server)
        .await;

    let client = rw_client(&server).await;
    let err = client.post_once("/retest").await.unwrap_err();

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1, "a 429 is not retried");
    assert!(requests[0].body.is_empty(), "no body is sent");
    let api = err.downcast_ref::<ApiError>().expect("429 surfaces as ApiError");
    assert_eq!(api.status, StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn post_once_is_write_guarded() {
    let server = MockServer::start().await;
    let client = ro_client_for_stream(&server).await;
    let err = client.post_once("/retest").await.unwrap_err();
    assert!(err.downcast_ref::<WriteGuardError>().is_some(), "{err}");
    assert!(server.received_requests().await.unwrap().is_empty());
}

#[tokio::test]
async fn retry_default_retries_429() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/vendors"))
        .respond_with(cooldown_429())
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/vendors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"id": 1})))
        .mount(&server)
        .await;

    let client = rw_client(&server).await;
    let v = client
        .send_inner(Method::GET, "/vendors", None, Retry429::Default)
        .await
        .unwrap();

    assert_eq!(v["id"], 1);
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

#[test]
fn agent_422_prints_message_blockers_and_available_actions() {
    let body = r#"{
        "name": "AgentActionNotAvailable", "statusCode": 422, "code": 422,
        "message": "The agent does not offer the finalize action right now.",
        "requestedAction": "finalize",
        "blockers": [
            {"reason": "RESIDUAL_RISK_NOT_REVIEWED", "message": "Review the residual risk first.",
             "resolution": {"type": "UI_ONLY", "action": "Review residual risk"}},
            {"reason": "ASSESSMENT_IN_PROGRESS", "message": "An assessment is running.",
             "resolution": {"type": "WAIT"}}
        ],
        "availableActions": ["stop_assessment"],
        "currentStep": "ASSESS"
    }"#;
    let msg = format_api_error(
        &Method::POST,
        "https://x/vendors/1/security-reviews/2/finalizations",
        StatusCode::UNPROCESSABLE_ENTITY,
        body,
    );
    assert!(msg.contains("422"), "{msg}");
    assert!(
        msg.contains("The agent does not offer the finalize action right now."),
        "{msg}"
    );
    assert!(
        msg.contains(
            "Blocker RESIDUAL_RISK_NOT_REVIEWED: Review the residual risk first. [UI_ONLY: Review residual risk]"
        ),
        "{msg}"
    );
    assert!(
        msg.contains("Blocker ASSESSMENT_IN_PROGRESS: An assessment is running. [WAIT]"),
        "{msg}"
    );
    assert!(msg.contains("Available actions: stop_assessment"), "{msg}");
    assert!(msg.contains("Current step: ASSESS"), "{msg}");
}

#[test]
fn plain_errors_carry_no_agent_guidance() {
    let msg = format_api_error(
        &Method::GET,
        "https://x/vendors",
        StatusCode::BAD_REQUEST,
        r#"{"message":"bad"}"#,
    );
    assert!(!msg.contains("Blocker") && !msg.contains("Available actions"), "{msg}");
}

#[test]
fn retry_delay_honors_a_long_retry_after_up_to_the_cap() {
    // Drata's edge sent 162 during the v0.2.0 shakedown; a 60s cap wasted every retry.
    assert_eq!(retry_delay(Some("162")), 162);
    assert_eq!(retry_delay(Some("99999")), MAX_RETRY_DELAY_SECS);
    assert_eq!(retry_delay(None), DEFAULT_RETRY_DELAY_SECS);
    assert_eq!(retry_delay(Some("soon")), DEFAULT_RETRY_DELAY_SECS);
}

async fn three_page_server(route: &str) -> MockServer {
    let server = MockServer::start().await;
    for (cursor_in, cursor_out, ids) in [
        (None, Some("p2"), [1, 2]),
        (Some("p2"), Some("p3"), [3, 4]),
        (Some("p3"), None, [5, 6]),
    ] {
        let mut m = Mock::given(method("GET")).and(path(route));
        m = match cursor_in {
            Some(c) => m.and(query_param("cursor", c)),
            None => m.and(query_param_is_missing("cursor")),
        };
        m.respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "data": ids.iter().map(|i| serde_json::json!({"id": i})).collect::<Vec<_>>(),
            "pagination": {"cursor": cursor_out}
        })))
        .mount(&server)
        .await;
    }
    server
}

#[tokio::test]
async fn get_all_limited_stops_fetching_once_the_limit_is_reached() {
    let server = three_page_server("/assets").await;
    let client = ro_client_for_stream(&server).await;
    let items = client.get_all_limited("/assets", Some(3)).await.unwrap();
    let ids: Vec<i64> = items.iter().map(|v| v["id"].as_i64().unwrap()).collect();
    assert_eq!(ids, vec![1, 2, 3]);
    let reqs = server.received_requests().await.unwrap();
    assert_eq!(reqs.len(), 2, "page 3 must not be fetched");
    assert!(
        reqs[0].url.query().unwrap().contains("size=3"),
        "page size shrinks to the limit: {}",
        reqs[0].url
    );
}

#[tokio::test]
async fn stream_all_limited_writes_exactly_limit_lines() {
    let server = three_page_server("/events").await;
    let client = ro_client_for_stream(&server).await;
    let mut out: Vec<u8> = Vec::new();
    let n = client.stream_all_limited("/events", Some(1), &mut out).await.unwrap();
    assert_eq!(n, 1);
    assert_eq!(String::from_utf8(out).unwrap().lines().count(), 1);
    assert_eq!(server.received_requests().await.unwrap().len(), 1);
}

#[tokio::test]
async fn page_requests_are_paced_under_the_rate_limit() {
    // Local request latency alone runs into the hundreds of ms in a debug build,
    // so a 150ms floor is not measurable here; a 1s interval is, and the drain
    // uses the same code path whatever the interval.
    assert_eq!(MIN_PAGE_INTERVAL, Duration::from_millis(150));
    let server = three_page_server("/events").await;
    let interval = Duration::from_secs(1);
    let client = ro_client_for_stream(&server).await.with_page_interval(interval);
    let start = std::time::Instant::now();
    let items = client.get_all("/events").await.unwrap();
    assert_eq!(items.len(), 6);
    assert!(
        start.elapsed() >= interval * 2,
        "3 pages took {:?}, under two {:?} intervals",
        start.elapsed(),
        interval
    );
}
