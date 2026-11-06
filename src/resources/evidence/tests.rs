#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{EvidenceAction, RenewalScheduleType};

#[test]
fn example_only_for_create_with_flag() {
    let create_example = EvidenceAction::Create {
        workspace_id: Some("w1".to_string()),
        name: None,
        description: None,
        renewal_schedule_type: None,
        filed_at: None,
        owner_id: None,
        file: None,
        file_key: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = EvidenceAction::Create {
        workspace_id: Some("w1".to_string()),
        name: Some("e".to_string()),
        description: None,
        renewal_schedule_type: None,
        filed_at: None,
        owner_id: None,
        file: None,
        file_key: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = EvidenceAction::List {
        workspace_id: "w1".to_string(),
        all: false,
        expand: vec![],
    };
    assert!(example_if_requested(&list).is_none());
}

#[test]
fn renewal_schedule_type_str_roundtrips() {
    assert_eq!(renewal_schedule_type_str(&RenewalScheduleType::OneMonth), "ONE_MONTH");
    assert_eq!(renewal_schedule_type_str(&RenewalScheduleType::OneYear), "ONE_YEAR");
    assert_eq!(renewal_schedule_type_str(&RenewalScheduleType::None), "NONE");
    assert_eq!(renewal_schedule_type_str(&RenewalScheduleType::Custom), "CUSTOM");
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({});
    set_opt(&mut body, "name", Some("Ev"));
    set_opt(&mut body, "description", None);
    assert_eq!(body["name"], json!("Ev"));
    assert!(body.get("description").is_none());
}

fn parse(args: &[&str]) -> Result<crate::cli::Cli, clap::Error> {
    use clap::Parser;
    crate::cli::Cli::try_parse_from(["drata", "evidence", "create", "1", "--name", "n"].iter().chain(args))
}

#[test]
fn evidence_create_file_key_requires_owner_and_filed_at() {
    // --file-key alone, or missing any one of its three companions, is a clap usage error.
    let err = parse(&["--file-key", "k"]).unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
    for missing in ["--owner-id", "--filed-at", "--renewal-schedule-type"] {
        let mut args = vec![
            "--file-key",
            "k",
            "--owner-id",
            "7",
            "--filed-at",
            "2026-11-01",
            "--renewal-schedule-type",
            "ONE_YEAR",
        ];
        let at = args.iter().position(|a| *a == missing).unwrap();
        args.drain(at..at + 2);
        let err = parse(&args).unwrap_err();
        assert_eq!(
            err.kind(),
            clap::error::ErrorKind::MissingRequiredArgument,
            "without {missing}"
        );
        assert!(err.to_string().contains(missing), "{err}");
    }
    assert!(
        parse(&[
            "--file-key",
            "k",
            "--owner-id",
            "7",
            "--filed-at",
            "2026-11-01",
            "--renewal-schedule-type",
            "one_year",
        ])
        .is_ok()
    );
}

#[test]
fn evidence_create_file_requires_the_same_companions() {
    let err = parse(&["--file", "/dev/null"]).unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::MissingRequiredArgument);
}

#[test]
fn evidence_create_file_and_file_key_conflict() {
    let err = parse(&[
        "--file",
        "/dev/null",
        "--file-key",
        "k",
        "--owner-id",
        "7",
        "--filed-at",
        "2026-11-01",
        "--renewal-schedule-type",
        "ONE_YEAR",
    ])
    .unwrap_err();
    assert_eq!(err.kind(), clap::error::ErrorKind::ArgumentConflict);
}

#[test]
fn evidence_create_without_artifact_needs_no_companions() {
    assert!(parse(&[]).is_ok());
}

#[test]
fn evidence_create_body_carries_file_key_source() {
    let fields = CreateFields {
        description: None,
        renewal_schedule_type: Some(&RenewalScheduleType::OneYear),
        filed_at: Some("2026-11-01"),
        owner_id: Some(7),
    };
    let body = build_create_body("n", &fields, Some("acct/evidence-library/1/x/f.txt"));
    assert_eq!(
        body,
        json!({
            "name": "n",
            "renewalScheduleType": "ONE_YEAR",
            "filedAt": "2026-11-01",
            "ownerId": 7,
            "fileKey": "acct/evidence-library/1/x/f.txt",
        })
    );
}

#[test]
fn evidence_create_form_carries_filed_at_and_owner() {
    let fields = CreateFields {
        description: Some("d"),
        renewal_schedule_type: Some(&RenewalScheduleType::SixMonths),
        filed_at: Some("2026-11-01"),
        owner_id: Some(7),
    };
    let form = build_create_form(std::path::Path::new("/tmp/a.pdf"), "n", &fields);
    let get = |k: &str| form.fields.iter().find(|(n, _)| n == k).map(|(_, v)| v.as_str());
    assert_eq!(form.files.len(), 1);
    assert_eq!(form.files[0].field, "file");
    assert_eq!(get("filedAt"), Some("2026-11-01"));
    assert_eq!(get("ownerId"), Some("7"));
    assert_eq!(get("renewalScheduleType"), Some("SIX_MONTHS"));
    assert_eq!(get("fileKey"), None);
}
