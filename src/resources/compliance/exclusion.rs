//! Compliance-check exclusion bodies and the list path. Pure.

use crate::cli::{ComplianceCheckExclusionExpand, ComplianceCheckType};
use crate::query::{QueryPath, wire_name};
use eyre::{Result, bail};
use serde_json::{Value, json};

/// The spec's `maxItems` on `targetIds`.
pub const MAX_TARGETS: usize = 500;

/// The spec's `maxLength` on `reason`.
const MAX_REASON: usize = 30000;

/// Personnel IDs: the spec's `^[1-9]\d*$`.
fn check_target_ids(ids: &[String]) -> Result<()> {
    if ids.is_empty() || ids.len() > MAX_TARGETS {
        bail!("--target-id takes 1 to {MAX_TARGETS} personnel IDs, got {}", ids.len());
    }
    if let Some(bad) = ids
        .iter()
        .find(|id| id.is_empty() || id.starts_with('0') || !id.bytes().all(|b| b.is_ascii_digit()))
    {
        bail!("--target-id `{bad}` is not a personnel ID (a positive whole number)");
    }
    Ok(())
}

fn check_types(checks: &[ComplianceCheckType]) -> Result<Vec<String>> {
    if checks.is_empty() {
        bail!("--compliance-check takes at least one compliance check");
    }
    Ok(checks.iter().map(wire_name).collect())
}

/// `POST .../compliance-check-exclusions` body. `reason` and `start_date` are
/// required by the spec; `endDate` is sent only when given (omitted means
/// open-ended).
pub fn build_create_body(
    target_ids: &[String],
    checks: &[ComplianceCheckType],
    reason: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> Result<Value> {
    check_target_ids(target_ids)?;
    let check_names = check_types(checks)?;
    let reason = reason.ok_or_else(|| eyre::eyre!("requires --reason (or use --data/--example)"))?;
    if reason.chars().count() > MAX_REASON {
        bail!("--reason takes at most {MAX_REASON} characters");
    }
    let start_date = start_date.ok_or_else(|| eyre::eyre!("requires --start-date (or use --data/--example)"))?;
    let mut body = json!({
        "targetIds": target_ids,
        "complianceCheckTypes": check_names,
        "reason": reason,
        "startDate": start_date,
    });
    if let Some(end) = end_date {
        body["endDate"] = json!(end);
    }
    Ok(body)
}

/// `POST .../action-delete` body.
pub fn build_delete_body(target_ids: &[String], checks: &[ComplianceCheckType]) -> Result<Value> {
    check_target_ids(target_ids)?;
    Ok(json!({ "targetIds": target_ids, "complianceCheckTypes": check_types(checks)? }))
}

/// `GET .../compliance-check-exclusions` with the `complianceCheck` and
/// `expand[]` filters.
pub fn list_path(
    workspace_id: &str,
    check: Option<&ComplianceCheckType>,
    expand: &[ComplianceCheckExclusionExpand],
) -> String {
    let mut q = QueryPath::new(super::collection_path(workspace_id));
    q.push_opt("complianceCheck", check.map(wire_name))
        .push_each("expand[]", expand.iter().map(wire_name));
    q.finish()
}

#[cfg(test)]
mod tests;
