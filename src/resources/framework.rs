//! `drata framework ...` - frameworks in a workspace.
//!
//! Paths: `/workspaces/{workspaceId}/frameworks` (list/create),
//! `/workspaces/{workspaceId}/frameworks/{frameworkId}` (update),
//! `/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements` (list/create).
//! The legacy `/workspaces/{workspaceId}/framework-requirements` endpoints are
//! left to `raw` (they lack a framework-scoped path and are marked legacy in the spec).
//! Confirmed camelCase: `shortName`, `frameworkId`, `requirementId`.
//! No enum body fields for the create/update operations.

use crate::cli::FrameworkAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use crate::spec;
use eyre::Result;
use serde_json::{Value, json};
use tracing::{debug, instrument};

pub fn example_if_requested(action: &FrameworkAction) -> Option<Result<String>> {
    match action {
        FrameworkAction::Create { example: true, .. } => {
            Some(example_skeleton("POST", "/workspaces/{workspaceId}/frameworks"))
        }
        _ => None,
    }
}

fn example_skeleton(method: &str, path: &str) -> Result<String> {
    spec::example_for_operation(method, path)?
        .ok_or_else(|| eyre::eyre!("operation `{} {}` has no JSON request body", method, path))
}

pub async fn handle(action: &FrameworkAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        FrameworkAction::List { workspace_id } => list(client, config, workspace_id).await,
        FrameworkAction::Create {
            workspace_id,
            name,
            short_name,
            description,
            example: _,
        } => {
            create(
                client,
                config,
                workspace_id,
                name.as_deref(),
                short_name.as_deref(),
                description.as_deref(),
            )
            .await
        }
        FrameworkAction::Update {
            workspace_id,
            framework_id,
            name,
            description,
        } => {
            update(
                client,
                config,
                workspace_id,
                framework_id,
                name.as_deref(),
                description.as_deref(),
            )
            .await
        }
        FrameworkAction::Requirements {
            workspace_id,
            framework_id,
        } => requirements(client, config, workspace_id, framework_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, workspace_id: &str) -> Result<()> {
    debug!(workspace_id, "framework list");
    let all = client
        .get_all(&format!("/workspaces/{}/frameworks", workspace_id))
        .await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn create(
    client: &DrataClient,
    config: &Config,
    workspace_id: &str,
    name: Option<&str>,
    short_name: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    debug!(workspace_id, "framework create");
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "shortName", short_name);
    set_opt(&mut body, "description", description);
    let result = client
        .post(&format!("/workspaces/{}/frameworks", workspace_id), body)
        .await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn update(
    client: &DrataClient,
    config: &Config,
    workspace_id: &str,
    framework_id: &str,
    name: Option<&str>,
    description: Option<&str>,
) -> Result<()> {
    debug!(workspace_id, framework_id, "framework update");
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "description", description);
    let result = client
        .put(
            &format!("/workspaces/{}/frameworks/{}", workspace_id, framework_id),
            body,
        )
        .await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn requirements(client: &DrataClient, config: &Config, workspace_id: &str, framework_id: &str) -> Result<()> {
    debug!(workspace_id, framework_id, "framework requirements");
    let all = client
        .get_all(&format!(
            "/workspaces/{}/frameworks/{}/requirements",
            workspace_id, framework_id
        ))
        .await?;
    let result = json!({ "data": all });
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

#[cfg(test)]
mod tests;
