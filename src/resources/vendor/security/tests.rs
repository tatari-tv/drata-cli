#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{
    SecurityReviewAction, SecurityReviewDecision, SecurityReviewStatus, SecurityReviewType, VendorSecurityReviewAction,
};

// ---------------------------------------------------------------------------
// Enum -> spec string pinning
// These tests catch any future spec drift on the serialized values.
// ---------------------------------------------------------------------------

#[test]
fn status_str_matches_spec() {
    assert_eq!(status_str(&SecurityReviewStatus::NotYetStarted), "NOT_YET_STARTED");
    assert_eq!(status_str(&SecurityReviewStatus::InProgress), "IN_PROGRESS");
    assert_eq!(status_str(&SecurityReviewStatus::Completed), "COMPLETED");
    assert_eq!(status_str(&SecurityReviewStatus::NotRequired), "NOT_REQUIRED");
}

#[test]
fn type_str_matches_spec() {
    assert_eq!(type_str(&SecurityReviewType::Security), "SECURITY");
    assert_eq!(type_str(&SecurityReviewType::SocReport), "SOC_REPORT");
    assert_eq!(type_str(&SecurityReviewType::UploadReport), "UPLOAD_REPORT");
}

#[test]
fn action_str_matches_spec() {
    assert_eq!(action_str(&SecurityReviewAction::Finalize), "finalize");
    assert_eq!(action_str(&SecurityReviewAction::Reopen), "reopen");
}

#[test]
fn decision_str_matches_spec() {
    assert_eq!(decision_str(&SecurityReviewDecision::Pending), "PENDING");
    assert_eq!(decision_str(&SecurityReviewDecision::Approved), "APPROVED");
    assert_eq!(
        decision_str(&SecurityReviewDecision::ApprovedWithConditions),
        "APPROVED_WITH_CONDITIONS"
    );
    assert_eq!(decision_str(&SecurityReviewDecision::Rejected), "REJECTED");
}

// ---------------------------------------------------------------------------
// list path / filter encoding (spec params status[] / type[] / decision[])
// ---------------------------------------------------------------------------

#[test]
fn list_path_no_filters_is_bare() {
    let path = build_list_path("v1", &[], &[], &[], None, None, None, None, &[]);
    assert_eq!(path, "/vendors/v1/security-reviews");
}

#[test]
fn list_path_encodes_bracketed_array_filters() {
    let path = build_list_path(
        "v1",
        &[SecurityReviewStatus::InProgress],
        &[SecurityReviewType::SocReport],
        &[SecurityReviewDecision::Approved],
        None,
        None,
        None,
        None,
        &[],
    );
    // Brackets must be percent-encoded in the key (status%5B%5D=...), matching expand[].
    assert_eq!(
        path,
        "/vendors/v1/security-reviews?status%5B%5D=IN_PROGRESS&type%5B%5D=SOC_REPORT&decision%5B%5D=APPROVED"
    );
}

#[test]
fn list_path_repeats_multi_value_filters() {
    let path = build_list_path(
        "v1",
        &[SecurityReviewStatus::NotYetStarted, SecurityReviewStatus::Completed],
        &[],
        &[],
        None,
        None,
        None,
        None,
        &[],
    );
    assert_eq!(
        path,
        "/vendors/v1/security-reviews?status%5B%5D=NOT_YET_STARTED&status%5B%5D=COMPLETED"
    );
}

// ---------------------------------------------------------------------------
// example_if_requested gating
// ---------------------------------------------------------------------------

#[test]
fn example_only_for_create_with_flag() {
    let create_example = VendorSecurityReviewAction::Create {
        vendor_id: "v1".to_string(),
        review_deadline_at: Some("2026-12-31".to_string()),
        status: Some(SecurityReviewStatus::NotYetStarted),
        review_type: Some(SecurityReviewType::Security),
        title: None,
        note: None,
        requested_at: None,
        requester_user_id: None,
        data: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());
}

#[test]
fn example_none_when_create_flag_false() {
    let create_no_example = VendorSecurityReviewAction::Create {
        vendor_id: "v1".to_string(),
        review_deadline_at: Some("2026-12-31".to_string()),
        status: Some(SecurityReviewStatus::NotYetStarted),
        review_type: Some(SecurityReviewType::Security),
        title: None,
        note: None,
        requested_at: None,
        requester_user_id: None,
        data: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());
}

#[test]
fn example_only_for_update_with_flag() {
    let update_example = VendorSecurityReviewAction::Update {
        vendor_id: "v1".to_string(),
        security_review_id: 42,
        title: None,
        soc_form: None,
        updated_at: None,
        data: None,
        example: true,
    };
    assert!(example_if_requested(&update_example).is_some());
}

#[test]
fn example_none_when_update_flag_false() {
    let update_no_example = VendorSecurityReviewAction::Update {
        vendor_id: "v1".to_string(),
        security_review_id: 42,
        title: None,
        soc_form: None,
        updated_at: None,
        data: None,
        example: false,
    };
    assert!(example_if_requested(&update_no_example).is_none());
}

#[test]
fn example_none_for_non_create_non_update_actions() {
    let list = VendorSecurityReviewAction::List {
        vendor_id: "v1".to_string(),
        status: vec![],
        review_type: vec![],
        decision: vec![],
        created_from: None,
        created_to: None,
        updated_from: None,
        updated_to: None,
        expand: vec![],
        all: false,
    };
    assert!(example_if_requested(&list).is_none());

    let get = VendorSecurityReviewAction::Get {
        vendor_id: "v1".to_string(),
        security_review_id: 42,
        expand: vec![],
    };
    assert!(example_if_requested(&get).is_none());

    let actions = VendorSecurityReviewAction::Actions {
        vendor_id: "v1".to_string(),
        security_review_id: 42,
    };
    assert!(example_if_requested(&actions).is_none());

    let run_action = VendorSecurityReviewAction::RunAction {
        vendor_id: "v1".to_string(),
        security_review_id: 42,
        action: SecurityReviewAction::Finalize,
        decision: None,
        note: None,
        reset_decision: false,
    };
    assert!(example_if_requested(&run_action).is_none());

    let questionnaires = VendorSecurityReviewAction::Questionnaires {
        vendor_id: "v1".to_string(),
        security_review_id: 42,
    };
    assert!(example_if_requested(&questionnaires).is_none());
}

#[test]
fn create_example_skeleton_is_valid_json() {
    let parsed: serde_json::Value = serde_json::from_str(SECURITY_REVIEW_CREATE_EXAMPLE).unwrap();
    assert_eq!(parsed["securityReviewStatus"], serde_json::json!("NOT_YET_STARTED"));
    assert_eq!(parsed["securityReviewType"], serde_json::json!("SECURITY"));
    assert!(parsed.get("reviewDeadlineAt").is_some());
}

#[test]
fn update_example_skeleton_is_valid_json() {
    let parsed: serde_json::Value = serde_json::from_str(SECURITY_REVIEW_UPDATE_EXAMPLE).unwrap();
    assert!(parsed.get("title").is_some());
    assert!(parsed.get("socForm").is_some());
    assert!(parsed.get("updatedAt").is_some());
    // Confirm update skeleton does NOT include create-only fields.
    assert!(parsed.get("securityReviewStatus").is_none());
    assert!(parsed.get("securityReviewType").is_none());
    assert!(parsed.get("reviewDeadlineAt").is_none());
}

// ---------------------------------------------------------------------------
// Body building helpers
// ---------------------------------------------------------------------------

#[test]
fn set_opt_str_inserts_only_present_values() {
    let mut body = serde_json::json!({"existing": "value"});
    set_opt_str(&mut body, "title", Some("My Title"));
    set_opt_str(&mut body, "socForm", None);
    assert_eq!(body["title"], serde_json::json!("My Title"));
    assert!(body.get("socForm").is_none(), "None value must not appear in body");
}

#[test]
fn set_opt_str_overwrites_existing_key() {
    let mut body = serde_json::json!({"title": "old"});
    set_opt_str(&mut body, "title", Some("new"));
    assert_eq!(body["title"], serde_json::json!("new"));
}

#[test]
fn create_body_uses_translated_keys() {
    // Verify body key translation: CLI flags --status/--type -> securityReviewStatus/securityReviewType
    let mut body = serde_json::json!({
        "securityReviewStatus": status_str(&SecurityReviewStatus::InProgress),
        "securityReviewType": type_str(&SecurityReviewType::SocReport),
        "reviewDeadlineAt": "2026-12-31",
    });
    set_opt_str(&mut body, "title", Some("My Review"));
    set_opt_str(&mut body, "note", None);

    assert_eq!(body["securityReviewStatus"], serde_json::json!("IN_PROGRESS"));
    assert_eq!(body["securityReviewType"], serde_json::json!("SOC_REPORT"));
    assert_eq!(body["reviewDeadlineAt"], serde_json::json!("2026-12-31"));
    assert_eq!(body["title"], serde_json::json!("My Review"));
    // note was None, must be absent
    assert!(body.get("note").is_none());
    // Flag names must NOT appear as keys
    assert!(
        body.get("status").is_none(),
        "--status must be translated to securityReviewStatus"
    );
    assert!(
        body.get("type").is_none(),
        "--type must be translated to securityReviewType"
    );
}

#[test]
fn update_body_has_only_update_fields() {
    let body = build_update_body(
        Some("Updated title"),
        Some(r#"{"reportOpinion": {}}"#),
        Some("2025-07-01T16:45:55.246Z"),
    )
    .unwrap();
    assert_eq!(body["title"], serde_json::json!("Updated title"));
    assert_eq!(body["socForm"], serde_json::json!({"reportOpinion": {}}));
    assert_eq!(body["updatedAt"], serde_json::json!("2025-07-01T16:45:55.246Z"));
    // Create-only fields never appear in an update body.
    assert!(body.get("securityReviewStatus").is_none());
    assert!(body.get("securityReviewType").is_none());
    assert!(body.get("reviewDeadlineAt").is_none());
}

#[test]
fn update_body_rejects_non_json_soc_form() {
    let err = build_update_body(None, Some("SOC2_TYPE_II"), None).unwrap_err();
    assert!(err.to_string().contains("--soc-form"), "{err}");
}

#[test]
fn update_body_omits_unset_updated_at() {
    let body = build_update_body(Some("t"), None, None).unwrap();
    assert!(body.get("updatedAt").is_none());
}

#[test]
fn run_action_finalize_body_has_decision_and_note() {
    let body = build_run_action_body(
        &SecurityReviewAction::Finalize,
        Some(&SecurityReviewDecision::ApprovedWithConditions),
        Some("pen test due in Q3"),
        false,
    )
    .unwrap();
    assert_eq!(
        body,
        serde_json::json!({
            "action": "finalize",
            "decision": "APPROVED_WITH_CONDITIONS",
            "note": "pen test due in Q3",
        })
    );
}

#[test]
fn run_action_bare_finalize_sends_only_action() {
    let body = build_run_action_body(&SecurityReviewAction::Finalize, None, None, false).unwrap();
    assert_eq!(body, serde_json::json!({"action": "finalize"}));
}

#[test]
fn run_action_reopen_body_has_reset_decision() {
    let body = build_run_action_body(&SecurityReviewAction::Reopen, None, None, true).unwrap();
    assert_eq!(body, serde_json::json!({"action": "reopen", "resetDecision": true}));
}

#[test]
fn run_action_rejects_finalize_flags_on_reopen() {
    let err = build_run_action_body(
        &SecurityReviewAction::Reopen,
        Some(&SecurityReviewDecision::Rejected),
        None,
        false,
    )
    .unwrap_err();
    assert!(err.to_string().contains("--action finalize"), "{err}");
    assert!(build_run_action_body(&SecurityReviewAction::Reopen, None, Some("n"), false).is_err());
}

#[test]
fn run_action_rejects_reset_decision_on_finalize() {
    let err = build_run_action_body(&SecurityReviewAction::Finalize, None, None, true).unwrap_err();
    assert!(err.to_string().contains("--action reopen"), "{err}");
}

#[test]
fn list_path_encodes_created_and_updated_bounds() {
    let path = build_list_path(
        "v1",
        &[],
        &[],
        &[],
        Some("2026-01-01"),
        Some("2026-02-01"),
        Some("2026-03-01"),
        Some("2026-04-01"),
        &[],
    );
    assert_eq!(
        path,
        "/vendors/v1/security-reviews?createdAtFrom=2026-01-01&createdAtTo=2026-02-01\
         &updatedAtFrom=2026-03-01&updatedAtTo=2026-04-01"
    );
}

#[test]
fn create_with_file_requires_document_type() {
    use crate::cli::Cli;
    use clap::Parser;
    let base = [
        "drata",
        "vendor",
        "security-review",
        "create-with-file",
        "1",
        "--file",
        "/dev/null",
        "--title",
        "t",
        "--review-deadline-at",
        "2026-12-31",
        "--status",
        "NOT_YET_STARTED",
        "--type",
        "SECURITY",
    ];
    let err = Cli::try_parse_from(base).unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    assert!(err.to_string().contains("--document-type"), "{err}");

    let with_type = base.iter().copied().chain(["--document-type", "bridge_letter"]);
    assert!(Cli::try_parse_from(with_type).is_ok(), "enum is case-insensitive");
}

// ---------------------------------------------------------------------------
// Multipart part shapes (Phase 3)
// Pure assertions on the constructed `Multipart`: file part name and count,
// scalar field names/values, required-set vs optional behavior. Wire format
// (boundary, headers) is covered by src/client/tests.rs.
// ---------------------------------------------------------------------------

use std::path::PathBuf;

/// Helper: find a scalar field value by name in a `Multipart`.
fn field<'a>(form: &'a Multipart, name: &str) -> Option<&'a str> {
    form.fields.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
}

#[test]
fn create_with_file_uses_single_file_part_named_file() {
    let form = build_create_with_file_form(
        &PathBuf::from("/tmp/report.pdf"),
        "Annual review",
        "2026-12-31",
        &SecurityReviewStatus::InProgress,
        &SecurityReviewType::SocReport,
        &SecurityReviewDocumentType::ComplianceReport,
        None,
        None,
        None,
    );
    assert_eq!(form.files.len(), 1, "create-with-file sends exactly one file part");
    assert_eq!(
        form.files[0].field, "file",
        "the file part field is `file`, not `files`"
    );
    assert_eq!(form.files[0].path, PathBuf::from("/tmp/report.pdf"));
}

#[test]
fn create_with_file_required_fields_use_translated_keys() {
    let form = build_create_with_file_form(
        &PathBuf::from("/tmp/report.pdf"),
        "Annual review",
        "2026-12-31",
        &SecurityReviewStatus::InProgress,
        &SecurityReviewType::SocReport,
        &SecurityReviewDocumentType::ComplianceReport,
        None,
        None,
        None,
    );
    assert_eq!(field(&form, "title"), Some("Annual review"));
    assert_eq!(field(&form, "reviewDeadlineAt"), Some("2026-12-31"));
    // --status / --type translate to the long body keys, same as JSON create.
    assert_eq!(field(&form, "securityReviewStatus"), Some("IN_PROGRESS"));
    assert_eq!(field(&form, "securityReviewType"), Some("SOC_REPORT"));
    // Flag names must NOT leak through as field names.
    assert!(field(&form, "status").is_none());
    assert!(field(&form, "type").is_none());
}

#[test]
fn create_with_file_always_sends_document_type_and_omits_unset_optionals() {
    let form = build_create_with_file_form(
        &PathBuf::from("/tmp/report.pdf"),
        "Annual review",
        "2026-12-31",
        &SecurityReviewStatus::NotYetStarted,
        &SecurityReviewType::Security,
        &SecurityReviewDocumentType::SocDocument,
        None,
        None,
        None,
    );
    // documentType is required by the spec, so it is always sent.
    assert_eq!(field(&form, "documentType"), Some("SOC_DOCUMENT"));
    assert!(field(&form, "note").is_none());
    assert!(field(&form, "requestedAt").is_none());
    assert!(field(&form, "requesterUserId").is_none());
}

#[test]
fn create_with_file_includes_set_optionals() {
    let form = build_create_with_file_form(
        &PathBuf::from("/tmp/report.pdf"),
        "Annual review",
        "2026-12-31",
        &SecurityReviewStatus::Completed,
        &SecurityReviewType::UploadReport,
        &SecurityReviewDocumentType::ComplianceReport,
        Some("a note"),
        Some("2026-06-28T00:00:00.000Z"),
        Some(42),
    );
    assert_eq!(field(&form, "documentType"), Some("COMPLIANCE_REPORT"));
    assert_eq!(field(&form, "note"), Some("a note"));
    assert_eq!(field(&form, "requestedAt"), Some("2026-06-28T00:00:00.000Z"));
    // u64 requester id is stringified for the multipart text field.
    assert_eq!(field(&form, "requesterUserId"), Some("42"));
}

#[test]
fn files_form_appends_each_file_under_files_field() {
    let files = vec![
        PathBuf::from("/tmp/q1.pdf"),
        PathBuf::from("/tmp/q2.docx"),
        PathBuf::from("/tmp/q3.xlsx"),
    ];
    let form = build_files_form(&files);
    assert_eq!(form.files.len(), 3, "every input file becomes a part");
    for part in &form.files {
        assert_eq!(part.field, "files", "questionnaire uploads use the `files` array field");
    }
    assert_eq!(form.files[0].path, PathBuf::from("/tmp/q1.pdf"));
    assert_eq!(form.files[2].path, PathBuf::from("/tmp/q3.xlsx"));
    // The questionnaire endpoints carry no scalar fields.
    assert!(form.fields.is_empty());
}

#[test]
fn files_form_single_file() {
    let files = vec![PathBuf::from("/tmp/only.pdf")];
    let form = build_files_form(&files);
    assert_eq!(form.files.len(), 1);
    assert_eq!(form.files[0].field, "files");
}

#[test]
fn map_with_file_415_rewrites_415_into_actionable_error() {
    let api = ApiError {
        status: StatusCode::UNSUPPORTED_MEDIA_TYPE,
        body: "unsupported".to_string(),
        formatted: "415 from server".to_string(),
    };
    let mapped = map_with_file_415(eyre::Report::new(api), "/vendors/v1/security-reviews/with-file");
    let msg = format!("{}", mapped);
    assert!(msg.contains("415"), "message mentions the 415 status");
    assert!(
        msg.contains("drata raw POST"),
        "message points at the raw JSON fallback"
    );
    assert!(msg.contains("/vendors/v1/security-reviews/with-file"));
}

#[test]
fn map_with_file_415_passes_through_non_415_errors() {
    let api = ApiError {
        status: StatusCode::BAD_REQUEST,
        body: "bad".to_string(),
        formatted: "400 bad request".to_string(),
    };
    let mapped = map_with_file_415(eyre::Report::new(api), "/vendors/v1/security-reviews/with-file");
    let msg = format!("{}", mapped);
    // A 400 is unchanged - no spurious 415 guidance.
    assert!(!msg.contains("drata raw POST"));
    assert!(msg.contains("400 bad request"));
}

#[test]
fn create_with_file_action_constructs() {
    // The clap variant carries its own required set distinct from JSON create.
    let action = VendorSecurityReviewAction::CreateWithFile {
        vendor_id: "v1".to_string(),
        file: PathBuf::from("/tmp/report.pdf"),
        title: "t".to_string(),
        review_deadline_at: "2026-12-31".to_string(),
        status: SecurityReviewStatus::NotYetStarted,
        review_type: SecurityReviewType::Security,
        document_type: SecurityReviewDocumentType::BridgeLetter,
        note: None,
        requested_at: None,
        requester_user_id: None,
    };
    // create-with-file never prints an example skeleton.
    assert!(example_if_requested(&action).is_none());
}

#[test]
fn upload_questionnaire_actions_have_no_example() {
    let upload = VendorSecurityReviewAction::UploadQuestionnaire {
        vendor_id: "v1".to_string(),
        file: vec![PathBuf::from("/tmp/q1.pdf")],
    };
    assert!(example_if_requested(&upload).is_none());

    let upload_to_review = VendorSecurityReviewAction::UploadQuestionnaireToReview {
        vendor_id: "v1".to_string(),
        security_review_id: 42,
        file: vec![PathBuf::from("/tmp/q1.pdf")],
    };
    assert!(example_if_requested(&upload_to_review).is_none());
}
