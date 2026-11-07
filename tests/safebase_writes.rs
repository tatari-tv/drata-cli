//! Curated SafeBase writes against wiremock. Upload runs across two servers:
//! one plays the SafeBase API, the other the signed storage URL it hands back.
//! Every other write verb gets one test asserting the exact JSON body.

use clap::Parser;
use drata_cli::backend::Backend;
use drata_cli::cli::{Cli, Commands};
use drata_cli::client::{DrataClient, WriteGuardError};
use drata_cli::confirm::{ConfirmFn, always_no, always_yes};
use drata_cli::resources::safebase::upload::{UploadError, md5b64};
use drata_cli::resources::safebase::{Reply, execute};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::time::Duration;
use uuid::Uuid;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, Request, ResponseTemplate};

const SB_KEY: &str = "sb-test-key";
const FILE_ID: &str = "0b5a7c3e-1111-4222-8333-444455556666";
const EXTERNAL_ID: &str = "6f9619ff-8b86-4011-b42d-00c04fc964ff";
const SIGNED_QUERY: &str = "X-Goog-Algorithm=GOOG4-RSA-SHA256&X-Goog-Signature=cafef00d";
/// `tests/fixtures/zzz-clitest.pdf`: its size and md5b64, from
/// `wc -c tests/fixtures/zzz-clitest.pdf` and
/// `openssl dgst -md5 -binary tests/fixtures/zzz-clitest.pdf | base64`.
const FIXTURE_SIZE: usize = 134;
const FIXTURE_MD5B64: &str = "/nZ6d/ougolRF+QdP/1xsw==";

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/zzz-clitest.pdf")
}

fn writer(server: &MockServer) -> DrataClient {
    DrataClient::new(Backend::Safebase, SB_KEY.to_string(), None, true)
        .expect("client builds")
        .with_base_url(server.uri())
}

async fn run_with(client: &DrataClient, confirm: &ConfirmFn, argv: &[&str]) -> eyre::Result<Reply> {
    let mut full = vec!["drata", "safebase"];
    full.extend_from_slice(argv);
    let cli = Cli::try_parse_from(full).expect("argv parses");
    let Commands::Safebase { action } = cli.command else {
        panic!("not a safebase command");
    };
    execute(&action, client, confirm, &mut Vec::new()).await
}

async fn run(server: &MockServer, argv: &[&str]) -> eyre::Result<Reply> {
    run_with(&writer(server), &always_yes(), argv).await
}

async fn requests(server: &MockServer) -> Vec<Request> {
    server.received_requests().await.expect("recording on")
}

async fn only_request(server: &MockServer) -> Request {
    let all = requests(server).await;
    assert_eq!(all.len(), 1, "{all:?}");
    all.into_iter().next().expect("one request")
}

fn body_of(req: &Request) -> Value {
    serde_json::from_slice(&req.body).expect("JSON body")
}

fn count(all: &[Request], verb: &str, route: &str) -> usize {
    all.iter()
        .filter(|r| r.method.as_str() == verb && r.url.path() == route)
        .count()
}

fn document(size: usize, checksum: &str) -> Value {
    json!({"id": "d-1", "name": "SOC 2", "externalId": EXTERNAL_ID, "size": size,
           "checksum": checksum, "uploadStatus": "pending"})
}

/// The SafeBase API and the signed storage URL, with steps 1 and 2 mounted.
struct Upload {
    api: MockServer,
    storage: MockServer,
}

impl Upload {
    async fn start(storage_response: ResponseTemplate) -> Self {
        let api = MockServer::start().await;
        let storage = MockServer::start().await;
        let signed = format!("{}/org/documents/{FILE_ID}?{SIGNED_QUERY}", storage.uri());
        Mock::given(method("POST"))
            .and(path("/documents/upload"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true, "result": {
                "fileId": FILE_ID,
                "signedUrl": signed,
                "headers": {
                    "content-type": "application/pdf",
                    "content-md5": FIXTURE_MD5B64,
                    "x-goog-content-length-range": format!("0,{FIXTURE_SIZE}"),
                    "x-goog-meta-original-name": "zzz-clitest.pdf",
                }
            }})))
            .mount(&api)
            .await;
        Mock::given(method("PUT"))
            .respond_with(storage_response)
            .mount(&storage)
            .await;
        Upload { api, storage }
    }

    /// Mount step 3's answer.
    async fn create_responds(&self, response: ResponseTemplate) {
        Mock::given(method("PUT"))
            .and(path("/documents/upload"))
            .respond_with(response)
            .mount(&self.api)
            .await;
    }

    /// Mount the externalId lookup's answer.
    async fn lookup_responds(&self, response: ResponseTemplate) {
        Mock::given(method("GET"))
            .and(path(format!("/document/{EXTERNAL_ID}")))
            .and(query_param("type", "external"))
            .respond_with(response)
            .mount(&self.api)
            .await;
    }

    /// The step 3 request (exactly one).
    async fn create_request(&self) -> Request {
        let all = requests(&self.api).await;
        assert_eq!(count(&all, "PUT", "/documents/upload"), 1, "{all:?}");
        all.into_iter()
            .find(|r| r.method.as_str() == "PUT")
            .expect("step 3 request")
    }
}

fn upload_argv<'a>(file: &'a str, extra: &[&'a str]) -> Vec<&'a str> {
    let mut argv = vec!["document", "upload", file, "--name", "SOC 2"];
    argv.extend_from_slice(extra);
    argv
}

#[test]
fn md5b64_of_the_fixture_matches_openssl() {
    // openssl dgst -md5 -binary tests/fixtures/zzz-clitest.pdf | base64
    let bytes = std::fs::read(fixture()).expect("fixture");
    assert_eq!(bytes.len(), FIXTURE_SIZE);
    assert_eq!(md5b64(&bytes), FIXTURE_MD5B64);
}

#[tokio::test]
async fn upload_sends_bytes_to_the_signed_url_without_credentials() {
    let up = Upload::start(ResponseTemplate::new(200)).await;
    up.create_responds(
        ResponseTemplate::new(200).set_body_json(json!({"ok": true, "result": document(FIXTURE_SIZE, FIXTURE_MD5B64)})),
    )
    .await;
    let file = fixture();
    let reply = run(&up.api, &upload_argv(file.to_str().expect("utf-8"), &[]))
        .await
        .expect("upload succeeds");
    assert!(matches!(reply, Reply::One { .. }), "{reply:?}");

    let step1 = requests(&up.api)
        .await
        .into_iter()
        .find(|r| r.method.as_str() == "POST")
        .expect("step 1");
    assert_eq!(
        body_of(&step1),
        json!({"fileName": "zzz-clitest.pdf", "mimeType": "application/pdf",
               "md5b64": FIXTURE_MD5B64, "fileSize": FIXTURE_SIZE})
    );
    assert_eq!(
        step1.headers.get("x-sb-api-key").and_then(|v| v.to_str().ok()),
        Some(SB_KEY)
    );

    let sent = only_request(&up.storage).await;
    assert_eq!(sent.method.as_str(), "PUT", "Phase 0: the signed URL takes PUT");
    assert_eq!(sent.url.path(), format!("/org/documents/{FILE_ID}"));
    assert_eq!(sent.url.query(), Some(SIGNED_QUERY));
    assert!(
        sent.headers.get("x-sb-api-key").is_none(),
        "SafeBase key sent to storage"
    );
    assert!(
        sent.headers.get("authorization").is_none(),
        "Authorization sent to storage"
    );
    for (name, value) in [
        ("content-type", "application/pdf"),
        ("content-md5", FIXTURE_MD5B64),
        ("x-goog-content-length-range", "0,134"),
        ("x-goog-meta-original-name", "zzz-clitest.pdf"),
    ] {
        assert_eq!(
            sent.headers.get(name).and_then(|v| v.to_str().ok()),
            Some(value),
            "{name}"
        );
    }
    assert_eq!(sent.body, std::fs::read(fixture()).expect("fixture"));

    let body = body_of(&up.create_request().await);
    assert_eq!(body["fileId"], json!(FILE_ID));
    assert_eq!(body["name"], json!("SOC 2"));
    assert_eq!(body["mimeType"], json!("application/pdf"));
    let ext = body["externalId"].as_str().expect("externalId string");
    assert_eq!(Uuid::parse_str(ext).expect("a UUID").get_version_num(), 4);
    assert!(body.get("replacementDocumentId").is_none(), "{body}");
    assert!(body.get("type").is_none(), "{body}");
}

#[tokio::test]
async fn upload_replace_sends_replacement_document_id() {
    let up = Upload::start(ResponseTemplate::new(200)).await;
    up.create_responds(
        ResponseTemplate::new(200).set_body_json(json!({"ok": true, "result": document(FIXTURE_SIZE, FIXTURE_MD5B64)})),
    )
    .await;
    let file = fixture();
    run(
        &up.api,
        &upload_argv(
            file.to_str().expect("utf-8"),
            &["--replace", "d-1", "--type", "Report", "--external-id", EXTERNAL_ID],
        ),
    )
    .await
    .expect("replace succeeds");

    assert_eq!(
        body_of(&up.create_request().await),
        json!({"name": "SOC 2", "fileId": FILE_ID, "mimeType": "application/pdf",
               "externalId": EXTERNAL_ID, "type": "Report", "replacementDocumentId": "d-1"})
    );
}

/// A client whose step 3 times out fast; the API holds the PUT for 5 s.
async fn timed_out_upload(lookup: ResponseTemplate) -> (Upload, eyre::Result<Reply>) {
    let up = Upload::start(ResponseTemplate::new(200)).await;
    up.create_responds(
        ResponseTemplate::new(200)
            .set_body_json(json!({"ok": true, "result": document(FIXTURE_SIZE, FIXTURE_MD5B64)}))
            .set_delay(Duration::from_secs(5)),
    )
    .await;
    up.lookup_responds(lookup).await;
    let client = writer(&up.api)
        .with_timeout(Duration::from_millis(300))
        .expect("timeout client");
    let file = fixture();
    let result = run_with(
        &client,
        &always_yes(),
        &upload_argv(file.to_str().expect("utf-8"), &["--external-id", EXTERNAL_ID]),
    )
    .await;
    (up, result)
}

async fn assert_one_put_one_lookup(up: &Upload) {
    let all = requests(&up.api).await;
    assert_eq!(count(&all, "PUT", "/documents/upload"), 1, "never a second PUT");
    let lookups: Vec<&Request> = all.iter().filter(|r| r.method.as_str() == "GET").collect();
    assert_eq!(lookups.len(), 1, "{all:?}");
    assert_eq!(lookups[0].url.path(), format!("/document/{EXTERNAL_ID}"));
    assert_eq!(lookups[0].url.query(), Some("type=external"));
}

#[tokio::test]
async fn upload_step3_timeout_finds_the_matching_document_and_warns() {
    let (up, result) = timed_out_upload(
        ResponseTemplate::new(200).set_body_json(json!({"ok": true, "result": document(FIXTURE_SIZE, FIXTURE_MD5B64)})),
    )
    .await;
    let reply = result.expect("found document is success");
    let Reply::Warned { warning, row, .. } = reply else {
        panic!("expected a warned reply, got {reply:?}");
    };
    assert!(warning.contains("did not answer"), "{warning}");
    assert_eq!(row["id"], json!("d-1"));
    assert_one_put_one_lookup(&up).await;
}

#[tokio::test]
async fn upload_step3_timeout_with_a_different_size_is_outcome_unknown() {
    let (up, result) = timed_out_upload(
        ResponseTemplate::new(200)
            .set_body_json(json!({"ok": true, "result": document(FIXTURE_SIZE + 1, FIXTURE_MD5B64)})),
    )
    .await;
    let err = result.expect_err("size mismatch is not success");
    let msg = format!("{err:#}");
    assert!(msg.contains("outcome unknown"), "{msg}");
    assert!(msg.contains(FILE_ID) && msg.contains(EXTERNAL_ID), "{msg}");
    assert!(
        matches!(err.downcast_ref::<UploadError>(), Some(UploadError::Unknown { .. })),
        "{err:?}"
    );
    assert_one_put_one_lookup(&up).await;
}

#[tokio::test]
async fn upload_step3_timeout_with_a_different_checksum_is_outcome_unknown() {
    let (up, result) = timed_out_upload(
        ResponseTemplate::new(200).set_body_json(json!({"ok": true, "result": document(FIXTURE_SIZE, "other==")})),
    )
    .await;
    let msg = format!("{:#}", result.expect_err("checksum mismatch"));
    assert!(msg.contains("outcome unknown"), "{msg}");
    assert_one_put_one_lookup(&up).await;
}

#[tokio::test]
async fn upload_step3_timeout_with_no_document_is_outcome_unknown() {
    let (up, result) =
        timed_out_upload(ResponseTemplate::new(404).set_body_json(json!({"statusCode": 404, "message": "Not found"})))
            .await;
    let msg = format!("{:#}", result.expect_err("not found"));
    assert!(
        msg.contains("outcome unknown") && msg.contains("found no document"),
        "{msg}"
    );
    assert!(msg.contains(FILE_ID) && msg.contains(EXTERNAL_ID), "{msg}");
    assert_one_put_one_lookup(&up).await;
}

#[tokio::test]
async fn upload_step3_http_error_is_definite_and_skips_the_lookup() {
    let up = Upload::start(ResponseTemplate::new(200)).await;
    up.create_responds(
        ResponseTemplate::new(400).set_body_json(json!({"statusCode": 400, "message": "Invalid request"})),
    )
    .await;
    let file = fixture();
    let err = run(&up.api, &upload_argv(file.to_str().expect("utf-8"), &[]))
        .await
        .expect_err("400 fails");
    assert!(
        matches!(err.downcast_ref::<UploadError>(), Some(UploadError::Create { .. })),
        "{err:?}"
    );
    assert!(err.to_string().contains(FILE_ID), "{err}");
    let all = requests(&up.api).await;
    assert_eq!(count(&all, "PUT", "/documents/upload"), 1);
    assert!(all.iter().all(|r| r.method.as_str() != "GET"), "no lookup: {all:?}");
}

#[tokio::test]
async fn upload_step2_failure_prints_the_file_id_and_stops() {
    let up =
        Upload::start(ResponseTemplate::new(403).set_body_string("<Error><Code>SignatureDoesNotMatch</Code></Error>"))
            .await;
    let file = fixture();
    let err = run(&up.api, &upload_argv(file.to_str().expect("utf-8"), &[]))
        .await
        .expect_err("storage 403 fails");
    let msg = err.to_string();
    assert!(
        matches!(err.downcast_ref::<UploadError>(), Some(UploadError::Send { .. })),
        "{err:?}"
    );
    assert!(msg.contains(FILE_ID) && msg.contains("SignatureDoesNotMatch"), "{msg}");
    assert!(!msg.contains("X-Goog-Signature"), "signed query leaked: {msg}");
    let all = requests(&up.api).await;
    assert_eq!(count(&all, "PUT", "/documents/upload"), 0, "no step 3");
}

#[tokio::test]
async fn upload_step1_failure_sends_nothing_to_storage() {
    let up = Upload {
        api: MockServer::start().await,
        storage: MockServer::start().await,
    };
    Mock::given(method("POST"))
        .and(path("/documents/upload"))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({
            "statusCode": 403, "message": "Missing required scope(s): documents:edit.", "error": "forbidden"})))
        .mount(&up.api)
        .await;
    let file = fixture();
    let err = run(&up.api, &upload_argv(file.to_str().expect("utf-8"), &[]))
        .await
        .expect_err("403 fails");
    assert!(err.to_string().contains("documents:edit"), "{err}");
    assert!(requests(&up.storage).await.is_empty());
}

#[tokio::test]
async fn upload_local_checks_send_no_request() {
    let server = MockServer::start().await;
    let dir = tempfile::tempdir().expect("tempdir");

    let file = fixture();
    let err = run(
        &server,
        &upload_argv(file.to_str().expect("utf-8"), &["--external-id", "drata_policy:64"]),
    )
    .await
    .expect_err("non-UUID external id");
    assert!(err.to_string().contains("is not a UUID"), "{err}");

    let exe = dir.path().join("tool.exe");
    std::fs::write(&exe, b"MZ").expect("write exe");
    let err = run(&server, &upload_argv(exe.to_str().expect("utf-8"), &[]))
        .await
        .expect_err("exe not allowed");
    assert!(err.to_string().contains("not a type SafeBase accepts"), "{err}");

    let big = dir.path().join("big.pdf");
    std::fs::File::create(&big)
        .and_then(|f| f.set_len(78_643_201))
        .expect("sparse file");
    let err = run(&server, &upload_argv(big.to_str().expect("utf-8"), &[]))
        .await
        .expect_err("too big");
    assert!(err.to_string().contains("at most 78643200 bytes"), "{err}");

    assert!(requests(&server).await.is_empty(), "a local check sent a request");
}

#[tokio::test]
async fn upload_on_a_read_only_credential_sends_no_request() {
    let server = MockServer::start().await;
    let client = DrataClient::new(Backend::Safebase, SB_KEY.to_string(), None, false)
        .expect("client builds")
        .with_base_url(server.uri());
    let file = fixture();
    let err = run_with(&client, &always_yes(), &upload_argv(file.to_str().expect("utf-8"), &[]))
        .await
        .expect_err("write guard");
    assert!(format!("{err:#}").contains("write blocked"), "{err:#}");
    assert!(requests(&server).await.is_empty());
}

#[tokio::test]
async fn declined_confirm_sends_no_request() {
    let server = MockServer::start().await;
    let file = fixture();
    for argv in [
        upload_argv(file.to_str().expect("utf-8"), &[]),
        vec!["request", "approve", "r-1", "--account-id", "a-1"],
        vec!["member", "remove", "a-1", "--email", "x@y.co"],
        vec!["kb", "add", "--data", r#"{"question": "q"}"#],
    ] {
        let err = run_with(&writer(&server), &always_no(), &argv)
            .await
            .expect_err("aborted");
        assert_eq!(err.to_string(), "aborted", "{argv:?}");
    }
    assert!(requests(&server).await.is_empty());
}

fn account_reply() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({"ok": true, "result": {"id": "a-1", "name": "Acme"}}))
}

#[tokio::test]
async fn request_approve_sends_the_account_id() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/requests/r-1/approve"))
        .respond_with(account_reply())
        .expect(1)
        .mount(&server)
        .await;
    let reply = run(&server, &["request", "approve", "r-1", "--account-id", "a-1"])
        .await
        .expect("approve ok");
    assert!(matches!(reply, Reply::One { .. }), "{reply:?}");
    assert_eq!(body_of(&only_request(&server).await), json!({"accountId": "a-1"}));
}

#[tokio::test]
async fn request_approve_409_fails_and_prints_errors() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/requests/r-1/approve"))
        .respond_with(ResponseTemplate::new(409).set_body_json(json!({
            "ok": false, "errorCode": "request_already_handled",
            "errors": ["Request has already been handled."]})))
        .mount(&server)
        .await;
    let err = run(&server, &["request", "approve", "r-1", "--account-id", "a-1"])
        .await
        .expect_err("409 fails");
    let msg = format!("{err:#}");
    assert!(msg.contains("409"), "{msg}");
    assert!(msg.contains("Request has already been handled."), "{msg}");
}

#[tokio::test]
async fn request_decline_sends_message_only_on_opt_in() {
    for (extra, expected) in [
        (vec![], json!({})),
        (
            vec!["--message", "not this quarter"],
            json!({"message": "not this quarter"}),
        ),
        (vec!["--notify"], json!({"message": true})),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/requests/r-1/decline"))
            .respond_with(account_reply())
            .mount(&server)
            .await;
        let mut argv = vec!["request", "decline", "r-1"];
        argv.extend(extra);
        run(&server, &argv).await.expect("decline ok");
        assert_eq!(body_of(&only_request(&server).await), expected, "{argv:?}");
    }
}

#[test]
fn message_and_notify_conflict() {
    let err = Cli::try_parse_from([
        "drata",
        "safebase",
        "request",
        "decline",
        "r-1",
        "--message",
        "m",
        "--notify",
    ])
    .expect_err("conflict");
    assert_eq!(err.kind(), clap::error::ErrorKind::ArgumentConflict);
}

#[tokio::test]
async fn member_add_sends_one_member_with_names() {
    let member = json!({"email": "ada@acme.com", "firstName": "Ada", "lastName": "Byron"});
    for (extra, message) in [
        (vec![], None),
        (vec!["--message", "welcome"], Some(json!("welcome"))),
        (vec!["--notify"], Some(json!(true))),
    ] {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/accounts/a-1/members"))
            .respond_with(ResponseTemplate::new(201).set_body_json(json!({"ok": true, "result": [
                {"accountId": "a-1", "email": "ada@acme.com", "firstName": "Ada", "lastName": "Byron"}]})))
            .mount(&server)
            .await;
        let mut argv = vec![
            "member",
            "add",
            "a-1",
            "--email",
            "ada@acme.com",
            "--first-name",
            "Ada",
            "--last-name",
            "Byron",
        ];
        argv.extend(extra);
        let reply = run(&server, &argv).await.expect("add ok");
        assert!(matches!(reply, Reply::Rows { .. }), "{reply:?}");
        let mut expected = json!({"members": [member]});
        if let Some(m) = message {
            expected["message"] = m;
        }
        assert_eq!(body_of(&only_request(&server).await), expected, "{argv:?}");
    }
}

#[tokio::test]
async fn member_remove_deletes_with_an_email_body() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/accounts/a-1/members"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"ok": true})))
        .mount(&server)
        .await;
    let reply = run(
        &server,
        &["member", "remove", "a-1", "--email", "a@acme.com", "b@acme.com"],
    )
    .await
    .expect("remove ok");
    assert!(matches!(reply, Reply::Ok), "{reply:?}");
    let req = only_request(&server).await;
    assert_eq!(req.method.as_str(), "DELETE");
    assert_eq!(body_of(&req), json!({"members": ["a@acme.com", "b@acme.com"]}));
}

#[tokio::test]
async fn kb_add_posts_the_data() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/kb/entries"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"ok": true, "result": {"id": "k-1"}})))
        .mount(&server)
        .await;
    let data = r#"{"question": "Do you encrypt at rest?", "answer": "Yes", "accessLevel": "public"}"#;
    run(&server, &["kb", "add", "--data", data]).await.expect("add ok");
    assert_eq!(
        body_of(&only_request(&server).await),
        json!({"question": "Do you encrypt at rest?", "answer": "Yes", "accessLevel": "public"})
    );
}

#[tokio::test]
async fn kb_update_patches_the_entry() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path("/kb/entries/k-1"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({"ok": true, "result": {"id": "k-1"}})))
        .mount(&server)
        .await;
    run(&server, &["kb", "update", "k-1", "--data", r#"{"answer": null}"#])
        .await
        .expect("update ok");
    let req = only_request(&server).await;
    assert_eq!(req.method.as_str(), "PATCH");
    assert_eq!(body_of(&req), json!({"answer": null}));
}

#[tokio::test]
async fn write_verbs_on_a_read_only_credential_send_no_request() {
    let server = MockServer::start().await;
    let client = DrataClient::new(Backend::Safebase, SB_KEY.to_string(), None, false)
        .expect("client builds")
        .with_base_url(server.uri());
    for argv in [
        vec!["request", "approve", "r-1", "--account-id", "a-1"],
        vec!["request", "decline", "r-1"],
        vec![
            "member",
            "add",
            "a-1",
            "--email",
            "e",
            "--first-name",
            "f",
            "--last-name",
            "l",
        ],
        vec!["member", "remove", "a-1", "--email", "e"],
        vec!["kb", "add", "--data", "{}"],
        vec!["kb", "update", "k-1", "--data", "{}"],
    ] {
        let err = run_with(&client, &always_yes(), &argv).await.expect_err("guarded");
        assert!(err.downcast_ref::<WriteGuardError>().is_some(), "{argv:?}: {err:?}");
    }
    assert!(requests(&server).await.is_empty());
}
