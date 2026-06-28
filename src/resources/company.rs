//! `drata company ...` - company information (read-only, single-object response).
//!
//! Path: GET `/company`. Returns a single object (not a list envelope).
//! Confirmed camelCase from spec: `accountId`, `legalName`, `logoUrl`,
//! `securityTraining`, `backgroundCheck`, `agentEnabled`.

use crate::cli::CompanyAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use eyre::Result;
use tracing::{debug, instrument};

pub async fn handle(action: &CompanyAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        CompanyAction::Get => get(client, config).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config) -> Result<()> {
    debug!("company get");
    let resp = client.get("/company").await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[cfg(test)]
mod tests;
