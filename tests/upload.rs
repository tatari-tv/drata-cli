//! `drata upload-url create --file` across two servers: wiremock plays Drata
//! (`POST /upload-urls`) and, separately, the presigned S3 URL it hands back.
//! Proves the PUT reaches only the presigned server, carries no Drata
//! credential, carries the Content-Type the URL was requested with, and that a
//! rejection prints S3's body with the signed query removed.

use clap::Parser;
use drata_cli::backend::Backend;
use drata_cli::cli::{Cli, OutputFormat};
use drata_cli::client::DrataClient;
use drata_cli::config::{Config, TokenSource};
use drata_cli::confirm::default_confirm;
use serde_json::json;
use std::io::Write;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

const SIGNED_QUERY: &str = "X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Signature=cafef00d";
const OBJECT_KEY: &str = "acct/evidence-library/1/uuid/zzz-clitest.txt";

fn test_config() -> Config {
    Config {
        api_key: "test-key".to_string(),
        region: "us".to_string(),
        allow_writes: true,
        profile: "test".to_string(),
        output_format: OutputFormat::Json,
        token_source: TokenSource::CliFlag,
    }
}

struct Servers {
    drata: MockServer,
    s3: MockServer,
}

/// Start both servers: Drata answers `/upload-urls` with a presigned URL on the
/// S3 server; S3 answers the PUT with `s3_response`.
async fn servers(s3_response: ResponseTemplate) -> Servers {
    let drata = MockServer::start().await;
    let s3 = MockServer::start().await;
    let presigned = format!("{}/bucket/{OBJECT_KEY}?{SIGNED_QUERY}", s3.uri());
    Mock::given(method("POST"))
        .and(path("/upload-urls"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "presignedUrl": presigned,
            "objectKey": OBJECT_KEY,
            "expiresAt": "2026-11-06T00:15:00.000Z",
            "webUploadUrl": null,
        })))
        .mount(&drata)
        .await;
    Mock::given(method("PUT")).respond_with(s3_response).mount(&s3).await;
    Servers { drata, s3 }
}

async fn run(servers: &Servers, file: &std::path::Path) -> eyre::Result<()> {
    let file = file.to_str().expect("utf-8 temp path");
    let cli = Cli::try_parse_from([
        "drata",
        "upload-url",
        "create",
        "--purpose",
        "evidence",
        "--workspace",
        "1",
        "--file",
        file,
        "--yes",
    ])
    .expect("argv parses");
    let config = test_config();
    let client = DrataClient::new(Backend::Drata, config.api_key.clone(), Some(&config.region), true)
        .expect("client builds")
        .with_base_url(servers.drata.uri());
    drata_cli::dispatch(&cli, &client, &config, &default_confirm(cli.yes)).await
}

fn temp_file(name: &str, contents: &[u8]) -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join(name);
    std::fs::File::create(&path)
        .and_then(|mut f| f.write_all(contents))
        .expect("write temp file");
    (dir, path)
}

async fn only(server: &MockServer) -> Request {
    let mut requests = server.received_requests().await.unwrap_or_default();
    assert_eq!(requests.len(), 1, "exactly one request");
    requests.remove(0)
}

#[tokio::test]
async fn presigned_put_has_no_authorization_and_requested_content_type() {
    let servers = servers(ResponseTemplate::new(200)).await;
    let (dir, file) = temp_file("zzz-clitest.txt", b"hello s3");

    run(&servers, &file).await.expect("upload succeeds");

    let url_req = only(&servers.drata).await;
    let url_body: serde_json::Value = serde_json::from_slice(&url_req.body).expect("JSON body");
    assert_eq!(url_body["fileName"], "zzz-clitest.txt");
    assert_eq!(url_body["contentType"], "text/plain");
    assert!(
        url_req.headers.get("authorization").is_some(),
        "Drata call is authenticated"
    );

    let put = only(&servers.s3).await;
    assert_eq!(put.method.as_str(), "PUT");
    assert_eq!(put.url.path(), format!("/bucket/{OBJECT_KEY}"));
    assert_eq!(put.url.query(), Some(SIGNED_QUERY));
    assert!(put.headers.get("authorization").is_none(), "no Drata credential to S3");
    assert_eq!(
        put.headers.get("content-type").and_then(|v| v.to_str().ok()),
        Some("text/plain"),
        "Content-Type equals the requested contentType"
    );
    assert_eq!(put.body, b"hello s3");
    drop(dir);
}

#[tokio::test]
async fn presigned_403_prints_xml_body_without_query() {
    let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<Error><Code>AccessDenied</Code><Message>Request has expired</Message></Error>";
    let servers = servers(ResponseTemplate::new(403).set_body_string(xml)).await;
    let (dir, file) = temp_file("zzz-clitest.txt", b"hello s3");

    let err = run(&servers, &file).await.expect_err("403 fails the command");
    let msg = format!("{err:?}");

    assert!(msg.contains("403"), "{msg}");
    assert!(msg.contains("<Code>AccessDenied</Code>"), "raw XML body printed: {msg}");
    assert!(
        msg.contains(&format!("PUT {}/bucket/{OBJECT_KEY}", servers.s3.uri())),
        "redacted URL printed: {msg}"
    );
    assert!(!msg.contains("X-Amz"), "signed query leaked: {msg}");
    assert!(!msg.contains("zzz-clitest.txt?"), "query leaked: {msg}");
    drop(dir);
}
