#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::EmploymentStatus;

#[test]
fn employment_status_str_roundtrips() {
    assert_eq!(
        employment_status_str(&EmploymentStatus::CurrentEmployee),
        "CURRENT_EMPLOYEE"
    );
    assert_eq!(
        employment_status_str(&EmploymentStatus::FormerEmployee),
        "FORMER_EMPLOYEE"
    );
    assert_eq!(
        employment_status_str(&EmploymentStatus::ServiceAccount),
        "SERVICE_ACCOUNT"
    );
    assert_eq!(employment_status_str(&EmploymentStatus::FutureHire), "FUTURE_HIRE");
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({});
    set_opt(&mut body, "startedAt", Some("2024-01-01"));
    set_opt(&mut body, "separatedAt", None);
    assert_eq!(body["startedAt"], json!("2024-01-01"));
    assert!(body.get("separatedAt").is_none());
}
