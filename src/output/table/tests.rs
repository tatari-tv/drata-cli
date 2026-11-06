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
    spec_response_object("get", path, "200", false)
}

/// `spec_response_row` for any method and success status. With `every_key`,
/// a property with no `example` is present as `null`, so a key-sniffing
/// renderer still sees the whole shape.
fn spec_response_object(method: &str, path: &str, status: &str, every_key: bool) -> Value {
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
        &spec["paths"][path][method]["responses"][status]["content"]["application/json"]["schema"],
    );
    if let Some(items) = schema["properties"].get("data").and_then(|d| d.get("items")) {
        schema = resolve(schemas, items);
    }
    let mut row = serde_json::Map::new();
    for (key, prop) in schema["properties"].as_object().unwrap() {
        match resolve(schemas, prop).get("example") {
            Some(example) => {
                row.insert(key.clone(), example.clone());
            }
            None if every_key => {
                row.insert(key.clone(), Value::Null);
            }
            None => {}
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

#[test]
fn agent_workflow_responses_print_whole_as_json() {
    // No sniff may claim an agent response: a 202 acceptance body, a
    // finalization or a residual-risk evaluation prints whole, not as a
    // partial table of some other resource.
    let cases = [
        ("post", "agent", "202"),
        ("post", "criteria-confirmations", "200"),
        ("post", "assessment-runs", "202"),
        ("delete", "assessment-runs/current", "202"),
        ("post", "public-document-collections", "202"),
        ("get", "access-request-requirements", "200"),
        ("post", "access-requests", "202"),
        ("post", "access-links", "202"),
        ("post", "access-request-skips", "202"),
        ("post", "documents", "201"),
        ("post", "follow-up-questionnaires", "202"),
        ("get", "follow-up-questionnaire", "200"),
        ("post", "follow-up-questionnaire-sends", "202"),
        ("get", "residual-risk", "200"),
        ("post", "finalizations", "200"),
        ("get", "summary/status", "200"),
        ("get", "workflow", "200"),
    ];
    for (method, segment, status) in cases {
        let path = format!("{REVIEW}/{segment}");
        let body = spec_response_object(method, &path, status, true);
        assert!(
            body.as_object().is_some_and(|o| !o.is_empty()),
            "{method} {path}: no response properties"
        );
        assert!(
            pick_renderer(&body).is_none(),
            "{method} {path}: a renderer claimed {body}"
        );
        assert!(render(&body, DEFAULT_WIDTH).is_none(), "{method} {path}");
    }
}

const MONITOR: &str = "/workspaces/{workspaceId}/monitoring-tests/{testId}";

#[test]
fn monitor_phase_six_spec_rows_pick_their_own_renderer() {
    let cases: [(Value, RowRenderer, &str); 5] = [
        (
            spec_response_object("post", &format!("{MONITOR}/exclusions"), "201", false),
            render_item_results,
            "EXCLUDED",
        ),
        (
            spec_response_object("post", &format!("{MONITOR}/exclusions/action-delete"), "200", false),
            render_item_results,
            "DELETED",
        ),
        (
            spec_response_object("post", &format!("{MONITOR}/codebase-exclusions"), "201", false),
            render_item_results,
            "EXCLUDED",
        ),
        (
            spec_response_row(&format!("{MONITOR}/codebase-exclusions")),
            render_codebase_exclusions,
            "example-org/example-repo",
        ),
        (
            spec_response_object("post", &format!("{MONITOR}/action-retest"), "200", false),
            render_retests,
            "TESTING",
        ),
    ];
    for (row, want, cell) in cases {
        let got = pick_renderer(&row).unwrap_or_else(|| panic!("no renderer for {row}"));
        assert!(std::ptr::fn_addr_eq(got, want), "wrong renderer for {row}");
        let out = render(&json!({ "data": [row] }), DEFAULT_WIDTH).unwrap();
        assert!(out.contains(cell), "`{cell}` missing from\n{out}");
    }
}

#[test]
fn item_results_show_the_target_and_the_error() {
    let v = json!({"data": [
        {"statusCode": 201, "outcome": "EXCLUDED", "data": {"id": "sg-1", "connectionId": 1, "exclusionIds": [10, 11]}},
        {"statusCode": 422, "outcome": "FAILED_INVALID_TARGET", "error": {"message": "not a current failure"},
         "data": {"id": "sg-2", "connectionId": null, "exclusionIds": []}},
        {"statusCode": 200, "outcome": "DELETED", "data": {"exclusionId": 7}}
    ]});
    let out = render(&v, DEFAULT_WIDTH).unwrap();
    for cell in [
        "sg-1",
        "10,11",
        "sg-2",
        "not a current failure",
        "FAILED_INVALID_TARGET",
        "7",
    ] {
        assert!(out.contains(cell), "`{cell}` missing from\n{out}");
    }
}

#[test]
fn a_monitoring_test_keeps_the_monitor_renderer() {
    // Create and recipe answer a whole monitoring test, which carries `testId`
    // and `checkStatus` among its keys; the retest sniff must not take it.
    let test = spec_response_object("post", "/workspaces/{workspaceId}/monitoring-tests", "201", true);
    assert!(std::ptr::fn_addr_eq(
        pick_renderer(&test).unwrap(),
        render_monitors as RowRenderer
    ));
}

#[test]
fn recipe_validation_prints_whole_as_json() {
    let body = spec_response_object(
        "post",
        "/workspaces/{workspaceId}/monitoring-tests/action-validate-recipe",
        "200",
        true,
    );
    assert!(pick_renderer(&body).is_none(), "a renderer claimed {body}");
}

#[test]
fn personnel_phase_seven_spec_rows_pick_their_own_renderer() {
    let ws = "/workspaces/{workspaceId}";
    let cases: [(Value, RowRenderer, &str); 5] = [
        (
            spec_response_row(&format!("{ws}/compliance-check-exclusions")),
            render_compliance_exclusions,
            "Employee is on extended leave",
        ),
        (
            spec_response_row("/custom-field-definitions"),
            render_custom_fields,
            "Regulatory Owner",
        ),
        (spec_response_row("/groups"), render_groups, "Engineering Team"),
        (
            spec_response_row(&format!("{ws}/scoped-personnel-groups")),
            render_scoped_groups,
            "IT Ops",
        ),
        (
            // `employmentStatus` is an enum with no example, so every key is present as null.
            spec_response_object("get", "/personnel-search", "200", true),
            render_personnel,
            "2023-01-01",
        ),
    ];
    for (row, want, cell) in cases {
        let got = pick_renderer(&row).unwrap_or_else(|| panic!("no renderer for {row}"));
        assert!(std::ptr::fn_addr_eq(got, want), "wrong renderer for {row}");
        let out = render(&json!({ "data": [row] }), DEFAULT_WIDTH).unwrap();
        assert!(out.contains(cell), "`{cell}` missing from\n{out}");
    }
}

#[test]
fn phase_seven_single_objects_print_whole_as_json() {
    // Counts, resource attributes and the exclusion results carry no row shape
    // a sniff may claim: each prints whole.
    let ws = "/workspaces/{workspaceId}";
    for (method, path, status) in [
        ("get", format!("{ws}/scoped-personnel-counts"), "200"),
        (
            "get",
            format!("{ws}/custom-connections/{{connectionId}}/resources/{{resourceId}}/attributes"),
            "200",
        ),
        ("post", format!("{ws}/compliance-check-exclusions"), "201"),
        ("post", format!("{ws}/compliance-check-exclusions/action-delete"), "200"),
    ] {
        let body = spec_response_object(method, &path, status, true);
        assert!(pick_renderer(&body).is_none(), "{path}: a renderer claimed {body}");
    }
}

#[test]
fn custom_field_and_group_rows_show_their_columns() {
    let fields = json!({"data": [{"customFieldId": 42, "name": "Owner", "type": "TEXT", "isRequired": true,
        "isHidden": false, "entityTypes": ["RISK", "ASSET"]}]});
    let out = render(&fields, DEFAULT_WIDTH).unwrap();
    for cell in ["42", "Owner", "TEXT", "RISK,ASSET"] {
        assert!(out.contains(cell), "`{cell}` missing from\n{out}");
    }
    let exclusions = json!({"data": [{"id": 7, "targetId": "9", "targetType": "PERSONNEL", "reason": "left",
        "complianceCheckTypes": ["OFFBOARDING"], "startDate": "2026-07-01", "endDate": null}]});
    let out = render(&exclusions, DEFAULT_WIDTH).unwrap();
    for cell in ["7", "9", "OFFBOARDING", "left"] {
        assert!(out.contains(cell), "`{cell}` missing from\n{out}");
    }
}

#[test]
fn identity_phase_eight_spec_rows_pick_their_own_renderer() {
    for segment in ["infrastructure-user-identities", "version-control-user-identities"] {
        let base = format!("/workspaces/{{workspaceId}}/{segment}");
        let row = spec_response_row(&base);
        let got = pick_renderer(&row).unwrap_or_else(|| panic!("no renderer for {row}"));
        assert!(
            std::ptr::fn_addr_eq(got, render_identities as RowRenderer),
            "wrong renderer for {row}"
        );
        let out = render(&json!({ "data": [row] }), DEFAULT_WIDTH).unwrap();
        assert!(
            out.contains("jdoe@example.com"),
            "`jdoe@example.com` missing from\n{out}"
        );

        // Action rows share their keys with policy action rows, so the command
        // names its table and no sniff claims the shape.
        let action = spec_response_row(&format!("{base}/{{identityId}}/actions"));
        assert!(pick_renderer(&action).is_none(), "a sniff claimed {action}");
        let out = render_as(&json!({ "data": [action] }), DEFAULT_WIDTH, Table::IdentityActions).unwrap();
        assert!(out.contains("Link User"), "`Link User` missing from\n{out}");
        assert!(out.starts_with("ACTION"), "{out}");
    }
}

#[test]
fn identity_and_policy_action_rows_share_their_keys() {
    // Why the identity table is named, not sniffed: the two shapes are the same.
    let keys = |path: &str| {
        let mut k: Vec<String> = every_key("get", path).as_object().unwrap().keys().cloned().collect();
        k.sort();
        k
    };
    assert_eq!(
        keys("/workspaces/{workspaceId}/infrastructure-user-identities/{identityId}/actions"),
        keys("/policies/{policyId}/actions")
    );
}

#[test]
fn render_as_keeps_the_envelope_rules() {
    assert_eq!(
        render_as(&json!({"data": []}), DEFAULT_WIDTH, Table::IdentityActions).as_deref(),
        Some("(no results)")
    );
    assert!(render_as(&json!("text"), DEFAULT_WIDTH, Table::IdentityActions).is_none());
    let one = json!({"action": "LINK_USER", "label": "Link User", "description": "d"});
    assert!(
        render_as(&one, DEFAULT_WIDTH, Table::IdentityActions)
            .unwrap()
            .contains("LINK_USER")
    );
}

#[test]
fn policy_language_spec_rows_pick_their_own_renderer() {
    let cases: [(&str, RowRenderer, &str); 3] = [
        (
            "/policies/{policyId}/policy-versions/{policyVersionId}/languages",
            render_policy_languages,
            "information-security-policy-es.pdf",
        ),
        ("/policy-language-versions", render_policy_languages, "en-US"),
        ("/policy-language-settings", render_policy_language_settings, "en-US"),
    ];
    for (path, want, cell) in cases {
        let row = spec_response_row(path);
        let got = pick_renderer(&row).unwrap_or_else(|| panic!("no renderer for {row}"));
        assert!(std::ptr::fn_addr_eq(got, want), "wrong renderer for {path}: {row}");
        let out = render(&json!({ "data": [row] }), DEFAULT_WIDTH).unwrap();
        assert!(out.contains(cell), "`{cell}` missing from\n{out}");
    }
}

#[test]
fn evidence_item_rows_pick_the_item_table_and_library_rows_keep_the_library_table() {
    let item = "/workspaces/{workspaceId}/evidence/{evidenceId}";
    let cases: [(Value, RowRenderer, &str); 5] = [
        (
            spec_response_row("/workspaces/{workspaceId}/evidence"),
            render_evidence_items,
            "ONE_YEAR",
        ),
        (spec_response_row(item), render_evidence_items, "READY"),
        (
            spec_response_object("post", "/workspaces/{workspaceId}/evidence", "201", true),
            render_evidence_item_mutations,
            "Security Policy Document",
        ),
        (
            spec_response_row(&format!("{item}/artifacts")),
            render_evidence_artifacts,
            "S3_FILE",
        ),
        (
            spec_response_row("/workspaces/{workspaceId}/evidence-library"),
            render_evidence,
            "REQ-12",
        ),
    ];
    for (row, want, cell) in cases {
        let got = pick_renderer(&row).unwrap_or_else(|| panic!("no renderer for {row}"));
        assert!(std::ptr::fn_addr_eq(got, want), "wrong renderer for {row}");
        let out = render(&json!({ "data": [row] }), DEFAULT_WIDTH).unwrap();
        assert!(out.contains(cell), "`{cell}` missing from\n{out}");
    }
    let library = spec_response_row("/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}");
    assert!(std::ptr::fn_addr_eq(
        pick_renderer(&library).unwrap(),
        render_evidence as RowRenderer
    ));
}

/// A spec response row with every property present (`null` where the spec has
/// no example): access review rows carry required arrays with no example.
fn every_key(method: &str, path: &str) -> Value {
    spec_response_object(method, path, "200", true)
}

#[test]
fn access_review_spec_rows_pick_their_own_renderer() {
    let app = "/access-review-applications/{applicationId}";
    let period_app = "/access-review-periods/{periodId}/applications/{reviewAppId}";
    let cases: [(Value, RowRenderer, &str); 8] = [
        (
            every_key("get", "/access-review-applications"),
            render_access_review_apps,
            "MANUALLY_ADDED",
        ),
        (every_key("get", app), render_access_review_apps, "https://acme.com"),
        (
            every_key("get", &format!("{app}/users")),
            render_access_review_users,
            "jane.doe@acme.com",
        ),
        (
            every_key("get", "/access-review-periods"),
            render_access_review_periods,
            "APPLICATIONS",
        ),
        (
            every_key("get", "/access-review-periods/{periodId}"),
            render_access_review_periods,
            "COMPLETED",
        ),
        (
            every_key("get", period_app),
            render_access_review_period_apps,
            "IN_PROGRESS",
        ),
        (
            every_key("post", &format!("{period_app}/actions")),
            render_access_review_period_apps,
            "NOT_STARTED",
        ),
        (
            every_key("get", &format!("{period_app}/users")),
            render_access_review_decisions,
            "APPROVED",
        ),
    ];
    for (row, want, cell) in cases {
        let got = pick_renderer(&row).unwrap_or_else(|| panic!("no renderer for {row}"));
        assert!(std::ptr::fn_addr_eq(got, want), "wrong renderer for {row}");
        let out = render(&json!({ "data": [row] }), DEFAULT_WIDTH).unwrap();
        assert!(out.contains(cell), "`{cell}` missing from\n{out}");
    }
    // The evidence package is a signed URL: no table, so it prints whole as JSON.
    let package = every_key("get", "/access-review-periods/{periodId}/evidence-package");
    assert!(pick_renderer(&package).is_none(), "{package}");
}

#[test]
fn connection_spec_rows_pick_the_connection_renderer() {
    let cases: [(&str, &str, &str); 4] = [
        (
            "/aws-connections",
            "arn:aws:iam::123456789012:role/DrataAuditRole",
            "ACTIVE",
        ),
        (
            "/azure-connections",
            "28ffcc9b-be6d-4bbf-b53a-057049debc38",
            "My Azure Connection",
        ),
        ("/gcp-connections", "my-project", "My GCP Connection"),
        (
            "/gitlab-connections",
            "2025-07-01T16:45:55.246Z",
            "My GitLab Connection",
        ),
    ];
    for (path, target, other) in cases {
        for (method, p, status) in [
            ("get", path.to_string(), "200"),
            ("get", format!("{path}/{{connectionId}}"), "200"),
            ("post", path.to_string(), "201"),
        ] {
            let row = spec_response_object(method, &p, status, true);
            let got = pick_renderer(&row).unwrap_or_else(|| panic!("no renderer for {row}"));
            assert!(
                std::ptr::fn_addr_eq(got, render_connections as RowRenderer),
                "{method} {p}"
            );
            let out = render(&json!({ "data": [row] }), DEFAULT_WIDTH).unwrap();
            assert!(out.contains(target), "`{target}` missing from\n{out}");
            assert!(out.contains(other), "`{other}` missing from\n{out}");
        }
    }
}

#[test]
fn dcf_suggestion_rows_pick_the_suggestion_table() {
    let row = spec_response_row("/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}/dcf-suggestions");
    let got = pick_renderer(&row).unwrap_or_else(|| panic!("no renderer for {row}"));
    assert!(
        std::ptr::fn_addr_eq(got, render_dcf_suggestions as RowRenderer),
        "wrong renderer for {row}"
    );
    let out = render(&json!({ "data": [row] }), DEFAULT_WIDTH).unwrap();
    assert!(out.contains("DCF-1"), "`DCF-1` missing from\n{out}");
    assert!(out.contains("HIGH"), "`HIGH` missing from\n{out}");
}
