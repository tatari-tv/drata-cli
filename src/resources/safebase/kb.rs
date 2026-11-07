//! `drata safebase kb search`.

use super::{Reply, list_pages};
use crate::cli::SafebaseKbAction;
use crate::client::{DrataClient, pages::KB_SEARCH};
use crate::output::Table;
use crate::query::QueryPath;
use eyre::Result;
use std::io::Write;
use tracing::instrument;

#[instrument(skip(client, stream))]
pub async fn fetch<W: Write>(action: &SafebaseKbAction, client: &DrataClient, stream: &mut W) -> Result<Reply> {
    match action {
        SafebaseKbAction::Search { query, all } => {
            let path = search_path(query.as_deref());
            list_pages(client, &KB_SEARCH, &path, *all, Table::SafebaseKbEntries, stream).await
        }
    }
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
}
