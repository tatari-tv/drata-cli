//! `drata safebase <noun> <verb>` - the curated SafeBase reads and writes.
//!
//! SafeBase answers `{ok, result}` (`{ok}` alone on a few writes). The client
//! has already checked the envelope, so a handler reads `result` and renders it
//! with a named table (`output::Table`): the row shapes are SafeBase's own and
//! share keys with Drata rows, so none of them is sniffed. A list prints
//! `{"data": [...]}` like the Drata lists; a `get` prints the one object.
//!
//! The three paged lists (accounts, documents, KB search) fetch every page by
//! default and stream NDJSON under `--all`, the Drata list contract
//! (`resources::vendor`). The rest return the whole set in `result`.
//!
//! Every write confirms first (`ConfirmFn`) and goes through the client's
//! write guard. SafeBase's emails are opt-in: a write sends `message` only
//! under `--message` or `--notify` (`notice_message`).

use crate::cli::{EmailNotice, SafebaseAction};
use crate::client::DrataClient;
use crate::client::pages::PagedOp;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::output::{Table, print_value_as, render_text};
use eyre::{Result, bail};
use serde_json::{Value, json};
use std::io::{self, Write};
use tracing::{debug, instrument};

mod account;
mod document;
mod kb;
mod member;
mod request;

pub use document::upload;

/// What a curated read produced, before anything is printed.
#[derive(Debug)]
pub enum Reply {
    /// An `{ok}`-only response.
    Ok,
    /// One plain line (the account URL).
    Text(String),
    /// A list of `table` rows.
    Rows { table: Table, rows: Vec<Value> },
    /// A single object shown as one `table` row.
    One { table: Table, row: Value },
    /// A single row that came with a warning for stderr (an upload whose
    /// final step timed out but whose document was then found).
    Warned { warning: String, table: Table, row: Value },
    /// `--all`: this many items were already written to the sink as NDJSON.
    Streamed(u64),
}

impl Reply {
    /// The JSON the command prints in JSON mode.
    pub fn value(&self) -> Value {
        match self {
            Reply::Ok => json!({"ok": true}),
            Reply::Text(t) => json!(t),
            Reply::Rows { rows, .. } => json!({ "data": rows }),
            Reply::One { row, .. } | Reply::Warned { row, .. } => row.clone(),
            Reply::Streamed(n) => json!(n),
        }
    }

    /// The table the command prints in table mode, `None` for replies that
    /// are not a table.
    pub fn table_text(&self, width: usize) -> Option<String> {
        match self {
            Reply::Rows { table, .. } | Reply::One { table, .. } | Reply::Warned { table, .. } => {
                render_text(&self.value(), width, *table)
            }
            _ => None,
        }
    }
}

/// Route a curated SafeBase command and print its reply. Login, logout and
/// raw never reach here.
pub async fn handle(action: &SafebaseAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    let reply = execute(action, client, confirm, &mut io::stdout()).await?;
    match reply {
        Reply::Ok => println!("ok"),
        Reply::Text(line) => println!("{line}"),
        Reply::Warned { ref warning, table, .. } => {
            eprintln!("warning: {warning}");
            print_value_as(&reply.value(), &config.output_format, table)
        }
        Reply::Rows { table, .. } | Reply::One { table, .. } => {
            print_value_as(&reply.value(), &config.output_format, table)
        }
        Reply::Streamed(_) => {}
    }
    Ok(())
}

/// Run a curated read or write against the API. Writes ask `confirm` first.
/// `--all` lists write NDJSON to `stream`; everything else comes back as data.
pub async fn execute<W: Write>(
    action: &SafebaseAction,
    client: &DrataClient,
    confirm: &ConfirmFn,
    stream: &mut W,
) -> Result<Reply> {
    match action {
        SafebaseAction::Account { action } => account::fetch(action, client, stream).await,
        SafebaseAction::Member { action } => member::execute(action, client, confirm).await,
        SafebaseAction::Document { action } => document::execute(action, client, confirm, stream).await,
        SafebaseAction::Request { action } => request::execute(action, client, confirm).await,
        SafebaseAction::Kb { action } => kb::execute(action, client, confirm, stream).await,
        SafebaseAction::Login { .. } | SafebaseAction::Logout | SafebaseAction::Raw(_) => {
            bail!("safebase::execute called on a command that is not a curated verb")
        }
    }
}

/// Ask `confirm` before a write; a "no" is an `aborted` error, never a send.
fn confirmed(confirm: &ConfirmFn, method: &str, path: &str) -> Result<()> {
    if !confirm(method, path)? {
        bail!("aborted");
    }
    Ok(())
}

/// The `message` field for an opt-in SafeBase email: the text under
/// `--message`, `true` under `--notify` (the spec's "email with no custom
/// message"), `None` (omit the field, no email) otherwise. Pure.
fn notice_message(notice: &EmailNotice) -> Option<Value> {
    match (&notice.message, notice.notify) {
        (Some(text), _) => Some(json!(text)),
        (None, true) => Some(json!(true)),
        (None, false) => None,
    }
}

/// A response's `result` as `table` rows: `ok` when it has none (an
/// `{ok}`-only body), a list for an array, one row for an object.
fn reply_from(resp: &Value, table: Table) -> Reply {
    match resp.get("result") {
        None => Reply::Ok,
        Some(Value::Array(rows)) => Reply::Rows {
            table,
            rows: rows.clone(),
        },
        Some(one) => Reply::One {
            table,
            row: one.clone(),
        },
    }
}

/// The array at `pointer` inside a response's `result`, or an error: a 2xx
/// whose `result` lacks the list is a broken response, never an empty list.
fn items_at<'a>(resp: &'a Value, pointer: &str, op: &str) -> Result<&'a Vec<Value>> {
    resp.get("result")
        .and_then(|r| r.pointer(pointer))
        .and_then(Value::as_array)
        .ok_or_else(|| eyre::eyre!("{op}: response has no `result{pointer}` array: {resp}"))
}

/// A paged list: every page buffered as `table` rows, or under `all`
/// streamed to `stream` as NDJSON.
#[instrument(skip(client, stream))]
async fn list_pages<W: Write>(
    client: &DrataClient,
    op: &PagedOp,
    path: &str,
    all: bool,
    table: Table,
    stream: &mut W,
) -> Result<Reply> {
    debug!(path, all, "safebase paged list");
    if all {
        Ok(Reply::Streamed(client.stream_all_pages(op, path, stream).await?))
    } else {
        Ok(Reply::Rows {
            table,
            rows: client.get_all_pages(op, path).await?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notice_message_is_opt_in() {
        let none = EmailNotice::default();
        assert_eq!(notice_message(&none), None);
        let notify = EmailNotice {
            message: None,
            notify: true,
        };
        assert_eq!(notice_message(&notify), Some(json!(true)));
        let text = EmailNotice {
            message: Some("welcome".to_string()),
            notify: false,
        };
        assert_eq!(notice_message(&text), Some(json!("welcome")));
    }

    #[test]
    fn warned_reply_prints_the_row_like_one() {
        let row = json!({"id": "d-1", "name": "SOC 2"});
        let warned = Reply::Warned {
            warning: "w".to_string(),
            table: Table::SafebaseDocuments,
            row: row.clone(),
        };
        assert_eq!(warned.value(), row);
        let text = warned.table_text(120).expect("a table");
        assert!(text.contains("d-1") && text.contains("SOC 2"), "{text}");
    }
}
