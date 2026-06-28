//! `drata device ...` - devices (read-only curated surface).
//!
//! The custom-connection create/delete ops require a `connectionId` and are
//! left to `raw` (they have a different path prefix `/custom-connections/`).
//! Confirmed camelCase response fields: `osVersion`, `serialNumber`, `macAddress`,
//! `lastCheckedAt`, `sourceType`, `isDeviceCompliant`.
//!
//! Phase 4 adds: `--all` NDJSON streaming, `--expand`, confirm-on-mutation,
//! document upload (multipart POST).

use crate::cli::{DeviceAction, DeviceDocumentType};
use crate::client::{DrataClient, Multipart};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::expand::append_expand;
use crate::output::print_value;
use eyre::{Result, bail};
use serde_json::json;
use std::io;
use tracing::{debug, instrument};

pub async fn handle(action: &DeviceAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        DeviceAction::List { all, expand } => list(client, config, *all, expand).await,
        DeviceAction::Get { device_id, expand } => get(client, config, device_id, expand).await,
        DeviceAction::ForPersonnel { personnel_id, expand } => {
            for_personnel(client, config, personnel_id, expand).await
        }
        DeviceAction::Apps { device_id } => apps(client, config, device_id).await,
        DeviceAction::Upload {
            device_id,
            file,
            doc_type,
        } => upload(client, config, confirm, device_id, file, doc_type).await,
    }
}

/// The spec's `DeviceDocumentTypeEnum` wire value for a CLI document type.
fn device_document_type_str(t: &DeviceDocumentType) -> &'static str {
    match t {
        DeviceDocumentType::PasswordManagerEvidence => "PASSWORD_MANAGER_EVIDENCE",
        DeviceDocumentType::AutoUpdatesEvidence => "AUTO_UPDATES_EVIDENCE",
        DeviceDocumentType::HardDriveEncryptionEvidence => "HARD_DRIVE_ENCRYPTION_EVIDENCE",
        DeviceDocumentType::AntivirusEvidence => "ANTIVIRUS_EVIDENCE",
        DeviceDocumentType::LockScreenEvidence => "LOCK_SCREEN_EVIDENCE",
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, all: bool, expand: &[String]) -> Result<()> {
    debug!(all, expand_len = expand.len(), "device list");
    let base = append_expand("/devices", expand);
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
async fn get(client: &DrataClient, config: &Config, device_id: &str, expand: &[String]) -> Result<()> {
    debug!(device_id, expand_len = expand.len(), "device get");
    let path = append_expand(&format!("/devices/{}", device_id), expand);
    let resp = client.get(&path).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn for_personnel(client: &DrataClient, config: &Config, personnel_id: &str, expand: &[String]) -> Result<()> {
    debug!(personnel_id, expand_len = expand.len(), "device for-personnel");
    let base = append_expand(&format!("/personnel/{}/devices", personnel_id), expand);
    let all = client.get_all(&base).await?;
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

#[instrument(skip(client, config, confirm))]
async fn upload(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    device_id: &str,
    file: &std::path::Path,
    doc_type: &DeviceDocumentType,
) -> Result<()> {
    debug!(device_id, file = %file.display(), "device upload document");
    let path = format!("/devices/{}/documents", device_id);
    if !confirm("POST", &path)? {
        bail!("aborted");
    }
    // Spec requires the `type` field alongside the file.
    let mut form = Multipart::single("file", file);
    form.add_field("type", device_document_type_str(doc_type));
    let result = client.post_multipart(&path, &form).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[cfg(test)]
mod tests;
