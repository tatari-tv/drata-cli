#[cfg(test)]
use super::*;

#[test]
fn event_actions_are_constructible() {
    let list = EventAction::List { all: false };
    assert!(matches!(list, EventAction::List { .. }));

    let get = EventAction::Get {
        event_id: "e1".to_string(),
    };
    assert!(matches!(get, EventAction::Get { .. }));
}
