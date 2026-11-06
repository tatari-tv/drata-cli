#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{Cli, Commands, PersonnelAction};
use clap::Parser;

fn args_of(extra: &[&str]) -> PersonnelSearchArgs {
    let argv = ["drata", "personnel", "search"].iter().chain(extra.iter());
    match Cli::try_parse_from(argv).unwrap().command {
        Commands::Personnel {
            action: PersonnelAction::Search(a),
        } => *a,
        other => panic!("not a personnel search: {other:?}"),
    }
}

fn built(extra: &[&str]) -> (String, Vec<String>) {
    let (q, facets) = build_search(&args_of(extra)).unwrap();
    (q.finish(), facets)
}

#[test]
fn no_flags_search_every_workspace() {
    assert_eq!(built(&[]), ("/personnel-search".to_string(), vec![]));
}

#[test]
fn workspace_flag_picks_the_per_workspace_op() {
    let (path, _) = built(&["--workspace-id", "7"]);
    assert_eq!(path, "/workspaces/7/personnel-search");
}

#[test]
fn filters_are_bracketed_and_repeated_per_value() {
    let (path, facets) = built(&[
        "--q",
        "Smith",
        "--employment-status",
        "current_employee",
        "FUTURE_HIRE",
        "--offboarding-status",
        "fail",
        "--group-id",
        "3",
        "4",
        "--sync-status",
        "not_synced",
        "--expand",
        "user",
        "customfields",
        "--facet",
        "groupids",
    ]);
    assert_eq!(
        path,
        "/personnel-search?q=Smith\
         &employmentStatus%5B%5D=CURRENT_EMPLOYEE&employmentStatus%5B%5D=FUTURE_HIRE\
         &offboardingStatus%5B%5D=FAIL\
         &groupIds%5B%5D=3&groupIds%5B%5D=4\
         &syncStatus=NOT_SYNCED\
         &expand%5B%5D=user&expand%5B%5D=customFields"
    );
    assert_eq!(facets, vec!["groupIds".to_string()]);
}

#[test]
fn every_status_flag_reaches_its_own_query_key() {
    let flags = [
        ("--compliance-status", "complianceStatus"),
        ("--accepted-policies-status", "acceptedPoliciesStatus"),
        ("--bg-check-status", "bgCheckStatus"),
        ("--identity-mfa-status", "identityMfaStatus"),
        ("--security-training-status", "securityTrainingStatus"),
        ("--hipaa-training-status", "hipaaTrainingStatus"),
        ("--ai-awareness-training-status", "aiAwarenessTrainingStatus"),
        ("--offboarding-status", "offboardingStatus"),
        ("--agent-installed-status", "agentInstalledStatus"),
        ("--password-manager-status", "passwordManagerStatus"),
        ("--disk-encryption-status", "diskEncryptionStatus"),
        ("--antivirus-status", "antivirusStatus"),
        ("--auto-updates-status", "autoUpdatesStatus"),
        ("--lock-screen-status", "lockScreenStatus"),
    ];
    for (flag, key) in flags {
        let (path, _) = built(&[flag, "PASS"]);
        assert_eq!(path, format!("/personnel-search?{key}%5B%5D=PASS"), "{flag}");
    }
}

#[test]
fn custom_field_filters_use_id_or_name_with_a_bracketed_index() {
    let (path, _) = built(&["--custom-field", "42=Engineering", "Department=Ops team"]);
    assert_eq!(
        path,
        "/personnel-search?customFields%5B0%5D%5BcustomFieldId%5D=42&customFields%5B0%5D%5Bvalue%5D=Engineering\
         &customFields%5B1%5D%5Bname%5D=Department&customFields%5B1%5D%5Bvalue%5D=Ops%20team"
    );
}

#[test]
fn range_filters_send_only_the_bounds_given() {
    let (path, _) = built(&[
        "--custom-range-field",
        "99=5..10",
        "Score=..7",
        "Start=2025-01-01T00:00:00Z..",
    ]);
    assert_eq!(
        path,
        "/personnel-search?customRangeFields%5B0%5D%5BcustomFieldId%5D=99\
         &customRangeFields%5B0%5D%5Bgte%5D=5&customRangeFields%5B0%5D%5Blte%5D=10\
         &customRangeFields%5B1%5D%5Bname%5D=Score&customRangeFields%5B1%5D%5Blte%5D=7\
         &customRangeFields%5B2%5D%5Bname%5D=Start&customRangeFields%5B2%5D%5Bgte%5D=2025-01-01T00:00:00Z"
    );
}

#[test]
fn malformed_custom_filters_fail_before_any_request() {
    for (flag, value) in [
        ("--custom-field", "no-equals"),
        ("--custom-field", "=value"),
        ("--custom-range-field", "99=5"),
        ("--custom-range-field", "99=.."),
    ] {
        let err = build_search(&args_of(&[flag, value])).unwrap_err().to_string();
        assert!(err.contains(flag), "{err}");
    }
}

#[test]
fn text_filters_over_191_chars_fail() {
    let long = "x".repeat(192);
    let err = build_search(&args_of(&["--first-name", &long]))
        .unwrap_err()
        .to_string();
    assert!(err.contains("--first-name"), "{err}");
    assert!(build_search(&args_of(&["--first-name", &"x".repeat(191)])).is_ok());
}
