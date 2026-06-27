//! `drata evidence ...` - evidence library items in a workspace.
//!
//! All paths are workspace-scoped: `/workspaces/{workspaceId}/evidence-library/...`.
//! Confirmed camelCase from spec: `evidenceLibraryId`, `renewalScheduleType`,
//! `implementationGuidance`, `evidenceTemplateCode`.
//! Enum field promoted to `clap::ValueEnum`: `renewalScheduleType`
//! (ONE_MONTH/TWO_MONTHS/THREE_MONTHS/SIX_MONTHS/ONE_YEAR/CUSTOM/NONE).

use crate::cli::{EvidenceAction, RenewalScheduleType};
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use crate::spec;
use eyre::Result;
use serde_json::{Value, json};
use tracing::{debug, instrument};

pub fn example_if_requested(action: &EvidenceAction) -> Option<Result<String>> {
    match action {
        EvidenceAction::Create { example: true, .. } => {
            Some(example_skeleton("POST", "/workspaces/{workspaceId}/evidence-library"))
        }
        _ => None,
    }
}

fn example_skeleton(method: &str, path: &str) -> Result<String> {
    spec::example_for_operation(method, path)?
        .ok_or_else(|| eyre::eyre!("operation `{} {}` has no JSON request body", method, path))
}

pub async fn handle(action: &EvidenceAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        EvidenceAction::List { workspace_id } => list(client, config, workspace_id).await,
        EvidenceAction::Get {
            workspace_id,
            evidence_id,
        } => get(client, config, workspace_id, evidence_id).await,
        EvidenceAction::Create {
            workspace_id,
            name,
            description,
            renewal_schedule_type,
            example: _,
        } => {
            create(
                client,
                config,
                workspace_id,
                name.as_deref(),
                description.as_deref(),
                renewal_schedule_type.as_ref(),
            )
            .await
        }
        EvidenceAction::Update {
            workspace_id,
            evidence_id,
            name,
            description,
            renewal_schedule_type,
        } => {
            update(
                client,
                config,
                workspace_id,
                evidence_id,
                name.as_deref(),
                description.as_deref(),
                renewal_schedule_type.as_ref(),
            )
            .await
        }
        EvidenceAction::Remove {
            workspace_id,
            evidence_id,
        } => remove(client, config, workspace_id, evidence_id).await,
        EvidenceAction::GetVersion {
            workspace_id,
            evidence_id,
            version_id,
        } => get_version(client, config, workspace_id, evidence_id, version_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, workspace_id: &str) -> Result<()> {
    debug!(workspace_id, "evidence list");
    let all = client
        .get_all(&format!("/workspaces/{}/evidence-library", workspace_id))
        .await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, workspace_id: &str, evidence_id: &str) -> Result<()> {
    debug!(workspace_id, evidence_id, "evidence get");
    let resp = client
        .get(&format!(
            "/workspaces/{}/evidence-library/{}",
            workspace_id, evidence_id
        ))
        .await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn create(
    client: &DrataClient,
    config: &Config,
    workspace_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    renewal_schedule_type: Option<&RenewalScheduleType>,
) -> Result<()> {
    debug!(workspace_id, "evidence create");
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "description", description);
    if let Some(rst) = renewal_schedule_type {
        body["renewalScheduleType"] = json!(renewal_schedule_type_str(rst));
    }
    let result = client
        .post(&format!("/workspaces/{}/evidence-library", workspace_id), body)
        .await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn update(
    client: &DrataClient,
    config: &Config,
    workspace_id: &str,
    evidence_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    renewal_schedule_type: Option<&RenewalScheduleType>,
) -> Result<()> {
    debug!(workspace_id, evidence_id, "evidence update");
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "description", description);
    if let Some(rst) = renewal_schedule_type {
        body["renewalScheduleType"] = json!(renewal_schedule_type_str(rst));
    }
    let result = client
        .put(
            &format!("/workspaces/{}/evidence-library/{}", workspace_id, evidence_id),
            body,
        )
        .await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn remove(client: &DrataClient, config: &Config, workspace_id: &str, evidence_id: &str) -> Result<()> {
    debug!(workspace_id, evidence_id, "evidence remove");
    let result = client
        .delete(&format!(
            "/workspaces/{}/evidence-library/{}",
            workspace_id, evidence_id
        ))
        .await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn get_version(
    client: &DrataClient,
    config: &Config,
    workspace_id: &str,
    evidence_id: &str,
    version_id: &str,
) -> Result<()> {
    debug!(workspace_id, evidence_id, version_id, "evidence get-version");
    let resp = client
        .get(&format!(
            "/workspaces/{}/evidence-library/{}/versions/{}",
            workspace_id, evidence_id, version_id
        ))
        .await?;
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

fn renewal_schedule_type_str(rst: &RenewalScheduleType) -> &'static str {
    match rst {
        RenewalScheduleType::OneMonth => "ONE_MONTH",
        RenewalScheduleType::TwoMonths => "TWO_MONTHS",
        RenewalScheduleType::ThreeMonths => "THREE_MONTHS",
        RenewalScheduleType::SixMonths => "SIX_MONTHS",
        RenewalScheduleType::OneYear => "ONE_YEAR",
        RenewalScheduleType::Custom => "CUSTOM",
        RenewalScheduleType::None => "NONE",
    }
}

#[cfg(test)]
mod tests;
