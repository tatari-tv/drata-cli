//! `drata audit ...` - audits and their requests (read-only).
//!
//! Paths: GET /workspaces/{workspaceId}/audits and
//! GET /workspaces/{workspaceId}/audits/{auditId}.
//! Confirmed camelCase fields from spec: `frameworkType`, `auditType`,
//! `isInternalAudit`, `startDate`, `endDate`, `completedAt`.
//!
//! Sub-commands: list/get. Audit requests are deferred to `raw` (require knowing
//! both workspaceId and auditId; complexity exceeds the curation threshold).

use crate::cli::AuditAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use eyre::Result;
use serde_json::json;
use tracing::{debug, instrument};

pub async fn handle(action: &AuditAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        AuditAction::List { workspace_id, all } => list(client, config, workspace_id, *all).await,
        AuditAction::Get { workspace_id, audit_id } => get(client, config, workspace_id, audit_id).await,
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
async fn get(client: &DrataClient, config: &Config, workspace_id: &str, audit_id: &str) -> Result<()> {
    debug!(workspace_id, audit_id, "audit get");
    let resp = client
        .get(&format!("/workspaces/{}/audits/{}", workspace_id, audit_id))
        .await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[cfg(test)]
mod tests;
