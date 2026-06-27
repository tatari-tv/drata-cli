#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{RiskAction, RiskTreatmentPlan};

#[test]
fn example_only_for_create_with_flag() {
    let create_example = RiskAction::Create {
        register_id: "1".to_string(),
        title: None,
        description: None,
        treatment_plan: None,
        impact: None,
        likelihood: None,
        status: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = RiskAction::Create {
        register_id: "1".to_string(),
        title: Some("t".to_string()),
        description: None,
        treatment_plan: None,
        impact: None,
        likelihood: None,
        status: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = RiskAction::List {
        register_id: "1".to_string(),
    };
    assert!(example_if_requested(&list).is_none());
}

#[test]
fn set_opt_inserts_only_present_values() {
    let mut body = json!({});
    set_opt(&mut body, "title", Some("My Risk"));
    set_opt(&mut body, "description", None);
    assert_eq!(body["title"], json!("My Risk"));
    assert!(body.get("description").is_none());
}

#[test]
fn treatment_plan_str_roundtrips() {
    assert_eq!(treatment_plan_str(&RiskTreatmentPlan::Untreated), "UNTREATED");
    assert_eq!(treatment_plan_str(&RiskTreatmentPlan::Accept), "ACCEPT");
    assert_eq!(treatment_plan_str(&RiskTreatmentPlan::Mitigate), "MITIGATE");
}
