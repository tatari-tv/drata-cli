#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::ControlAction;

#[test]
fn example_only_for_create_with_flag() {
    let create_example = ControlAction::Create {
        workspace_id: Some("w1".to_string()),
        name: None,
        description: None,
        code: None,
        question: None,
        activity: None,
        external_evidence_metadata: "[]".to_string(),
        base64_files: "[]".to_string(),
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = ControlAction::Create {
        workspace_id: Some("w1".to_string()),
        name: Some("ctrl".to_string()),
        description: None,
        code: None,
        question: None,
        activity: None,
        external_evidence_metadata: "[]".to_string(),
        base64_files: "[]".to_string(),
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = ControlAction::List {
        workspace_id: "w1".to_string(),
        all: false,
        expand: vec![],
        required_approvals: None,
    };
    assert!(example_if_requested(&list).is_none());
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({});
    set_opt(&mut body, "name", Some("My Control"));
    set_opt(&mut body, "description", None);
    assert_eq!(body["name"], json!("My Control"));
    assert!(body.get("description").is_none());
}

#[test]
fn control_create_body_has_evidence_fields() {
    let body = build_create_body("n", "d", "C-1", None, Some("act"), "[]", "[]").unwrap();
    // Both are spec-required string fields whose content is a JSON array.
    assert_eq!(body["externalEvidenceMetadata"], json!("[]"));
    assert_eq!(body["base64Files"], json!("[]"));
    assert_eq!(body["name"], json!("n"));
    assert_eq!(body["code"], json!("C-1"));
    assert_eq!(body["activity"], json!("act"));
    assert!(body.get("question").is_none());
}

#[test]
fn control_create_body_sends_evidence_arrays_verbatim_as_strings() {
    let meta = r#"[{"filename":"a.pdf","name":"A"}]"#;
    let body = build_create_body("n", "d", "C-1", None, None, meta, "[]").unwrap();
    assert_eq!(body["externalEvidenceMetadata"], json!(meta));
}

#[test]
fn control_create_body_rejects_non_array_evidence() {
    let err = build_create_body("n", "d", "C-1", None, None, "{}", "[]").unwrap_err();
    assert!(err.to_string().contains("--external-evidence-metadata"), "{err}");
    let err = build_create_body("n", "d", "C-1", None, None, "[]", "not json").unwrap_err();
    assert!(err.to_string().contains("--base64-files"), "{err}");
}

#[test]
fn control_create_evidence_flags_default_to_empty_arrays() {
    use crate::cli::{Cli, Commands};
    use clap::Parser;
    let cli = Cli::try_parse_from(["drata", "control", "create", "1", "--name", "n"]).unwrap();
    let Commands::Control {
        action:
            ControlAction::Create {
                external_evidence_metadata,
                base64_files,
                ..
            },
    } = cli.command
    else {
        panic!("expected control create");
    };
    assert_eq!(external_evidence_metadata, "[]");
    assert_eq!(base64_files, "[]");
}

#[test]
fn control_list_path_encodes_required_approvals() {
    use crate::cli::RequiredApprovals;
    let path = build_list_path("1", Some(&RequiredApprovals::NeedsApproval), &["owners".to_string()]);
    assert_eq!(
        path,
        "/workspaces/1/controls?requiredApprovals=NEEDS_APPROVAL&expand%5B%5D=owners"
    );
    assert_eq!(build_list_path("1", None, &[]), "/workspaces/1/controls");
}
