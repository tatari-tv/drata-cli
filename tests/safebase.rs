//! `drata safebase raw`: the shared raw implementation routed to SafeBase.
//! Wiremock tests drive `raw::handle` from parsed argv, the way `main` does.

use clap::Parser;
use drata_cli::backend::Backend;
use drata_cli::cli::{Cli, Commands, OutputFormat, SafebaseAction};
use drata_cli::client::{DrataClient, WriteGuardError};
use drata_cli::config::{Config, TokenSource};
use drata_cli::confirm::{always_no, always_yes};
use drata_cli::raw;
use serde_json::json;
use wiremock::matchers::{any, body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const SB_KEY: &str = "sb-test-key";

fn config(allow_writes: bool) -> Config {
    Config {
        backend: Backend::Safebase,
        api_key: SB_KEY.to_string(),
        region: None,
        allow_writes,
        profile: "test".to_string(),
        output_format: OutputFormat::Json,
        token_source: TokenSource::CliFlag,
    }
}

fn client(server: &MockServer, allow_writes: bool) -> DrataClient {
    DrataClient::new(Backend::Safebase, SB_KEY.to_string(), None, allow_writes)
        .expect("client builds")
        .with_base_url(server.uri())
}

fn raw_args(argv: &[&str]) -> drata_cli::cli::RawArgs {
    let mut full = vec!["drata", "safebase", "raw"];
    full.extend_from_slice(argv);
    let cli = Cli::try_parse_from(full).expect("argv parses");
    match cli.command {
        Commands::Safebase {
            action: SafebaseAction::Raw(args),
        } => args,
        other => panic!("parsed as {other:?}"),
    }
}

#[tokio::test]
async fn safebase_raw_patch_sends_body() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/document/1"))
        .and(header("x-sb-api-key", SB_KEY))
        .and(body_json(json!({"name": "SOC 2", "aiOptOut": true})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
        .expect(1)
        .mount(&server)
        .await;

    let args = raw_args(&["PATCH", "/document/1", "--data", r#"{"name":"SOC 2","aiOptOut":true}"#]);
    raw::handle(&args, &client(&server, true), &config(true), &always_yes())
        .await
        .expect("raw patch ok");

    let requests = server.received_requests().await.expect("recording on");
    assert_eq!(requests.len(), 1);
    assert!(requests[0].headers.get("authorization").is_none());
}

#[tokio::test]
async fn safebase_raw_patch_on_read_only_key_sends_no_request() {
    let server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
        .expect(0)
        .mount(&server)
        .await;

    let args = raw_args(&["PATCH", "/document/1", "--data", "{}"]);
    let err = raw::handle(&args, &client(&server, false), &config(false), &always_yes())
        .await
        .expect_err("read-only key must not PATCH");
    assert!(err.downcast_ref::<WriteGuardError>().is_some(), "{err}");
}

#[tokio::test]
async fn safebase_raw_declined_confirm_sends_no_request() {
    let server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
        .expect(0)
        .mount(&server)
        .await;

    let args = raw_args(&["PATCH", "/document/1", "--data", "{}"]);
    let err = raw::handle(&args, &client(&server, true), &config(true), &always_no())
        .await
        .expect_err("declined");
    assert_eq!(err.to_string(), "aborted");
}

#[tokio::test]
async fn safebase_raw_get_carries_query_and_skips_confirm() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/documents"))
        .and(wiremock::matchers::query_param("limit", "5"))
        .and(header("x-sb-api-key", SB_KEY))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"ok": true, "result": {"documents": [], "count": 0}})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let args = raw_args(&["GET", "/documents", "--query", "limit=5"]);
    raw::handle(&args, &client(&server, false), &config(false), &always_no())
        .await
        .expect("raw get ok");
}

#[tokio::test]
async fn safebase_raw_multipart_questionnaire_sends_qnr_and_scalar_fields() {
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
    let file_path = file.path().to_str().expect("utf8 path");
    let args = raw_args(&[
        "POST",
        "/questionnaires",
        "--file",
        file_path,
        "--file-field",
        "qnr",
        "--field",
        "accountId=a1",
        "productId=p1",
        "dueDate=2026-12-31T23:59:59Z",
        "notes=n1",
    ]);
    raw::handle(&args, &client(&server, true), &config(true), &always_yes())
        .await
        .expect("multipart ok");

    let requests = server.received_requests().await.expect("recording on");
    let body = String::from_utf8_lossy(&requests[0].body).to_string();
    for part in [
        r#"name="qnr""#,
        r#"name="accountId""#,
        r#"name="productId""#,
        r#"name="dueDate""#,
        r#"name="notes""#,
    ] {
        assert!(body.contains(part), "multipart body lacks {part}");
    }
}

#[test]
fn safebase_raw_example_needs_no_key_and_reads_the_safebase_spec() {
    let args = raw_args(&["--example", "PATCH", "/document/{id}"]);
    let cli = Cli::try_parse_from(["drata", "safebase", "raw", "--example", "PATCH", "/document/{id}"]).unwrap();
    assert!(args.example);
    let out = drata_cli::example_if_requested(&cli)
        .expect("example requested")
        .expect("skeleton");
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    // Every PATCH /document/{id} field is optional and a skeleton carries
    // required fields only, so the object is empty.
    assert!(value.is_object(), "{value}");

    let cli = Cli::try_parse_from(["drata", "safebase", "raw", "--example", "POST", "/documents/upload"]).unwrap();
    let out = drata_cli::example_if_requested(&cli).unwrap().unwrap();
    let value: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert!(value.get("fileName").is_some(), "{value}");
}

#[test]
fn safebase_raw_example_errors_name_the_safebase_spec() {
    let cli = Cli::try_parse_from(["drata", "safebase", "raw", "--example", "POST", "/vendors"]).unwrap();
    let err = drata_cli::example_if_requested(&cli).unwrap().unwrap_err();
    assert!(err.to_string().contains("SafeBase spec"), "{err}");

    // A multipart op has no JSON body, so no skeleton.
    let cli = Cli::try_parse_from(["drata", "safebase", "raw", "--example", "POST", "/questionnaires"]).unwrap();
    let err = drata_cli::example_if_requested(&cli).unwrap().unwrap_err();
    assert!(err.to_string().contains("no JSON request body"), "{err}");
}
