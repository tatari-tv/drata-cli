#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{PolicyLanguageVersionExpand, PolicyLanguageVersionListExpand, PolicyVersionStatus, policy_language};

#[test]
fn list_path_encodes_statuses_per_value() {
    assert_eq!(list_path("7", "9", &[]), "/policies/7/policy-versions/9/languages");
    assert_eq!(
        list_path(
            "7",
            "9",
            &[PolicyVersionStatus::NeedsApproval, PolicyVersionStatus::Published]
        ),
        "/policies/7/policy-versions/9/languages?statuses%5B%5D=NEEDS_APPROVAL&statuses%5B%5D=PUBLISHED"
    );
}

#[test]
fn versions_path_carries_every_filter() {
    assert_eq!(versions_path(None, None, &[], &[]), "/policy-language-versions");
    assert_eq!(
        versions_path(
            Some(9),
            Some(7),
            &[PolicyVersionStatus::Draft],
            &[PolicyLanguageVersionListExpand::PolicyVersion]
        ),
        "/policy-language-versions?policyVersionId=9&policyId=7&statuses%5B%5D=DRAFT&expand%5B%5D=policyVersion"
    );
}

#[test]
fn get_path_expands_and_encodes_the_id() {
    assert_eq!(get_path("3", &[]), "/policy-language-versions/3");
    assert_eq!(
        get_path("3", &[PolicyLanguageVersionExpand::DownloadPdfUrl]),
        "/policy-language-versions/3?expand%5B%5D=downloadPdfUrl"
    );
    assert_eq!(get_path("a/b", &[]), "/policy-language-versions/a%2Fb");
}

#[test]
fn language_matches_the_spec_case_insensitively() {
    assert_eq!(policy_language("FR-fr").unwrap(), "fr-FR");
    assert_eq!(policy_language("nl").unwrap(), "nl");
    assert!(policy_language("klingon").is_err());
}
