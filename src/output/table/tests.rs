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

#[test]
fn render_ticket_rows_by_external_ticket_id() {
    // Shape of `TicketResponsePublicV2Dto`; `isDone` must not be mistaken for
    // a questionnaire (`isCompleted`) or any looser sniff.
    let v = json!({
        "data": [{
            "id": 12, "externalTicketId": "SEC-101", "isDone": false,
            "createdAt": "2025-07-01T16:45:55.246Z", "updatedAt": "2025-07-02T16:45:55.246Z",
            "connectionId": 4, "lastSyncedAt": null, "completionDate": null
        }]
    });
    assert!(std::ptr::fn_addr_eq(
        pick_renderer(&v["data"][0]).unwrap(),
        render_tickets as RowRenderer
    ));
    let out = render(&v, DEFAULT_WIDTH).unwrap();
    assert!(out.contains("EXTERNAL_ID"));
    assert!(out.contains("SEC-101"));
}

/// One row built from the `example` of every property in the op's 200
/// response (the item schema for a `data[]` list, the schema itself for a
/// single object), straight from the committed spec.
fn spec_response_row(path: &str) -> Value {
    let spec: Value = serde_json::from_str(include_str!("../../../spec/drata-openapi-v2.json")).unwrap();
    let schemas = &spec["components"]["schemas"];
    fn resolve<'a>(schemas: &'a Value, mut node: &'a Value) -> Value {
        while let Some(r) = node.get("$ref").and_then(|r| r.as_str()) {
            node = &schemas[r.rsplit('/').next().unwrap()];
        }
        node.clone()
    }
    let mut schema = resolve(
        schemas,
        &spec["paths"][path]["get"]["responses"]["200"]["content"]["application/json"]["schema"],
    );
    if let Some(items) = schema["properties"].get("data").and_then(|d| d.get("items")) {
        schema = resolve(schemas, items);
    }
    let mut row = serde_json::Map::new();
    for (key, prop) in schema["properties"].as_object().unwrap() {
        if let Some(example) = resolve(schemas, prop).get("example") {
            row.insert(key.clone(), example.clone());
        }
    }
    Value::Object(row)
}

const REVIEW: &str = "/vendors/{vendorId}/security-reviews/{securityReviewId}";

#[test]
fn spec_example_rows_pick_their_own_renderer() {
    let cases: [(String, RowRenderer, &str); 10] = [
        (format!("{REVIEW}/notes"), render_findings, "Procurement approved"),
        (
            format!("{REVIEW}/observations"),
            render_findings,
            "SOC 2 Type II report",
        ),
        (format!("{REVIEW}/assessments"), render_assessments, "My Criteria"),
        (
            format!("{REVIEW}/assessments/{{assessmentId}}"),
            render_assessments,
            "My Criteria",
        ),
        (format!("{REVIEW}/assessments-stats"), render_assessment_stats, "44"),
        (
            "/vendor-criteria".to_string(),
            render_criteria,
            "Data Security Criteria",
        ),
        (
            "/vendor-questionnaire-templates".to_string(),
            render_templates,
            "Security Assessment",
        ),
        (
            "/vendor-questionnaire-templates/{questionnaireId}/questions".to_string(),
            render_questions,
            "Privacy Policy",
        ),
        ("/vendor-risk-levels".to_string(), render_risk_levels, "Critical"),
        (
            "/workspaces/{workspaceId}/vendor-criteria-creations".to_string(),
            render_criteria_creations,
            "IN_PROGRESS",
        ),
    ];
    for (path, want, cell) in cases {
        let row = spec_response_row(&path);
        let got = pick_renderer(&row).unwrap_or_else(|| panic!("{path}: no renderer for {row}"));
        assert!(std::ptr::fn_addr_eq(got, want), "{path}: wrong renderer for {row}");
        let out = render(&json!({ "data": [row] }), DEFAULT_WIDTH).unwrap();
        assert!(out.contains(cell), "{path}: `{cell}` missing from\n{out}");
    }
}

#[test]
fn vendor_question_rows_with_answers_render_the_answer() {
    let row = spec_response_row("/vendors/{vendorId}/questionnaires/{questionnaireId}/questions");
    assert!(std::ptr::fn_addr_eq(
        pick_renderer(&row).unwrap(),
        render_questions as RowRenderer
    ));
    let out = render(&row, DEFAULT_WIDTH).unwrap();
    assert!(out.contains("Yes, you can get to this right here!"), "{out}");
}

#[test]
fn new_sniffs_do_not_steal_the_rows_that_came_first() {
    // A control (`code` + `question`), a vendor and a security review keep their renderers.
    let control = json!({"id": 1, "code": "DCF-1", "question": "q", "activity": "a"});
    assert!(std::ptr::fn_addr_eq(
        pick_renderer(&control).unwrap(),
        render_controls as RowRenderer
    ));
    let vendor = json!({"id": 1, "name": "x", "category": "SECURITY", "risk": "LOW"});
    assert!(std::ptr::fn_addr_eq(
        pick_renderer(&vendor).unwrap(),
        render_vendors as RowRenderer
    ));
    let review = json!({"id": 1, "reviewDeadlineAt": "2027-01-01", "vendorId": 3});
    assert!(std::ptr::fn_addr_eq(
        pick_renderer(&review).unwrap(),
        render_security_reviews as RowRenderer
    ));
}

#[test]
fn criteria_creation_sniff_needs_exactly_its_four_keys() {
    let extra = json!({"id": 1, "workspaceId": 1, "status": "X", "createdAt": "t", "name": "n"});
    assert!(pick_renderer(&extra).is_none_or(|r| !std::ptr::fn_addr_eq(r, render_criteria_creations as RowRenderer)));
}
