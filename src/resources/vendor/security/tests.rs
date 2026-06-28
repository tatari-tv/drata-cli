#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{SecurityReviewAction, SecurityReviewStatus, SecurityReviewType, VendorSecurityReviewAction};

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

// ---------------------------------------------------------------------------
// example_if_requested gating
// ---------------------------------------------------------------------------

#[test]
fn example_only_for_create_with_flag() {
    let create_example = VendorSecurityReviewAction::Create {
        vendor_id: "v1".to_string(),
        review_deadline_at: "2026-12-31".to_string(),
        status: SecurityReviewStatus::NotYetStarted,
        review_type: SecurityReviewType::Security,
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
        review_deadline_at: "2026-12-31".to_string(),
        status: SecurityReviewStatus::NotYetStarted,
        review_type: SecurityReviewType::Security,
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
        example: false,
    };
    assert!(example_if_requested(&update_no_example).is_none());
}

#[test]
fn example_none_for_non_create_non_update_actions() {
    let list = VendorSecurityReviewAction::List {
        vendor_id: "v1".to_string(),
        status: None,
        review_type: None,
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
fn update_body_only_has_title_and_soc_form() {
    // UpdateDTO has only title + socForm; create's fields must not appear
    let mut body = serde_json::json!({});
    set_opt_str(&mut body, "title", Some("Updated title"));
    set_opt_str(&mut body, "socForm", Some("SOC2_TYPE_II"));

    assert_eq!(body["title"], serde_json::json!("Updated title"));
    assert_eq!(body["socForm"], serde_json::json!("SOC2_TYPE_II"));
    // Confirm create-only fields are absent
    assert!(body.get("securityReviewStatus").is_none());
    assert!(body.get("securityReviewType").is_none());
    assert!(body.get("reviewDeadlineAt").is_none());
}

#[test]
fn run_action_body_uses_action_string() {
    let finalize_body = serde_json::json!({ "action": action_str(&SecurityReviewAction::Finalize) });
    let reopen_body = serde_json::json!({ "action": action_str(&SecurityReviewAction::Reopen) });

    assert_eq!(finalize_body["action"], serde_json::json!("finalize"));
    assert_eq!(reopen_body["action"], serde_json::json!("reopen"));
}
