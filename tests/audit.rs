//! `drata audit request dcf-suggestion ...` on the wire: the per-item failure
//! rule for `generate` and `accept`, the bodies, the write guard and the
//! confirm prompt.

use clap::Parser;
use drata_cli::backend::Backend;
use drata_cli::cli::{Cli, OutputFormat};
use drata_cli::client::DrataClient;
use drata_cli::config::{Config, TokenSource};
use drata_cli::confirm::{ConfirmFn, default_confirm};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, ResponseTemplate};

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

struct Run {
    result: eyre::Result<()>,
    requests: Vec<wiremock::Request>,
}

async fn run(argv: &[&str], response: ResponseTemplate, allow_writes: bool, confirm: ConfirmFn) -> Run {
    let server = MockServer::start().await;
    Mock::given(any()).respond_with(response).mount(&server).await;
    let cli = Cli::try_parse_from(std::iter::once("drata").chain(argv.iter().copied())).expect("argv parses");
    let config = test_config();
    let client = DrataClient::new(
        Backend::Drata,
        config.api_key.clone(),
        Some(&config.region),
        allow_writes,
    )
    .expect("client builds")
    .with_base_url(server.uri());
    let result = drata_cli::dispatch(&cli, &client, &config, &confirm).await;
    let requests = server.received_requests().await.unwrap_or_default();
    Run { result, requests }
}

fn body(req: &wiremock::Request) -> Value {
    serde_json::from_slice(&req.body).expect("json body")
}

const DCF: [&str; 3] = ["audit", "request", "dcf-suggestion"];

fn argv<'a>(rest: &[&'a str]) -> Vec<&'a str> {
    DCF.iter().chain(rest.iter()).copied().collect()
}

const GENERATE_ONE_FAILED: &str = r#"{"data": [
    {"requestId": 1, "status": "STARTED", "error": null},
    {"requestId": 2, "status": "FAILED", "error": "could not start"},
    {"requestId": 3, "status": "SKIPPED", "error": null}
]}"#;

#[tokio::test]
async fn generate_with_one_failed_row_exits_1_after_printing_every_row() {
    let r = run(
        &argv(&["generate", "1", "2", "--request-id", "1", "2", "3"]),
        ResponseTemplate::new(202).set_body_string(GENERATE_ONE_FAILED),
        true,
        default_confirm(true),
    )
    .await;
    let msg = format!("{:#}", r.result.expect_err("one FAILED row exits 1"));
    assert_eq!(msg, "1 of 3 items failed");
    assert_eq!(r.requests.len(), 1, "the 202 is not polled");
    assert_eq!(
        r.requests[0].url.path(),
        "/workspaces/1/audits/2/requests/action-generate-dcf-suggestions"
    );
    assert_eq!(body(&r.requests[0]), json!({"requestIds": [1, 2, 3]}));
}

#[tokio::test]
async fn generate_with_started_and_skipped_rows_exits_0() {
    let ok = r#"{"data": [{"requestId": 1, "status": "STARTED", "error": null},
                          {"requestId": 3, "status": "SKIPPED", "error": null}]}"#;
    let r = run(
        &argv(&["generate", "1", "2", "--request-id", "1", "3"]),
        ResponseTemplate::new(202).set_body_string(ok),
        true,
        default_confirm(true),
    )
    .await;
    r.result.expect("STARTED and SKIPPED are not failures");
}

#[tokio::test]
async fn accept_with_an_error_row_exits_1_and_a_clean_response_exits_0() {
    let failed = r#"{"data": [
        {"requestId": 1, "acceptedCount": 1, "mappedCount": 1, "unresolvedCount": 0, "error": null},
        {"requestId": 2, "acceptedCount": 0, "mappedCount": 0, "unresolvedCount": 0, "error": "could not process"}
    ]}"#;
    let r = run(
        &argv(&[
            "accept",
            "1",
            "2",
            "--request-id",
            "1",
            "2",
            "--confidence-level",
            "high",
        ]),
        ResponseTemplate::new(200).set_body_string(failed),
        true,
        default_confirm(true),
    )
    .await;
    assert_eq!(
        format!("{:#}", r.result.expect_err("an error row exits 1")),
        "1 of 2 items failed"
    );
    assert_eq!(
        body(&r.requests[0]),
        json!({"requestIds": [1, 2], "confidenceLevels": ["HIGH"]})
    );

    let clean =
        r#"{"data": [{"requestId": 1, "acceptedCount": 1, "mappedCount": 1, "unresolvedCount": 0, "error": null}]}"#;
    let r = run(
        &argv(&["accept", "1", "2", "--all-requests"]),
        ResponseTemplate::new(200).set_body_string(clean),
        true,
        default_confirm(true),
    )
    .await;
    r.result.expect("no error rows exits 0");
    assert_eq!(body(&r.requests[0]), json!({"allRequests": true}));
}

#[tokio::test]
async fn accept_all_requests_confirm_prompt_names_every_matching_request() {
    let prompts = Arc::new(Mutex::new(Vec::new()));
    let seen = prompts.clone();
    let confirm: ConfirmFn = Box::new(move |method, what| {
        seen.lock().expect("lock").push(format!("{method} {what}"));
        Ok(false)
    });
    let r = run(
        &argv(&["accept", "1", "2", "--all-requests"]),
        ResponseTemplate::new(200),
        true,
        confirm,
    )
    .await;
    assert_eq!(r.result.expect_err("declined").to_string(), "aborted");
    assert!(r.requests.is_empty(), "declined means no request");
    let prompts = prompts.lock().expect("lock");
    assert!(prompts[0].contains("EVERY matching request"), "{prompts:?}");
}

#[tokio::test]
async fn run_action_sends_the_decision_and_the_write_guard_blocks_without_writes() {
    let decided = ResponseTemplate::new(200).set_body_json(json!({"id": 4, "status": "ACCEPTED"}));
    let r = run(
        &argv(&["run-action", "1", "2", "3", "4", "--action", "accept"]),
        decided,
        true,
        default_confirm(true),
    )
    .await;
    r.result.expect("run-action succeeds");
    assert_eq!(
        r.requests[0].url.path(),
        "/workspaces/1/audits/2/requests/3/dcf-suggestions/4/actions"
    );
    assert_eq!(body(&r.requests[0]), json!({"action": "ACCEPT"}));

    let r = run(
        &argv(&["run-action", "1", "2", "3", "4", "--action", "reject"]),
        ResponseTemplate::new(200),
        false,
        default_confirm(true),
    )
    .await;
    assert!(r.result.is_err(), "the write guard refuses");
    assert!(r.requests.is_empty());
}

#[tokio::test]
async fn list_sends_repeated_filters() {
    let page = ResponseTemplate::new(200).set_body_json(json!({"data": [], "pagination": {"cursor": null}}));
    let r = run(
        &argv(&[
            "list",
            "1",
            "2",
            "3",
            "--status",
            "pending",
            "accepted",
            "--confidence-level",
            "low",
        ]),
        page,
        true,
        default_confirm(true),
    )
    .await;
    r.result.expect("list succeeds");
    assert_eq!(
        r.requests[0].url.path(),
        "/workspaces/1/audits/2/requests/3/dcf-suggestions"
    );
    assert_eq!(
        r.requests[0].url.query(),
        Some("status%5B%5D=PENDING&status%5B%5D=ACCEPTED&confidenceLevel%5B%5D=LOW&size=500")
    );
}
