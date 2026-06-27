//! `drata device ...` - devices (read-only curated surface).
//!
//! The custom-connection create/delete ops require a `connectionId` and are
//! left to `raw` (they have a different path prefix `/custom-connections/`).
//! Confirmed camelCase response fields: `osVersion`, `serialNumber`, `macAddress`,
//! `lastCheckedAt`, `sourceType`, `isDeviceCompliant`.

use crate::cli::DeviceAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use eyre::Result;
use serde_json::json;
use tracing::{debug, instrument};

pub async fn handle(action: &DeviceAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        DeviceAction::List => list(client, config).await,
        DeviceAction::Get { device_id } => get(client, config, device_id).await,
        DeviceAction::ForPersonnel { personnel_id } => for_personnel(client, config, personnel_id).await,
        DeviceAction::Apps { device_id } => apps(client, config, device_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config) -> Result<()> {
    debug!("device list");
    let all = client.get_all("/devices").await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, device_id: &str) -> Result<()> {
    debug!(device_id, "device get");
    let resp = client.get(&format!("/devices/{}", device_id)).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn for_personnel(client: &DrataClient, config: &Config, personnel_id: &str) -> Result<()> {
    debug!(personnel_id, "device for-personnel");
    let all = client.get_all(&format!("/personnel/{}/devices", personnel_id)).await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn apps(client: &DrataClient, config: &Config, device_id: &str) -> Result<()> {
    debug!(device_id, "device apps");
    let all = client.get_all(&format!("/devices/{}/apps", device_id)).await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[cfg(test)]
mod tests;
