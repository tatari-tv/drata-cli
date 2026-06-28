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
