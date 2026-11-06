//! `drata risk ...` - risks in a risk register.
//!
//! Paths are all nested under `/risk-registers/{riskRegisterId}/risks`.
//! Confirmed camelCase from spec: `treatmentPlan`, `identifiedAt`, `residualImpact`.
//! Enum fields promoted to `clap::ValueEnum`: `treatmentPlan` (UNTREATED/ACCEPT/
//! TRANSFER/AVOID/MITIGATE), `status` (ACTIVE/ARCHIVED/CLOSED).
//!
//! Phase 4 adds: `--all` NDJSON streaming, `--expand`, confirm-on-mutation,
//! document upload (multipart POST).

use crate::cli::{RiskAction, RiskSearchArgs, RiskSearchFacet, RiskStatus, RiskTicketAction, RiskTreatmentPlan};
use crate::client::{DrataClient, Multipart};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::expand::append_expand;
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use crate::resources::{search, ticket};
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
        RiskAction::Ticket {
            action: RiskTicketAction::Create { args, .. },
        } => ticket::create_example(args, ticket::RISK_CREATE_SPEC_PATH),
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
            vendor_id,
            example: _,
        } => {
            create(
                client,
                config,
                confirm,
                register_id.as_deref(),
                title.as_deref(),
                description.as_deref(),
                treatment_plan.as_ref(),
                *impact,
                *likelihood,
                status.as_ref(),
                *vendor_id,
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
            vendor_id,
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
                *vendor_id,
            )
            .await
        }
        RiskAction::Insights { register_id } => insights(client, config, register_id).await,
        RiskAction::Upload {
            register_id,
            risk_id,
            files,
        } => upload(client, config, confirm, register_id, risk_id, files).await,
        RiskAction::Remove { register_id, risk_id } => remove(client, config, confirm, register_id, risk_id).await,
        RiskAction::Search(args) => {
            let (query, facets) = build_search(args)?;
            search::run(client, config, query, &facets).await
        }
        RiskAction::Ticket { action } => {
            let (tickets, op) = ticket::risk_target(action)?;
            ticket::handle(client, config, confirm, &tickets, op).await
        }
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
    register_id: Option<&str>,
    title: Option<&str>,
    description: Option<&str>,
    treatment_plan: Option<&RiskTreatmentPlan>,
    impact: Option<f64>,
    likelihood: Option<f64>,
    status: Option<&RiskStatus>,
    vendor_id: Option<u64>,
) -> Result<()> {
    // register_id is clap-required unless --example (which never reaches here).
    let register_id =
        register_id.ok_or_else(|| eyre::eyre!("`drata risk create` requires <register_id> (or use --example)"))?;
    debug!(register_id, "risk create");
    // Spec requires title and description.
    let title = title.ok_or_else(|| eyre::eyre!("`drata risk create` requires --title (or use --example)"))?;
    let description =
        description.ok_or_else(|| eyre::eyre!("`drata risk create` requires --description (or use --example)"))?;
    if !confirm("POST", &format!("/risk-registers/{}/risks", register_id))? {
        bail!("aborted");
    }
    let mut body = json!({ "title": title, "description": description });
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
    if let Some(v) = vendor_id {
        body["vendorId"] = json!(v);
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
    vendor_id: Option<u64>,
) -> Result<()> {
    debug!(register_id, risk_id, ?vendor_id, "risk update");
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
    if let Some(v) = vendor_id {
        body["vendorId"] = json!(v);
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
    files: &[std::path::PathBuf],
) -> Result<()> {
    debug!(register_id, risk_id, file_count = files.len(), "risk upload document");
    // The spec's request body is a required `files` array; send each file as a
    // `files` part. Validate before confirming.
    if files.is_empty() {
        bail!("`drata risk upload` requires at least one --file");
    }
    let path = format!("/risk-registers/{}/risks/{}/documents", register_id, risk_id);
    if !confirm("POST", &path)? {
        bail!("aborted");
    }
    let mut form = Multipart::new();
    for file in files {
        form.add_file("files", file.clone());
    }
    let result = client.post_multipart(&path, &form).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config, confirm))]
async fn remove(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    register_id: &str,
    risk_id: &str,
) -> Result<()> {
    debug!(register_id, risk_id, "risk remove");
    let path = format!("/risk-registers/{}/risks/{}", register_id, risk_id);
    if !confirm("DELETE", &path)? {
        bail!("aborted");
    }
    let result = client.delete(&path).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// The search query and the facet wire names. `/risks-search` spans every
/// register; `/risk-registers/{id}/risks-search` is one register and does not
/// accept the `riskRegisterId` facet, so that combination fails before any
/// request. Pure.
fn build_search(args: &RiskSearchArgs) -> Result<(QueryPath, Vec<String>)> {
    if args.register_id.is_some() && args.facet.contains(&RiskSearchFacet::RiskRegisterId) {
        bail!("--facet riskRegisterId is only valid when searching every register (omit --register-id)");
    }
    let base = match &args.register_id {
        Some(id) => format!("/risk-registers/{}/risks-search", id),
        None => "/risks-search".to_string(),
    };
    let mut q = QueryPath::new(base);
    q.push_opt("q", args.q.as_deref())
        .push_each("status[]", args.status.iter().map(wire_name))
        .push_each("treatmentPlan[]", args.treatment_plan.iter().map(wire_name))
        .push_each("type[]", args.kinds.iter().map(wire_name))
        .push_opt("inherentScoreGte", args.inherent_score_gte)
        .push_opt("inherentScoreLte", args.inherent_score_lte)
        .push_opt("residualScoreGte", args.residual_score_gte)
        .push_opt("residualScoreLte", args.residual_score_lte)
        .push_each("categories[]", &args.categories)
        .push_each("controls[]", &args.controls)
        .push_each("ownerEmails[]", &args.owner_emails)
        .push_each("reviewerEmails[]", &args.reviewer_emails)
        .push_each("vendorNames[]", &args.vendor_names)
        .push_each("expand[]", args.expand.iter().map(wire_name));
    let facets = args.facet.iter().map(wire_name).collect();
    Ok((q, facets))
}

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
