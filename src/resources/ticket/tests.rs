#![allow(clippy::unwrap_used)]
use super::*;

fn args() -> TicketCreateArgs {
    TicketCreateArgs {
        connection_id: Some(3),
        project_id: None,
        issue_type_id: None,
        list_id: None,
        team_id: None,
        fields: vec!["summary".into(), "string".into(), "Fix it".into()],
        use_rich_content: false,
        data: None,
        example: false,
    }
}

#[test]
fn parent_paths_are_distinct() {
    assert_eq!(control_tickets("1", "2"), "/workspaces/1/controls/2/tickets");
    assert_eq!(risk_tickets("1", "2"), "/risk-registers/1/risks/2/tickets");
    assert_eq!(monitor_tickets("1", "2"), "/workspaces/1/monitoring-tests/2/tickets");
}

#[test]
fn list_path_adds_is_done_only_when_set() {
    assert_eq!(build_list_path("/t", None), "/t");
    assert_eq!(build_list_path("/t", Some(true)), "/t?isDone=true");
    assert_eq!(build_list_path("/t", Some(false)), "/t?isDone=false");
}

#[test]
fn link_body_is_ticket_url() {
    assert_eq!(
        build_link_body("https://x.test/T-1"),
        json!({"ticketUrl": "https://x.test/T-1"})
    );
}

#[test]
fn unlink_and_create_paths_extend_the_collection() {
    assert_eq!(build_unlink_path("/t", "9"), "/t/9");
    assert_eq!(build_create_path("/t"), "/t/action-create");
}

#[test]
fn create_body_has_required_fields_and_only_set_options() {
    let body = build_create_body(&args()).unwrap();
    assert_eq!(
        body,
        json!({"connectionId": 3, "fields": [{"field": "summary", "type": "string", "value": "Fix it"}]})
    );
}

#[test]
fn create_body_carries_every_option() {
    let mut a = args();
    a.project_id = Some("P".into());
    a.issue_type_id = Some("I".into());
    a.list_id = Some("L".into());
    a.team_id = Some("T".into());
    a.use_rich_content = true;
    a.fields.extend(["due".into(), "date".into(), "2026-12-01".into()]);
    let body = build_create_body(&a).unwrap();
    assert_eq!(body["projectId"], json!("P"));
    assert_eq!(body["issueTypeId"], json!("I"));
    assert_eq!(body["listId"], json!("L"));
    assert_eq!(body["teamId"], json!("T"));
    assert_eq!(body["useRichContent"], json!(true));
    assert_eq!(body["fields"].as_array().unwrap().len(), 2);
}

#[test]
fn create_body_requires_connection_id_and_a_field() {
    let mut a = args();
    a.connection_id = None;
    assert!(build_create_body(&a).is_err());
    let mut a = args();
    a.fields.clear();
    assert!(build_create_body(&a).is_err());
}

#[test]
fn create_body_rejects_unknown_field_type() {
    let mut a = args();
    a.fields = vec!["summary".into(), "bogus".into(), "x".into()];
    let err = build_create_body(&a).unwrap_err().to_string();
    assert!(err.contains("bogus"), "{err}");
}

#[test]
fn create_example_only_with_flag() {
    let mut a = args();
    assert!(create_example(&a, RISK_CREATE_SPEC_PATH).is_none());
    a.example = true;
    let skeleton = create_example(&a, RISK_CREATE_SPEC_PATH).unwrap().unwrap();
    assert!(skeleton.contains("connectionId"));
    let skeleton = create_example(&a, MONITOR_CREATE_SPEC_PATH).unwrap().unwrap();
    assert!(skeleton.contains("fields"));
}

#[test]
fn control_target_has_no_create() {
    let action = ControlTicketAction::Unlink {
        workspace_id: "1".into(),
        control_id: "2".into(),
        ticket_id: "3".into(),
    };
    let (path, op) = control_target(&action);
    assert_eq!(path, "/workspaces/1/controls/2/tickets");
    assert!(matches!(op, TicketOp::Unlink { ticket_id: "3" }));
}

#[test]
fn risk_create_without_ids_is_an_error() {
    let action = RiskTicketAction::Create {
        register_id: None,
        risk_id: None,
        args: args(),
    };
    assert!(risk_target(&action).is_err());
}

#[test]
fn monitor_target_resolves_create() {
    let action = MonitorTicketAction::Create {
        workspace_id: Some("1".into()),
        test_id: Some("2".into()),
        args: args(),
    };
    let (path, op) = monitor_target(&action).unwrap();
    assert_eq!(path, "/workspaces/1/monitoring-tests/2/tickets");
    assert!(matches!(op, TicketOp::Create(_)));
}
