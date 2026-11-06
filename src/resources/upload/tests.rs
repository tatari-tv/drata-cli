#![allow(clippy::unwrap_used)]
use super::*;
use std::io::Write;

fn upload(name: &str, content_type: &str) -> UploadFile {
    UploadFile {
        name: name.to_string(),
        content_type: content_type.to_string(),
        bytes: Vec::new(),
    }
}

#[test]
fn request_body_with_file_carries_name_and_content_type() {
    let f = upload("report.pdf", "application/pdf");
    let body = build_request_body(UploadPurpose::Evidence, Some(1), Some(&f)).unwrap();
    assert_eq!(
        body,
        json!({"purpose": "evidence", "workspaceId": 1, "fileName": "report.pdf", "contentType": "application/pdf"})
    );
}

#[test]
fn request_body_without_file_has_no_file_fields() {
    let body = build_request_body(UploadPurpose::Evidence, Some(3), None).unwrap();
    assert_eq!(body, json!({"purpose": "evidence", "workspaceId": 3}));
}

#[test]
fn evidence_purpose_requires_workspace() {
    let err = build_request_body(UploadPurpose::Evidence, None, None).unwrap_err();
    assert!(
        err.to_string().contains("--purpose evidence requires --workspace"),
        "{err}"
    );
}

#[test]
fn load_file_derives_name_and_mime() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("zzz-clitest.txt");
    std::fs::File::create(&path).unwrap().write_all(b"hi").unwrap();
    let f = load_file(&path).unwrap();
    assert_eq!(f.name, "zzz-clitest.txt");
    assert_eq!(f.content_type, "text/plain");
    assert_eq!(f.bytes, b"hi");
}

#[test]
fn load_file_unknown_extension_is_octet_stream() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("blob.zzzunknown");
    std::fs::File::create(&path).unwrap();
    assert_eq!(load_file(&path).unwrap().content_type, "application/octet-stream");
}

#[test]
fn load_file_missing_errors() {
    let err = load_file(Path::new("/no/such/upload.txt")).unwrap_err();
    assert!(err.to_string().contains("Failed to read upload file"), "{err}");
}

#[test]
fn response_str_missing_field_errors() {
    let err = response_str(&json!({"objectKey": "k"}), "presignedUrl").unwrap_err();
    assert!(err.to_string().contains("presignedUrl"), "{err}");
}
