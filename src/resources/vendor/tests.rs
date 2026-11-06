#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::VendorAction;

#[test]
fn vendor_name_reads_name() {
    let v = json!({"id": 1, "name": "Okta"});
    assert_eq!(vendor_name(&v), "Okta");
}

#[test]
fn vendor_name_defaults_empty() {
    let v = json!({"id": 1});
    assert_eq!(vendor_name(&v), "");
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({"name": "x"});
    set_opt(&mut body, "category", Some("SECURITY"));
    set_opt(&mut body, "risk", None);
    assert_eq!(body["category"], json!("SECURITY"));
    assert!(body.get("risk").is_none());
}

#[test]
fn example_only_for_create_with_flag() {
    let create_example = VendorAction::Create {
        name: None,
        category: None,
        risk: None,
        status: None,
        url: None,
        notes: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = VendorAction::Create {
        name: Some("x".to_string()),
        category: None,
        risk: None,
        status: None,
        url: None,
        notes: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = VendorAction::List {
        patterns: vec![],
        all: false,
        expand: vec![],
        status: vec![],
        category: vec![],
        vendor_type: vec![],
        vendor_type_id: vec![],
        vendor_user_id: vec![],
        sub_processor: None,
        action_required: None,
        review_deadline_status: vec![],
    };
    assert!(example_if_requested(&list).is_none());
}

#[test]
fn example_skeleton_is_valid_json() {
    let parsed: Value = serde_json::from_str(VENDOR_CREATE_EXAMPLE).unwrap();
    assert_eq!(parsed["name"], json!("Example Vendor"));
}

#[test]
fn vendor_list_encodes_status_filters() {
    use crate::cli::{ReviewDeadlineStatus, VendorCategory, VendorStatus, VendorType};
    let path = build_list_path(
        &["documents".to_string()],
        &[VendorStatus::Active, VendorStatus::OnHold],
        &[VendorCategory::Cs],
        &[VendorType::Supplier],
        &[4, 5],
        &[9],
        Some(true),
        Some(false),
        &[ReviewDeadlineStatus::DueSoon, ReviewDeadlineStatus::Overdue],
    );
    // Array filters repeat their bracketed key once per value (never
    // comma-joined); the two booleans are scalars; expand[] comes last.
    assert_eq!(
        path,
        "/vendors?statuses%5B%5D=ACTIVE&statuses%5B%5D=ON_HOLD\
         &categories%5B%5D=CS\
         &types%5B%5D=SUPPLIER\
         &vendorTypeIds%5B%5D=4&vendorTypeIds%5B%5D=5\
         &vendorUserIds%5B%5D=9\
         &isSubProcessor=true&actionRequired=false\
         &nextReviewDeadlineStatus%5B%5D=DUE_SOON&nextReviewDeadlineStatus%5B%5D=OVERDUE\
         &expand%5B%5D=documents"
    );
}

#[test]
fn vendor_list_without_filters_is_bare() {
    let path = build_list_path(&[], &[], &[], &[], &[], &[], None, None, &[]);
    assert_eq!(path, "/vendors");
}

#[test]
fn vendor_list_status_flag_is_space_separated_and_case_insensitive() {
    use crate::cli::{Cli, Commands};
    use clap::Parser;
    let cli = Cli::try_parse_from(["drata", "vendor", "list", "--status", "active", "on_hold"]).unwrap();
    let Commands::Vendor {
        action: VendorAction::List { status, .. },
    } = cli.command
    else {
        panic!("expected vendor list");
    };
    assert_eq!(status.len(), 2);
}
