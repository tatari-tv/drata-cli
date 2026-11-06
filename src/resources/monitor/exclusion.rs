//! Monitoring-test exclusions and codebase exclusions: the bulk create and
//! `action-delete` bodies and the codebase list path. Pure.

use crate::cli::CodebaseExclusionExpand;
use crate::query::{QueryPath, wire_name};
use eyre::{Result, bail};
use serde_json::{Value, json};

/// The spec's `maxItems` on every exclusion request list.
pub const MAX_ITEMS: usize = 300;

fn check_count(flag: &str, n: usize) -> Result<()> {
    if n == 0 || n > MAX_ITEMS {
        bail!("{flag} takes 1 to {MAX_ITEMS} items, got {n}");
    }
    Ok(())
}

fn require_reason(reason: Option<&str>) -> Result<&str> {
    reason.ok_or_else(|| eyre::eyre!("requires --reason (or use --data/--example)"))
}

/// One `--failure <id> <connection-id>` pair as a `MonitorExclusionTargetRequestPublicV2Dto`.
/// `connectionId` is required and nullable: `null` sends JSON null.
fn failure_target(id: &str, connection_id: &str) -> Result<Value> {
    let connection_id = if connection_id.eq_ignore_ascii_case("null") {
        Value::Null
    } else {
        match connection_id.parse::<u64>() {
            Ok(n) => json!(n),
            Err(_) => bail!("--failure {id} {connection_id}: the connection ID must be a number or `null`"),
        }
    };
    Ok(json!({ "id": id, "connectionId": connection_id }))
}

/// `POST .../exclusions` body from `--reason` and the flat `--failure` pairs
/// (clap collects `num_args = 2` values in order).
pub fn build_create_body(reason: Option<&str>, failures: &[String]) -> Result<Value> {
    let reason = require_reason(reason)?;
    if !failures.len().is_multiple_of(2) {
        bail!("--failure takes two values: <id> <connection-id>");
    }
    check_count("--failure", failures.len() / 2)?;
    let exclusions = failures
        .chunks(2)
        .map(|pair| failure_target(&pair[0], &pair[1]))
        .collect::<Result<Vec<_>>>()?;
    Ok(json!({ "reason": reason, "exclusions": exclusions }))
}

/// `POST .../codebase-exclusions` body.
pub fn build_codebase_create_body(reason: Option<&str>, finding_ids: &[String]) -> Result<Value> {
    let reason = require_reason(reason)?;
    check_count("--finding-id", finding_ids.len())?;
    Ok(json!({ "reason": reason, "findingIds": finding_ids }))
}

/// `POST .../action-delete` body, shared by exclusions and codebase exclusions
/// (both take `MonitorExclusionsDeleteRequestPublicV2Dto`).
pub fn build_delete_body(exclusion_ids: &[u64]) -> Result<Value> {
    check_count("--exclusion-id", exclusion_ids.len())?;
    Ok(json!({ "exclusionIds": exclusion_ids }))
}

/// `GET .../codebase-exclusions` with the `repositoryName` and `expand[]` filters.
pub fn build_codebase_list_path(
    collection: &str,
    repository_name: Option<&str>,
    expand: &[CodebaseExclusionExpand],
) -> String {
    let mut q = QueryPath::new(collection);
    q.push_opt("repositoryName", repository_name)
        .push_each("expand[]", expand.iter().map(wire_name));
    q.finish()
}
