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
fn example_none_when_flag_false() {
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
fn example_none_for_non_create_actions() {
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
}

#[test]
fn example_skeleton_is_valid_json() {
    let parsed: serde_json::Value = serde_json::from_str(SECURITY_REVIEW_CREATE_EXAMPLE).unwrap();
    assert_eq!(parsed["securityReviewStatus"], serde_json::json!("NOT_YET_STARTED"));
    assert_eq!(parsed["securityReviewType"], serde_json::json!("SECURITY"));
}
