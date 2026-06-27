//! `drata risk ...` - risks in a risk register.
//!
//! Paths are all nested under `/risk-registers/{riskRegisterId}/risks`.
//! Confirmed camelCase from spec: `treatmentPlan`, `identifiedAt`, `residualImpact`.
//! Enum fields promoted to `clap::ValueEnum`: `treatmentPlan` (UNTREATED/ACCEPT/
//! TRANSFER/AVOID/MITIGATE), `status` (ACTIVE/ARCHIVED/CLOSED).

use crate::cli::{RiskAction, RiskStatus, RiskTreatmentPlan};
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use crate::spec;
use eyre::Result;
use serde_json::{Value, json};
use tracing::{debug, instrument};

pub fn example_if_requested(action: &RiskAction) -> Option<Result<String>> {
    match action {
        RiskAction::Create { example: true, .. } => {
            Some(example_skeleton("POST", "/risk-registers/{riskRegisterId}/risks"))
        }
        _ => None,
    }
}

fn example_skeleton(method: &str, path: &str) -> Result<String> {
    spec::example_for_operation(method, path)?
        .ok_or_else(|| eyre::eyre!("operation `{} {}` has no JSON request body", method, path))
}

pub async fn handle(action: &RiskAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        RiskAction::List { register_id } => list(client, config, register_id).await,
        RiskAction::Get { register_id, risk_id } => get(client, config, register_id, risk_id).await,
        RiskAction::Create {
            register_id,
            title,
            description,
            treatment_plan,
            impact,
            likelihood,
            status,
            example: _,
        } => {
            create(
                client,
                config,
                register_id,
                title.as_deref(),
                description.as_deref(),
                treatment_plan.as_ref(),
                *impact,
                *likelihood,
                status.as_ref(),
            )
            .await
        }
        RiskAction::Update {
            register_id,
            risk_id,
            title,
            description,
            treatment_plan,
            impact,
            likelihood,
            status,
        } => {
            update(
                client,
                config,
                register_id,
                risk_id,
                title.as_deref(),
                description.as_deref(),
                treatment_plan.as_ref(),
                *impact,
                *likelihood,
                status.as_ref(),
            )
            .await
        }
        RiskAction::Insights { register_id } => insights(client, config, register_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, register_id: &str) -> Result<()> {
    debug!(register_id, "risk list");
    let all = client
        .get_all(&format!("/risk-registers/{}/risks", register_id))
        .await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, register_id: &str, risk_id: &str) -> Result<()> {
    debug!(register_id, risk_id, "risk get");
    let resp = client
        .get(&format!("/risk-registers/{}/risks/{}", register_id, risk_id))
        .await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(client, config))]
async fn create(
    client: &DrataClient,
    config: &Config,
    register_id: &str,
    title: Option<&str>,
    description: Option<&str>,
    treatment_plan: Option<&RiskTreatmentPlan>,
    impact: Option<f64>,
    likelihood: Option<f64>,
    status: Option<&RiskStatus>,
) -> Result<()> {
    debug!(register_id, "risk create");
    let mut body = json!({});
    set_opt(&mut body, "title", title);
    set_opt(&mut body, "description", description);
    if let Some(tp) = treatment_plan {
        body["treatmentPlan"] = json!(treatment_plan_str(tp));
    }
    if let Some(v) = impact {
        body["impact"] = json!(v);
    }
    if let Some(v) = likelihood {
        body["likelihood"] = json!(v);
    }
    if let Some(s) = status {
        body["status"] = json!(risk_status_str(s));
    }
    let result = client
        .post(&format!("/risk-registers/{}/risks", register_id), body)
        .await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(client, config))]
async fn update(
    client: &DrataClient,
    config: &Config,
    register_id: &str,
    risk_id: &str,
    title: Option<&str>,
    description: Option<&str>,
    treatment_plan: Option<&RiskTreatmentPlan>,
    impact: Option<f64>,
    likelihood: Option<f64>,
    status: Option<&RiskStatus>,
) -> Result<()> {
    debug!(register_id, risk_id, "risk update");
    let mut body = json!({});
    set_opt(&mut body, "title", title);
    set_opt(&mut body, "description", description);
    if let Some(tp) = treatment_plan {
        body["treatmentPlan"] = json!(treatment_plan_str(tp));
    }
    if let Some(v) = impact {
        body["impact"] = json!(v);
    }
    if let Some(v) = likelihood {
        body["likelihood"] = json!(v);
    }
    if let Some(s) = status {
        body["status"] = json!(risk_status_str(s));
    }
    let result = client
        .put(&format!("/risk-registers/{}/risks/{}", register_id, risk_id), body)
        .await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn insights(client: &DrataClient, config: &Config, register_id: &str) -> Result<()> {
    debug!(register_id, "risk insights");
    let resp = client.get(&format!("/risk-registers/{}/insights", register_id)).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn set_opt(body: &mut Value, key: &str, value: Option<&str>) {
    if let Some(v) = value {
        body[key] = json!(v);
    }
}

fn treatment_plan_str(tp: &RiskTreatmentPlan) -> &'static str {
    match tp {
        RiskTreatmentPlan::Untreated => "UNTREATED",
        RiskTreatmentPlan::Accept => "ACCEPT",
        RiskTreatmentPlan::Transfer => "TRANSFER",
        RiskTreatmentPlan::Avoid => "AVOID",
        RiskTreatmentPlan::Mitigate => "MITIGATE",
    }
}

fn risk_status_str(s: &RiskStatus) -> &'static str {
    match s {
        RiskStatus::Active => "ACTIVE",
        RiskStatus::Archived => "ARCHIVED",
        RiskStatus::Closed => "CLOSED",
    }
}

#[cfg(test)]
mod tests;
