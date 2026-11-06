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
        Response::Accepted => ResponseTemplate::new(202)
            .set_body_json(json!({"securityReviewId": 2, "workflowId": 1, "action": "start_assessment"})),
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

#[tokio::test]
async fn ticket_link_sends_ticket_url_to_the_parent_collection() {
    let req = sent(
        &["risk", "ticket", "link", "1", "2", "--ticket-url", "https://x.test/T-1"],
        Response::Object,
    )
    .await;
    assert_eq!(req.url.path(), "/risk-registers/1/risks/2/tickets");
    assert_eq!(body(&req), json!({"ticketUrl": "https://x.test/T-1"}));
}

#[tokio::test]
async fn ticket_list_encodes_done_filter() {
    let req = sent(
        &["control", "ticket", "list", "1", "2", "--done", "false"],
        Response::Page,
    )
    .await;
    assert_eq!(req.url.path(), "/workspaces/1/controls/2/tickets");
    assert!(req.url.query().unwrap_or_default().contains("isDone=false"));
}

#[tokio::test]
async fn monitor_ticket_create_sends_connection_and_fields() {
    let req = sent(
        &[
            "monitor",
            "ticket",
            "create",
            "1",
            "2",
            "--connection-id",
            "3",
            "--project-id",
            "P",
            "--field",
            "summary",
            "string",
            "Fix it",
            "--field",
            "labels",
            "labels",
            "a",
            "--use-rich-content",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(req.url.path(), "/workspaces/1/monitoring-tests/2/tickets/action-create");
    assert_eq!(
        body(&req),
        json!({
            "connectionId": 3,
            "projectId": "P",
            "fields": [
                {"field": "summary", "type": "string", "value": "Fix it"},
                {"field": "labels", "type": "labels", "value": "a"}
            ],
            "useRichContent": true
        })
    );
}

#[tokio::test]
async fn ticket_create_data_is_the_whole_body() {
    let req = sent(
        &[
            "risk",
            "ticket",
            "create",
            "1",
            "2",
            "--data",
            r#"{"connectionId":9,"fields":[]}"#,
        ],
        Response::Object,
    )
    .await;
    assert_eq!(body(&req), json!({"connectionId": 9, "fields": []}));
}

#[tokio::test]
async fn risk_search_encodes_filters_for_one_register() {
    let req = sent(
        &[
            "risk",
            "search",
            "--register-id",
            "4",
            "--q",
            "access",
            "--status",
            "active",
            "closed",
            "--treatment-plan",
            "mitigate",
            "--type",
            "external",
            "--owner-email",
            "a@x.test",
        ],
        Response::Page,
    )
    .await;
    assert_eq!(req.url.path(), "/risk-registers/4/risks-search");
    let q = req.url.query().unwrap_or_default().to_string();
    for want in [
        "q=access",
        "status%5B%5D=ACTIVE",
        "status%5B%5D=CLOSED",
        "treatmentPlan%5B%5D=MITIGATE",
        "type%5B%5D=EXTERNAL",
        "ownerEmails%5B%5D=a@x.test",
    ] {
        assert!(q.contains(want), "{want} missing from {q}");
    }
    assert!(!q.contains("facets"), "no facets requested: {q}");
}

#[tokio::test]
async fn risk_search_facet_requests_one_page_and_prints_the_envelope() {
    // Page responses carry a null cursor, so a single request is also what the
    // default path sends; the facet assertion is what differs.
    let req = sent(
        &["risk", "search", "--facet", "status", "riskRegisterId"],
        Response::Page,
    )
    .await;
    assert_eq!(req.url.path(), "/risks-search");
    let q = req.url.query().unwrap_or_default().to_string();
    assert!(q.contains("facets%5B%5D=status"), "{q}");
    assert!(q.contains("facets%5B%5D=riskRegisterId"), "{q}");
    assert!(!q.contains("size="), "facet mode fetches the server's one page: {q}");
}

/// Mount `response` on a fresh mock and run a vendor search through the shared
/// search verb; returns the value it would print and the requests it sent.
async fn searched(facets: &[&str], response: serde_json::Value) -> (serde_json::Value, Vec<wiremock::Request>) {
    let server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_json(response))
        .mount(&server)
        .await;
    let client = DrataClient::new("test-key".to_string(), "us", true)
        .expect("client builds")
        .with_base_url(server.uri());
    let facets: Vec<String> = facets.iter().map(|f| f.to_string()).collect();
    let query = drata_cli::query::QueryPath::new("/vendors-search");
    let value = drata_cli::resources::search::fetch(&client, query, &facets)
        .await
        .expect("search succeeds");
    (value, server.received_requests().await.unwrap_or_default())
}

#[tokio::test]
async fn vendor_search_facet_returns_envelope() {
    // The server has a next page; facet mode must still request only one.
    let page = json!({
        "data": [{"id": 1, "name": "Okta"}],
        "pagination": {"cursor": "next"},
        "facets": {"status": [{"value": "ACTIVE", "count": 1}]}
    });
    let (value, requests) = searched(&["status"], page.clone()).await;
    assert_eq!(requests.len(), 1, "one page");
    let q = requests[0].url.query().unwrap_or_default().to_string();
    assert!(q.contains("facets%5B%5D=status"), "{q}");
    assert_eq!(value, page, "the whole envelope, facets included");
    assert!(value.get("facets").is_some());
}

#[tokio::test]
async fn vendor_search_without_facet_walks_every_page_and_drops_the_envelope() {
    let server = MockServer::start().await;
    Mock::given(wiremock::matchers::query_param("cursor", "next"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{"id": 2}], "pagination": {"cursor": null}
        })))
        .mount(&server)
        .await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": [{"id": 1}], "pagination": {"cursor": "next"}
        })))
        .mount(&server)
        .await;
    let client = DrataClient::new("test-key".to_string(), "us", true)
        .expect("client builds")
        .with_base_url(server.uri());
    let value = drata_cli::resources::search::fetch(&client, drata_cli::query::QueryPath::new("/vendors-search"), &[])
        .await
        .expect("search succeeds");
    assert_eq!(value, json!({"data": [{"id": 1}, {"id": 2}]}));
}

#[tokio::test]
async fn vendor_search_encodes_filters() {
    let req = sent(
        &[
            "vendor",
            "search",
            "--q",
            "okta",
            "--status",
            "active",
            "on_hold",
            "--risk",
            "high",
            "--sub-processor",
            "true",
            "--expand",
            "documents",
            "vendorUser",
        ],
        Response::Page,
    )
    .await;
    assert_eq!(req.url.path(), "/vendors-search");
    let q = req.url.query().unwrap_or_default().to_string();
    for want in [
        "q=okta",
        "status%5B%5D=ACTIVE",
        "status%5B%5D=ON_HOLD",
        "risk=HIGH",
        "isSubProcessor=true",
        "expand%5B%5D=documents",
        "expand%5B%5D=vendorUser",
    ] {
        assert!(q.contains(want), "{want} missing from {q}");
    }
}

#[tokio::test]
async fn note_create_sends_text_source_and_assessment() {
    let req = sent(
        &[
            "vendor",
            "security-review",
            "note",
            "create",
            "1",
            "2",
            "--note",
            "hello",
            "--source",
            "PROC-1",
            "--assessment-id",
            "9",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(req.url.path(), "/vendors/1/security-reviews/2/notes");
    assert_eq!(
        body(&req),
        json!({"note": "hello", "source": "PROC-1", "vendorAssessmentId": 9})
    );
}

#[tokio::test]
async fn observation_update_clear_source_sends_null() {
    let req = sent(
        &[
            "vendor",
            "security-review",
            "observation",
            "update",
            "1",
            "2",
            "3",
            "--clear-source",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(req.url.path(), "/vendors/1/security-reviews/2/observations/3");
    assert_eq!(body(&req), json!({"source": null}));
}

#[tokio::test]
async fn assessment_list_encodes_filters() {
    let req = sent(
        &[
            "vendor",
            "security-review",
            "assessment",
            "list",
            "1",
            "2",
            "--criteria-name",
            "mfa",
            "--status",
            "not_met",
            "n_a",
            "--severity",
            "high",
        ],
        Response::Page,
    )
    .await;
    let q = req.url.query().unwrap_or_default().to_string();
    for want in [
        "criteriaName=mfa",
        "statuses%5B%5D=NOT_MET",
        "statuses%5B%5D=N_A",
        "severities%5B%5D=HIGH",
    ] {
        assert!(q.contains(want), "{want} missing from {q}");
    }
}

#[test]
fn example_runs_without_ids_for_every_finding_body() {
    for argv in [
        ["vendor", "security-review", "note", "create", "--example"],
        ["vendor", "security-review", "note", "update", "--example"],
        ["vendor", "security-review", "observation", "create", "--example"],
        ["vendor", "security-review", "observation", "update", "--example"],
    ] {
        let cli = Cli::try_parse_from(std::iter::once("drata").chain(argv)).expect("argv parses");
        let example = drata_cli::example_if_requested(&cli)
            .expect("example requested")
            .expect("skeleton generated");
        assert!(example.trim_start().starts_with('{'), "{argv:?}: {example}");
    }
}

#[test]
fn finding_create_without_ids_or_example_is_a_usage_error() {
    let err = Cli::try_parse_from(["drata", "vendor", "security-review", "note", "create", "--note", "x"])
        .expect_err("ids are required without --example");
    assert_eq!(err.exit_code(), 2);
}

#[tokio::test]
async fn finalize_body_carries_decision_and_note() {
    let req = sent(
        &[
            "vendor",
            "security-review",
            "finalize",
            "1",
            "2",
            "--decision",
            "approved_with_conditions",
            "--note",
            "ok",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(req.url.path(), "/vendors/1/security-reviews/2/finalizations");
    assert_eq!(
        body(&req),
        json!({"decision": "APPROVED_WITH_CONDITIONS", "note": "ok"})
    );
}

#[tokio::test]
async fn confirm_criteria_sends_criterion_ids() {
    let req = sent(
        &[
            "vendor",
            "security-review",
            "confirm-criteria",
            "1",
            "2",
            "--criterion-id",
            "101",
            "102",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(body(&req), json!({"criterionIds": [101, 102]}));
}

#[tokio::test]
async fn stop_assessment_sends_delete_with_an_empty_object() {
    let req = sent(
        &["vendor", "security-review", "stop-assessment", "1", "2"],
        Response::Accepted,
    )
    .await;
    assert_eq!(req.method.as_str(), "DELETE");
    assert_eq!(req.url.path(), "/vendors/1/security-reviews/2/assessment-runs/current");
    assert_eq!(body(&req), json!({}));
}

#[tokio::test]
async fn add_documents_sends_multipart_files_and_type() {
    let req = sent(
        &[
            "vendor",
            "security-review",
            "add-documents",
            "1",
            "2",
            "--file",
            table::FIXTURE,
            table::FIXTURE,
            "--type",
            "bridge_letter",
        ],
        Response::Object,
    )
    .await;
    let content_type = req
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default()
        .to_string();
    assert!(content_type.starts_with("multipart/form-data"), "{content_type}");
    let raw = String::from_utf8_lossy(&req.body);
    assert_eq!(raw.matches("name=\"files\"").count(), 2, "two files parts");
    assert!(raw.contains("name=\"type\"\r\n\r\nBRIDGE_LETTER"), "type field");
}

#[tokio::test]
async fn finalizations_422_fails_and_prints_the_servers_message() {
    let server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(422).set_body_json(json!({
            "name": "AgentActionNotAvailable",
            "statusCode": 422,
            "code": 422,
            "message": "The agent does not offer the finalize action right now.",
            "requestedAction": "finalize",
            "blockers": [{
                "reason": "ASSESSMENT_NOT_COMPLETED",
                "message": "The assessment has not completed.",
                "resolution": {"type": "PUBLIC_ACTION", "action": "start_assessment"}
            }],
            "availableActions": ["start_assessment"],
            "currentStep": "ASSESS"
        })))
        .mount(&server)
        .await;
    let cli = Cli::try_parse_from([
        "drata",
        "vendor",
        "security-review",
        "finalize",
        "1",
        "2",
        "--decision",
        "approved",
        "--yes",
    ])
    .expect("argv parses");
    let config = test_config();
    let client = DrataClient::new(config.api_key.clone(), &config.region, true)
        .expect("client builds")
        .with_base_url(server.uri());
    let err = drata_cli::dispatch(&cli, &client, &config, &default_confirm(cli.yes))
        .await
        .expect_err("a 422 fails the command, so main exits non-zero");
    let msg = format!("{err:#}");
    assert!(msg.contains("422"), "{msg}");
    assert!(
        msg.contains("The agent does not offer the finalize action right now."),
        "{msg}"
    );
    assert!(msg.contains("Blocker ASSESSMENT_NOT_COMPLETED"), "{msg}");
    assert!(msg.contains("Available actions: start_assessment"), "{msg}");
    let requests = server.received_requests().await.unwrap_or_default();
    assert_eq!(requests.len(), 1, "a 422 is not retried");
    assert_eq!(requests[0].url.path(), "/vendors/1/security-reviews/2/finalizations");
}

/// Dispatch `provide-trust-center-access-link` with `--data <data>` against a
/// fresh mock; returns the dispatch result and every request the server saw.
async fn access_link(data: &str) -> (eyre::Result<()>, Vec<wiremock::Request>) {
    let server = MockServer::start().await;
    Mock::given(any())
        .respond_with(template(Response::Accepted))
        .mount(&server)
        .await;
    let cli = Cli::try_parse_from([
        "drata",
        "vendor",
        "security-review",
        "provide-trust-center-access-link",
        "1",
        "2",
        "--data",
        data,
        "--yes",
    ])
    .expect("argv parses");
    let config = test_config();
    let client = DrataClient::new(config.api_key.clone(), &config.region, true)
        .expect("client builds")
        .with_base_url(server.uri());
    let result = drata_cli::dispatch(&cli, &client, &config, &default_confirm(cli.yes)).await;
    (result, server.received_requests().await.unwrap_or_default())
}

#[tokio::test]
async fn access_link_inline_data_is_rejected_before_any_request() {
    let (result, requests) = access_link(r#"{"accessLink":"https://t.example/share?token=s3cret"}"#).await;
    let msg = format!("{:#}", result.expect_err("inline JSON is rejected"));
    assert!(msg.contains("@file") && msg.contains("stdin"), "{msg}");
    assert!(!msg.contains("s3cret"), "the link leaked into the error: {msg}");
    assert!(requests.is_empty(), "server saw {} requests", requests.len());
}

#[tokio::test]
async fn access_link_file_data_sends_the_access_link_body() {
    let (result, requests) = access_link(table::ACCESS_LINK_DATA).await;
    result.expect("dispatch succeeds");
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].url.path(), "/vendors/1/security-reviews/2/access-links");
    assert_eq!(
        body(&requests[0]),
        json!({"accessLink": "https://trust.vendor.test/share?token=fixture"})
    );
}
