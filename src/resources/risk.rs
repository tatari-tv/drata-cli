//! `drata risk ...` - risks in a risk register.
//!
//! Paths are all nested under `/risk-registers/{riskRegisterId}/risks`.
//! Confirmed camelCase from spec: `treatmentPlan`, `identifiedAt`, `residualImpact`.
//! Enum fields promoted to `clap::ValueEnum`: `treatmentPlan` (UNTREATED/ACCEPT/
//! TRANSFER/AVOID/MITIGATE), `status` (ACTIVE/ARCHIVED/CLOSED).
//!
//! Phase 4 adds: `--all` NDJSON streaming, `--expand`, confirm-on-mutation,
//! document upload (multipart POST).

use crate::cli::{RiskAction, RiskStatus, RiskTreatmentPlan};
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::expand::append_expand;
use crate::output::print_value;
use crate::spec;
use eyre::{Result, bail};
use serde_json::{Value, json};
use std::io;
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

pub async fn handle(action: &RiskAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        RiskAction::List {
            register_id,
            all,
            expand,
        } => list(client, config, register_id, *all, expand).await,
        RiskAction::Get {
            register_id,
            risk_id,
            expand,
        } => get(client, config, register_id, risk_id, expand).await,
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
                confirm,
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
                confirm,
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
        RiskAction::Upload {
            register_id,
            risk_id,
            file,
        } => upload(client, config, confirm, register_id, risk_id, file).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, register_id: &str, all: bool, expand: &[String]) -> Result<()> {
    debug!(register_id, all, expand_len = expand.len(), "risk list");
    let base = append_expand(&format!("/risk-registers/{}/risks", register_id), expand);
    if all {
        let mut stdout = io::stdout();
        client.stream_all(&base, &mut stdout).await?;
    } else {
        let items = client.get_all(&base).await?;
        let result = json!({ "data": items });
        print_value(&result, &config.output_format);
    }
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, register_id: &str, risk_id: &str, expand: &[String]) -> Result<()> {
    debug!(register_id, risk_id, expand_len = expand.len(), "risk get");
    let path = append_expand(&format!("/risk-registers/{}/risks/{}", register_id, risk_id), expand);
    let resp = client.get(&path).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(client, config, confirm))]
async fn create(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    register_id: &str,
    title: Option<&str>,
    description: Option<&str>,
    treatment_plan: Option<&RiskTreatmentPlan>,
    impact: Option<f64>,
    likelihood: Option<f64>,
    status: Option<&RiskStatus>,
) -> Result<()> {
    debug!(register_id, "risk create");
    if !confirm("POST", &format!("/risk-registers/{}/risks", register_id))? {
        bail!("aborted");
    }
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
#[instrument(skip(client, config, confirm))]
async fn update(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
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
    if !confirm("PUT", &format!("/risk-registers/{}/risks/{}", register_id, risk_id))? {
        bail!("aborted");
    }
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

#[instrument(skip(client, config, confirm))]
async fn upload(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    register_id: &str,
    risk_id: &str,
    file: &std::path::Path,
) -> Result<()> {
    debug!(register_id, risk_id, file = %file.display(), "risk upload document");
    let path = format!("/risk-registers/{}/risks/{}/documents", register_id, risk_id);
    if !confirm("POST", &path)? {
        bail!("aborted");
    }
    let result = client.post_multipart(&path, file).await?;
    print_value(&result, &config.output_format);
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
