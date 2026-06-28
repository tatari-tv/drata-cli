#![allow(clippy::unwrap_used)]
use super::*;
use serde_json::json;

#[test]
fn render_vendors_shows_core_columns() {
    let v = json!({
        "data": [
            {"id": 1, "name": "Okta", "category": "SECURITY", "risk": "LOW",
             "status": "ACTIVE", "renewalDate": "2026-01-01"},
            {"id": 2, "name": "AWS", "category": "ENGINEERING", "risk": "MODERATE",
             "status": "APPROVED", "renewalDate": null}
        ]
    });
    let out = render(&v, DEFAULT_WIDTH).unwrap();
    assert!(out.contains("NAME"));
    assert!(out.contains("CATEGORY"));
    assert!(out.contains("Okta"));
    assert!(out.contains("SECURITY"));
    assert!(out.contains("AWS"));
    // Numeric IDs render as numbers, not blank.
    assert!(out.contains("1"));
    assert!(out.contains("2"));
}

#[test]
fn render_single_vendor_get_envelope() {
    let v = json!({"id": 7, "name": "Datadog", "category": "ENGINEERING", "risk": "LOW", "status": "ACTIVE"});
    let out = render(&v, DEFAULT_WIDTH).unwrap();
    assert!(out.contains("Datadog"));
    assert!(out.contains("7"));
}

#[test]
fn render_questionnaires_shape() {
    let v = json!({
        "data": [
            {"id": 10, "title": "SIG Lite", "recipientEmail": "sec@vendor.com",
             "isCompleted": false, "dateSent": "2026-06-01"}
        ]
    });
    let out = render(&v, DEFAULT_WIDTH).unwrap();
    assert!(out.contains("TITLE"));
    assert!(out.contains("RECIPIENT"));
    assert!(out.contains("sec@vendor.com"));
    assert!(out.contains("false"));
}

#[test]
fn render_empty_data_is_no_results() {
    let v = json!({"data": []});
    let out = render(&v, DEFAULT_WIDTH).unwrap();
    assert!(out.contains("no results"));
}

#[test]
fn render_unknown_shape_returns_none() {
    let v = json!({"data": [{"weird": "shape"}]});
    assert!(render(&v, DEFAULT_WIDTH).is_none());
}

#[test]
fn truncate_is_char_boundary_safe() {
    // A multibyte string must not panic when truncated mid-character.
    let s = "héllo wörld with ünicode";
    let out = truncate(s, 8);
    assert!(out.chars().count() <= 8);
    assert!(out.ends_with('…'));
}

// ---------------------------------------------------------------------------
// Security review renderer
// ---------------------------------------------------------------------------

#[test]
fn render_security_reviews_list_shape() {
    let v = json!({
        "data": [
            {
                "id": 1,
                "title": "Annual SOC2 Review",
                "status": "IN_PROGRESS",
                "type": "SOC_REPORT",
                "decision": null,
                "reviewDeadlineAt": "2026-12-31T00:00:00.000Z"
            },
            {
                "id": 2,
                "title": "Security Assessment",
                "status": "NOT_YET_STARTED",
                "type": "SECURITY",
                "decision": "APPROVED",
                "reviewDeadlineAt": "2026-06-30T00:00:00.000Z"
            }
        ]
    });
    let out = render(&v, DEFAULT_WIDTH).unwrap();
    assert!(out.contains("ID"));
    assert!(out.contains("TITLE"));
    assert!(out.contains("STATUS"));
    assert!(out.contains("TYPE"));
    assert!(out.contains("DECISION"));
    assert!(out.contains("DEADLINE"));
    assert!(out.contains("Annual SOC2 Review"));
    assert!(out.contains("IN_PROGRESS"));
    assert!(out.contains("SOC_REPORT"));
    assert!(out.contains("Security Assessment"));
    assert!(out.contains("NOT_YET_STARTED"));
    assert!(out.contains("APPROVED"));
    // Numeric IDs render as numbers.
    assert!(out.contains("1"));
    assert!(out.contains("2"));
}

#[test]
fn render_security_review_single_get() {
    // Single-object get response (no "data" wrapper) is wrapped as a one-row list.
    let v = json!({
        "id": 42,
        "title": "Vendor Review",
        "status": "COMPLETED",
        "type": "UPLOAD_REPORT",
        "decision": "APPROVED",
        "reviewDeadlineAt": "2026-09-01T00:00:00.000Z"
    });
    let out = render(&v, DEFAULT_WIDTH).unwrap();
    assert!(out.contains("42"));
    assert!(out.contains("Vendor Review"));
    assert!(out.contains("COMPLETED"));
    assert!(out.contains("UPLOAD_REPORT"));
    assert!(out.contains("APPROVED"));
}

#[test]
fn render_security_reviews_null_title_and_decision() {
    // Nullable fields (title, decision) render as empty string, not "null".
    let v = json!({
        "data": [
            {
                "id": 5,
                "title": null,
                "status": "NOT_YET_STARTED",
                "type": "SECURITY",
                "decision": null,
                "reviewDeadlineAt": "2027-01-01T00:00:00.000Z"
            }
        ]
    });
    let out = render(&v, DEFAULT_WIDTH).unwrap();
    assert!(out.contains("5"));
    assert!(out.contains("NOT_YET_STARTED"));
    // Null fields must not render as the word "null".
    assert!(!out.contains("null"), "null fields should render as empty, got: {out}");
}
