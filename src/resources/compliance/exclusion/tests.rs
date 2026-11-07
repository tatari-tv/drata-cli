#![allow(clippy::unwrap_used)]
use super::*;

fn ids(n: usize) -> Vec<String> {
    (1..=n).map(|i| i.to_string()).collect()
}

#[test]
fn create_body_carries_targets_checks_reason_and_dates() {
    let body = build_create_body(
        &ids(2),
        &[ComplianceCheckType::Offboarding, ComplianceCheckType::HddEncryption],
        Some("left early"),
        Some("2026-07-01T00:00:00Z"),
        Some("2026-08-01T00:00:00Z"),
    )
    .unwrap();
    assert_eq!(
        body,
        json!({
            "targetIds": ["1", "2"],
            "complianceCheckTypes": ["OFFBOARDING", "HDD_ENCRYPTION"],
            "reason": "left early",
            "startDate": "2026-07-01T00:00:00Z",
            "endDate": "2026-08-01T00:00:00Z",
        })
    );
}

#[test]
fn create_body_omits_end_date_when_open_ended() {
    let body = build_create_body(
        &ids(1),
        &[ComplianceCheckType::Offboarding],
        Some("r"),
        Some("2026-07-01T00:00:00Z"),
        None,
    )
    .unwrap();
    assert!(body.get("endDate").is_none());
}

#[test]
fn create_requires_reason_start_date_checks_and_targets() {
    let checks = [ComplianceCheckType::Offboarding];
    let start = Some("2026-07-01T00:00:00Z");
    let e = |r: Result<Value>| r.unwrap_err().to_string();
    assert!(e(build_create_body(&ids(1), &checks, None, start, None)).contains("--reason"));
    assert!(e(build_create_body(&ids(1), &checks, Some("r"), None, None)).contains("--start-date"));
    assert!(e(build_create_body(&ids(1), &[], Some("r"), start, None)).contains("--compliance-check"));
    assert!(e(build_create_body(&[], &checks, Some("r"), start, None)).contains("--target-id"));
}

#[test]
fn target_ids_are_positive_whole_numbers_at_most_500() {
    let checks = [ComplianceCheckType::Offboarding];
    assert!(build_delete_body(&ids(500), &checks).is_ok());
    assert!(build_delete_body(&ids(501), &checks).is_err());
    for bad in ["0", "012", "-1", "1.5", "abc", ""] {
        let err = build_delete_body(&[bad.to_string()], &checks).unwrap_err().to_string();
        assert!(err.contains("not a personnel ID"), "{bad}: {err}");
    }
}

#[test]
fn delete_body_is_targets_and_checks_only() {
    let body = build_delete_body(&ids(1), &[ComplianceCheckType::Offboarding]).unwrap();
    assert_eq!(
        body,
        json!({"targetIds": ["1"], "complianceCheckTypes": ["OFFBOARDING"]})
    );
    assert!(build_delete_body(&ids(1), &[]).is_err());
}

#[test]
fn list_path_encodes_check_and_expand() {
    assert_eq!(list_path("1", None, &[]), "/workspaces/1/compliance-check-exclusions");
    assert_eq!(
        list_path(
            "1",
            Some(&ComplianceCheckType::Offboarding),
            &[
                ComplianceCheckExclusionExpand::ComplianceCheckTypes,
                ComplianceCheckExclusionExpand::CreatedBy
            ]
        ),
        "/workspaces/1/compliance-check-exclusions?complianceCheck=OFFBOARDING\
         &expand%5B%5D=complianceCheckTypes&expand%5B%5D=createdBy"
    );
}
