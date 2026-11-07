//! `drata safebase kb search | add | update`.

use super::{Reply, confirmed, list_pages, reply_from};
use crate::cli::SafebaseKbAction;
use crate::client::{DrataClient, encode_path_segment, pages::KB_SEARCH};
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::Table;
use crate::query::QueryPath;
use eyre::{Result, bail};
use serde_json::Value;
use std::io::Write;
use tracing::{debug, instrument};

const ENTRIES_PATH: &str = "/kb/entries";

#[instrument(skip(client, confirm, stream))]
pub async fn execute<W: Write>(
    action: &SafebaseKbAction,
    client: &DrataClient,
    confirm: &ConfirmFn,
    stream: &mut W,
) -> Result<Reply> {
    match action {
        SafebaseKbAction::Search { query, all } => {
            let path = search_path(query.as_deref());
            list_pages(client, &KB_SEARCH, &path, *all, Table::SafebaseKbEntries, stream).await
        }
        SafebaseKbAction::Add { data } => {
            let body = object_body(data)?;
            debug!("safebase kb add");
            confirmed(confirm, "POST", ENTRIES_PATH)?;
            let resp = client.post(ENTRIES_PATH, body).await?;
            Ok(reply_from(&resp, Table::SafebaseKbEntries))
        }
        SafebaseKbAction::Update { id, data } => {
            let body = object_body(data)?;
            let path = entry_path(id);
            debug!(path, "safebase kb update");
            confirmed(confirm, "PATCH", &path)?;
            let resp = client.patch(&path, body).await?;
            Ok(reply_from(&resp, Table::SafebaseKbEntries))
        }
    }
}

/// A `--data` body, which must be a JSON object: both KB writes take one.
fn object_body(data: &str) -> Result<Value> {
    let body = read_data(data, DataSource::Any)?;
    if !body.is_object() {
        bail!("--data must be a JSON object, got: {body}");
    }
    Ok(body)
}

/// `/kb/entries/{id}`. Pure.
fn entry_path(id: &str) -> String {
    format!("{ENTRIES_PATH}/{}", encode_path_segment(id))
}

/// `GET /kb/search` path with the optional `query`. Pure.
fn search_path(query: Option<&str>) -> String {
    let mut q = QueryPath::new(KB_SEARCH.path);
    q.push_opt("query", query);
    q.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_path_adds_query_only_when_given() {
        assert_eq!(search_path(None), "/kb/search");
        assert_eq!(search_path(Some("soc 2")), "/kb/search?query=soc%202");
    }

    #[test]
    fn entry_path_encodes_the_id() {
        assert_eq!(entry_path("k-1"), "/kb/entries/k-1");
        assert_eq!(entry_path("k/1"), "/kb/entries/k%2F1");
    }

    #[test]
    fn object_body_rejects_non_objects() {
        assert!(object_body(r#"{"question": "q"}"#).is_ok());
        let err = object_body("[1]").expect_err("array is not an object");
        assert!(err.to_string().contains("must be a JSON object"), "{err}");
    }
}
