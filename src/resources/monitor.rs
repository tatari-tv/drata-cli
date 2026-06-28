//! `drata monitor ...` - monitoring tests in a workspace.
//!
//! Paths under `/workspaces/{workspaceId}/monitoring-tests`. Confirmed camelCase
//! fields from spec: `checkResultStatus`, `checkStatus`, `lastPassedAt`,
//! `failedSince`, `disabledByUser`. Update body: `name`, `enabled`, `description`.
//!
//! Sub-commands: list/get/update/exclusions/failures/passes.

use crate::cli::MonitorAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::output::print_value;
use eyre::{Result, bail};
use serde_json::json;
use tracing::{debug, instrument};

pub async fn handle(action: &MonitorAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        MonitorAction::List { workspace_id, all } => list(client, config, workspace_id, *all).await,
        MonitorAction::Get { workspace_id, test_id } => get(client, config, workspace_id, test_id).await,
        MonitorAction::Update {
            workspace_id,
            test_id,
            name,
            enabled,
            description,
        } => {
            update(
                client,
                config,
                confirm,
                workspace_id,
                test_id,
                name.as_deref(),
                *enabled,
                description.as_deref(),
            )
            .await
        }
        MonitorAction::Exclusions { workspace_id, test_id } => exclusions(client, config, workspace_id, test_id).await,
        MonitorAction::Failures { workspace_id, test_id } => failures(client, config, workspace_id, test_id).await,
        MonitorAction::Passes { workspace_id, test_id } => passes(client, config, workspace_id, test_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, workspace_id: &str, all: bool) -> Result<()> {
    debug!(workspace_id, all, "monitor list");
    let path = format!("/workspaces/{}/monitoring-tests", workspace_id);
    if all {
        let mut stdout = std::io::stdout();
        client.stream_all(&path, &mut stdout).await?;
    } else {
        let items = client.get_all(&path).await?;
        let result = json!({ "data": items });
        print_value(&result, &config.output_format);
    }
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, workspace_id: &str, test_id: &str) -> Result<()> {
    debug!(workspace_id, test_id, "monitor get");
    let resp = client
        .get(&format!("/workspaces/{}/monitoring-tests/{}", workspace_id, test_id))
        .await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config, confirm))]
async fn update(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    workspace_id: &str,
    test_id: &str,
    name: Option<&str>,
    enabled: Option<bool>,
    description: Option<&str>,
) -> Result<()> {
    debug!(workspace_id, test_id, "monitor update");
    let path = format!("/workspaces/{}/monitoring-tests/{}", workspace_id, test_id);
    if !confirm("PUT", &path)? {
        bail!("aborted");
    }
    let mut body = json!({});
    if let Some(v) = name {
        body["name"] = json!(v);
    }
    if let Some(v) = enabled {
        body["enabled"] = json!(v);
    }
    if let Some(v) = description {
        body["description"] = json!(v);
    }
    let result = client.put(&path, body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn exclusions(client: &DrataClient, config: &Config, workspace_id: &str, test_id: &str) -> Result<()> {
    debug!(workspace_id, test_id, "monitor exclusions");
    let items = client
        .get_all(&format!(
            "/workspaces/{}/monitoring-tests/{}/exclusions",
            workspace_id, test_id
        ))
        .await?;
    let result = json!({ "data": items });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn failures(client: &DrataClient, config: &Config, workspace_id: &str, test_id: &str) -> Result<()> {
    debug!(workspace_id, test_id, "monitor failures");
    let items = client
        .get_all(&format!(
            "/workspaces/{}/monitoring-tests/{}/failures",
            workspace_id, test_id
        ))
        .await?;
    let result = json!({ "data": items });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn passes(client: &DrataClient, config: &Config, workspace_id: &str, test_id: &str) -> Result<()> {
    debug!(workspace_id, test_id, "monitor passes");
    let items = client
        .get_all(&format!(
            "/workspaces/{}/monitoring-tests/{}/passes",
            workspace_id, test_id
        ))
        .await?;
    let result = json!({ "data": items });
    print_value(&result, &config.output_format);
    Ok(())
}

#[cfg(test)]
mod tests;
