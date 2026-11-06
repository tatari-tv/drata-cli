//! Every Phase 4 enum flag accepts exactly the spec's values for its op's
//! param: no value the spec omits (which the server would answer with a 400),
//! none the spec lists left out.

use clap::{Parser, ValueEnum};
use drata_cli::cli::*;
use drata_cli::query::wire_name;
use serde_json::Value;
use std::collections::BTreeSet;

fn spec() -> Value {
    serde_json::from_str(include_str!("../spec/drata-openapi-v2.json")).expect("spec parses")
}

fn resolve<'a>(spec: &'a Value, mut node: &'a Value) -> &'a Value {
    while let Some(r) = node.get("$ref").and_then(Value::as_str) {
        node = &spec["components"]["schemas"][r.rsplit('/').next().expect("ref has a name")];
    }
    node
}

/// The spec's enum values for query param `name` of `GET path`.
fn spec_values(path: &str, name: &str) -> BTreeSet<String> {
    let spec = spec();
    let params = spec["paths"][path]["get"]["parameters"]
        .as_array()
        .unwrap_or_else(|| panic!("no GET {path}"));
    let param = params
        .iter()
        .map(|p| resolve(&spec, p))
        .find(|p| p["name"] == name)
        .unwrap_or_else(|| panic!("GET {path} has no param {name}"));
    let mut schema = resolve(&spec, &param["schema"]);
    if let Some(items) = schema.get("items") {
        schema = resolve(&spec, items);
    }
    schema["enum"]
        .as_array()
        .unwrap_or_else(|| panic!("GET {path} {name} has no enum"))
        .iter()
        .map(|v| v.as_str().expect("string enum").to_string())
        .collect()
}

fn flag_values<E: ValueEnum>() -> BTreeSet<String> {
    E::value_variants().iter().map(wire_name).collect()
}

fn same<E: ValueEnum>(path: &str, name: &str) {
    assert_eq!(flag_values::<E>(), spec_values(path, name), "GET {path} {name}");
}

const REVIEW: &str = "/vendors/{vendorId}/security-reviews/{securityReviewId}";

#[test]
fn vendor_search_enums_match_the_spec() {
    let p = "/vendors-search";
    same::<VendorSearchStatus>(p, "status[]");
    same::<VendorCategory>(p, "category[]");
    same::<VendorRisk>(p, "risk");
    same::<ImpactLevel>(p, "impactLevel");
    same::<ReviewDeadlineStatus>(p, "nextReviewDeadlineStatus[]");
    same::<SecurityReviewStatus>(p, "securityReviewStatus[]");
    same::<SecurityReviewDecision>(p, "securityReviewDecision[]");
    same::<VendorSearchExpand>(p, "expand[]");
    same::<VendorSearchFacet>(p, "facets[]");
}

#[test]
fn criteria_creation_template_and_review_enums_match_the_spec() {
    let p = "/vendor-criteria";
    same::<ImpactLevel>(p, "impactLevels[]");
    same::<CriterionType>(p, "criterionTypes[]");
    same::<Severity>(p, "severities[]");
    same::<CriteriaExpand>(p, "expand[]");
    same::<CriteriaCreationStatus>("/workspaces/{workspaceId}/vendor-criteria-creations", "statuses[]");
    let p = "/vendor-questionnaire-templates";
    same::<TemplateStatus>(p, "status");
    same::<VendorRisk>(p, "riskLevels[]");
    same::<TemplateCategory>(p, "categories[]");
    let p = "/vendor-security-reviews";
    same::<SecurityReviewStatus>(p, "status[]");
    same::<SecurityReviewType>(p, "type[]");
    same::<SecurityReviewDecision>(p, "decision[]");
    same::<ReviewListExpand>(p, "expand[]");
}

#[test]
fn finding_and_assessment_enums_match_the_spec() {
    same::<FindingExpand>(&format!("{REVIEW}/notes"), "expand[]");
    same::<FindingExpand>(&format!("{REVIEW}/notes/{{securityReviewNoteId}}"), "expand[]");
    same::<FindingExpand>(&format!("{REVIEW}/observations"), "expand[]");
    same::<FindingExpand>(&format!("{REVIEW}/observations/{{observationId}}"), "expand[]");
    same::<AssessmentStatus>(&format!("{REVIEW}/assessments"), "statuses[]");
    same::<Severity>(&format!("{REVIEW}/assessments"), "severities[]");
}

fn parses(argv: &[&str]) -> bool {
    Cli::try_parse_from(std::iter::once("drata").chain(argv.iter().copied())).is_ok()
}

#[test]
fn vendor_search_status_takes_spec_values_and_rejects_current() {
    assert!(parses(&["vendor", "search", "--status", "on_hold"]));
    assert!(!parses(&["vendor", "search", "--status", "CURRENT"]));
    // `vendor list` still takes it.
    assert!(parses(&["vendor", "list", "--status", "CURRENT"]));
}

#[test]
fn template_category_takes_spec_values_and_rejects_none() {
    assert!(parses(&[
        "vendor-questionnaire-template",
        "list",
        "--category",
        "security"
    ]));
    assert!(!parses(&[
        "vendor-questionnaire-template",
        "list",
        "--category",
        "NONE"
    ]));
    // `vendor search` still takes it.
    assert!(parses(&["vendor", "search", "--category", "NONE"]));
}

#[test]
fn rejection_is_a_clap_usage_error() {
    let err = Cli::try_parse_from(["drata", "vendor", "search", "--status", "CURRENT"]).expect_err("rejected");
    assert_eq!(err.exit_code(), 2);
}
