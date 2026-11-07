use super::*;
use std::path::PathBuf;

fn allowed() -> Vec<String> {
    upload_limits().expect("spec limits").1
}

fn args(replace: Option<&str>, doc_type: Option<&str>) -> UploadArgs {
    UploadArgs {
        file: PathBuf::from("soc2.pdf"),
        name: "SOC 2".to_string(),
        doc_type: doc_type.map(str::to_string),
        replace: replace.map(str::to_string),
        external_id: None,
    }
}

#[test]
fn upload_limits_come_from_the_spec() {
    let (max, mimes) = upload_limits().expect("spec limits");
    assert_eq!(max, 78_643_200);
    assert!(mimes.iter().any(|m| m == "application/pdf"), "{mimes:?}");
    assert!(!mimes.iter().any(|m| m == "application/octet-stream"), "{mimes:?}");
}

#[test]
fn pick_mime_takes_an_allowed_guess() {
    assert_eq!(
        pick_mime(Path::new("a.pdf"), &allowed()).expect("pdf"),
        "application/pdf"
    );
    assert_eq!(pick_mime(Path::new("a.csv"), &allowed()).expect("csv"), "text/csv");
}

#[test]
fn pick_mime_rejects_types_outside_the_enum() {
    let err = pick_mime(Path::new("a.exe"), &allowed()).expect_err("exe is not allowed");
    assert!(err.to_string().contains("not a type SafeBase accepts"), "{err}");
    let err = pick_mime(Path::new("noext"), &allowed()).expect_err("no extension");
    assert!(err.to_string().contains("no type from its extension"), "{err}");
}

#[test]
fn check_size_allows_the_maximum_and_rejects_one_more() {
    assert!(check_size(Path::new("f"), 78_643_200, 78_643_200).is_ok());
    let err = check_size(Path::new("f"), 78_643_201, 78_643_200).expect_err("over the max");
    assert!(err.to_string().contains("at most 78643200 bytes"), "{err}");
}

#[test]
fn external_id_accepts_a_uuid_and_rejects_anything_else() {
    assert_eq!(
        external_id(Some("6F9619FF-8B86-D011-B42D-00C04FC964FF")).expect("uuid"),
        "6f9619ff-8b86-d011-b42d-00c04fc964ff"
    );
    let err = external_id(Some("drata_policy:64")).expect_err("not a uuid");
    assert!(err.to_string().contains("is not a UUID"), "{err}");
    let fresh = external_id(None).expect("fresh");
    assert_eq!(Uuid::parse_str(&fresh).expect("parses").get_version_num(), 4);
}

#[test]
fn create_body_adds_type_and_replacement_only_when_given() {
    assert_eq!(
        create_body(&args(None, None), "f-1", "application/pdf", "e-1"),
        json!({"name": "SOC 2", "fileId": "f-1", "mimeType": "application/pdf", "externalId": "e-1"})
    );
    let body = create_body(&args(Some("d-1"), Some("Report")), "f-1", "application/pdf", "e-1");
    assert_eq!(body["replacementDocumentId"], json!("d-1"));
    assert_eq!(body["type"], json!("Report"));
}

#[test]
fn matches_local_needs_size_and_checksum() {
    let doc = json!({"size": 4, "checksum": "abc"});
    assert!(matches_local(&doc, 4, "abc"));
    assert!(!matches_local(&doc, 5, "abc"));
    assert!(!matches_local(&doc, 4, "abd"));
    assert!(!matches_local(&json!({"size": 4}), 4, "abc"));
}

#[test]
fn parse_ticket_reads_the_result() {
    let resp = json!({"ok": true, "result": {
        "fileId": "f-1", "signedUrl": "https://x/y?sig", "headers": {"content-type": "application/pdf"}
    }});
    let t = parse_ticket(&resp).expect("ticket");
    assert_eq!(t.file_id, "f-1");
    assert_eq!(t.signed_url, "https://x/y?sig");
    assert_eq!(
        t.headers,
        vec![("content-type".to_string(), "application/pdf".to_string())]
    );
}

#[test]
fn parse_ticket_failure_carries_the_file_id_when_known() {
    let resp = json!({"ok": true, "result": {"fileId": "f-1", "headers": {}}});
    let err = parse_ticket(&resp).err().expect("no signedUrl");
    assert!(err.to_string().contains("fileId f-1"), "{err}");
    let resp = json!({"ok": true, "result": {"fileId": "f-1", "signedUrl": "u", "headers": {"a": 1}}});
    let err = parse_ticket(&resp).err().expect("non-string header");
    assert!(err.to_string().contains("header `a`"), "{err}");
    let err = parse_ticket(&json!({"ok": true, "result": {}}))
        .err()
        .expect("no fileId");
    assert!(matches!(err, UploadError::Request { file_id: None, .. }), "{err}");
}

#[test]
fn outcome_is_open_only_for_transport_errors() {
    assert!(!outcome_is_open(&eyre!("API error 400")));
}
