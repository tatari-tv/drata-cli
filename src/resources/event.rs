//! `drata event ...` - events (read-only list/get).
//!
//! Paths: GET /events, GET /events/{eventId}.
//! Confirmed camelCase fields from spec: `userId`, `connectionId`,
//! `requestDescription`, `testName`, `testId`, `createdAt`.
//!
//! Download-job creation/status is deferred to `raw` (requires event ID and a
//! multi-step polling workflow; `raw` is sufficient for occasional use).

use crate::cli::EventAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use eyre::Result;
use serde_json::json;
use tracing::{debug, instrument};

pub async fn handle(action: &EventAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        EventAction::List { all } => list(client, config, *all).await,
        EventAction::Get { event_id } => get(client, config, event_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, all: bool) -> Result<()> {
    debug!(all, "event list");
    if all {
        let mut stdout = std::io::stdout();
        client.stream_all("/events", &mut stdout).await?;
    } else {
        let items = client.get_all("/events").await?;
        let result = json!({ "data": items });
        print_value(&result, &config.output_format);
    }
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, event_id: &str) -> Result<()> {
    debug!(event_id, "event get");
    let resp = client.get(&format!("/events/{}", event_id)).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[cfg(test)]
mod tests;
