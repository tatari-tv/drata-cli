#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::Cli;
use clap::Parser;

/// Parse `vendor security-review <argv>` and return its agent action.
fn action(argv: &[&str]) -> SecurityReviewAgentAction {
    let cli = Cli::try_parse_from(
        ["drata", "vendor", "security-review"]
            .into_iter()
            .chain(argv.iter().copied()),
    )
    .unwrap();
    match cli.command {
        crate::cli::Commands::Vendor {
            action:
                crate::cli::VendorAction::SecurityReview {
                    action: crate::cli::VendorSecurityReviewAction::Agent(a),
                },
        } => a,
        other => panic!("not an agent verb: {other:?}"),
    }
}

fn planned(argv: &[&str]) -> Call {
    plan(&action(argv)).unwrap()
}

#[test]
fn review_path_appends_the_segment() {
    assert_eq!(
        review_path("v1", 2, "summary/status"),
        "/vendors/v1/security-reviews/2/summary/status"
    );
}

#[test]
fn every_verb_plans_its_method_and_path() {
    let cases: [(&[&str], &str, &str); 17] = [
        (&["start-agent", "1", "2"], "POST", "agent"),
        (&["confirm-criteria", "1", "2"], "POST", "criteria-confirmations"),
        (&["start-assessment", "1", "2"], "POST", "assessment-runs"),
        (&["stop-assessment", "1", "2"], "DELETE", "assessment-runs/current"),
        (
            &["collect-public-documents", "1", "2"],
            "POST",
            "public-document-collections",
        ),
        (
            &["access-request-requirements", "1", "2"],
            "GET",
            "access-request-requirements",
        ),
        (&["request-trust-center-access", "1", "2"], "POST", "access-requests"),
        (
            &[
                "provide-trust-center-access-link",
                "1",
                "2",
                "--access-link",
                "https://t.example",
            ],
            "POST",
            "access-links",
        ),
        (
            &["skip-trust-center-access-request", "1", "2"],
            "POST",
            "access-request-skips",
        ),
        (&["add-documents", "1", "2", "--file", "a.pdf"], "POST", "documents"),
        (
            &["generate-follow-up-questionnaire", "1", "2"],
            "POST",
            "follow-up-questionnaires",
        ),
        (&["follow-up-questionnaire", "1", "2"], "GET", "follow-up-questionnaire"),
        (
            &["send-follow-up-questionnaire", "1", "2", "--email", "a@x.test"],
            "POST",
            "follow-up-questionnaire-sends",
        ),
        (&["residual-risk", "1", "2"], "GET", "residual-risk"),
        (
            &["finalize", "1", "2", "--decision", "approved"],
            "POST",
            "finalizations",
        ),
        (&["summary", "1", "2"], "GET", "summary/status"),
        (&["workflow", "1", "2"], "GET", "workflow"),
    ];
    for (argv, method, segment) in cases {
        let call = planned(argv);
        assert_eq!(call.method(), method, "{argv:?}");
        assert_eq!(
            call.path(),
            format!("/vendors/1/security-reviews/2/{segment}"),
            "{argv:?}"
        );
    }
}

#[test]
fn action_verbs_send_an_empty_object() {
    for verb in [
        "start-agent",
        "start-assessment",
        "collect-public-documents",
        "skip-trust-center-access-request",
        "generate-follow-up-questionnaire",
    ] {
        match planned(&[verb, "1", "2"]) {
            Call::Post { body, .. } => assert_eq!(body, json!({}), "{verb}"),
            other => panic!("{verb}: {other:?}"),
        }
    }
}

#[test]
fn stop_assessment_is_a_delete_with_an_empty_object() {
    match planned(&["stop-assessment", "1", "2"]) {
        Call::Delete { body, .. } => assert_eq!(body, json!({})),
        other => panic!("{other:?}"),
    }
}

#[test]
fn confirm_criteria_body_omits_ids_for_the_default_set() {
    assert_eq!(build_confirm_criteria_body(&[]), json!({}));
    assert_eq!(
        build_confirm_criteria_body(&[101, 102]),
        json!({"criterionIds": [101, 102]})
    );
}

#[test]
fn access_request_body_wraps_the_fields_object() {
    assert_eq!(build_access_request_body(None).unwrap(), json!({}));
    assert_eq!(
        build_access_request_body(Some(json!({"firstName": "Jane", "agreeToTerms": true}))).unwrap(),
        json!({"fields": {"firstName": "Jane", "agreeToTerms": true}})
    );
    let err = build_access_request_body(Some(json!(["x"]))).unwrap_err().to_string();
    assert!(err.contains("JSON object") && err.contains("an array"), "{err}");
}

#[test]
fn request_trust_center_access_reads_inline_fields() {
    match planned(&[
        "request-trust-center-access",
        "1",
        "2",
        "--fields",
        r#"{"companyName":"Acme"}"#,
    ]) {
        Call::Post { body, .. } => assert_eq!(body, json!({"fields": {"companyName": "Acme"}})),
        other => panic!("{other:?}"),
    }
}

#[test]
fn access_link_body_requires_the_link() {
    assert_eq!(
        build_access_link_body(Some("https://t.example/share")).unwrap(),
        json!({"accessLink": "https://t.example/share"})
    );
    assert!(build_access_link_body(None).is_err());
}

#[test]
fn documents_form_sends_every_file_under_files_and_the_type() {
    let files: Vec<PathBuf> = vec!["a.pdf".into(), "b.pdf".into()];
    let form = build_documents_form(&files, Some(&SecurityReviewDocumentType::BridgeLetter)).unwrap();
    let parts: Vec<(&str, &std::path::Path)> = form
        .files
        .iter()
        .map(|f| (f.field.as_str(), f.path.as_path()))
        .collect();
    assert_eq!(
        parts,
        [
            ("files", std::path::Path::new("a.pdf")),
            ("files", std::path::Path::new("b.pdf"))
        ]
    );
    assert_eq!(form.fields, [("type".to_string(), "BRIDGE_LETTER".to_string())]);

    let untyped = build_documents_form(&files[..1], None).unwrap();
    assert!(untyped.fields.is_empty(), "no type unless asked");
}

#[test]
fn documents_form_rejects_more_than_ten_files() {
    let files: Vec<PathBuf> = (0..11).map(|i| PathBuf::from(format!("{i}.pdf"))).collect();
    let err = build_documents_form(&files, None).unwrap_err().to_string();
    assert!(err.contains("1 to 10") && err.contains("11"), "{err}");
    assert!(build_documents_form(&files[..10], None).is_ok());
    assert!(build_documents_form(&[], None).is_err());
}

#[test]
fn follow_up_send_body_carries_only_given_fields() {
    let email = vec!["sec@vendor.test".to_string()];
    assert_eq!(
        build_follow_up_send_body(&email, &[], &[], None, None).unwrap(),
        json!({"email": ["sec@vendor.test"]})
    );
    assert_eq!(
        build_follow_up_send_body(
            &email,
            &["m@x.test".to_string()],
            &["a@x.test".to_string()],
            Some("Subject"),
            Some("Body")
        )
        .unwrap(),
        json!({
            "email": ["sec@vendor.test"],
            "cc": ["m@x.test"],
            "bcc": ["a@x.test"],
            "emailSubject": "Subject",
            "emailContent": "Body"
        })
    );
}

#[test]
fn follow_up_send_body_enforces_recipient_limits() {
    assert!(build_follow_up_send_body(&[], &[], &[], None, None).is_err());
    let six: Vec<String> = (0..6).map(|i| format!("{i}@x.test")).collect();
    let err = build_follow_up_send_body(&six[..1], &six, &[], None, None)
        .unwrap_err()
        .to_string();
    assert!(err.contains("--cc") && err.contains("6"), "{err}");
    assert!(build_follow_up_send_body(&six, &[], &[], None, None).is_err());
    assert!(build_follow_up_send_body(&six[..5], &six[..5], &six[..5], None, None).is_ok());
}

#[test]
fn finalize_body_requires_a_decision() {
    assert_eq!(
        build_finalize_body(Some(&SecurityReviewDecision::ApprovedWithConditions), Some("ok")).unwrap(),
        json!({"decision": "APPROVED_WITH_CONDITIONS", "note": "ok"})
    );
    assert_eq!(
        build_finalize_body(Some(&SecurityReviewDecision::Pending), None).unwrap(),
        json!({"decision": "PENDING"})
    );
    let err = build_finalize_body(None, Some("ok")).unwrap_err().to_string();
    assert!(err.contains("--decision"), "{err}");
}

#[test]
fn data_replaces_the_flag_body() {
    match planned(&["finalize", "1", "2", "--data", r#"{"decision":"REJECTED"}"#]) {
        Call::Post { body, .. } => assert_eq!(body, json!({"decision": "REJECTED"})),
        other => panic!("{other:?}"),
    }
}

#[test]
fn example_without_ids_plans_nothing_but_prints_a_skeleton() {
    for verb in [
        "confirm-criteria",
        "request-trust-center-access",
        "provide-trust-center-access-link",
        "send-follow-up-questionnaire",
        "finalize",
    ] {
        let a = action(&[verb, "--example"]);
        let skeleton = example_if_requested(&a).expect("example requested").unwrap();
        assert!(skeleton.trim_start().starts_with('{'), "{verb}: {skeleton}");
        let err = plan(&a).unwrap_err().to_string();
        assert!(err.contains("requires <vendor_id>"), "{verb}: {err}");
    }
    assert!(example_if_requested(&action(&["workflow", "1", "2"])).is_none());
}
