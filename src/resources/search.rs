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
use serde_json::{Value, json};
use tracing::{debug, instrument};

/// Append `facets[]=<name>` once per facet. Pure.
pub fn facet_path(mut query: QueryPath, facets: &[String]) -> String {
    query.push_each("facets[]", facets);
    query.finish()
}

/// Fetch a search over `query`. Every page as `{data: rows}` when `facets` is
/// empty; one page and the whole `{data, pagination, facets}` envelope when it
/// is not.
#[instrument(skip(client, query))]
pub async fn fetch(client: &DrataClient, query: QueryPath, facets: &[String]) -> Result<Value> {
    if facets.is_empty() {
        let items = client.get_all(&query.finish()).await?;
        debug!(rows = items.len(), "search: all pages");
        Ok(json!({ "data": items }))
    } else {
        debug!(?facets, "search: single page with facets");
        client.get(&facet_path(query, facets)).await
    }
}

/// Run a search and print it: rows in the configured format, or (with facets)
/// the whole envelope as JSON.
#[instrument(skip(client, config, query))]
pub async fn run(client: &DrataClient, config: &Config, query: QueryPath, facets: &[String]) -> Result<()> {
    let value = fetch(client, query, facets).await?;
    let format = if facets.is_empty() { &config.output_format } else { &OutputFormat::Json };
    print_value(&value, format);
    Ok(())
}

#[cfg(test)]
mod tests;
