//! `drata control ...` - controls in a workspace.
//!
//! All paths are workspace-scoped: `/workspaces/{workspaceId}/controls/...`.
//! No enum body fields - the spec uses free-text for control fields.
//! Confirmed camelCase: `controlId`, `workspaceId`.
//!
//! Phase 4 adds: `--all` NDJSON streaming, `--expand`, confirm-on-mutation.
//! The control create op is multipart (`POST /workspaces/{id}/controls`);
//! when `--file` is provided the upload path is used.

use crate::cli::ControlAction;
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

pub fn example_if_requested(action: &ControlAction) -> Option<Result<String>> {
    match action {
        ControlAction::Create { example: true, .. } => {
            Some(example_skeleton("POST", "/workspaces/{workspaceId}/controls"))
        }
        _ => None,
    }
}

fn example_skeleton(method: &str, path: &str) -> Result<String> {
    spec::example_for_operation(method, path)?
        .ok_or_else(|| eyre::eyre!("operation `{} {}` has no JSON request body", method, path))
}

pub async fn handle(action: &ControlAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        ControlAction::List {
            workspace_id,
            all,
            expand,
        } => list(client, config, workspace_id, *all, expand).await,
        ControlAction::Get {
            workspace_id,
            control_id,
            expand,
        } => get(client, config, workspace_id, control_id, expand).await,
        ControlAction::Create {
            workspace_id,
            name,
            description,
            question,
            activity,
            example: _,
        } => {
            create(
                client,
                config,
                confirm,
                workspace_id,
                name.as_deref(),
                description.as_deref(),
                question.as_deref(),
                activity.as_deref(),
            )
            .await
        }
        ControlAction::Update {
            workspace_id,
            control_id,
            name,
            description,
            question,
            activity,
        } => {
            update(
                client,
                config,
                confirm,
                workspace_id,
                control_id,
                name.as_deref(),
                description.as_deref(),
                question.as_deref(),
                activity.as_deref(),
            )
            .await
        }
        ControlAction::Requirements {
            workspace_id,
            control_id,
        } => requirements(client, config, workspace_id, control_id).await,
        ControlAction::Compare { workspace_id } => compare(client, config, workspace_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, workspace_id: &str, all: bool, expand: &[String]) -> Result<()> {
    debug!(workspace_id, all, expand_len = expand.len(), "control list");
    let base = append_expand(&format!("/workspaces/{}/controls", workspace_id), expand);
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
    control_id: &str,
    expand: &[String],
) -> Result<()> {
    debug!(workspace_id, control_id, expand_len = expand.len(), "control get");
    let path = append_expand(&format!("/workspaces/{}/controls/{}", workspace_id, control_id), expand);
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
    question: Option<&str>,
    activity: Option<&str>,
) -> Result<()> {
    debug!(workspace_id, "control create");
    let path = format!("/workspaces/{}/controls", workspace_id);
    if !confirm("POST", &path)? {
        bail!("aborted");
    }
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "description", description);
    set_opt(&mut body, "question", question);
    set_opt(&mut body, "activity", activity);
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
    control_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    question: Option<&str>,
    activity: Option<&str>,
) -> Result<()> {
    debug!(workspace_id, control_id, "control update");
    let path = format!("/workspaces/{}/controls/{}", workspace_id, control_id);
    if !confirm("PUT", &path)? {
        bail!("aborted");
    }
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "description", description);
    set_opt(&mut body, "question", question);
    set_opt(&mut body, "activity", activity);
    let result = client.put(&path, body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn requirements(client: &DrataClient, config: &Config, workspace_id: &str, control_id: &str) -> Result<()> {
    debug!(workspace_id, control_id, "control requirements");
    let all = client
        .get_all(&format!(
            "/workspaces/{}/controls/{}/requirements",
            workspace_id, control_id
        ))
        .await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn compare(client: &DrataClient, config: &Config, workspace_id: &str) -> Result<()> {
    debug!(workspace_id, "control compare");
    let resp = client
        .get(&format!("/workspaces/{}/controls-requirement-comparison", workspace_id))
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

#[cfg(test)]
mod tests;
