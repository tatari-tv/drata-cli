#![allow(clippy::unwrap_used)]
use super::*;

fn request() -> AuditRequestRef {
    AuditRequestRef {
        workspace_id: "1".to_string(),
        audit_id: "a/b".to_string(),
        request_id: 7,
    }
}

fn suggestion() -> DcfSuggestionRef {
    DcfSuggestionRef {
        workspace_id: "1".to_string(),
        audit_id: "2".to_string(),
        request_id: 7,
        suggestion_id: 9,
    }
}

#[test]
fn paths_address_the_requests_and_their_suggestions() {
    assert_eq!(
        generate_path("1", "2"),
        "/workspaces/1/audits/2/requests/action-generate-dcf-suggestions"
    );
    assert_eq!(
        accept_path("1", "2"),
        "/workspaces/1/audits/2/requests/action-accept-dcf-suggestions"
    );
    assert_eq!(
        item_path(&suggestion()),
        "/workspaces/1/audits/2/requests/7/dcf-suggestions/9"
    );
    assert_eq!(
        actions_path(&suggestion()),
        "/workspaces/1/audits/2/requests/7/dcf-suggestions/9/actions"
    );
}

#[test]
fn list_path_repeats_each_filter_value_and_encodes_the_audit_id() {
    assert_eq!(
        list_path(&request(), &[], &[], None, None),
        "/workspaces/1/audits/a%2Fb/requests/7/dcf-suggestions"
    );
    let path = list_path(
        &request(),
        &[DcfSuggestionStatus::Pending, DcfSuggestionStatus::Accepted],
        &[DcfConfidenceLevel::High],
        Some(&DcfSuggestionSort::CreatedAt),
        Some(&DcfSortDirection::Desc),
    );
    assert_eq!(
        path,
        "/workspaces/1/audits/a%2Fb/requests/7/dcf-suggestions?status%5B%5D=PENDING&status%5B%5D=ACCEPTED&confidenceLevel%5B%5D=HIGH&sort=createdAt&sortDir=DESC"
    );
}

#[test]
fn generate_body_carries_exactly_the_request_ids() {
    assert_eq!(build_generate_body(&[1, 2]).unwrap(), json!({"requestIds": [1, 2]}));
}

#[test]
fn generate_body_rejects_empty_duplicate_and_oversized_lists() {
    assert!(build_generate_body(&[]).is_err());
    let dup = build_generate_body(&[1, 2, 1]).unwrap_err().to_string();
    assert!(dup.contains("request id 1 is given twice"), "{dup}");
    let many: Vec<u64> = (1..=101).collect();
    assert!(build_generate_body(&many).is_err());
    let max: Vec<u64> = (1..=100).collect();
    assert!(build_generate_body(&max).is_ok());
}

#[test]
fn accept_body_sends_request_ids_or_all_requests_never_both() {
    assert_eq!(
        build_accept_body(&[1, 2], false, &[]).unwrap(),
        json!({"requestIds": [1, 2]})
    );
    assert_eq!(build_accept_body(&[], true, &[]).unwrap(), json!({"allRequests": true}));
    assert!(build_accept_body(&[1], true, &[]).is_err());
    assert!(build_accept_body(&[], false, &[]).is_err());
}

#[test]
fn accept_body_sends_confidence_levels_only_when_given_and_deduplicated() {
    let body = build_accept_body(
        &[1],
        false,
        &[
            DcfConfidenceLevel::High,
            DcfConfidenceLevel::High,
            DcfConfidenceLevel::Medium,
        ],
    )
    .unwrap();
    assert_eq!(body, json!({"requestIds": [1], "confidenceLevels": ["HIGH", "MEDIUM"]}));
    assert!(
        build_accept_body(&[1], false, &[])
            .unwrap()
            .get("confidenceLevels")
            .is_none()
    );
}

#[test]
fn accept_body_caps_request_ids_at_500() {
    let many: Vec<u64> = (1..=501).collect();
    assert!(build_accept_body(&many, false, &[]).is_err());
}

#[test]
fn action_body_is_the_decision() {
    assert_eq!(
        build_action_body(&DcfSuggestionDecision::Reject),
        json!({"action": "REJECT"})
    );
}

#[test]
fn accept_prompt_warns_only_for_all_requests() {
    let path = "/p";
    assert_eq!(accept_prompt(path, &json!({"requestIds": [1]})), "/p");
    assert!(accept_prompt(path, &json!({"allRequests": true})).contains("EVERY matching request"));
}

#[test]
fn run_action_example_ids_are_required_without_example() {
    let target = ExampleDcfSuggestionRef {
        workspace_id: Some("1".to_string()),
        audit_id: None,
        request_id: Some(1),
        suggestion_id: Some(1),
    };
    assert!(suggestion_ref(&target).unwrap_err().to_string().contains("<audit_id>"));
}
