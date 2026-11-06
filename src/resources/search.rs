//! Shared `search` verb behavior for the `*-search` ops (`risk search`, and
//! later `vendor search` and `personnel search`).
//!
//! Default: fetch every page and print rows like a list. With facets: fetch a
//! single page with `facets[]` set and print the whole `{data, pagination,
//! facets}` envelope. The envelope is always printed as JSON because the table
//! renderer shows only `data[]` and would drop the facet counts.

use crate::cli::OutputFormat;
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use crate::query::QueryPath;
use eyre::Result;
use serde_json::json;
use tracing::{debug, instrument};

/// Append `facets[]=<name>` once per facet. Pure.
pub fn facet_path(mut query: QueryPath, facets: &[String]) -> String {
    query.push_each("facets[]", facets);
    query.finish()
}

/// Run a search over `query`. Every page and rows when `facets` is empty; one
/// page and the whole envelope when it is not.
#[instrument(skip(client, config, query))]
pub async fn run(client: &DrataClient, config: &Config, query: QueryPath, facets: &[String]) -> Result<()> {
    if facets.is_empty() {
        let items = client.get_all(&query.finish()).await?;
        debug!(rows = items.len(), "search: all pages");
        print_value(&json!({ "data": items }), &config.output_format);
    } else {
        debug!(?facets, "search: single page with facets");
        let envelope = client.get(&facet_path(query, facets)).await?;
        print_value(&envelope, &OutputFormat::Json);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
