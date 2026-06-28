#[cfg(test)]
use super::*;

#[test]
fn monitor_actions_are_constructible() {
    let list = MonitorAction::List {
        workspace_id: "ws1".to_string(),
        all: false,
    };
    assert!(matches!(list, MonitorAction::List { .. }));

    let get = MonitorAction::Get {
        workspace_id: "ws1".to_string(),
        test_id: "t1".to_string(),
    };
    assert!(matches!(get, MonitorAction::Get { .. }));

    let update = MonitorAction::Update {
        workspace_id: "ws1".to_string(),
        test_id: "t1".to_string(),
        name: None,
        enabled: Some(true),
        description: None,
    };
    assert!(matches!(update, MonitorAction::Update { .. }));

    let excl = MonitorAction::Exclusions {
        workspace_id: "ws1".to_string(),
        test_id: "t1".to_string(),
    };
    assert!(matches!(excl, MonitorAction::Exclusions { .. }));

    let fail = MonitorAction::Failures {
        workspace_id: "ws1".to_string(),
        test_id: "t1".to_string(),
    };
    assert!(matches!(fail, MonitorAction::Failures { .. }));

    let pass = MonitorAction::Passes {
        workspace_id: "ws1".to_string(),
        test_id: "t1".to_string(),
    };
    assert!(matches!(pass, MonitorAction::Passes { .. }));
}
