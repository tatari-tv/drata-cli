//! `drata asset ...` - assets (CRUD).
//!
//! Paths: `/assets` (list/create), `/assets/{assetId}` (get/update/delete).
//! Confirmed camelCase from spec: `assetType`, `assetProvider`, `removedAt`,
//! `assetClassTypes`, `externalId`, `ownerId`.
//! Enum field promoted to `clap::ValueEnum`: `assetType` (PHYSICAL/VIRTUAL).

use crate::cli::{AssetAction, AssetType};
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use crate::spec;
use eyre::Result;
use serde_json::{Value, json};
use tracing::{debug, instrument};

pub fn example_if_requested(action: &AssetAction) -> Option<Result<String>> {
    match action {
        AssetAction::Create { example: true, .. } => Some(example_skeleton("POST", "/assets")),
        _ => None,
    }
}

fn example_skeleton(method: &str, path: &str) -> Result<String> {
    spec::example_for_operation(method, path)?
        .ok_or_else(|| eyre::eyre!("operation `{} {}` has no JSON request body", method, path))
}

pub async fn handle(action: &AssetAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        AssetAction::List => list(client, config).await,
        AssetAction::Get { asset_id } => get(client, config, asset_id).await,
        AssetAction::Create {
            name,
            description,
            asset_type,
            notes,
            example: _,
        } => {
            create(
                client,
                config,
                name.as_deref(),
                description.as_deref(),
                asset_type.as_ref(),
                notes.as_deref(),
            )
            .await
        }
        AssetAction::Update {
            asset_id,
            name,
            description,
            asset_type,
            notes,
        } => {
            update(
                client,
                config,
                asset_id,
                name.as_deref(),
                description.as_deref(),
                asset_type.as_ref(),
                notes.as_deref(),
            )
            .await
        }
        AssetAction::Remove { asset_id } => remove(client, config, asset_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config) -> Result<()> {
    debug!("asset list");
    let all = client.get_all("/assets").await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, asset_id: &str) -> Result<()> {
    debug!(asset_id, "asset get");
    let resp = client.get(&format!("/assets/{}", asset_id)).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn create(
    client: &DrataClient,
    config: &Config,
    name: Option<&str>,
    description: Option<&str>,
    asset_type: Option<&AssetType>,
    notes: Option<&str>,
) -> Result<()> {
    debug!("asset create");
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "description", description);
    set_opt(&mut body, "notes", notes);
    if let Some(at) = asset_type {
        body["assetType"] = json!(asset_type_str(at));
    }
    let result = client.post("/assets", body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(client, config))]
async fn update(
    client: &DrataClient,
    config: &Config,
    asset_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    asset_type: Option<&AssetType>,
    notes: Option<&str>,
) -> Result<()> {
    debug!(asset_id, "asset update");
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "description", description);
    set_opt(&mut body, "notes", notes);
    if let Some(at) = asset_type {
        body["assetType"] = json!(asset_type_str(at));
    }
    let result = client.put(&format!("/assets/{}", asset_id), body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn remove(client: &DrataClient, config: &Config, asset_id: &str) -> Result<()> {
    debug!(asset_id, "asset remove");
    let result = client.delete(&format!("/assets/{}", asset_id)).await?;
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

fn asset_type_str(at: &AssetType) -> &'static str {
    match at {
        AssetType::Physical => "PHYSICAL",
        AssetType::Virtual => "VIRTUAL",
    }
}

#[cfg(test)]
mod tests;
