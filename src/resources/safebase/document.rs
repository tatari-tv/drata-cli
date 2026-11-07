//! `drata safebase document list | get | types | upload`.

use super::{Reply, items_at, list_pages, reply_from};
use crate::cli::SafebaseDocumentAction;
use crate::client::{DrataClient, encode_path_segment, pages::DOCUMENTS};
use crate::confirm::ConfirmFn;
use crate::output::Table;
use crate::query::QueryPath;
use eyre::Result;
use std::io::Write;
use tracing::{debug, instrument};

pub mod upload;

#[instrument(skip(client, confirm, stream))]
pub async fn execute<W: Write>(
    action: &SafebaseDocumentAction,
    client: &DrataClient,
    confirm: &ConfirmFn,
    stream: &mut W,
) -> Result<Reply> {
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
        SafebaseDocumentAction::Upload(args) => upload::run(args, client, confirm).await,
    }
}

/// `GET /documents` path with the optional `query` filter. Pure.
fn list_path(query: Option<&str>) -> String {
    let mut q = QueryPath::new(DOCUMENTS.path);
    q.push_opt("query", query);
    q.finish()
}

/// `GET /document/{id}` path; `external` adds the spec's `type=external`
/// lookup (`type` defaults to `document`). A UUID external id is sent in the
/// lowercase hyphenated form `upload --external-id` stores, so the lookup
/// matches whatever case was typed. Pure.
fn get_path(id: &str, external: bool) -> String {
    let id = match uuid::Uuid::parse_str(id) {
        Ok(u) if external => u.hyphenated().to_string(),
        _ => id.to_string(),
    };
    let mut q = QueryPath::new(format!("/document/{}", encode_path_segment(&id)));
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

    #[test]
    fn get_path_lowercases_a_uuid_external_id_only() {
        let upper = "7E3B1C2A-9D4F-4A6B-8C1D-2E3F4A5B6C7D";
        assert_eq!(
            get_path(upper, true),
            "/document/7e3b1c2a-9d4f-4a6b-8c1d-2e3f4a5b6c7d?type=external"
        );
        assert_eq!(get_path(upper, false), format!("/document/{upper}"));
        assert_eq!(get_path("Not-A-Uuid", true), "/document/Not-A-Uuid?type=external");
    }
}
