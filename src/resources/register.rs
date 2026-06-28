//! `drata register ...` - risk registers.
//!
//! Paths under `/risk-registers`. Required to discover `riskRegisterId` before
//! using the `risk` command. Confirmed camelCase fields from spec:
//! `ownerIds`, `workspaceIds`, `createdAt`.
//!
//! All four verbs (list/get/create/update) plus delete. No multipart operations.

use crate::cli::RegisterAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::output::print_value;
use crate::spec;
use eyre::{Result, bail};
use serde_json::json;
use tracing::{debug, instrument};

pub fn example_if_requested(action: &RegisterAction) -> Option<Result<String>> {
    match action {
        RegisterAction::Create { example: true, .. } => Some(example_skeleton("POST", "/risk-registers")),
        _ => None,
    }
}

fn example_skeleton(method: &str, path: &str) -> Result<String> {
    spec::example_for_operation(method, path)?
        .ok_or_else(|| eyre::eyre!("operation `{} {}` has no JSON request body", method, path))
}

pub async fn handle(action: &RegisterAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        RegisterAction::List => list(client, config).await,
        RegisterAction::Get { register_id } => get(client, config, register_id).await,
        RegisterAction::Create {
            name,
            description,
            owner_ids,
            workspace_ids,
            example: _,
        } => {
            create(
                client,
                config,
                confirm,
                name.as_deref(),
                description.as_deref(),
                owner_ids.as_deref(),
                workspace_ids.as_deref(),
            )
            .await
        }
        RegisterAction::Update {
            register_id,
            name,
            description,
            owner_ids,
            workspace_ids,
        } => {
            update(
                client,
                config,
                confirm,
                register_id,
                name.as_deref(),
                description.as_deref(),
                owner_ids.as_deref(),
                workspace_ids.as_deref(),
            )
            .await
        }
        RegisterAction::Remove { register_id } => remove(client, config, confirm, register_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config) -> Result<()> {
    debug!("register list");
    let items = client.get_all("/risk-registers").await?;
    let result = json!({ "data": items });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, register_id: &str) -> Result<()> {
    debug!(register_id, "register get");
    let resp = client.get(&format!("/risk-registers/{}", register_id)).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config, confirm))]
async fn create(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    name: Option<&str>,
    description: Option<&str>,
    owner_ids: Option<&[u64]>,
    workspace_ids: Option<&[u64]>,
) -> Result<()> {
    debug!("register create");
    if !confirm("POST", "/risk-registers")? {
        bail!("aborted");
    }
    let mut body = json!({});
    if let Some(v) = name {
        body["name"] = json!(v);
    }
    if let Some(v) = description {
        body["description"] = json!(v);
    }
    // None = field omitted (not provided); Some([]) = send empty array to clear.
    if let Some(ids) = owner_ids {
        body["ownerIds"] = json!(ids);
    }
    if let Some(ids) = workspace_ids {
        body["workspaceIds"] = json!(ids);
    }
    let result = client.post("/risk-registers", body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config, confirm))]
async fn update(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    register_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    owner_ids: Option<&[u64]>,
    workspace_ids: Option<&[u64]>,
) -> Result<()> {
    debug!(register_id, "register update");
    if !confirm("PUT", &format!("/risk-registers/{}", register_id))? {
        bail!("aborted");
    }
    let mut body = json!({});
    if let Some(v) = name {
        body["name"] = json!(v);
    }
    if let Some(v) = description {
        body["description"] = json!(v);
    }
    // None = field omitted (not provided); Some([]) = send empty array to clear.
    if let Some(ids) = owner_ids {
        body["ownerIds"] = json!(ids);
    }
    if let Some(ids) = workspace_ids {
        body["workspaceIds"] = json!(ids);
    }
    let result = client.put(&format!("/risk-registers/{}", register_id), body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config, confirm))]
async fn remove(client: &DrataClient, config: &Config, confirm: &ConfirmFn, register_id: &str) -> Result<()> {
    debug!(register_id, "register remove");
    if !confirm("DELETE", &format!("/risk-registers/{}", register_id))? {
        bail!("aborted");
    }
    let result = client.delete(&format!("/risk-registers/{}", register_id)).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[cfg(test)]
mod tests;
