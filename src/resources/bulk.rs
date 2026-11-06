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

/// Where a bulk op keeps its rows and what makes one fail.
#[derive(Debug, Clone, Copy)]
pub struct Rule {
    /// The response key holding one row per requested item.
    pub rows_key: &'static str,
    pub failed: FailedFn,
}

/// Monitoring-test exclusions: rows under `data`, each with a per-item `statusCode`.
pub const STATUS_CODE: Rule = Rule {
    rows_key: "data",
    failed: status_code_failed,
};

/// Compliance-check exclusions: rows under `results`, each with an `outcome`.
pub const OUTCOME: Rule = Rule {
    rows_key: "results",
    failed: outcome_failed,
};

/// DCF suggestion generation: rows under `data`, each with a `status` of
/// `STARTED`, `SKIPPED` or `FAILED`.
pub const GENERATE_STATUS: Rule = Rule {
    rows_key: "data",
    failed: generate_failed,
};

/// DCF suggestion acceptance: rows under `data`, each with an `error` that is
/// null when the request was processed.
pub const ACCEPT_ERROR: Rule = Rule {
    rows_key: "data",
    failed: error_failed,
};

/// A row whose `status` is `FAILED` failed; one without a `status` failed too.
/// `SKIPPED` (a run is already going) is not a failure. Pure.
pub fn generate_failed(row: &Value) -> bool {
    row.get("status")
        .and_then(Value::as_str)
        .is_none_or(|status| status == "FAILED")
}

/// A row with a non-null `error` failed; a row with no `error` key failed too,
/// since the spec requires the key. Pure.
pub fn error_failed(row: &Value) -> bool {
    row.get("error").is_none_or(|error| !error.is_null())
}

/// A row whose `outcome` is `FAILED_*` failed; one without an `outcome` failed
/// too. `SKIPPED_*` is not a failure: the server reports an item it left
/// unchanged (already excluded, no such exclusion, outside the scope) next to
/// the ones it changed, and does not fail the batch for it. Pure.
pub fn outcome_failed(row: &Value) -> bool {
    row.get("outcome")
        .and_then(Value::as_str)
        .is_none_or(|outcome| outcome.starts_with("FAILED_"))
}

/// A row whose per-item HTTP `statusCode` is 400 or above failed. The
/// monitoring-test exclusion ops count a skip (already excluded, no such
/// exclusion) as 409 or 422, the same as the whole-request status the server
/// answers when every item is skipped.
pub fn status_code_failed(row: &Value) -> bool {
    row.get("statusCode")
        .and_then(Value::as_u64)
        .is_none_or(|code| code >= 400)
}

/// The rows of a per-item response. Anything without the rule's rows array is
/// a loud error: an empty result would hide what happened to each item. Pure.
pub fn rows<'a>(value: &'a Value, rule: &Rule) -> Result<&'a Vec<Value>> {
    match value.get(rule.rows_key).and_then(Value::as_array) {
        Some(rows) => Ok(rows),
        None => bail!("expected per-item results under `{}`, got: {}", rule.rows_key, value),
    }
}

/// `Err` naming how many rows failed, `Ok` when none did. `http_status` is the
/// non-2xx the rows arrived with, if any; it fails the command even if no row
/// reports a failure. Pure.
pub fn check(value: &Value, rule: &Rule, http_status: Option<u16>) -> Result<()> {
    let rows = rows(value, rule)?;
    let failures = rows.iter().filter(|r| (rule.failed)(r)).count();
    match (failures, http_status) {
        (0, None) => Ok(()),
        (n, None) => bail!("{n} of {} items failed", rows.len()),
        (n, Some(status)) => bail!("HTTP {status}: {n} of {} items failed", rows.len()),
    }
}

/// The per-item body of a non-2xx answer, when it carries one.
fn rows_of_error(err: &eyre::Report, rule: &Rule) -> Option<(u16, Value)> {
    let api = err.downcast_ref::<ApiError>()?;
    let body: Value = serde_json::from_str(&api.body).ok()?;
    body.get(rule.rows_key).and_then(Value::as_array)?;
    Some((api.status.as_u16(), body))
}

/// POST a bulk body, print every result row, then fail if any row failed.
#[instrument(skip(client, config, body))]
pub async fn post(client: &DrataClient, config: &Config, path: &str, body: Value, rule: Rule) -> Result<()> {
    debug!(path, "bulk post");
    let (value, http_status) = match client.post(path, body).await {
        Ok(value) => (value, None),
        Err(err) => match rows_of_error(&err, &rule) {
            Some((status, value)) => (value, Some(status)),
            None => return Err(err),
        },
    };
    print_value(&value, &config.output_format);
    let result = check(&value, &rule, http_status);
    debug!(path, ok = result.is_ok(), "bulk post complete");
    result
}

#[cfg(test)]
mod tests;
