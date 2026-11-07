//! Every enum flag accepts exactly the spec's values for its op's
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

/// The spec's enum values for body field `field` of `POST path`, read from the
/// `content_type` request schema.
fn body_values(path: &str, content_type: &str, field: &str) -> BTreeSet<String> {
    let spec = spec();
    let schema = resolve(
        &spec,
        &spec["paths"][path]["post"]["requestBody"]["content"][content_type]["schema"],
    );
    let mut prop = resolve(&spec, &schema["properties"][field]);
    if let Some(inner) = prop.get("allOf").and_then(|a| a.get(0)) {
        prop = resolve(&spec, inner);
    }
    prop["enum"]
        .as_array()
        .unwrap_or_else(|| panic!("POST {path} {field} has no enum"))
        .iter()
        .map(|v| v.as_str().expect("string enum").to_string())
        .collect()
}

#[test]
fn agent_body_enums_match_the_spec() {
    assert_eq!(
        flag_values::<SecurityReviewDecision>(),
        body_values(&format!("{REVIEW}/finalizations"), "application/json", "decision"),
        "finalize --decision"
    );
    assert_eq!(
        flag_values::<SecurityReviewDocumentType>(),
        body_values(&format!("{REVIEW}/documents"), "multipart/form-data", "type"),
        "add-documents --type"
    );
}

#[test]
fn finalize_requires_a_spec_decision() {
    assert!(parses(&[
        "vendor",
        "security-review",
        "finalize",
        "1",
        "2",
        "--decision",
        "pending"
    ]));
    assert!(!parses(&[
        "vendor",
        "security-review",
        "finalize",
        "1",
        "2",
        "--decision",
        "MAYBE"
    ]));
}

#[test]
fn monitor_enums_match_the_spec() {
    same::<CodebaseExclusionExpand>(
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions",
        "expand[]",
    );
}

#[test]
fn codebase_exclusion_expand_takes_spec_values_only() {
    let list = ["monitor", "codebase-exclusion", "list", "1", "2", "--expand"];
    assert!(parses(&[&list[..], &["exclusionDesignator"]].concat()));
    assert!(parses(&[&list[..], &["EXCLUSIONDESIGNATOR"]].concat()));
    assert!(!parses(&[&list[..], &["connection"]].concat()));
}

#[test]
fn personnel_search_enums_match_the_spec() {
    for p in ["/personnel-search", "/workspaces/{workspaceId}/personnel-search"] {
        same::<EmploymentStatus>(p, "employmentStatus[]");
        same::<PersonnelSyncStatus>(p, "syncStatus");
        same::<PersonnelSearchExpand>(p, "expand[]");
        same::<PersonnelSearchFacet>(p, "facets[]");
        for name in [
            "complianceStatus[]",
            "acceptedPoliciesStatus[]",
            "bgCheckStatus[]",
            "identityMfaStatus[]",
            "securityTrainingStatus[]",
            "hipaaTrainingStatus[]",
            "aiAwarenessTrainingStatus[]",
            "offboardingStatus[]",
            "agentInstalledStatus[]",
            "passwordManagerStatus[]",
            "diskEncryptionStatus[]",
            "antivirusStatus[]",
            "autoUpdatesStatus[]",
            "lockScreenStatus[]",
        ] {
            same::<ComplianceCheckStatus>(p, name);
        }
    }
}

#[test]
fn compliance_check_and_custom_field_enums_match_the_spec() {
    let p = "/workspaces/{workspaceId}/compliance-check-exclusions";
    same::<ComplianceCheckType>(p, "complianceCheck");
    same::<ComplianceCheckExclusionExpand>(p, "expand[]");
    same::<CustomFieldEntityType>("/custom-field-definitions", "entityType");
    assert_eq!(
        flag_values::<ComplianceCheckType>(),
        body_array_values(p, "complianceCheckTypes"),
        "create --compliance-check"
    );
    assert_eq!(
        flag_values::<ComplianceCheckType>(),
        body_array_values(&format!("{p}/action-delete"), "complianceCheckTypes"),
        "remove --compliance-check"
    );
    assert_eq!(
        flag_values::<CustomFieldType>(),
        body_values("/custom-field-definitions", "application/json", "fieldType"),
        "custom-field create --field-type"
    );
}

/// The spec's enum values for the array body field `field` of `POST path`.
fn body_array_values(path: &str, field: &str) -> BTreeSet<String> {
    let spec = spec();
    let schema = resolve(
        &spec,
        &spec["paths"][path]["post"]["requestBody"]["content"]["application/json"]["schema"],
    );
    let items = resolve(&spec, &schema["properties"][field]["items"]);
    items["enum"]
        .as_array()
        .unwrap_or_else(|| panic!("POST {path} {field} items have no enum"))
        .iter()
        .map(|v| v.as_str().expect("string enum").to_string())
        .collect()
}

#[test]
fn personnel_and_exclusion_enum_flags_take_spec_values_only() {
    assert!(parses(&["personnel", "search", "--bg-check-status", "not_assessed"]));
    assert!(!parses(&["personnel", "search", "--bg-check-status", "UNKNOWN"]));
    assert!(parses(&[
        "personnel",
        "search",
        "--employment-status",
        "service_account"
    ]));
    assert!(!parses(&["personnel", "search", "--sync-status", "PARTIAL"]));
    let create = [
        "compliance-check-exclusion",
        "remove",
        "1",
        "--target-id",
        "1",
        "--compliance-check",
    ];
    assert!(parses(&[&create[..], &["hdd_encryption"]].concat()));
    assert!(!parses(&[&create[..], &["DISK_ENCRYPTION"]].concat()));
    assert!(parses(&[
        "custom-field",
        "create",
        "--name",
        "n",
        "--field-type",
        "options_numeric"
    ]));
    assert!(!parses(&[
        "custom-field",
        "create",
        "--name",
        "n",
        "--field-type",
        "FORMULA"
    ]));
}

#[test]
fn identity_enums_match_the_spec_for_both_kinds() {
    for kind in ["infrastructure", "version-control"] {
        let list = format!("/workspaces/{{workspaceId}}/{kind}-user-identities");
        same::<IdentityExpand>(&list, "expand[]");
        same::<IdentityExpand>(&format!("{list}/{{identityId}}"), "expand[]");
    }
    let schemas = &spec()["components"]["schemas"];
    for (schema, values) in [
        (
            "InfrastructureUserIdentityActionEnum",
            flag_values::<InfrastructureIdentityActionKind>(),
        ),
        (
            "VersionControlUserIdentityActionEnum",
            flag_values::<VersionControlIdentityActionKind>(),
        ),
    ] {
        let want: BTreeSet<String> = schemas[schema]["enum"]
            .as_array()
            .unwrap_or_else(|| panic!("{schema} has no enum"))
            .iter()
            .map(|v| v.as_str().expect("string enum").to_string())
            .collect();
        assert_eq!(values, want, "{schema}");
    }
}

#[test]
fn identity_action_is_per_kind_and_client_type_is_the_spec_enum() {
    let infra = ["infrastructure-identity", "run-action", "1", "2", "--action"];
    let vc = ["version-control-identity", "run-action", "1", "2", "--action"];
    assert!(parses(&[&infra[..], &["unmarkServiceAccount"]].concat()));
    assert!(parses(&[&infra[..], &["LINKUSER", "--user-id", "1"]].concat()));
    assert!(!parses(&[&infra[..], &["retractWriteAccess"]].concat()));
    assert!(parses(&[&vc[..], &["retractWriteAccess"]].concat()));
    let spec_clients = drata_cli::spec::enum_values("ClientTypeEnum").expect("spec has ClientTypeEnum");
    for client in spec_clients {
        assert!(
            parses(&[
                "infrastructure-identity",
                "list",
                "1",
                "--client-type",
                &client.to_lowercase()
            ]),
            "{client}"
        );
    }
    assert!(!parses(&[
        "infrastructure-identity",
        "list",
        "1",
        "--client-type",
        "nope"
    ]));
}

#[test]
fn policy_language_enums_match_the_spec() {
    let languages = "/policies/{policyId}/policy-versions/{policyVersionId}/languages";
    same::<PolicyVersionStatus>(languages, "statuses[]");
    same::<PolicyVersionStatus>("/policy-language-versions", "statuses[]");
    same::<PolicyLanguageVersionListExpand>("/policy-language-versions", "expand[]");
    same::<PolicyLanguageVersionExpand>("/policy-language-versions/{policyLanguageVersionId}", "expand[]");
    let sources = &spec()["components"]["schemas"]["CreatePolicySourceTypeEnum"]["enum"];
    let want: BTreeSet<String> = sources
        .as_array()
        .expect("CreatePolicySourceTypeEnum has values")
        .iter()
        .map(|v| v.as_str().expect("string enum").to_string())
        .collect();
    assert_eq!(flag_values::<PolicySourceType>(), want);
}

#[test]
fn policy_version_language_is_the_spec_enum_and_the_version_command_keeps_its_get_form() {
    let spec_languages = drata_cli::spec::enum_values("PolicyLanguageEnum").expect("spec has PolicyLanguageEnum");
    for language in spec_languages {
        assert!(
            parses(&[
                "policy",
                "version",
                "replace-file",
                "1",
                "2",
                "--file",
                "f.pdf",
                "--language",
                &language.to_uppercase()
            ]),
            "{language}"
        );
    }
    assert!(!parses(&[
        "policy",
        "version",
        "replace-file",
        "1",
        "2",
        "--file",
        "f.pdf",
        "--language",
        "klingon"
    ]));
    assert!(parses(&["policy", "version", "1", "2"]), "the get form is kept");
    assert!(!parses(&["policy", "version", "1"]), "get needs both ids");
    assert!(parses(&[
        "policy-language",
        "list",
        "1",
        "2",
        "--status",
        "draft",
        "published"
    ]));
    assert!(!parses(&["policy-language", "list", "1", "2", "--status", "bogus"]));
}

#[test]
fn evidence_item_enums_match_the_spec() {
    let items = "/workspaces/{workspaceId}/evidence";
    let item = "/workspaces/{workspaceId}/evidence/{evidenceId}";
    same::<EvidenceItemExpand>(items, "expand[]");
    same::<EvidenceItemExpand>(item, "expand[]");
    same::<ComputedEvidenceStatus>(items, "evidenceStatuses[]");
    same::<ArtifactType>(items, "artifactTypes[]");
    same::<ArtifactType>(&format!("{item}/artifacts"), "artifactTypes[]");
    assert_eq!(
        flag_values::<ArtifactActionKind>(),
        body_values(
            &format!("{item}/artifacts/{{artifactId}}/actions"),
            "application/json",
            "action"
        ),
        "artifact run-action --action"
    );
    assert_eq!(
        flag_values::<RenewalScheduleType>(),
        body_values(items, "application/json", "renewalScheduleType"),
        "evidence-item create --renewal-schedule-type"
    );
}

#[test]
fn evidence_item_enum_flags_take_spec_values_case_insensitively() {
    assert!(parses(&[
        "evidence-item",
        "list",
        "1",
        "--status",
        "needs_artifact",
        "--artifact-type",
        "s3_file",
        "--expand",
        "OWNERS"
    ]));
    assert!(!parses(&["evidence-item", "list", "1", "--status", "ERROR"]));
    assert!(parses(&[
        "evidence-item",
        "artifact",
        "run-action",
        "1",
        "2",
        "3",
        "--action",
        "RESTORE"
    ]));
    assert!(!parses(&[
        "evidence-item",
        "artifact",
        "run-action",
        "1",
        "2",
        "3",
        "--action",
        "delete"
    ]));
}

#[test]
fn evidence_item_artifact_flags_need_filed_at_and_ids_are_numeric() {
    assert!(!parses(&[
        "evidence-item",
        "create",
        "1",
        "--name",
        "x",
        "--url",
        "https://example.com"
    ]));
    assert!(!parses(&["evidence-item", "get", "1", "abc"]));
    assert!(!parses(&[
        "evidence-item",
        "create",
        "1",
        "--name",
        "x",
        "--data",
        "{}"
    ]));
}

#[test]
fn access_review_enums_match_the_spec() {
    same::<AccessReviewPeriodStatus>("/access-review-periods", "status");
    for path in [
        "/access-review-periods/{periodId}/actions",
        "/access-review-periods/{periodId}/applications/{reviewAppId}/actions",
    ] {
        assert_eq!(
            flag_values::<AccessReviewActionKind>(),
            body_values(path, "application/json", "action"),
            "POST {path} --action"
        );
    }
}

#[test]
fn access_review_flags_take_spec_values_case_insensitively_and_numeric_ids() {
    assert!(parses(&["access-review", "period", "list", "--status", "completed"]));
    assert!(!parses(&["access-review", "period", "list", "--status", "OPEN"]));
    assert!(parses(&[
        "access-review",
        "period",
        "run-action",
        "1",
        "--action",
        "Complete"
    ]));
    assert!(!parses(&[
        "access-review",
        "period",
        "run-action",
        "1",
        "--action",
        "reopen"
    ]));
    assert!(!parses(&["access-review", "app", "get", "abc"]));
    assert!(!parses(&[
        "access-review",
        "app",
        "users",
        "set",
        "1",
        "--user",
        "only-an-id"
    ]));
    assert!(!parses(&["access-review", "app", "users", "set", "1"]));
    assert!(!parses(&[
        "access-review",
        "app",
        "users",
        "set",
        "1",
        "--role",
        "a:Admin"
    ]));
    assert!(!parses(&[
        "access-review",
        "app",
        "users",
        "set",
        "1",
        "--user",
        "a:b:c@d.e:Admin"
    ]));
    assert!(!parses(&[
        "access-review",
        "period",
        "create",
        "--starting-date",
        "a",
        "--ending-date",
        "b",
        "--application",
        "1"
    ]));
    assert!(parses(&["access-review", "period", "create", "--example"]));
}

#[test]
fn connection_expand_matches_the_spec_for_every_provider() {
    for provider in ["aws", "azure", "gcp", "gitlab"] {
        let list = format!("/{provider}-connections");
        same::<ConnectionExpand>(&list, "expand[]");
        same::<ConnectionExpand>(&format!("{list}/{{connectionId}}"), "expand[]");
    }
}

#[test]
fn dcf_suggestion_enums_match_the_spec() {
    let list = "/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}/dcf-suggestions";
    same::<DcfSuggestionStatus>(list, "status[]");
    same::<DcfConfidenceLevel>(list, "confidenceLevel[]");
    same::<DcfSuggestionSort>(list, "sort");
    same::<DcfSortDirection>(list, "sortDir");
    assert_eq!(
        flag_values::<DcfSuggestionDecision>(),
        body_values(
            "/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}/dcf-suggestions/{suggestionId}/actions",
            "application/json",
            "action"
        ),
        "decide --action"
    );
    // `accept --confidence-level` is the body's `confidenceLevels` array of the same enum.
    let spec = spec();
    let accept = "/workspaces/{workspaceId}/audits/{auditId}/requests/action-accept-dcf-suggestions";
    let schema = resolve(
        &spec,
        &spec["paths"][accept]["post"]["requestBody"]["content"]["application/json"]["schema"],
    );
    let items = resolve(&spec, &schema["properties"]["confidenceLevels"]["items"]);
    let want: BTreeSet<String> = items["enum"]
        .as_array()
        .expect("enum")
        .iter()
        .map(|v| v.as_str().expect("string").to_string())
        .collect();
    assert_eq!(flag_values::<DcfConfidenceLevel>(), want, "accept --confidence-level");
}

#[test]
fn dcf_suggestion_flags_take_spec_values_case_insensitively_and_never_split_commas() {
    let base = ["audit", "request", "dcf-suggestion"];
    let with = |rest: &[&str]| -> Vec<String> { base.iter().chain(rest.iter()).map(|s| s.to_string()).collect() };
    let ok = |rest: &[&str]| {
        let argv = with(rest);
        parses(&argv.iter().map(String::as_str).collect::<Vec<_>>())
    };
    assert!(ok(&["list", "1", "2", "3", "--status", "pending", "Accepted"]));
    assert!(!ok(&["list", "1", "2", "3", "--status", "rejected"]));
    assert!(!ok(&["list", "1", "2", "3", "--status", "pending,accepted"]));
    assert!(ok(&[
        "accept",
        "1",
        "2",
        "--all-requests",
        "--confidence-level",
        "high",
        "low"
    ]));
    assert!(!ok(&[
        "accept",
        "1",
        "2",
        "--all-requests",
        "--confidence-level",
        "certain"
    ]));
    assert!(!ok(&["accept", "1", "2", "--request-id", "1", "--all-requests"]));
    assert!(!ok(&["generate", "1", "2", "--request-id", "1,2"]));
    assert!(!ok(&["run-action", "1", "2", "3", "4", "--action", "defer"]));
    assert!(ok(&["run-action", "1", "2", "3", "4", "--action", "Reject"]));
}

#[test]
fn vendor_list_and_control_list_enums_match_the_spec() {
    same::<VendorStatus>("/vendors", "statuses[]");
    same::<VendorType>("/vendors", "types[]");
    same::<RequiredApprovals>("/workspaces/{workspaceId}/controls", "requiredApprovals");
}

#[test]
fn risk_search_enums_match_the_spec() {
    let all = "/risks-search";
    let scoped = "/risk-registers/{riskRegisterId}/risks-search";
    same::<RiskSearchExpand>(all, "expand[]");
    same::<RiskSearchExpand>(scoped, "expand[]");
    same::<RiskSearchFacet>(all, "facets[]");
    // The one-register search takes every facet but `riskRegisterId`, which
    // `risk search --register-id` refuses before any request.
    let mut scoped_facets = flag_values::<RiskSearchFacet>();
    assert!(scoped_facets.remove(&wire_name(&RiskSearchFacet::RiskRegisterId)));
    assert_eq!(scoped_facets, spec_values(scoped, "facets[]"), "GET {scoped} facets[]");
}

#[test]
fn upload_purpose_matches_the_spec() {
    assert_eq!(
        flag_values::<UploadPurpose>(),
        body_values("/upload-urls", "application/json", "purpose"),
        "upload-url create --purpose"
    );
}

/// The spec's enum values for request-body field `field` of `method path`,
/// read from the `content_type` schema: through an `allOf` wrapper and, for an
/// array field, its `items`.
fn request_values(method: &str, path: &str, content_type: &str, field: &str) -> BTreeSet<String> {
    let spec = spec();
    let schema = resolve(
        &spec,
        &spec["paths"][path][method]["requestBody"]["content"][content_type]["schema"],
    );
    let mut prop = resolve(&spec, &schema["properties"][field]);
    if let Some(inner) = prop.get("allOf").and_then(|a| a.get(0)) {
        prop = resolve(&spec, inner);
    }
    if let Some(items) = prop.get("items") {
        prop = resolve(&spec, items);
    }
    prop["enum"]
        .as_array()
        .unwrap_or_else(|| panic!("{method} {path} {field} has no enum"))
        .iter()
        .map(|v| v.as_str().expect("string enum").to_string())
        .collect()
}

#[test]
fn risk_search_filter_enums_match_the_spec_on_both_ops() {
    for path in ["/risks-search", "/risk-registers/{riskRegisterId}/risks-search"] {
        same::<RiskStatus>(path, "status[]");
        same::<RiskTreatmentPlan>(path, "treatmentPlan[]");
        same::<RiskKind>(path, "type[]");
    }
}

#[test]
fn ticket_field_types_match_the_spec_on_every_create_op() {
    let accepted: BTreeSet<String> = drata_cli::resources::ticket::FIELD_TYPES
        .iter()
        .map(|t| t.to_string())
        .collect();
    assert_eq!(
        accepted.len(),
        drata_cli::resources::ticket::FIELD_TYPES.len(),
        "duplicate type"
    );
    let spec = spec();
    for path in [
        "/risk-registers/{riskRegisterId}/risks/{riskId}/tickets/action-create",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets/action-create",
    ] {
        let body = resolve(
            &spec,
            &spec["paths"][path]["post"]["requestBody"]["content"]["application/json"]["schema"],
        );
        let field = resolve(&spec, &body["properties"]["fields"]["items"]);
        let kind = resolve(&spec, &field["properties"]["type"]["allOf"][0]);
        let want: BTreeSet<String> = kind["enum"]
            .as_array()
            .unwrap_or_else(|| panic!("POST {path} field type has no enum"))
            .iter()
            .map(|v| v.as_str().expect("string enum").to_string())
            .collect();
        assert_eq!(accepted, want, "POST {path} fields[].type");
    }
}

/// The enum flags v0.1.5 shipped without a spec-parity check. All match the
/// spec today; a divergence found here is reported, not fixed by changing the
/// accepted values, which would change behavior.
#[test]
fn pre_existing_enum_flags_match_the_spec() {
    let review_actions = "/vendors/{vendorId}/security-reviews/{securityReviewId}/actions";
    assert_eq!(
        flag_values::<SecurityReviewAction>(),
        request_values("post", review_actions, "application/json", "action"),
        "SecurityReviewAction: POST security-reviews/{{id}}/actions action"
    );
    for content_type in ["multipart/form-data", "application/json"] {
        assert_eq!(
            flag_values::<DeviceDocumentType>(),
            request_values("post", "/devices/{deviceId}/documents", content_type, "type"),
            "DeviceDocumentType: POST /devices/{{id}}/documents type"
        );
    }
    for (method, path) in [("post", "/assets"), ("put", "/assets/{assetId}")] {
        assert_eq!(
            flag_values::<AssetType>(),
            request_values(method, path, "application/json", "assetType"),
            "AssetType: assets assetType"
        );
        assert_eq!(
            flag_values::<AssetClassType>(),
            request_values(method, path, "application/json", "assetClassTypes"),
            "AssetClassType: assets assetClassTypes[]"
        );
    }
}
