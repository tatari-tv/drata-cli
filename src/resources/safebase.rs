//! `drata safebase <noun> <verb>` - the curated SafeBase reads.
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

use crate::cli::SafebaseAction;
use crate::client::DrataClient;
use crate::client::pages::PagedOp;
use crate::config::Config;
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
            Reply::One { row, .. } => row.clone(),
            Reply::Streamed(n) => json!(n),
        }
    }

    /// The table the command prints in table mode, `None` for replies that
    /// are not a table.
    pub fn table_text(&self, width: usize) -> Option<String> {
        match self {
            Reply::Rows { table, .. } | Reply::One { table, .. } => render_text(&self.value(), width, *table),
            _ => None,
        }
    }
}

/// Route a curated SafeBase read and print its reply. Login, logout and raw
/// never reach here.
pub async fn handle(action: &SafebaseAction, client: &DrataClient, config: &Config) -> Result<()> {
    let reply = fetch(action, client, &mut io::stdout()).await?;
    match reply {
        Reply::Ok => println!("ok"),
        Reply::Text(line) => println!("{line}"),
        Reply::Rows { table, .. } | Reply::One { table, .. } => {
            print_value_as(&reply.value(), &config.output_format, table)
        }
        Reply::Streamed(_) => {}
    }
    Ok(())
}

/// Run a curated read against the API. `--all` lists write NDJSON to
/// `stream`; everything else comes back as data.
pub async fn fetch<W: Write>(action: &SafebaseAction, client: &DrataClient, stream: &mut W) -> Result<Reply> {
    match action {
        SafebaseAction::Account { action } => account::fetch(action, client, stream).await,
        SafebaseAction::Member { action } => member::fetch(action, client).await,
        SafebaseAction::Document { action } => document::fetch(action, client, stream).await,
        SafebaseAction::Request { action } => request::fetch(action, client).await,
        SafebaseAction::Kb { action } => kb::fetch(action, client, stream).await,
        SafebaseAction::Login { .. } | SafebaseAction::Logout | SafebaseAction::Raw(_) => {
            bail!("safebase::fetch called on a command that is not a curated read")
        }
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
