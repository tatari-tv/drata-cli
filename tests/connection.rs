//! `drata connection ...` on the wire: the credential-carrying bodies come only
//! from `@file` or stdin, inline JSON is refused before any request, and a body
//! reaches the server unchanged.

#[allow(dead_code)]
#[path = "commands/table.rs"]
mod table;

use clap::Parser;
use drata_cli::cli::{Cli, OutputFormat};
use drata_cli::client::DrataClient;
use drata_cli::config::{Config, TokenSource};
use drata_cli::confirm::default_confirm;
use serde_json::json;
use wiremock::matchers::any;
use wiremock::{Mock, MockServer, ResponseTemplate};

fn test_config(allow_writes: bool) -> Config {
    Config {
        api_key: "test-key".to_string(),
        region: "us".to_string(),
        allow_writes,
        profile: "test".to_string(),
        output_format: OutputFormat::Json,
        token_source: TokenSource::CliFlag,
    }
}

/// Dispatch `drata <argv> --yes` against a fresh mock answering `{"id": 1}`.
async fn dispatch(argv: &[&str], allow_writes: bool) -> (eyre::Result<()>, Vec<wiremock::Request>) {
    let server = MockServer::start().await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": 1})))
        .mount(&server)
        .await;
    let cli = Cli::try_parse_from(
        std::iter::once("drata")
            .chain(argv.iter().copied())
            .chain(std::iter::once("--yes")),
    )
    .expect("argv parses");
    let config = test_config(allow_writes);
    let client = DrataClient::new(config.api_key.clone(), &config.region, allow_writes)
        .expect("client builds")
        .with_base_url(server.uri());
    let result = drata_cli::dispatch(&cli, &client, &config, &default_confirm(cli.yes)).await;
    (result, server.received_requests().await.unwrap_or_default())
}

#[tokio::test]
async fn inline_data_is_rejected_before_any_request_for_every_provider_and_verb() {
    let inline = r#"{"k":"v-s3cret"}"#;
    for provider in ["aws", "azure", "gcp", "gitlab"] {
        for argv in [
            vec!["connection", provider, "create", "--data", inline],
            vec!["connection", provider, "update", "1", "--data", inline],
        ] {
            let (result, requests) = dispatch(&argv, true).await;
            let msg = format!("{:#}", result.expect_err("inline JSON is rejected"));
            assert!(msg.contains("@file") && msg.contains("stdin"), "{msg}");
            assert!(!msg.contains("s3cret"), "the body leaked into the error: {msg}");
            assert!(requests.is_empty(), "{argv:?}: server saw {} requests", requests.len());
        }
    }
}

#[tokio::test]
async fn file_data_is_sent_unchanged_on_create_and_update() {
    let (result, requests) = dispatch(
        &["connection", "azure", "create", "--data", table::CONNECTION_DATA],
        true,
    )
    .await;
    result.expect("create succeeds");
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method.as_str(), "POST");
    assert_eq!(requests[0].url.path(), "/azure-connections");
    let want =
        json!({"name": "fixture connection", "applicationSecret": "fixture-not-a-real-secret", "workspaceIds": [1]});
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&requests[0].body).unwrap(),
        want
    );

    let (result, requests) = dispatch(
        &["connection", "gcp", "update", "5", "--data", table::CONNECTION_DATA],
        true,
    )
    .await;
    result.expect("update succeeds");
    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].method.as_str(), "PUT");
    assert_eq!(requests[0].url.path(), "/gcp-connections/5");
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&requests[0].body).unwrap(),
        want
    );
}

#[tokio::test]
async fn a_read_only_credential_blocks_create_before_any_request() {
    let (result, requests) = dispatch(
        &["connection", "aws", "create", "--data", table::CONNECTION_DATA],
        false,
    )
    .await;
    assert!(result.is_err());
    assert!(requests.is_empty());
}

#[test]
fn example_needs_no_data_or_id_and_data_is_required_otherwise() {
    for provider in ["aws", "azure", "gcp", "gitlab"] {
        assert!(Cli::try_parse_from(["drata", "connection", provider, "create", "--example"]).is_ok());
        assert!(Cli::try_parse_from(["drata", "connection", provider, "update", "--example"]).is_ok());
        assert!(Cli::try_parse_from(["drata", "connection", provider, "create"]).is_err());
        assert!(Cli::try_parse_from(["drata", "connection", provider, "update", "1"]).is_err());
    }
}

#[tokio::test]
async fn get_update_and_remove_send_a_numeric_or_prefixed_id_as_one_path_segment() {
    let cases: [(Vec<&str>, &str, &str); 6] = [
        (
            vec!["connection", "gitlab", "get", "42"],
            "GET",
            "/gitlab-connections/42",
        ),
        (
            vec![
                "connection",
                "aws",
                "get",
                "roleArn:arn:aws:iam::123456789012:role/DrataAuditRole",
            ],
            "GET",
            "/aws-connections/roleArn:arn:aws:iam::123456789012:role%2FDrataAuditRole",
        ),
        (
            vec![
                "connection",
                "azure",
                "update",
                "subscriptionId:28ffcc9b-be6d-4bbf-b53a-057049debc38",
                "--data",
                table::CONNECTION_DATA,
            ],
            "PUT",
            "/azure-connections/subscriptionId:28ffcc9b-be6d-4bbf-b53a-057049debc38",
        ),
        (
            vec!["connection", "gcp", "remove", "projectId:my-project"],
            "DELETE",
            "/gcp-connections/projectId:my-project",
        ),
        (
            vec!["connection", "aws", "remove", "10"],
            "DELETE",
            "/aws-connections/10",
        ),
        (
            vec!["connection", "gcp", "update", "7", "--data", table::CONNECTION_DATA],
            "PUT",
            "/gcp-connections/7",
        ),
    ];
    for (argv, method, path) in cases {
        let (result, requests) = dispatch(&argv, true).await;
        result.unwrap_or_else(|e| panic!("{argv:?}: {e:#}"));
        assert_eq!(requests.len(), 1, "{argv:?}");
        assert_eq!(requests[0].method.as_str(), method, "{argv:?}");
        assert_eq!(requests[0].url.path(), path, "{argv:?}");
    }
}

#[tokio::test]
async fn an_id_outside_the_providers_forms_is_refused_before_any_request() {
    for argv in [
        vec!["connection", "gitlab", "get", "projectId:my-project"],
        vec!["connection", "aws", "remove", "projectId:my-project"],
        vec![
            "connection",
            "azure",
            "update",
            "roleArn:x",
            "--data",
            table::CONNECTION_DATA,
        ],
    ] {
        let (result, requests) = dispatch(&argv, true).await;
        assert!(result.is_err(), "{argv:?}: accepted");
        assert!(requests.is_empty(), "{argv:?}: server saw {} requests", requests.len());
    }
}
