#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::Severity;

#[test]
fn list_path_repeats_bracketed_keys() {
    let path = build_list_path(
        "1",
        2,
        Some("mfa"),
        &[AssessmentStatus::NotMet, AssessmentStatus::NotApplicable],
        &[Severity::High],
    );
    assert_eq!(
        path,
        "/vendors/1/security-reviews/2/assessments?criteriaName=mfa\
         &statuses%5B%5D=NOT_MET&statuses%5B%5D=N_A&severities%5B%5D=HIGH"
    );
}

#[test]
fn list_path_without_filters_is_bare() {
    assert_eq!(
        build_list_path("1", 2, None, &[], &[]),
        "/vendors/1/security-reviews/2/assessments"
    );
}

#[test]
fn get_and_stats_paths() {
    assert_eq!(build_get_path("1", 2, 3), "/vendors/1/security-reviews/2/assessments/3");
    assert_eq!(
        build_stats_path("1", 2),
        "/vendors/1/security-reviews/2/assessments-stats"
    );
}
