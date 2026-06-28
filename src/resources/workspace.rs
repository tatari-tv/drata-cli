//! `drata workspace ...` - workspaces (read-only, list only).
//!
//! Path: GET `/workspaces`. Response is a paginated list of workspace objects.
//! Confirmed camelCase fields: `numInScopeControls`, `numInScopeRequirements`.

use crate::cli::WorkspaceAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use eyre::Result;
use serde_json::json;
use tracing::{debug, instrument};

pub async fn handle(action: &WorkspaceAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        WorkspaceAction::List => list(client, config).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config) -> Result<()> {
    debug!("workspace list");
    let all = client.get_all("/workspaces").await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[cfg(test)]
mod tests;
