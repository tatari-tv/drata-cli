//! `drata safebase document list | get | types`.

use super::{Reply, items_at, list_pages, reply_from};
use crate::cli::SafebaseDocumentAction;
use crate::client::{DrataClient, encode_path_segment, pages::DOCUMENTS};
use crate::output::Table;
use crate::query::QueryPath;
use eyre::Result;
use std::io::Write;
use tracing::{debug, instrument};

#[instrument(skip(client, stream))]
pub async fn fetch<W: Write>(action: &SafebaseDocumentAction, client: &DrataClient, stream: &mut W) -> Result<Reply> {
    match action {
        SafebaseDocumentAction::List { all, query } => {
            let path = list_path(query.as_deref());
            list_pages(client, &DOCUMENTS, &path, *all, Table::SafebaseDocuments, stream).await
        }
        SafebaseDocumentAction::Get { id, external } => {
            let path = get_path(id, *external);
            debug!(path, "safebase document get");
            let resp = client.get(&path).await?;
            Ok(reply_from(&resp, Table::SafebaseDocuments))
        }
        SafebaseDocumentAction::Types => {
            let resp = client.get("/document-types").await?;
            Ok(Reply::Rows {
                table: Table::SafebaseDocumentTypes,
                rows: items_at(&resp, "/items", "document types")?.clone(),
            })
        }
    }
}

/// `GET /documents` path with the optional `query` filter. Pure.
fn list_path(query: Option<&str>) -> String {
    let mut q = QueryPath::new(DOCUMENTS.path);
    q.push_opt("query", query);
    q.finish()
}

/// `GET /document/{id}` path; `external` adds the spec's `type=external`
/// lookup (`type` defaults to `document`). Pure.
fn get_path(id: &str, external: bool) -> String {
    let mut q = QueryPath::new(format!("/document/{}", encode_path_segment(id)));
    if external {
        q.push("type", "external");
    }
    q.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_path_adds_query_only_when_given() {
        assert_eq!(list_path(None), "/documents");
        assert_eq!(list_path(Some("soc 2")), "/documents?query=soc%202");
    }

    #[test]
    fn get_path_looks_up_by_external_id_only_on_request() {
        assert_eq!(get_path("d1", false), "/document/d1");
        assert_eq!(get_path("d1", true), "/document/d1?type=external");
        assert_eq!(get_path("a/b", false), "/document/a%2Fb");
    }
}
