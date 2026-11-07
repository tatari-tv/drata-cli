#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{RiskAction, RiskTreatmentPlan};

#[test]
fn example_only_for_create_with_flag() {
    let create_example = RiskAction::Create {
        register_id: Some("1".to_string()),
        title: None,
        description: None,
        treatment_plan: None,
        impact: None,
        likelihood: None,
        status: None,
        vendor_id: None,
        example: true,
    };
    assert!(example_if_requested(&create_example).is_some());

    let create_no_example = RiskAction::Create {
        register_id: Some("1".to_string()),
        title: Some("t".to_string()),
        description: None,
        treatment_plan: None,
        impact: None,
        likelihood: None,
        status: None,
        vendor_id: None,
        example: false,
    };
    assert!(example_if_requested(&create_no_example).is_none());

    let list = RiskAction::List {
        register_id: "1".to_string(),
        all: false,
        expand: vec![],
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

fn search_args() -> RiskSearchArgs {
    RiskSearchArgs {
        register_id: None,
        risk_register_ids: vec![],
        q: None,
        status: vec![],
        treatment_plan: vec![],
        kinds: vec![],
        inherent_score_gte: None,
        inherent_score_lte: None,
        residual_score_gte: None,
        residual_score_lte: None,
        categories: vec![],
        controls: vec![],
        owner_emails: vec![],
        reviewer_emails: vec![],
        vendor_names: vec![],
        expand: vec![],
        facet: vec![],
    }
}

#[test]
fn search_path_spans_registers_unless_one_is_named() {
    let (q, facets) = build_search(&search_args()).unwrap();
    assert_eq!(q.finish(), "/risks-search");
    assert!(facets.is_empty());
    let mut a = search_args();
    a.register_id = Some("7".into());
    assert_eq!(build_search(&a).unwrap().0.finish(), "/risk-registers/7/risks-search");
}

#[test]
fn search_path_encodes_arrays_scores_and_expand() {
    let mut a = search_args();
    a.q = Some("a b".into());
    a.status = vec![RiskStatus::Active, RiskStatus::Closed];
    a.residual_score_gte = Some(5.0);
    a.expand = vec![crate::cli::RiskSearchExpand::CustomFields];
    a.vendor_names = vec!["Okta".into()];
    let path = build_search(&a).unwrap().0.finish();
    assert_eq!(
        path,
        "/risks-search?q=a%20b&status%5B%5D=ACTIVE&status%5B%5D=CLOSED&residualScoreGte=5&vendorNames%5B%5D=Okta&expand%5B%5D=customFields"
    );
}

#[test]
fn search_across_chosen_registers_sends_each_register_id() {
    let mut a = search_args();
    a.risk_register_ids = vec![3, 9];
    assert_eq!(
        build_search(&a).unwrap().0.finish(),
        "/risks-search?riskRegisterId%5B%5D=3&riskRegisterId%5B%5D=9"
    );
}

#[test]
fn search_facets_use_camel_case_wire_names() {
    let mut a = search_args();
    a.facet = vec![RiskSearchFacet::TreatmentPlan, RiskSearchFacet::RiskRegisterId];
    assert_eq!(build_search(&a).unwrap().1, vec!["treatmentPlan", "riskRegisterId"]);
}

#[test]
fn register_facet_is_rejected_inside_one_register() {
    let mut a = search_args();
    a.register_id = Some("1".into());
    a.facet = vec![RiskSearchFacet::RiskRegisterId];
    assert!(build_search(&a).is_err());
}

#[test]
fn ticket_create_example_resolves_without_ids() {
    use crate::cli::{RiskTicketAction, TicketCreateArgs};
    let action = RiskAction::Ticket {
        action: RiskTicketAction::Create {
            register_id: None,
            risk_id: None,
            args: TicketCreateArgs {
                connection_id: None,
                project_id: None,
                issue_type_id: None,
                list_id: None,
                team_id: None,
                fields: vec![],
                use_rich_content: false,
                data: None,
                example: true,
            },
        },
    };
    assert!(example_if_requested(&action).is_some());
}
