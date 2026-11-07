//! `drata evidence-item ...` on the wire: the create and update bodies, the
//! evidence-files multipart upload, the remove prompt naming every artifact,
//! the write guard, and the partial-artifact exit rule.

use clap::Parser;
use drata_cli::cli::{Cli, OutputFormat};
use drata_cli::client::DrataClient;
use drata_cli::config::{Config, TokenSource};
use drata_cli::confirm::{ConfirmFn, default_confirm};
use serde_json::{Value, json};
use std::sync::{Arc, Mutex};
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, ResponseTemplate};

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE");

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
    let client = DrataClient::new(config.api_key.clone(), &config.region, allow_writes)
        .expect("client builds")
        .with_base_url(server.uri());
    let result = drata_cli::dispatch(&cli, &client, &config, &confirm).await;
    let requests = server.received_requests().await.unwrap_or_default();
    Run { result, requests }
}

/// Dispatch with `--yes` and expect success and exactly one request.
async fn sent(argv: &[&str], response: ResponseTemplate) -> wiremock::Request {
    let r = run(argv, response, true, default_confirm(true)).await;
    r.result.expect("dispatch succeeds");
    assert_eq!(r.requests.len(), 1, "exactly one request");
    r.requests.into_iter().next().expect("one request")
}

fn object() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"id": 1}))
}

fn body(req: &wiremock::Request) -> Value {
    serde_json::from_slice(&req.body).expect("JSON body")
}

#[tokio::test]
async fn create_posts_typed_artifacts_to_the_evidence_tag() {
    let req = sent(
        &[
            "evidence-item",
            "create",
            "1",
            "--name",
            "Training",
            "--url",
            "https://example.com/a",
            "--file-key",
            "acct/evidence-library/1/f.pdf",
            "--filed-at",
            "2026-11-01",
            "--owner-id",
            "9",
            "--renewal-schedule-type",
            "one_year",
        ],
        object(),
    )
    .await;
    assert_eq!(req.method.as_str(), "POST");
    assert_eq!(req.url.path(), "/workspaces/1/evidence");
    assert_eq!(
        body(&req),
        json!({
            "name": "Training",
            "ownerId": 9,
            "renewalScheduleType": "ONE_YEAR",
            "artifacts": [
                {"type": "URL", "url": "https://example.com/a", "filedAt": "2026-11-01"},
                {"type": "S3_FILE", "fileKey": "acct/evidence-library/1/f.pdf", "filedAt": "2026-11-01"},
            ],
        })
    );
}

#[tokio::test]
async fn update_sends_new_archived_and_restored_artifacts() {
    let req = sent(
        &[
            "evidence-item",
            "update",
            "1",
            "2",
            "--ticket-url",
            "https://jira.example.com/browse/SEC-1",
            "--filed-at",
            "2026-11-02",
            "--archive-artifact",
            "4",
            "5",
            "--restore-artifact",
            "6",
        ],
        object(),
    )
    .await;
    assert_eq!(req.method.as_str(), "PUT");
    assert_eq!(req.url.path(), "/workspaces/1/evidence/2");
    assert_eq!(
        body(&req),
        json!({
            "newArtifacts": [{
                "artifactType": "TICKET_PROVIDER",
                "ticketUrl": "https://jira.example.com/browse/SEC-1",
                "filedAt": "2026-11-02",
            }],
            "pastArtifacts": [4, 5],
            "currentArtifacts": [6],
        })
    );
}

#[tokio::test]
async fn create_data_is_sent_verbatim() {
    let data = r#"{"name": "n", "artifacts": [], "stepsToReproduce": "s"}"#;
    let req = sent(&["evidence-item", "create", "1", "--data", data], object()).await;
    assert_eq!(body(&req), serde_json::from_str::<Value>(data).unwrap());
}

#[tokio::test]
async fn upload_posts_one_multipart_file_part_to_evidence_files() {
    let req = sent(
        &["evidence-item", "upload", "1", "--file", FIXTURE],
        ResponseTemplate::new(201).set_body_json(json!({"fileKey": "acct/evidence-library/1/LICENSE"})),
    )
    .await;
    assert_eq!(req.method.as_str(), "POST");
    assert_eq!(req.url.path(), "/workspaces/1/evidence-files");
    let content_type = req
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    assert!(content_type.starts_with("multipart/form-data"), "{content_type}");
    let raw = String::from_utf8_lossy(&req.body);
    assert_eq!(raw.matches("name=\"file\"").count(), 1, "one file part");
}

#[tokio::test]
async fn upload_without_a_file_key_in_the_response_fails() {
    let r = run(
        &["evidence-item", "upload", "1", "--file", FIXTURE],
        ResponseTemplate::new(201).set_body_json(json!({"id": 1})),
        true,
        default_confirm(true),
    )
    .await;
    let err = r.result.expect_err("no fileKey is an error");
    assert!(err.to_string().contains("fileKey"), "{err}");
}

#[tokio::test]
async fn remove_prompt_names_every_artifact_and_a_no_sends_nothing() {
    let seen = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
    let log = Arc::clone(&seen);
    let confirm: ConfirmFn = Box::new(move |method: &str, path: &str| {
        log.lock().unwrap().push((method.to_string(), path.to_string()));
        Ok(false)
    });
    let r = run(
        &["evidence-item", "remove", "1", "2"],
        ResponseTemplate::new(204),
        true,
        confirm,
    )
    .await;
    assert!(r.result.unwrap_err().to_string().contains("aborted"));
    assert!(r.requests.is_empty(), "a declined remove sends nothing");
    let seen = seen.lock().unwrap();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].0, "DELETE");
    assert!(
        seen[0].1.starts_with("/workspaces/1/evidence/2 ") && seen[0].1.contains("all of its artifacts"),
        "{}",
        seen[0].1
    );
}

#[tokio::test]
async fn mutations_are_blocked_without_a_write_credential() {
    for argv in [
        &["evidence-item", "remove", "1", "2"][..],
        &[
            "evidence-item",
            "artifact",
            "run-action",
            "1",
            "2",
            "3",
            "--action",
            "archive",
        ][..],
        &["evidence-item", "upload", "1", "--file", FIXTURE][..],
    ] {
        let r = run(argv, object(), false, default_confirm(true)).await;
        assert!(r.result.is_err(), "{argv:?} must fail");
        assert!(r.requests.is_empty(), "{argv:?} sent a request");
    }
}

#[tokio::test]
async fn create_with_fewer_artifacts_created_than_requested_exits_non_zero() {
    let r = run(
        &[
            "evidence-item",
            "create",
            "1",
            "--name",
            "x",
            "--url",
            "https://example.com/a",
            "https://example.com/b",
            "--filed-at",
            "2026-11-01",
        ],
        ResponseTemplate::new(201).set_body_json(json!({"id": 7, "artifactsRequested": 2, "artifactsCreated": 1})),
        true,
        default_confirm(true),
    )
    .await;
    assert_eq!(r.requests.len(), 1);
    let err = r.result.expect_err("partial artifacts fail the command");
    assert!(err.to_string().contains("1 of 2"), "{err}");
}

#[tokio::test]
async fn artifact_verbs_send_their_bodies() {
    let req = sent(
        &["evidence-item", "artifact", "update", "1", "2", "3", "--clear-filed-at"],
        object(),
    )
    .await;
    assert_eq!(req.url.path(), "/workspaces/1/evidence/2/artifacts/3");
    assert_eq!(body(&req), json!({"filedAt": null}));
    let req = sent(
        &[
            "evidence-item",
            "artifact",
            "run-action",
            "1",
            "2",
            "3",
            "--action",
            "restore",
        ],
        object(),
    )
    .await;
    assert_eq!(req.url.path(), "/workspaces/1/evidence/2/artifacts/3/actions");
    assert_eq!(body(&req), json!({"action": "restore"}));
    let req = sent(
        &[
            "evidence-item",
            "artifact",
            "list",
            "1",
            "2",
            "--archived",
            "true",
            "--artifact-type",
            "url",
        ],
        ResponseTemplate::new(200).set_body_json(json!({"data": [], "pagination": {"cursor": null}})),
    )
    .await;
    let q = req.url.query().unwrap_or_default();
    assert!(
        q.contains("isArchived=true") && q.contains("artifactTypes%5B%5D=URL"),
        "{q}"
    );
}
