#![allow(clippy::unwrap_used)]
use super::*;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const SIGNED_QUERY: &str = "X-Amz-Signature=deadbeef&X-Amz-Credential=AKIA";

#[test]
fn redact_url_drops_query_and_fragment() {
    assert_eq!(
        redact_url("https://b.s3.amazonaws.com/a/b.txt?X-Amz-Signature=s#frag"),
        "https://b.s3.amazonaws.com/a/b.txt"
    );
    assert_eq!(redact_url("https://h/k#frag"), "https://h/k");
}

#[test]
fn redact_url_without_query_is_unchanged() {
    assert_eq!(redact_url("https://h/k.txt"), "https://h/k.txt");
    assert_eq!(redact_url(""), "");
}

#[tokio::test]
async fn put_sends_caller_headers_and_bytes_only() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/bucket/k.txt"))
        .and(header("content-type", "text/plain"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&server)
        .await;

    let url = format!("{}/bucket/k.txt?{SIGNED_QUERY}", server.uri());
    let headers = vec![("Content-Type".to_string(), "text/plain".to_string())];
    put_presigned(Method::PUT, &url, &headers, b"hello".to_vec())
        .await
        .unwrap();

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    assert!(requests[0].headers.get("authorization").is_none());
    assert_eq!(requests[0].body, b"hello");
    // The signed query reaches the server untouched.
    assert_eq!(requests[0].url.query(), Some(SIGNED_QUERY));
}

#[tokio::test]
async fn put_error_carries_raw_body_and_redacted_url() {
    let server = MockServer::start().await;
    let xml = "<?xml version=\"1.0\"?><Error><Code>SignatureDoesNotMatch</Code></Error>";
    Mock::given(method("PUT"))
        .respond_with(ResponseTemplate::new(403).set_body_string(xml))
        .mount(&server)
        .await;

    let url = format!("{}/bucket/k.txt?{SIGNED_QUERY}", server.uri());
    let err = put_presigned(Method::PUT, &url, &[], Vec::new()).await.unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("403"), "{msg}");
    assert!(msg.contains("SignatureDoesNotMatch"), "{msg}");
    assert!(msg.contains(&format!("PUT {}/bucket/k.txt", server.uri())), "{msg}");
    assert!(!msg.contains("X-Amz"), "signature leaked: {msg}");
    assert!(!msg.contains("k.txt?"), "query leaked: {msg}");
}

#[tokio::test]
async fn put_does_not_follow_redirects() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/bucket/k.txt"))
        .respond_with(ResponseTemplate::new(307).insert_header("location", "/elsewhere"))
        .mount(&server)
        .await;

    let url = format!("{}/bucket/k.txt?{SIGNED_QUERY}", server.uri());
    let err = put_presigned(Method::PUT, &url, &[], Vec::new()).await.unwrap_err();
    assert!(format!("{err:#}").contains("307"));
    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1, "redirect must not be followed");
}

#[tokio::test]
async fn transport_error_hides_the_query() {
    // Port 9 on localhost: nothing listens, so the connect fails.
    let url = format!("http://127.0.0.1:9/bucket/k.txt?{SIGNED_QUERY}");
    let err = put_presigned(Method::PUT, &url, &[], Vec::new()).await.unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("http://127.0.0.1:9/bucket/k.txt"), "{msg}");
    assert!(!msg.contains("X-Amz"), "signature leaked: {msg}");
}
