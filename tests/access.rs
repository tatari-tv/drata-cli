//! `drata access-review ...` on the wire: the `users set` body and its
//! replace-everything prompt, both `/actions` bodies, the period create body,
//! the write guard, and the evidence package as a plain GET.

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
        backend: Backend::Drata,
        api_key: "test-key".to_string(),
        region: Some("us".to_string()),
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
        config.region.as_deref(),
        allow_writes,
    )
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

fn page() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"data": [], "pagination": {"cursor": null}}))
}

fn body(req: &wiremock::Request) -> Value {
    serde_json::from_slice(&req.body).expect("JSON body")
}

/// Every `(method, prompt)` a recording confirm saw.
type Prompts = Arc<Mutex<Vec<(String, String)>>>;

/// A confirm that records every `(method, prompt)` and answers `answer`.
fn recording(answer: bool) -> (ConfirmFn, Prompts) {
    let seen = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
    let log = Arc::clone(&seen);
    let confirm: ConfirmFn = Box::new(move |method: &str, path: &str| {
        log.lock().expect("lock").push((method.to_string(), path.to_string()));
        Ok(answer)
    });
    (confirm, seen)
}

#[tokio::test]
async fn app_users_set_body_carries_every_user_passed_and_nothing_else() {
    let req = sent(
        &[
            "access-review",
            "app",
            "users",
            "set",
            "7",
            "--user",
            "1a2b:jane.doe:jane@acme.com",
            "3c4d:bob:bob@acme.com",
            "5e6f:eve:eve@acme.com",
            "--role",
            "1a2b:Admin",
            "1a2b:ReadOnly",
            "5e6f:Viewer",
        ],
        ResponseTemplate::new(200).set_body_json(json!({"savedCount": 3})),
    )
    .await;
    assert_eq!(req.method.as_str(), "PUT");
    assert_eq!(req.url.path(), "/access-review-applications/7/users");
    assert_eq!(
        body(&req),
        json!({"identities": [
            {"identityId": "1a2b", "username": "jane.doe", "email": "jane@acme.com", "roles": ["Admin", "ReadOnly"]},
            {"identityId": "3c4d", "username": "bob", "email": "bob@acme.com"},
            {"identityId": "5e6f", "username": "eve", "email": "eve@acme.com", "roles": ["Viewer"]},
        ]})
    );
}

#[tokio::test]
async fn app_users_set_prompt_says_omitted_users_are_removed_and_a_no_sends_nothing() {
    let (confirm, seen) = recording(false);
    let r = run(
        &["access-review", "app", "users", "set", "7", "--user", "a:b:c@d.e"],
        object(),
        true,
        confirm,
    )
    .await;
    assert!(r.result.expect_err("declined").to_string().contains("aborted"));
    assert!(r.requests.is_empty(), "a declined users set sends nothing");
    let seen = seen.lock().expect("lock");
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].0, "PUT");
    assert!(
        seen[0].1.starts_with("/access-review-applications/7/users (")
            && seen[0].1.contains("with 1 users")
            && seen[0].1.contains("users omitted from it are removed"),
        "{}",
        seen[0].1
    );
}

#[tokio::test]
async fn app_users_set_data_is_sent_verbatim_and_counted_in_the_prompt() {
    let data = r#"{"identities": [{"identityId": "x", "username": "u", "email": "e@x.io"}, {"identityId": "y", "username": "v", "email": "f@x.io"}]}"#;
    let (confirm, seen) = recording(true);
    let r = run(
        &["access-review", "app", "users", "set", "7", "--data", data],
        object(),
        true,
        confirm,
    )
    .await;
    r.result.expect("dispatch succeeds");
    assert_eq!(body(&r.requests[0]), serde_json::from_str::<Value>(data).expect("json"));
    assert!(seen.lock().expect("lock")[0].1.contains("with 2 users"));
}

#[tokio::test]
async fn app_users_set_rejects_a_repeated_identity_before_any_request() {
    let r = run(
        &[
            "access-review",
            "app",
            "users",
            "set",
            "7",
            "--user",
            "a:b:c@d.e",
            "a:x:y@d.e",
        ],
        object(),
        true,
        default_confirm(true),
    )
    .await;
    assert!(r.result.expect_err("duplicate").to_string().contains("given twice"));
    assert!(r.requests.is_empty());
}

#[tokio::test]
async fn period_run_action_posts_the_action() {
    let req = sent(
        &["access-review", "period", "run-action", "3", "--action", "COMPLETE"],
        object(),
    )
    .await;
    assert_eq!(req.method.as_str(), "POST");
    assert_eq!(req.url.path(), "/access-review-periods/3/actions");
    assert_eq!(body(&req), json!({"action": "complete"}));
}

#[tokio::test]
async fn period_app_run_action_posts_the_action_and_the_sweep_flag() {
    let req = sent(
        &[
            "access-review",
            "period",
            "app",
            "run-action",
            "3",
            "9",
            "--action",
            "complete",
        ],
        object(),
    )
    .await;
    assert_eq!(req.url.path(), "/access-review-periods/3/applications/9/actions");
    assert_eq!(body(&req), json!({"action": "complete"}));

    let (confirm, seen) = recording(true);
    let r = run(
        &[
            "access-review",
            "period",
            "app",
            "run-action",
            "3",
            "9",
            "--action",
            "complete",
            "--mark-unreviewed-out-of-scope",
        ],
        object(),
        true,
        confirm,
    )
    .await;
    r.result.expect("dispatch succeeds");
    assert_eq!(
        body(&r.requests[0]),
        json!({"action": "complete", "markUnreviewedUsersOutOfScope": true})
    );
    assert!(seen.lock().expect("lock")[0].1.contains("OUT_OF_SCOPE"));
}

#[tokio::test]
async fn period_create_scopes_applications_and_warns_of_the_one_active_period() {
    let (confirm, seen) = recording(true);
    let r = run(
        &[
            "access-review",
            "period",
            "create",
            "--starting-date",
            "2026-11-01T00:00:00Z",
            "--ending-date",
            "2026-12-01T00:00:00Z",
            "--application",
            "1:10:11",
            "2:12",
        ],
        object(),
        true,
        confirm,
    )
    .await;
    r.result.expect("dispatch succeeds");
    let req = &r.requests[0];
    assert_eq!(req.url.path(), "/access-review-periods");
    assert_eq!(
        body(req),
        json!({
            "startingDate": "2026-11-01T00:00:00Z",
            "endingDate": "2026-12-01T00:00:00Z",
            "applications": [
                {"applicationId": 1, "reviewersIds": [10, 11]},
                {"applicationId": 2, "reviewersIds": [12]},
            ],
        })
    );
    assert!(seen.lock().expect("lock")[0].1.contains("only one may be active"));
}

#[tokio::test]
async fn app_and_period_app_updates_send_their_bodies() {
    let req = sent(
        &[
            "access-review",
            "app",
            "update",
            "7",
            "--clear-website-url",
            "--reviewer-id",
            "4",
            "5",
        ],
        object(),
    )
    .await;
    assert_eq!(req.url.path(), "/access-review-applications/7");
    assert_eq!(body(&req), json!({"websiteUrl": null, "reviewersIds": [4, 5]}));
    let req = sent(
        &[
            "access-review",
            "app",
            "create",
            "--name",
            "AWS",
            "--reviewer-id",
            "1",
            "2",
        ],
        object(),
    )
    .await;
    assert_eq!(body(&req), json!({"name": "AWS", "reviewersIds": [1, 2]}));
    let req = sent(
        &[
            "access-review",
            "period",
            "app",
            "update",
            "3",
            "9",
            "--reviewer-id",
            "6",
        ],
        object(),
    )
    .await;
    assert_eq!(req.method.as_str(), "PUT");
    assert_eq!(req.url.path(), "/access-review-periods/3/applications/9");
    assert_eq!(body(&req), json!({"reviewersIds": [6]}));
}

#[tokio::test]
async fn reads_are_plain_gets() {
    let req = sent(
        &["access-review", "period", "evidence-package", "3"],
        ResponseTemplate::new(200).set_body_json(json!({
            "signedUrl": "https://somedomain.com/p.zip?Signature=ABC",
            "fileBuffer": null,
            "fileName": null,
        })),
    )
    .await;
    assert_eq!(req.method.as_str(), "GET");
    assert_eq!(req.url.path(), "/access-review-periods/3/evidence-package");
    let req = sent(&["access-review", "period", "list", "--status", "active"], page()).await;
    let q = req.url.query().unwrap_or_default();
    assert!(q.split('&').any(|kv| kv == "status=ACTIVE"), "{q}");
}

#[tokio::test]
async fn mutations_are_blocked_without_a_write_credential() {
    for argv in [
        &["access-review", "app", "remove", "1"][..],
        &["access-review", "app", "users", "set", "1", "--user", "a:b:c@d.e"][..],
        &["access-review", "period", "run-action", "1", "--action", "complete"][..],
        &[
            "access-review",
            "period",
            "app",
            "run-action",
            "1",
            "2",
            "--action",
            "complete",
        ][..],
    ] {
        let r = run(argv, object(), false, default_confirm(true)).await;
        assert!(r.result.is_err(), "{argv:?} must fail");
        assert!(r.requests.is_empty(), "{argv:?} sent a request");
    }
}

#[tokio::test]
async fn app_remove_prompt_names_the_period_side_effect() {
    let (confirm, seen) = recording(false);
    let r = run(&["access-review", "app", "remove", "7"], object(), true, confirm).await;
    assert!(r.result.is_err());
    assert!(r.requests.is_empty());
    let seen = seen.lock().expect("lock");
    assert_eq!(seen[0].0, "DELETE");
    assert!(
        seen[0].1.starts_with("/access-review-applications/7 (") && seen[0].1.contains("review period"),
        "{}",
        seen[0].1
    );
}

#[tokio::test]
async fn a_comma_is_never_a_list_separator() {
    // A comma in a role is part of the one role, sent as a single string.
    let req = sent(
        &[
            "access-review",
            "app",
            "users",
            "set",
            "7",
            "--user",
            "1a2b:jane:jane@acme.com",
            "--role",
            "1a2b:Admin,ReadOnly",
        ],
        object(),
    )
    .await;
    assert_eq!(body(&req)["identities"][0]["roles"], json!(["Admin,ReadOnly"]));
    // A comma between reviewer ids makes one non-numeric id: a clap usage error, no request.
    let err = Cli::try_parse_from([
        "drata",
        "access-review",
        "period",
        "create",
        "--starting-date",
        "a",
        "--ending-date",
        "b",
        "--application",
        "1:10,11",
    ])
    .expect_err("comma-joined reviewers are rejected");
    assert_eq!(err.exit_code(), 2);
    let msg = err.to_string();
    assert!(
        msg.contains("`10,11` is not a numeric reviewer id") && msg.contains("REVIEWER_ID[:REVIEWER_ID...]"),
        "{msg}"
    );
}

#[tokio::test]
async fn a_role_for_a_user_not_in_the_set_fails_before_any_request() {
    let r = run(
        &[
            "access-review",
            "app",
            "users",
            "set",
            "7",
            "--user",
            "1a2b:jane:jane@acme.com",
            "--role",
            "9z:Admin",
        ],
        object(),
        true,
        default_confirm(true),
    )
    .await;
    assert!(
        r.result
            .expect_err("unknown identity")
            .to_string()
            .contains("no --user gives")
    );
    assert!(r.requests.is_empty());
}
