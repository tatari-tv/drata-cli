//! Curated SafeBase reads: one wiremock test per verb asserting the method,
//! path, query and the rendered rows. Verbs run through
//! `resources::safebase::fetch` from parsed argv, the way `main` dispatches.

use clap::Parser;
use drata_cli::backend::Backend;
use drata_cli::cli::{Cli, Commands};
use drata_cli::client::DrataClient;
use drata_cli::resources::safebase::{Reply, fetch};
use serde_json::{Value, json};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const SB_KEY: &str = "sb-test-key";
const WIDTH: usize = 200;

fn client(server: &MockServer) -> DrataClient {
    DrataClient::new(Backend::Safebase, SB_KEY.to_string(), None, false)
        .expect("client builds")
        .with_base_url(server.uri())
}

/// Run `drata safebase <argv>` against the server; also returns NDJSON written
/// by `--all`.
async fn run(server: &MockServer, argv: &[&str]) -> eyre::Result<(Reply, String)> {
    let mut full = vec!["drata", "safebase"];
    full.extend_from_slice(argv);
    let cli = Cli::try_parse_from(full).expect("argv parses");
    let Commands::Safebase { action } = cli.command else {
        panic!("not a safebase command");
    };
    let mut sink = Vec::new();
    let reply = fetch(&action, &client(server), &mut sink).await?;
    Ok((reply, String::from_utf8(sink).expect("utf8")))
}

fn ok(result: Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"ok": true, "result": result}))
}

fn table_of(reply: &Reply) -> String {
    reply.table_text(WIDTH).expect("a table reply")
}

async fn only_request(server: &MockServer) -> wiremock::Request {
    let requests = server.received_requests().await.expect("recording on");
    assert_eq!(requests.len(), 1, "{requests:?}");
    requests.into_iter().next().unwrap()
}

#[tokio::test]
async fn account_list_gets_every_page_and_renders_rows() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .and(query_param("pageNumber", "1"))
        .and(query_param("pageSize", "1000"))
        .and(header("x-sb-api-key", SB_KEY))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": true,
            "result": [{"id": "a-1", "name": "Acme", "domains": ["acme.com", "acme.io"],
                        "numMembers": 3, "isNdaDone": true, "lastSeen": "2026-11-01T00:00:00Z"}],
            "pagination": {"pageNumber": 1, "pageSize": 1000, "totalCount": 1, "totalPages": 1}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["account", "list"]).await.expect("list ok");
    let text = table_of(&reply);
    assert!(text.starts_with("ID"), "{text}");
    for cell in ["a-1", "Acme", "acme.com,acme.io", "3", "true", "2026-11-01T00:00:00Z"] {
        assert!(text.contains(cell), "missing {cell} in\n{text}");
    }
}

#[tokio::test]
async fn account_list_all_streams_ndjson_items() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": true,
            "result": [{"id": "a-1"}, {"id": "a-2"}],
            "pagination": {"totalPages": 1}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let (reply, ndjson) = run(&server, &["account", "list", "--all"]).await.expect("stream ok");
    assert!(matches!(reply, Reply::Streamed(2)), "{reply:?}");
    assert_eq!(ndjson, "{\"id\":\"a-1\"}\n{\"id\":\"a-2\"}\n");
}

#[tokio::test]
async fn account_get_reads_one_account() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/a-1"))
        .respond_with(ok(
            json!({"id": "a-1", "name": "Acme", "numMembers": 2, "isNdaDone": false}),
        ))
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["account", "get", "a-1"]).await.expect("get ok");
    let text = table_of(&reply);
    assert!(
        text.contains("a-1") && text.contains("Acme") && text.contains("false"),
        "{text}"
    );
    assert_eq!(only_request(&server).await.url.query(), None);
}

#[tokio::test]
async fn account_url_prints_the_result_string() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/a-1/url"))
        .respond_with(ok(json!("https://trust.tatari.tv/?token=t")))
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["account", "url", "a-1"]).await.expect("url ok");
    assert!(
        matches!(&reply, Reply::Text(t) if t == "https://trust.tatari.tv/?token=t"),
        "{reply:?}"
    );
}

#[tokio::test]
async fn account_url_without_a_string_result_fails_loudly() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/a-1/url"))
        .respond_with(ok(json!({"url": "x"})))
        .mount(&server)
        .await;

    let err = run(&server, &["account", "url", "a-1"])
        .await
        .expect_err("not a string");
    assert!(err.to_string().contains("no string `result`"), "{err}");
}

#[tokio::test]
async fn member_list_sends_both_filters_and_renders_rows() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/members"))
        .and(query_param("email", "pat@acme.com"))
        .and(query_param("sfdcAccountId", "001XX"))
        .respond_with(ok(json!([{"accountId": "a-1", "firstName": "Pat", "lastName": "Lee",
                                 "email": "pat@acme.com", "lastSeen": "2026-11-02T00:00:00Z"}])))
        .expect(1)
        .mount(&server)
        .await;

    let (reply, _) = run(
        &server,
        &[
            "member",
            "list",
            "--email",
            "pat@acme.com",
            "--sfdc-account-id",
            "001XX",
        ],
    )
    .await
    .expect("list ok");
    let text = table_of(&reply);
    for cell in ["a-1", "Pat Lee", "pat@acme.com", "2026-11-02T00:00:00Z"] {
        assert!(text.contains(cell), "missing {cell} in\n{text}");
    }
}

#[tokio::test]
async fn member_list_without_filters_sends_no_query() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/members"))
        .respond_with(ok(
            json!([{"accountId": "a-1", "name": "Legacy Name", "email": "l@x.co"}]),
        ))
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["member", "list"]).await.expect("list ok");
    assert!(table_of(&reply).contains("Legacy Name"));
    assert_eq!(only_request(&server).await.url.query(), None);
}

#[tokio::test]
async fn document_list_pages_from_zero_with_query_and_renders_rows() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/documents"))
        .and(query_param("query", "soc 2"))
        .and(query_param("page", "0"))
        .and(query_param("limit", "100"))
        .respond_with(ok(json!({"count": 1, "documents": [{
            "id": "d-1", "name": "SOC 2 Type II", "type": "report", "size": 552,
            "expiresAt": null, "uploadStatus": "complete"}]})))
        .expect(1)
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["document", "list", "--query", "soc 2"])
        .await
        .expect("list ok");
    let text = table_of(&reply);
    for cell in ["d-1", "SOC 2 Type II", "report", "552", "complete"] {
        assert!(text.contains(cell), "missing {cell} in\n{text}");
    }
}

#[tokio::test]
async fn document_get_uses_the_document_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/document/d-1"))
        .respond_with(ok(
            json!({"id": "d-1", "name": "Pentest letter", "uploadStatus": "complete"}),
        ))
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["document", "get", "d-1"]).await.expect("get ok");
    assert!(table_of(&reply).contains("Pentest letter"));
    assert_eq!(only_request(&server).await.url.query(), None);
}

#[tokio::test]
async fn document_get_external_sends_type_external() {
    let server = MockServer::start().await;
    let ext = "6f1c9a7e-0000-4000-8000-000000000001";
    Mock::given(method("GET"))
        .and(path(format!("/document/{ext}")))
        .and(query_param("type", "external"))
        .respond_with(ok(json!({"id": "d-9", "externalId": ext, "name": "By external id"})))
        .expect(1)
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["document", "get", ext, "--external"])
        .await
        .expect("get ok");
    assert!(table_of(&reply).contains("By external id"));
}

#[tokio::test]
async fn document_types_renders_the_items() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/document-types"))
        .respond_with(ok(
            json!({"items": [{"id": "t-1", "name": "SOC 2 report"}], "count": 1}),
        ))
        .expect(1)
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["document", "types"]).await.expect("types ok");
    let text = table_of(&reply);
    assert!(
        text.starts_with("ID") && text.contains("t-1") && text.contains("SOC 2 report"),
        "{text}"
    );
}

#[tokio::test]
async fn document_types_without_items_fails_loudly() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/document-types"))
        .respond_with(ok(json!({"count": 0})))
        .mount(&server)
        .await;

    let err = run(&server, &["document", "types"]).await.expect_err("no items");
    assert!(err.to_string().contains("no `result/items` array"), "{err}");
}

#[tokio::test]
async fn request_list_sends_status_and_renders_rows() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/requests"))
        .and(query_param("status", "pending"))
        .respond_with(ok(json!([{"id": "r-1", "firstName": "Sam", "lastName": "Ng",
                                 "email": "sam@co.com", "companyName": "Co", "status": "pending",
                                 "createdAt": "2026-11-03T00:00:00Z"}])))
        .expect(1)
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["request", "list", "--status", "pending"])
        .await
        .expect("list ok");
    let text = table_of(&reply);
    for cell in ["r-1", "Sam Ng", "sam@co.com", "Co", "pending", "2026-11-03T00:00:00Z"] {
        assert!(text.contains(cell), "missing {cell} in\n{text}");
    }
}

#[tokio::test]
async fn request_list_rejects_a_status_outside_the_spec_enum() {
    let cli = Cli::try_parse_from(["drata", "safebase", "request", "list", "--status", "open"]);
    assert!(cli.is_err());
}

#[tokio::test]
async fn kb_search_sends_query_and_page_params_and_renders_rows() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/kb/search"))
        .and(query_param("query", "encryption"))
        .and(query_param("pageNumber", "1"))
        .and(query_param("pageSize", "1000"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": true,
            "result": {"count": 1, "items": [{"id": "k-1", "source": "portal", "accessLevel": "public",
                "content": "We encrypt\ndata at rest.", "modifiedAt": "2026-11-04T00:00:00Z"}]},
            "pagination": {"pageNumber": 1, "pageSize": 1000, "totalCount": 1, "totalPages": 1}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["kb", "search", "encryption"]).await.expect("search ok");
    let text = table_of(&reply);
    for cell in [
        "k-1",
        "portal",
        "public",
        "We encrypt data at rest.",
        "2026-11-04T00:00:00Z",
    ] {
        assert!(text.contains(cell), "missing {cell} in\n{text}");
    }
}

#[tokio::test]
async fn kb_search_without_a_query_omits_it() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/kb/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": true,
            "result": {"count": 0, "items": []},
            "pagination": {"totalPages": 0}
        })))
        .expect(1)
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["kb", "search"]).await.expect("search ok");
    assert_eq!(table_of(&reply), "(no results)");
    let req = only_request(&server).await;
    assert!(!req.url.query().unwrap_or("").contains("query="), "{:?}", req.url);
}

#[tokio::test]
async fn kb_search_all_streams_ndjson() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/kb/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "ok": true,
            "result": {"count": 1, "items": [{"id": "k-1"}]},
            "pagination": {"totalPages": 1}
        })))
        .mount(&server)
        .await;

    let (reply, ndjson) = run(&server, &["kb", "search", "x", "--all"]).await.expect("stream ok");
    assert!(matches!(reply, Reply::Streamed(1)), "{reply:?}");
    assert_eq!(ndjson, "{\"id\":\"k-1\"}\n");
}

#[tokio::test]
async fn ok_only_response_prints_ok() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/requests"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
        .mount(&server)
        .await;

    let (reply, _) = run(&server, &["request", "list"]).await.expect("ok");
    assert!(matches!(reply, Reply::Ok), "{reply:?}");
}

#[tokio::test]
async fn api_error_surfaces_as_a_failure() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts/a-1"))
        .respond_with(ResponseTemplate::new(404).set_body_json(
            json!({"statusCode": 404, "cause": "not_found", "name": "ApiError", "message": "Not found", "error": "not_found"}),
        ))
        .mount(&server)
        .await;

    let err = run(&server, &["account", "get", "a-1"]).await.expect_err("404");
    assert!(err.to_string().contains("Not found"), "{err}");
}
