#[cfg(test)]
use super::*;

#[test]
fn audit_actions_are_constructible() {
    let list = AuditAction::List {
        workspace_id: "ws1".to_string(),
        all: false,
    };
    assert!(matches!(list, AuditAction::List { .. }));

    let get = AuditAction::Get {
        workspace_id: "ws1".to_string(),
        audit_id: "a1".to_string(),
        expand: vec![],
    };
    assert!(matches!(get, AuditAction::Get { .. }));
}

#[test]
fn dcf_example_is_requested_for_body_verbs_only() {
    use crate::cli::{AuditRequestAction, DcfSuggestionAction, ExampleAuditRef};
    let generate = |example| AuditAction::Request {
        action: AuditRequestAction::DcfSuggestion {
            action: DcfSuggestionAction::Generate {
                audit: ExampleAuditRef {
                    workspace_id: None,
                    audit_id: None,
                },
                request_ids: vec![],
                data: None,
                example,
            },
        },
    };
    let skeleton = example_if_requested(&generate(true))
        .expect("example requested")
        .expect("skeleton builds");
    assert!(skeleton.contains("requestIds"), "{skeleton}");
    assert!(example_if_requested(&generate(false)).is_none());
}
