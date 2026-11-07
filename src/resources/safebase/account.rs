//! `drata safebase account list | get | url`.

use super::{Reply, list_pages, reply_from};
use crate::cli::SafebaseAccountAction;
use crate::client::{DrataClient, encode_path_segment, pages::ACCOUNTS};
use crate::output::Table;
use eyre::{Result, bail};
use std::io::Write;
use tracing::{debug, instrument};

#[instrument(skip(client, stream))]
pub async fn fetch<W: Write>(action: &SafebaseAccountAction, client: &DrataClient, stream: &mut W) -> Result<Reply> {
    match action {
        SafebaseAccountAction::List { all } => {
            list_pages(client, &ACCOUNTS, ACCOUNTS.path, *all, Table::SafebaseAccounts, stream).await
        }
        SafebaseAccountAction::Get { id } => {
            debug!(id, "safebase account get");
            let resp = client.get(&account_path(id)).await?;
            Ok(reply_from(&resp, Table::SafebaseAccounts))
        }
        SafebaseAccountAction::Url { id } => {
            debug!(id, "safebase account url");
            let resp = client.get(&format!("{}/url", account_path(id))).await?;
            match resp.get("result").and_then(|r| r.as_str()) {
                Some(url) => Ok(Reply::Text(url.to_string())),
                None => bail!("account url: response has no string `result`: {resp}"),
            }
        }
    }
}

/// `GET /accounts/{id}` path. Pure.
fn account_path(id: &str) -> String {
    format!("/accounts/{}", encode_path_segment(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_path_encodes_the_id_as_one_segment() {
        assert_eq!(account_path("abc"), "/accounts/abc");
        assert_eq!(account_path("a/b?c"), "/accounts/a%2Fb%3Fc");
    }
}
