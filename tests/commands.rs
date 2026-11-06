//! The command table, proven on the wire.
//!
//! Every row of `tests/commands/table.rs` is parsed with clap, dispatched
//! against wiremock with a write-enabled credential and `--yes`, and must send
//! exactly one request whose method and path match the row's operation. A
//! second test walks the clap tree and fails if any curated leaf subcommand has
//! no row, so a command cannot ship without proving the op it reaches.

#[path = "commands/table.rs"]
mod table;

use clap::{CommandFactory, Parser};
use drata_cli::cli::{Cli, OutputFormat};
use drata_cli::client::DrataClient;
use drata_cli::config::{Config, TokenSource};
use drata_cli::confirm::default_confirm;
use serde_json::json;
use std::collections::BTreeSet;
use table::{ROWS, Response, Row};
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Top-level commands with no curated op behind them: auth onboarding, the
/// `raw` passthrough, and the live `verify` harness.
const UNCURATED: &[&str] = &["raw", "login", "logout", "whoami", "auth", "verify"];

fn template(response: Response) -> ResponseTemplate {
    match response {
        Response::Page => ResponseTemplate::new(200).set_body_json(json!({"data": [], "pagination": {"cursor": null}})),
        Response::Object => ResponseTemplate::new(200).set_body_json(json!({"id": 1})),
        Response::Empty => ResponseTemplate::new(204),
    }
}

/// True when `actual` fills `template`: same segment count, literal segments
/// equal, and each `{param}` segment non-empty.
fn path_matches(template: &str, actual: &str) -> bool {
    let t: Vec<&str> = template.split('/').collect();
    let a: Vec<&str> = actual.split('/').collect();
    t.len() == a.len()
        && t.iter().zip(&a).all(|(want, got)| {
            if want.starts_with('{') && want.ends_with('}') {
                !got.is_empty()
            } else {
                want == got
            }
        })
}

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

/// Run one row; `Err` describes how it missed its op.
async fn prove(row: &Row) -> Result<(), String> {
    let server = MockServer::start().await;
    Mock::given(any())
        .respond_with(template(row.response))
        .mount(&server)
        .await;

    let argv = std::iter::once("drata")
        .chain(row.argv.iter().copied())
        .chain(std::iter::once("--yes"));
    let cli = Cli::try_parse_from(argv).map_err(|e| format!("clap rejected argv: {e}"))?;
    let config = test_config();
    let client = DrataClient::new(config.api_key.clone(), &config.region, true)
        .map_err(|e| format!("client build: {e}"))?
        .with_base_url(server.uri());
    let confirm = default_confirm(cli.yes);

    drata_cli::dispatch(&cli, &client, &config, &confirm)
        .await
        .map_err(|e| format!("dispatch failed: {e:#}"))?;

    let requests = server.received_requests().await.unwrap_or_default();
    if requests.len() != 1 {
        let seen: Vec<String> = requests
            .iter()
            .map(|r| format!("{} {}", r.method, r.url.path()))
            .collect();
        return Err(format!("expected exactly 1 request, saw {}: {seen:?}", requests.len()));
    }
    let req = &requests[0];
    if req.method.as_str() != row.method || !path_matches(row.path, req.url.path()) {
        return Err(format!("sent {} {}", req.method, req.url.path()));
    }
    Ok(())
}

#[tokio::test]
async fn every_row_reaches_its_operation() {
    let mut failures = Vec::new();
    for row in ROWS {
        if let Err(why) = prove(row).await {
            failures.push(format!(
                "{} {} via `{}`: {why}",
                row.method,
                row.path,
                row.argv.join(" ")
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "command-table rows that did not reach their op:\n{}",
        failures.join("\n")
    );
}

#[test]
fn rows_are_one_per_operation() {
    let mut seen = BTreeSet::new();
    for row in ROWS {
        assert!(
            seen.insert((row.method, row.path)),
            "duplicate command-table row for {} {}",
            row.method,
            row.path
        );
    }
}

/// Every leaf subcommand path under `cmd`, as space-joined names.
fn leaves(cmd: &clap::Command, prefix: &[String], out: &mut BTreeSet<String>) {
    let subs: Vec<&clap::Command> = cmd.get_subcommands().collect();
    if subs.is_empty() {
        out.insert(prefix.join(" "));
        return;
    }
    for sub in subs {
        let mut path = prefix.to_vec();
        path.push(sub.get_name().to_string());
        leaves(sub, &path, out);
    }
}

/// The subcommand path an argv selects: descend while the next token names a
/// subcommand of the current command.
fn leaf_of(root: &clap::Command, argv: &[&str]) -> String {
    let mut cmd = root;
    let mut path = Vec::new();
    for token in argv {
        match cmd.find_subcommand(token) {
            Some(sub) => {
                path.push(sub.get_name().to_string());
                cmd = sub;
            }
            None => break,
        }
    }
    path.join(" ")
}

#[test]
fn every_curated_leaf_has_a_row() {
    let root = Cli::command();
    let mut all = BTreeSet::new();
    for top in root.get_subcommands() {
        if UNCURATED.contains(&top.get_name()) {
            continue;
        }
        leaves(top, &[top.get_name().to_string()], &mut all);
    }
    let covered: BTreeSet<String> = ROWS.iter().map(|r| leaf_of(&root, r.argv)).collect();
    let missing: Vec<&String> = all.difference(&covered).collect();
    assert!(
        missing.is_empty(),
        "leaf subcommands with no command-table row: {missing:?}"
    );
}

#[test]
fn path_matches_fills_params_only() {
    assert!(path_matches("/vendors/{vendorId}", "/vendors/1"));
    assert!(!path_matches("/vendors/{vendorId}", "/vendors"));
    assert!(!path_matches("/vendors/{vendorId}", "/vendors/1/documents"));
    assert!(!path_matches(
        "/vendors/{vendorId}/documents",
        "/vendors/1/questionnaires"
    ));
}

#[test]
fn leaf_of_stops_at_first_non_subcommand() {
    let root = Cli::command();
    assert_eq!(
        leaf_of(&root, &["vendor", "security-review", "get", "1", "2"]),
        "vendor security-review get"
    );
    assert_eq!(leaf_of(&root, &["company", "get"]), "company get");
}

/// Dispatch `argv` against a fresh mock and return the one request it sent.
async fn sent(argv: &[&str], response: Response) -> wiremock::Request {
    let server = MockServer::start().await;
    Mock::given(any()).respond_with(template(response)).mount(&server).await;
    let cli = Cli::try_parse_from(
        std::iter::once("drata")
            .chain(argv.iter().copied())
            .chain(std::iter::once("--yes")),
    )
    .expect("argv parses");
    let config = test_config();
    let client = DrataClient::new(config.api_key.clone(), &config.region, true)
        .expect("client builds")
        .with_base_url(server.uri());
    drata_cli::dispatch(&cli, &client, &config, &default_confirm(cli.yes))
        .await
        .expect("dispatch succeeds");
    let mut requests = server.received_requests().await.unwrap_or_default();
    assert_eq!(requests.len(), 1, "exactly one request");
    requests.remove(0)
}

fn body(req: &wiremock::Request) -> serde_json::Value {
    serde_json::from_slice(&req.body).expect("JSON body")
}

#[tokio::test]
async fn vendor_update_sends_vendor_type_id() {
    let req = sent(&["vendor", "update", "1", "--vendor-type-id", "7"], Response::Object).await;
    assert_eq!(body(&req), json!({"vendorTypeId": 7}));
}

#[tokio::test]
async fn risk_create_and_update_send_vendor_id() {
    let req = sent(
        &[
            "risk",
            "create",
            "1",
            "--title",
            "t",
            "--description",
            "d",
            "--vendor-id",
            "9",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(body(&req)["vendorId"], json!(9));
    let req = sent(&["risk", "update", "1", "2", "--vendor-id", "9"], Response::Object).await;
    assert_eq!(body(&req), json!({"vendorId": 9}));
}

#[tokio::test]
async fn audit_get_sends_expand() {
    let req = sent(
        &[
            "audit",
            "get",
            "1",
            "2",
            "--expand",
            "dcfSuggestionsSummary",
            "controls",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(
        req.url.query(),
        Some("expand%5B%5D=dcfSuggestionsSummary&expand%5B%5D=controls")
    );
}

#[tokio::test]
async fn control_create_sends_evidence_defaults() {
    let req = sent(
        &[
            "control",
            "create",
            "1",
            "--name",
            "n",
            "--description",
            "d",
            "--code",
            "C-1",
        ],
        Response::Object,
    )
    .await;
    let b = body(&req);
    assert_eq!(b["externalEvidenceMetadata"], json!("[]"));
    assert_eq!(b["base64Files"], json!("[]"));
}

#[tokio::test]
async fn security_review_update_sends_updated_at() {
    let req = sent(
        &[
            "vendor",
            "security-review",
            "update",
            "1",
            "2",
            "--updated-at",
            "2025-07-01T16:45:55.246Z",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(body(&req), json!({"updatedAt": "2025-07-01T16:45:55.246Z"}));
}
