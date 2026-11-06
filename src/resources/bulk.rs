//! Per-item results inside one response: bulk ops answer `{"data": [row, ...]}`
//! with one row per requested item, each carrying its own outcome.
//!
//! The rule (plan Overview): print every row, then exit 1 if any row reports a
//! failure status. Some bulk ops answer a non-2xx (409, 422, 500) when no item
//! succeeded, still with every row; those rows print too, and the command fails
//! naming the HTTP status.

use crate::client::{ApiError, DrataClient};
use crate::config::Config;
use crate::output::print_value;
use eyre::{Result, bail};
use serde_json::Value;
use tracing::{debug, instrument};

/// Decides whether one result row failed.
pub type FailedFn = fn(&Value) -> bool;

/// A row whose per-item HTTP `statusCode` is 400 or above failed. The
/// monitoring-test exclusion ops count a skip (already excluded, no such
/// exclusion) as 409 or 422, the same as the whole-request status the server
/// answers when every item is skipped.
pub fn status_code_failed(row: &Value) -> bool {
    row.get("statusCode")
        .and_then(Value::as_u64)
        .is_none_or(|code| code >= 400)
}

/// The rows of a per-item response. Anything without a `data` array is a loud
/// error: an empty result would hide what happened to each item. Pure.
pub fn rows(value: &Value) -> Result<&Vec<Value>> {
    match value.get("data").and_then(Value::as_array) {
        Some(rows) => Ok(rows),
        None => bail!("expected per-item results under `data`, got: {}", value),
    }
}

/// `Err` naming how many rows failed, `Ok` when none did. `http_status` is the
/// non-2xx the rows arrived with, if any; it fails the command even if no row
/// reports a failure. Pure.
pub fn check(value: &Value, failed: FailedFn, http_status: Option<u16>) -> Result<()> {
    let rows = rows(value)?;
    let failures = rows.iter().filter(|r| failed(r)).count();
    match (failures, http_status) {
        (0, None) => Ok(()),
        (n, None) => bail!("{n} of {} items failed", rows.len()),
        (n, Some(status)) => bail!("HTTP {status}: {n} of {} items failed", rows.len()),
    }
}

/// The per-item body of a non-2xx answer, when it carries one.
fn rows_of_error(err: &eyre::Report) -> Option<(u16, Value)> {
    let api = err.downcast_ref::<ApiError>()?;
    let body: Value = serde_json::from_str(&api.body).ok()?;
    body.get("data").and_then(Value::as_array)?;
    Some((api.status.as_u16(), body))
}

/// POST a bulk body, print every result row, then fail if any row failed.
#[instrument(skip(client, config, body, failed))]
pub async fn post(client: &DrataClient, config: &Config, path: &str, body: Value, failed: FailedFn) -> Result<()> {
    debug!(path, "bulk post");
    let (value, http_status) = match client.post(path, body).await {
        Ok(value) => (value, None),
        Err(err) => match rows_of_error(&err) {
            Some((status, value)) => (value, Some(status)),
            None => return Err(err),
        },
    };
    print_value(&value, &config.output_format);
    let result = check(&value, failed, http_status);
    debug!(path, ok = result.is_ok(), "bulk post complete");
    result
}

#[cfg(test)]
mod tests;
