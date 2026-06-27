#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{EvidenceAction, RenewalScheduleType};

#[test]
fn example_only_for_create_with_flag() {
    let create_example = EvidenceAction::Create {
        workspace_id: "w1".to_string(),
        name: None,
        description: None,
        renewal_schedule_type: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = EvidenceAction::Create {
        workspace_id: "w1".to_string(),
        name: Some("e".to_string()),
        description: None,
        renewal_schedule_type: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = EvidenceAction::List {
        workspace_id: "w1".to_string(),
    };
    assert!(example_if_requested(&list).is_none());
}

#[test]
fn renewal_schedule_type_str_roundtrips() {
    assert_eq!(renewal_schedule_type_str(&RenewalScheduleType::OneMonth), "ONE_MONTH");
    assert_eq!(renewal_schedule_type_str(&RenewalScheduleType::OneYear), "ONE_YEAR");
    assert_eq!(renewal_schedule_type_str(&RenewalScheduleType::None), "NONE");
    assert_eq!(renewal_schedule_type_str(&RenewalScheduleType::Custom), "CUSTOM");
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({});
    set_opt(&mut body, "name", Some("Ev"));
    set_opt(&mut body, "description", None);
    assert_eq!(body["name"], json!("Ev"));
    assert!(body.get("description").is_none());
}
