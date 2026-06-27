//! `drata asset ...` - assets (CRUD).
//!
//! Paths: `/assets` (list/create), `/assets/{assetId}` (get/update/delete).
//! Confirmed camelCase from spec: `assetType`, `assetProvider`, `removedAt`,
//! `assetClassTypes`, `externalId`, `ownerId`.
//! Enum field promoted to `clap::ValueEnum`: `assetType` (PHYSICAL/VIRTUAL).
//!
//! Phase 4 adds: `--all` NDJSON streaming, `--expand`, confirm-on-mutation.

use crate::cli::{AssetAction, AssetType};
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

pub async fn handle(action: &AssetAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        AssetAction::List { all, expand } => list(client, config, *all, expand).await,
        AssetAction::Get { asset_id, expand } => get(client, config, asset_id, expand).await,
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
                confirm,
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
                confirm,
                asset_id,
                name.as_deref(),
                description.as_deref(),
                asset_type.as_ref(),
                notes.as_deref(),
            )
            .await
        }
        AssetAction::Remove { asset_id } => remove(client, config, confirm, asset_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, all: bool, expand: &[String]) -> Result<()> {
    debug!(all, expand_len = expand.len(), "asset list");
    let base = append_expand("/assets", expand);
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
async fn get(client: &DrataClient, config: &Config, asset_id: &str, expand: &[String]) -> Result<()> {
    debug!(asset_id, expand_len = expand.len(), "asset get");
    let path = append_expand(&format!("/assets/{}", asset_id), expand);
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
    name: Option<&str>,
    description: Option<&str>,
    asset_type: Option<&AssetType>,
    notes: Option<&str>,
) -> Result<()> {
    debug!("asset create");
    if !confirm("POST", "/assets")? {
        bail!("aborted");
    }
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
#[instrument(skip(client, config, confirm))]
async fn update(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    asset_id: &str,
    name: Option<&str>,
    description: Option<&str>,
    asset_type: Option<&AssetType>,
    notes: Option<&str>,
) -> Result<()> {
    debug!(asset_id, "asset update");
    if !confirm("PUT", &format!("/assets/{}", asset_id))? {
        bail!("aborted");
    }
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

#[instrument(skip(client, config, confirm))]
async fn remove(client: &DrataClient, config: &Config, confirm: &ConfirmFn, asset_id: &str) -> Result<()> {
    debug!(asset_id, "asset remove");
    if !confirm("DELETE", &format!("/assets/{}", asset_id))? {
        bail!("aborted");
    }
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
