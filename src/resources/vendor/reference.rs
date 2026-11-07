//! Vendor-adjacent read nouns: criteria, criteria-creation jobs, questionnaire
//! templates, risk levels, and security reviews across every vendor.
//!
//! | Command | Method | Path |
//! |---------|--------|------|
//! | `vendor-criteria list` | GET | `/vendor-criteria` |
//! | `vendor-criteria-creation list` | GET | `/workspaces/{ws}/vendor-criteria-creations` |
//! | `vendor-questionnaire-template list` | GET | `/vendor-questionnaire-templates` |
//! | `vendor-questionnaire-template questions` | GET | `/vendor-questionnaire-templates/{id}/questions` |
//! | `vendor-risk-level list` | GET | `/vendor-risk-levels` |
//! | `vendor-security-review list` | GET | `/vendor-security-reviews` |
//!
//! Every list has a `cursor` param, so each goes through `get_all`.

use crate::cli::{
    CriteriaCreationStatus, CriteriaExpand, CriterionType, ImpactLevel, ReviewListExpand, SecurityReviewDecision,
    SecurityReviewStatus, SecurityReviewType, Severity, TemplateCategory, TemplateStatus, VendorCriteriaAction,
    VendorCriteriaCreationAction, VendorQuestionnaireTemplateAction, VendorRisk, VendorRiskLevelAction,
    VendorSecurityReviewsAction,
};
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use crate::resources::vendor::security::{decision_str, status_str, type_str};
use eyre::Result;
use serde_json::json;
use tracing::{debug, instrument};

/// `GET /vendor-criteria` path. Pure.
#[allow(clippy::too_many_arguments)]
pub fn build_criteria_path(
    name: Option<&str>,
    impact_level: &[ImpactLevel],
    vendor_risk_level_id: &[u64],
    vendor_type_id: &[u64],
    draft: Option<bool>,
    security_review_id: Option<u64>,
    criterion_type: &[CriterionType],
    severity: &[Severity],
    expand: &[CriteriaExpand],
) -> String {
    let mut q = QueryPath::new("/vendor-criteria");
    q.push_opt("name", name)
        .push_each("impactLevels[]", impact_level.iter().map(wire_name))
        .push_each("vendorRiskLevelIds[]", vendor_risk_level_id)
        .push_each("vendorTypeIds[]", vendor_type_id)
        .push_opt("isDraft", draft)
        .push_opt("securityReviewId", security_review_id)
        .push_each("criterionTypes[]", criterion_type.iter().map(wire_name))
        .push_each("severities[]", severity.iter().map(wire_name))
        .push_each("expand[]", expand.iter().map(wire_name));
    q.finish()
}

/// `GET /workspaces/{workspaceId}/vendor-criteria-creations` path. Pure.
pub fn build_criteria_creation_path(workspace_id: &str, status: &[CriteriaCreationStatus]) -> String {
    let mut q = QueryPath::new(format!("/workspaces/{}/vendor-criteria-creations", workspace_id));
    q.push_each("statuses[]", status.iter().map(wire_name));
    q.finish()
}

/// `GET /vendor-questionnaire-templates` path. Pure.
pub fn build_template_path(
    title: Option<&str>,
    status: Option<&TemplateStatus>,
    risk_level: &[VendorRisk],
    category: &[TemplateCategory],
) -> String {
    let mut q = QueryPath::new("/vendor-questionnaire-templates");
    q.push_opt("title", title)
        .push_opt("status", status.map(wire_name))
        .push_each("riskLevels[]", risk_level.iter().map(wire_name))
        .push_each("categories[]", category.iter().map(wire_name));
    q.finish()
}

/// `GET /vendor-questionnaire-templates/{questionnaireId}/questions` path. Pure.
pub fn build_template_questions_path(questionnaire_id: &str) -> String {
    format!("/vendor-questionnaire-templates/{}/questions", questionnaire_id)
}

/// `GET /vendor-security-reviews` path (all vendors). Pure.
#[allow(clippy::too_many_arguments)]
pub fn build_security_reviews_path(
    status: &[SecurityReviewStatus],
    review_type: &[SecurityReviewType],
    decision: &[SecurityReviewDecision],
    created_from: Option<&str>,
    created_to: Option<&str>,
    updated_from: Option<&str>,
    updated_to: Option<&str>,
    vendor_id: &[u64],
    expand: &[ReviewListExpand],
) -> String {
    let mut q = QueryPath::new("/vendor-security-reviews");
    q.push_each("status[]", status.iter().map(status_str))
        .push_each("type[]", review_type.iter().map(type_str))
        .push_each("decision[]", decision.iter().map(decision_str))
        .push_opt("createdAtFrom", created_from)
        .push_opt("createdAtTo", created_to)
        .push_opt("updatedAtFrom", updated_from)
        .push_opt("updatedAtTo", updated_to)
        .push_each("vendorIds[]", vendor_id)
        .push_each("expand[]", expand.iter().map(wire_name));
    q.finish()
}

async fn print_all(client: &DrataClient, config: &Config, path: &str) -> Result<()> {
    debug!(path, "vendor reference list");
    let items = client.get_all(path).await?;
    print_value(&json!({ "data": items }), &config.output_format);
    Ok(())
}

#[instrument(skip_all)]
pub async fn criteria(action: &VendorCriteriaAction, client: &DrataClient, config: &Config) -> Result<()> {
    let VendorCriteriaAction::List {
        name,
        impact_level,
        vendor_risk_level_id,
        vendor_type_id,
        draft,
        security_review_id,
        criterion_type,
        severity,
        expand,
    } = action;
    let path = build_criteria_path(
        name.as_deref(),
        impact_level,
        vendor_risk_level_id,
        vendor_type_id,
        *draft,
        *security_review_id,
        criterion_type,
        severity,
        expand,
    );
    print_all(client, config, &path).await
}

#[instrument(skip_all)]
pub async fn criteria_creation(
    action: &VendorCriteriaCreationAction,
    client: &DrataClient,
    config: &Config,
) -> Result<()> {
    let VendorCriteriaCreationAction::List { workspace_id, status } = action;
    print_all(client, config, &build_criteria_creation_path(workspace_id, status)).await
}

#[instrument(skip_all)]
pub async fn template(action: &VendorQuestionnaireTemplateAction, client: &DrataClient, config: &Config) -> Result<()> {
    let path = match action {
        VendorQuestionnaireTemplateAction::List {
            title,
            status,
            risk_level,
            category,
        } => build_template_path(title.as_deref(), status.as_ref(), risk_level, category),
        VendorQuestionnaireTemplateAction::Questions { questionnaire_id } => {
            build_template_questions_path(questionnaire_id)
        }
    };
    print_all(client, config, &path).await
}

#[instrument(skip_all)]
pub async fn risk_level(action: &VendorRiskLevelAction, client: &DrataClient, config: &Config) -> Result<()> {
    let VendorRiskLevelAction::List = action;
    print_all(client, config, "/vendor-risk-levels").await
}

#[instrument(skip_all)]
pub async fn security_reviews(
    action: &VendorSecurityReviewsAction,
    client: &DrataClient,
    config: &Config,
) -> Result<()> {
    let VendorSecurityReviewsAction::List {
        status,
        review_type,
        decision,
        created_from,
        created_to,
        updated_from,
        updated_to,
        vendor_id,
        expand,
    } = action;
    let path = build_security_reviews_path(
        status,
        review_type,
        decision,
        created_from.as_deref(),
        created_to.as_deref(),
        updated_from.as_deref(),
        updated_to.as_deref(),
        vendor_id,
        expand,
    );
    print_all(client, config, &path).await
}

#[cfg(test)]
mod tests;
