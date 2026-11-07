#![allow(clippy::unwrap_used)]
use super::*;
use serde_json::json;
use std::time::Duration;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

fn client(server: &MockServer) -> DrataClient {
    DrataClient::new(Backend::Safebase, "sb-key".to_string(), None, false)
        .unwrap()
        .with_base_url(server.uri())
        .with_page_interval(Duration::ZERO)
}

/// The integer value of query parameter `name` on `req`.
fn query_u32(req: &Request, name: &str) -> u32 {
    req.url
        .query_pairs()
        .find(|(k, _)| k == name)
        .map(|(_, v)| v.parse().unwrap())
        .unwrap_or_else(|| panic!("request {} has no {name}", req.url))
}

/// An `/accounts` page body: `items` under `result`, `totalPages` under
/// `pagination`.
fn accounts_page(items: Value, total_pages: u64) -> Value {
    json!({ "ok": true, "result": items, "pagination": { "totalPages": total_pages } })
}

/// A server answering `/accounts` page N with one item `{"id": "a<N>"}` and
/// `totalPages`.
async fn accounts_server(total_pages: u64) -> MockServer {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .respond_with(move |req: &Request| {
            let n = query_u32(req, "pageNumber");
            ResponseTemplate::new(200).set_body_json(accounts_page(json!([{ "id": format!("a{n}") }]), total_pages))
        })
        .mount(&server)
        .await;
    server
}

async fn requests(server: &MockServer) -> Vec<Request> {
    server.received_requests().await.unwrap()
}

fn page_error(err: &eyre::Report) -> &PageError {
    err.downcast_ref::<PageError>()
        .unwrap_or_else(|| panic!("not a PageError: {err:#}"))
}

// ---------------------------------------------------------------------------
// /accounts
// ---------------------------------------------------------------------------

#[tokio::test]
async fn accounts_three_pages_return_every_item_in_three_requests() {
    let server = accounts_server(3).await;
    let items = client(&server).get_all_pages(&ACCOUNTS, "/accounts").await.unwrap();
    assert_eq!(
        items,
        vec![json!({"id": "a1"}), json!({"id": "a2"}), json!({"id": "a3"})]
    );

    let reqs = requests(&server).await;
    assert_eq!(reqs.len(), 3);
    for (i, req) in reqs.iter().enumerate() {
        assert_eq!(query_u32(req, "pageNumber"), i as u32 + 1);
        assert_eq!(query_u32(req, "pageSize"), 1000);
    }
}

#[tokio::test]
async fn accounts_zero_total_pages_is_empty_success_in_one_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(accounts_page(json!([]), 0)))
        .expect(1)
        .mount(&server)
        .await;
    let items = client(&server).get_all_pages(&ACCOUNTS, "/accounts").await.unwrap();
    assert!(items.is_empty());
}

#[tokio::test]
async fn accounts_exactly_500_pages_succeeds() {
    let server = accounts_server(500).await;
    let items = client(&server).get_all_pages(&ACCOUNTS, "/accounts").await.unwrap();
    assert_eq!(items.len(), 500);
    assert_eq!(items[499], json!({"id": "a500"}));
    assert_eq!(requests(&server).await.len(), 500);
}

#[tokio::test]
async fn accounts_past_page_500_names_the_cap() {
    let server = accounts_server(501).await;
    let err = client(&server).get_all_pages(&ACCOUNTS, "/accounts").await.unwrap_err();
    assert!(matches!(page_error(&err), PageError::PageCap { max: 500, .. }));
    let msg = err.to_string();
    assert!(msg.contains("500-page cap"), "{msg}");
    assert!(msg.contains("501 pages"), "{msg}");
    // Fails on the first page, before 500 pages of work are thrown away.
    assert_eq!(requests(&server).await.len(), 1);
}

#[tokio::test]
async fn accounts_missing_stop_field_is_an_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ok": true, "result": [{"id": "a1"}] })))
        .mount(&server)
        .await;
    let err = client(&server).get_all_pages(&ACCOUNTS, "/accounts").await.unwrap_err();
    assert!(matches!(
        page_error(&err),
        PageError::BadField {
            field: "/pagination/totalPages",
            ..
        }
    ));
    let msg = err.to_string();
    assert!(msg.contains("`/pagination/totalPages` is missing"), "{msg}");
    assert!(msg.contains("pageNumber=1"), "{msg}");
}

#[tokio::test]
async fn accounts_repeated_page_is_an_error() {
    // The server ignores pageNumber and serves page 1 forever.
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(accounts_page(json!([{"id": "a1"}]), 3)))
        .mount(&server)
        .await;
    let err = client(&server).get_all_pages(&ACCOUNTS, "/accounts").await.unwrap_err();
    assert!(matches!(page_error(&err), PageError::RepeatedPage { .. }));
    let msg = err.to_string();
    assert!(msg.contains("repeated page"), "{msg}");
    assert!(msg.contains("pageNumber=2"), "{msg}");
    assert_eq!(requests(&server).await.len(), 2);
}

#[tokio::test]
async fn accounts_empty_page_before_the_stop_is_an_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .respond_with(|req: &Request| {
            let items = match query_u32(req, "pageNumber") {
                1 => json!([{"id": "a1"}]),
                _ => json!([]),
            };
            ResponseTemplate::new(200).set_body_json(accounts_page(items, 3))
        })
        .mount(&server)
        .await;
    let err = client(&server).get_all_pages(&ACCOUNTS, "/accounts").await.unwrap_err();
    assert!(matches!(page_error(&err), PageError::EmptyPage { collected: 1, .. }));
    let msg = err.to_string();
    assert!(msg.contains("empty page before the stop condition"), "{msg}");
    assert!(msg.contains("pageNumber=2"), "{msg}");
}

#[tokio::test]
async fn accounts_items_on_a_zero_page_listing_are_an_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(accounts_page(json!([{"id": "a1"}]), 0)))
        .mount(&server)
        .await;
    let err = client(&server).get_all_pages(&ACCOUNTS, "/accounts").await.unwrap_err();
    assert!(matches!(
        page_error(&err),
        PageError::PastLastPage { total: 0, items: 1, .. }
    ));
}

#[tokio::test]
async fn accounts_mid_stream_failure_names_the_page_and_keeps_written_lines() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/accounts"))
        .respond_with(|req: &Request| match query_u32(req, "pageNumber") {
            1 => ResponseTemplate::new(200).set_body_json(accounts_page(json!([{"id": "a1"}, {"id": "a2"}]), 3)),
            _ => ResponseTemplate::new(500).set_body_json(json!({"statusCode": 500, "message": "boom"})),
        })
        .mount(&server)
        .await;
    let mut out = Vec::new();
    let err = client(&server)
        .stream_all_pages(&ACCOUNTS, "/accounts", &mut out)
        .await
        .unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("pageNumber=2"), "{msg}");
    assert!(msg.contains("after 2 items"), "{msg}");
    assert_eq!(String::from_utf8(out).unwrap(), "{\"id\":\"a1\"}\n{\"id\":\"a2\"}\n");
}

#[tokio::test]
async fn accounts_stream_writes_one_line_per_item() {
    let server = accounts_server(2).await;
    let mut out = Vec::new();
    let n = client(&server)
        .stream_all_pages(&ACCOUNTS, "/accounts", &mut out)
        .await
        .unwrap();
    assert_eq!(n, 2);
    assert_eq!(String::from_utf8(out).unwrap(), "{\"id\":\"a1\"}\n{\"id\":\"a2\"}\n");
}

// ---------------------------------------------------------------------------
// /documents
// ---------------------------------------------------------------------------

fn documents_page(items: Value, count: Value) -> Value {
    json!({ "ok": true, "result": { "documents": items, "count": count } })
}

#[tokio::test]
async fn documents_page_from_zero_and_stop_on_count() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/documents"))
        .respond_with(|req: &Request| {
            let items = match query_u32(req, "page") {
                0 => json!([{"id": "d1"}, {"id": "d2"}]),
                1 => json!([{"id": "d3"}, {"id": "d4"}]),
                2 => json!([{"id": "d5"}]),
                _ => json!([]),
            };
            ResponseTemplate::new(200).set_body_json(documents_page(items, json!(5)))
        })
        .mount(&server)
        .await;
    let items = client(&server)
        .get_all_pages(&DOCUMENTS, "/documents?query=soc%202")
        .await
        .unwrap();
    assert_eq!(items.len(), 5);
    assert_eq!(items[4], json!({"id": "d5"}));

    let reqs = requests(&server).await;
    assert_eq!(reqs.len(), 3);
    for (i, req) in reqs.iter().enumerate() {
        assert_eq!(query_u32(req, "page"), i as u32);
        assert_eq!(query_u32(req, "limit"), 100);
        assert!(req.url.query_pairs().any(|(k, v)| k == "query" && v == "soc 2"));
    }
}

#[tokio::test]
async fn documents_zero_count_is_empty_success_in_one_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/documents"))
        .respond_with(ResponseTemplate::new(200).set_body_json(documents_page(json!([]), json!(0))))
        .expect(1)
        .mount(&server)
        .await;
    assert!(
        client(&server)
            .get_all_pages(&DOCUMENTS, "/documents")
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn documents_collected_over_count_is_an_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/documents"))
        .respond_with(|req: &Request| {
            let n = query_u32(req, "page");
            let items = json!([{"id": format!("d{n}a")}, {"id": format!("d{n}b")}]);
            ResponseTemplate::new(200).set_body_json(documents_page(items, json!(3)))
        })
        .mount(&server)
        .await;
    let err = client(&server)
        .get_all_pages(&DOCUMENTS, "/documents")
        .await
        .unwrap_err();
    assert!(matches!(
        page_error(&err),
        PageError::CountExceeded {
            collected: 4,
            count: 3,
            ..
        }
    ));
    let msg = err.to_string();
    assert!(msg.contains("more than count 3"), "{msg}");
    assert!(msg.contains("page=1"), "{msg}");
}

#[tokio::test]
async fn documents_hard_page_cap_is_an_error() {
    let capped = PagedOp {
        max_pages: 2,
        ..DOCUMENTS
    };
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/documents"))
        .respond_with(|req: &Request| {
            let n = query_u32(req, "page");
            ResponseTemplate::new(200).set_body_json(documents_page(json!([{ "id": n }]), json!(10)))
        })
        .mount(&server)
        .await;
    let err = client(&server).get_all_pages(&capped, "/documents").await.unwrap_err();
    assert!(matches!(page_error(&err), PageError::PageCap { max: 2, .. }));
    let msg = err.to_string();
    assert!(msg.contains("2-page cap (the client's hard page cap)"), "{msg}");
    assert!(msg.contains("2 items collected"), "{msg}");
    assert_eq!(requests(&server).await.len(), 2);
}

#[test]
fn documents_and_kb_search_cap_at_max_pages() {
    assert_eq!(DOCUMENTS.max_pages, MAX_PAGES);
    assert_eq!(KB_SEARCH.max_pages, MAX_PAGES);
    assert_eq!(ACCOUNTS.max_pages, 500);
}

// ---------------------------------------------------------------------------
// /kb/search
// ---------------------------------------------------------------------------

#[tokio::test]
async fn kb_search_stops_on_total_pages_not_the_per_page_count() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/kb/search"))
        .respond_with(|req: &Request| {
            let n = query_u32(req, "pageNumber");
            // `result.count` is this page's count (1), never the total.
            ResponseTemplate::new(200).set_body_json(json!({
                "ok": true,
                "result": { "items": [{ "id": format!("k{n}") }], "count": 1 },
                "pagination": { "pageNumber": n, "totalPages": 2, "totalCount": 2 }
            }))
        })
        .mount(&server)
        .await;
    let items = client(&server)
        .get_all_pages(&KB_SEARCH, "/kb/search?query=soc")
        .await
        .unwrap();
    assert_eq!(items, vec![json!({"id": "k1"}), json!({"id": "k2"})]);
    let reqs = requests(&server).await;
    assert_eq!(reqs.len(), 2);
    assert_eq!(query_u32(&reqs[1], "pageNumber"), 2);
    assert_eq!(query_u32(&reqs[1], "pageSize"), 1000);
}

// ---------------------------------------------------------------------------
// Page shape (pure)
// ---------------------------------------------------------------------------

#[test]
fn items_field_missing_or_mistyped_is_an_error() {
    for body in [
        json!({ "ok": true, "pagination": { "totalPages": 1 } }),
        json!({ "ok": true, "result": { "id": "a1" }, "pagination": { "totalPages": 1 } }),
    ] {
        let err = ACCOUNTS.check("/accounts", 0, 0, None, &body).err().unwrap();
        assert!(
            matches!(page_error(&err), PageError::BadField { field: "/result", .. }),
            "{err}"
        );
        assert!(err.to_string().contains("not an array"), "{err}");
    }
}

#[test]
fn stop_field_null_or_mistyped_is_an_error() {
    // Without pageNumber, SafeBase sends `totalPages: null`.
    for total in [Value::Null, json!("3"), json!(-1)] {
        let body = json!({ "ok": true, "result": [{"id": "a1"}], "pagination": { "totalPages": total } });
        let err = ACCOUNTS.check("/accounts", 0, 0, None, &body).err().unwrap();
        assert!(matches!(page_error(&err), PageError::BadField { .. }), "{err}");
    }
    let body = json!({ "ok": true, "result": { "documents": [{"id": "d1"}], "count": "5" } });
    let err = DOCUMENTS.check("/documents", 0, 0, None, &body).err().unwrap();
    assert!(matches!(
        page_error(&err),
        PageError::BadField {
            field: "/result/count",
            ..
        }
    ));
}

#[test]
fn page_path_appends_to_an_existing_query() {
    assert_eq!(
        ACCOUNTS.page_path("/accounts", 0),
        "/accounts?pageNumber=1&pageSize=1000"
    );
    assert_eq!(
        DOCUMENTS.page_path("/documents?query=x", 2),
        "/documents?query=x&page=2&limit=100"
    );
}

#[tokio::test]
async fn drata_client_and_wrong_path_are_refused() {
    let server = MockServer::start().await;
    let drata = DrataClient::new(Backend::Drata, "k".to_string(), Some("us"), false)
        .unwrap()
        .with_base_url(server.uri());
    let err = drata.get_all_pages(&ACCOUNTS, "/accounts").await.unwrap_err();
    assert!(err.to_string().contains("SafeBase only"), "{err}");

    let err = client(&server)
        .get_all_pages(&ACCOUNTS, "/documents")
        .await
        .unwrap_err();
    assert!(err.to_string().contains("handed the path /documents"), "{err}");
    assert!(requests(&server).await.is_empty());
}
