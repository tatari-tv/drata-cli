//! `drata policy version create | replace-file` and `drata policy-language` on
//! the wire: the multipart shape (field names are the spec's), the JSON shape
//! for EXTERNAL sources, and the kept `policy version <policy> <version>` get.

use clap::Parser;
use drata_cli::cli::{Cli, OutputFormat};
use drata_cli::client::DrataClient;
use drata_cli::config::{Config, TokenSource};
use drata_cli::confirm::default_confirm;
use serde_json::json;
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

#[derive(Clone, Copy)]
enum Response {
    Object,
    Page,
}

async fn sent(argv: &[&str], response: Response) -> wiremock::Request {
    let server = MockServer::start().await;
    let template = match response {
        Response::Object => ResponseTemplate::new(200).set_body_json(json!({"id": 1})),
        Response::Page => ResponseTemplate::new(200).set_body_json(json!({"data": [], "pagination": {"cursor": null}})),
    };
    Mock::given(any()).respond_with(template).mount(&server).await;
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

fn multipart_text(req: &wiremock::Request) -> String {
    let content_type = req
        .headers
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_default();
    assert!(content_type.starts_with("multipart/form-data"), "{content_type}");
    String::from_utf8_lossy(&req.body).into_owned()
}

#[tokio::test]
async fn policy_version_replace_file_puts_multipart_with_the_specs_file_field() {
    let req = sent(
        &[
            "policy",
            "version",
            "replace-file",
            "7",
            "9",
            "--file",
            FIXTURE,
            "--language",
            "fr-fr",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(req.method.as_str(), "PUT");
    assert_eq!(req.url.path(), "/policies/7/policy-versions/9/file");
    let raw = multipart_text(&req);
    assert_eq!(raw.matches("name=\"file\"").count(), 1, "one file part");
    assert!(
        raw.contains("name=\"language\"\r\n\r\nfr-FR"),
        "language normalized to the spec value"
    );
}

#[tokio::test]
async fn policy_version_create_uploaded_posts_multipart() {
    let req = sent(
        &[
            "policy",
            "version",
            "create",
            "7",
            "--source-type",
            "uploaded",
            "--owner-id",
            "42",
            "--material-change",
            "true",
            "--renewal-date",
            "2027-01-01",
            "--file",
            FIXTURE,
            "--change-summary",
            "typo",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(req.method.as_str(), "POST");
    assert_eq!(req.url.path(), "/policies/7/policy-versions");
    let raw = multipart_text(&req);
    assert_eq!(raw.matches("name=\"file\"").count(), 1);
    assert!(raw.contains("name=\"sourceType\"\r\n\r\nUPLOADED"));
    assert!(raw.contains("name=\"isMaterialChange\"\r\n\r\ntrue"));
    assert!(raw.contains("name=\"changeSummary\"\r\n\r\ntypo"));
}

#[tokio::test]
async fn policy_version_create_external_posts_typed_json() {
    let req = sent(
        &[
            "policy",
            "version",
            "create",
            "7",
            "--source-type",
            "EXTERNAL",
            "--external-file-id",
            "2055",
            "--owner-id",
            "42",
            "--material-change",
            "false",
            "--renewal-date",
            "2027-01-01",
        ],
        Response::Object,
    )
    .await;
    assert_eq!(req.url.path(), "/policies/7/policy-versions");
    assert_eq!(
        body(&req),
        json!({
            "sourceType": "EXTERNAL",
            "ownerId": 42,
            "isMaterialChange": false,
            "renewalDate": "2027-01-01",
            "externalFileId": 2055,
        })
    );
}

#[tokio::test]
async fn policy_version_with_two_ids_still_gets_the_version() {
    let req = sent(&["policy", "version", "7", "9"], Response::Object).await;
    assert_eq!(req.method.as_str(), "GET");
    assert_eq!(req.url.path(), "/policies/7/policy-versions/9");
}

#[tokio::test]
async fn policy_language_list_and_versions_send_their_filters() {
    let req = sent(
        &["policy-language", "list", "7", "9", "--status", "draft", "published"],
        Response::Page,
    )
    .await;
    assert_eq!(req.url.path(), "/policies/7/policy-versions/9/languages");
    assert!(
        req.url.query().unwrap().contains("statuses%5B%5D=DRAFT"),
        "{:?}",
        req.url.query()
    );
    let req = sent(
        &["policy-language", "versions", "--policy-id", "7", "--expand", "policy"],
        Response::Page,
    )
    .await;
    let q = req.url.query().unwrap();
    assert!(q.contains("policyId=7") && q.contains("expand%5B%5D=policy"), "{q}");
}
