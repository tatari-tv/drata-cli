//! `drata evidence ...` - evidence library items in a workspace.
//!
//! All paths are workspace-scoped: `/workspaces/{workspaceId}/evidence-library/...`.
//! Confirmed camelCase from spec: `evidenceLibraryId`, `renewalScheduleType`,
//! `implementationGuidance`, `evidenceTemplateCode`.
//! Enum field promoted to `clap::ValueEnum`: `renewalScheduleType`
//! (ONE_MONTH/TWO_MONTHS/THREE_MONTHS/SIX_MONTHS/ONE_YEAR/CUSTOM/NONE).
//!
//! Phase 4 adds: `--all` NDJSON streaming, `--expand`, confirm-on-mutation,
//! and `--file` multipart upload on create and update.

use crate::cli::{EvidenceAction, RenewalScheduleType};
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

pub async fn handle(action: &EvidenceAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        EvidenceAction::List {
            workspace_id,
            all,
            expand,
        } => list(client, config, workspace_id, *all, expand).await,
        EvidenceAction::Get {
            workspace_id,
            evidence_id,
            expand,
        } => get(client, config, workspace_id, evidence_id, expand).await,
        EvidenceAction::Create {
            workspace_id,
            name,
            description,
            renewal_schedule_type,
            file,
            example: _,
        } => {
            create(
                client,
                config,
                confirm,
                workspace_id,
                name.as_deref(),
                description.as_deref(),
                renewal_schedule_type.as_ref(),
                file.as_deref(),
            )
            .await
        }
        EvidenceAction::Update {
            workspace_id,
            evidence_id,
            name,
            description,
            renewal_schedule_type,
            file,
        } => {
            update(
                client,
                config,
                confirm,
                workspace_id,
                evidence_id,
                name.as_deref(),
                description.as_deref(),
                renewal_schedule_type.as_ref(),
                file.as_deref(),
            )
            .await
        }
        EvidenceAction::Remove {
            workspace_id,
            evidence_id,
        } => remove(client, config, confirm, workspace_id, evidence_id).await,
        EvidenceAction::GetVersion {
            workspace_id,
            evidence_id,
            version_id,
            expand,
        } => get_version(client, config, workspace_id, evidence_id, version_id, expand).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, workspace_id: &str, all: bool, expand: &[String]) -> Result<()> {
    debug!(workspace_id, all, expand_len = expand.len(), "evidence list");
    let base = append_expand(&format!("/workspaces/{}/evidence-library", workspace_id), expand);
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
async fn get(
    client: &DrataClient,
    config: &Config,
    workspace_id: &str,
    evidence_id: &str,
    expand: &[String],
) -> Result<()> {
    debug!(workspace_id, evidence_id, expand_len = expand.len(), "evidence get");
    let path = append_expand(
        &format!("/workspaces/{}/evidence-library/{}", workspace_id, evidence_id),
        expand,
    );
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
    workspace_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    renewal_schedule_type: Option<&RenewalScheduleType>,
    file: Option<&std::path::Path>,
) -> Result<()> {
    debug!(workspace_id, has_file = file.is_some(), "evidence create");
    let path = format!("/workspaces/{}/evidence-library", workspace_id);
    if !confirm("POST", &path)? {
        bail!("aborted");
    }
    if let Some(f) = file {
        let result = client.post_multipart(&path, f).await?;
        print_value(&result, &config.output_format);
        return Ok(());
    }
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "description", description);
    if let Some(rst) = renewal_schedule_type {
        body["renewalScheduleType"] = json!(renewal_schedule_type_str(rst));
    }
    let result = client.post(&path, body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(client, config, confirm))]
async fn update(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    workspace_id: &str,
    evidence_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    renewal_schedule_type: Option<&RenewalScheduleType>,
    file: Option<&std::path::Path>,
) -> Result<()> {
    debug!(workspace_id, evidence_id, has_file = file.is_some(), "evidence update");
    let path = format!("/workspaces/{}/evidence-library/{}", workspace_id, evidence_id);
    if !confirm("PUT", &path)? {
        bail!("aborted");
    }
    if let Some(f) = file {
        // The spec specifies PUT multipart/form-data for evidence update
        // (PUT /workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}).
        let result = client.put_multipart(&path, f).await?;
        print_value(&result, &config.output_format);
        return Ok(());
    }
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "description", description);
    if let Some(rst) = renewal_schedule_type {
        body["renewalScheduleType"] = json!(renewal_schedule_type_str(rst));
    }
    let result = client.put(&path, body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config, confirm))]
async fn remove(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    workspace_id: &str,
    evidence_id: &str,
) -> Result<()> {
    debug!(workspace_id, evidence_id, "evidence remove");
    let path = format!("/workspaces/{}/evidence-library/{}", workspace_id, evidence_id);
    if !confirm("DELETE", &path)? {
        bail!("aborted");
    }
    let result = client.delete(&path).await?;
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
    expand: &[String],
) -> Result<()> {
    debug!(
        workspace_id,
        evidence_id,
        version_id,
        expand_len = expand.len(),
        "evidence get-version"
    );
    let path = append_expand(
        &format!(
            "/workspaces/{}/evidence-library/{}/versions/{}",
            workspace_id, evidence_id, version_id
        ),
        expand,
    );
    let resp = client.get(&path).await?;
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
