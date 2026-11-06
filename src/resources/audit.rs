//! `drata audit ...` - audits (read-only) and, under `audit request
//! dcf-suggestion`, the beta DCF suggestion ops (`audit/dcf.rs`).
//!
//! Paths: GET /workspaces/{workspaceId}/audits and
//! GET /workspaces/{workspaceId}/audits/{auditId}.
//! Confirmed camelCase fields from spec: `frameworkType`, `auditType`,
//! `isInternalAudit`, `startDate`, `endDate`, `completedAt`.
//!
//! Sub-commands: list/get/request. Other audit request ops stay on `raw`.

pub mod dcf;

use crate::cli::{AuditAction, AuditRequestAction};
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::expand::append_expand;
use crate::output::print_value;
use eyre::Result;
use serde_json::json;
use tracing::{debug, instrument};

pub async fn handle(action: &AuditAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        AuditAction::List { workspace_id, all } => list(client, config, workspace_id, *all).await,
        AuditAction::Get {
            workspace_id,
            audit_id,
            expand,
        } => get(client, config, workspace_id, audit_id, expand).await,
        AuditAction::Request {
            action: AuditRequestAction::DcfSuggestion { action },
        } => dcf::handle(action, client, config, confirm).await,
    }
}

pub fn example_if_requested(action: &AuditAction) -> Option<Result<String>> {
    match action {
        AuditAction::Request {
            action: AuditRequestAction::DcfSuggestion { action },
        } => dcf::example_if_requested(action),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, workspace_id: &str, all: bool) -> Result<()> {
    debug!(workspace_id, all, "audit list");
    let path = format!("/workspaces/{}/audits", workspace_id);
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
async fn get(
    client: &DrataClient,
    config: &Config,
    workspace_id: &str,
    audit_id: &str,
    expand: &[String],
) -> Result<()> {
    debug!(workspace_id, audit_id, expand_len = expand.len(), "audit get");
    let path = append_expand(&format!("/workspaces/{}/audits/{}", workspace_id, audit_id), expand);
    let resp = client.get(&path).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[cfg(test)]
mod tests;
