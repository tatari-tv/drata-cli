//! Table rendering for known Drata response shapes.
//!
//! Unlike PagerDuty, Drata list endpoints share one generic envelope:
//! `{ "data": [...], "pagination": {...} }`. So we cannot dispatch on the
//! envelope key. Instead, the dispatcher peeks at the fields present on the
//! first row to recognize a resource (a vendor has `category`/`risk`; a
//! questionnaire has `recipientEmail`/`isCompleted`). Single-object `get`
//! responses are wrapped as a one-row list. Unknown shapes return `None` so the
//! caller falls back to pretty JSON.
//!
//! Each renderer declares its `&["HEADERS"]` plus a slice of field-extractor
//! closures. The shrink-to-width engine and char-boundary-safe truncation are
//! lifted from `pagerduty-cli`.

use serde_json::Value;
use std::fmt::Write;

/// Default width used when the terminal size can't be detected.
pub const DEFAULT_WIDTH: usize = 120;

type RowRenderer = fn(&[Value], usize) -> String;

/// Dispatch on the detected shape of a Drata response. Returns `Some(table)`
/// when a renderer matches, `None` otherwise (caller falls back to JSON).
pub fn render(value: &Value, width: usize) -> Option<String> {
    // List envelope: `{ "data": [ {...}, ... ] }`.
    if let Some(arr) = value.get("data").and_then(|v| v.as_array()) {
        if arr.is_empty() {
            // Render an empty table rather than dumping `[]` as JSON.
            return Some("(no results)".to_string());
        }
        let renderer = pick_renderer(&arr[0])?;
        return Some(renderer(arr, width));
    }

    // Single-object response (e.g. `get`): wrap as a one-row list.
    if value.is_object() {
        let renderer = pick_renderer(value)?;
        let rows = [value.clone()];
        return Some(renderer(&rows, width));
    }

    None
}

/// Choose a renderer by sniffing the fields present on a sample row.
fn pick_renderer(sample: &Value) -> Option<RowRenderer> {
    let obj = sample.as_object()?;
    // Questionnaire rows carry `recipientEmail` + `isCompleted`.
    if obj.contains_key("recipientEmail") || obj.contains_key("isCompleted") {
        return Some(render_questionnaires);
    }
    // Vendor rows carry `category` + `risk` (plus `name`/`status`).
    if obj.contains_key("category") && obj.contains_key("risk") {
        return Some(render_vendors);
    }
    // Risk rows carry `treatmentPlan` + `riskId`.
    if obj.contains_key("treatmentPlan") || obj.contains_key("riskId") {
        return Some(render_risks);
    }
    // Control rows carry `code` + `question` (or just `code`).
    if obj.contains_key("code") && (obj.contains_key("question") || obj.contains_key("activity")) {
        return Some(render_controls);
    }
    // Device rows carry `serialNumber` or `isDeviceCompliant`.
    if obj.contains_key("serialNumber") || obj.contains_key("isDeviceCompliant") {
        return Some(render_devices);
    }
    // Personnel rows carry `employmentStatus`.
    if obj.contains_key("employmentStatus") {
        return Some(render_personnel);
    }
    // Policy rows carry `currentVersionId` or `scope`.
    if obj.contains_key("currentVersionId") || obj.contains_key("scope") {
        return Some(render_policies);
    }
    // Evidence library rows carry `evidenceTemplateCode`.
    if obj.contains_key("evidenceTemplateCode") || obj.contains_key("implementationGuidance") {
        return Some(render_evidence);
    }
    // Framework rows carry `numInScopeControls` or `shortName` (tag is unique to frameworks).
    if obj.contains_key("numInScopeControls") || (obj.contains_key("shortName") && obj.contains_key("slug")) {
        return Some(render_frameworks);
    }
    // Asset rows carry `assetType` + `assetProvider`.
    if obj.contains_key("assetType") || obj.contains_key("assetProvider") {
        return Some(render_assets);
    }
    // Workspace rows carry `primary` (bool) alongside `name`.
    if obj.contains_key("primary") && obj.contains_key("name") {
        return Some(render_workspaces);
    }
    // Risk register rows carry `owners` (array) alongside `workspaces` but no `primary`.
    if obj.contains_key("owners") && obj.contains_key("workspaces") {
        return Some(render_registers);
    }
    // User rows carry `firstName` + `lastName` (distinct from personnel `employmentStatus`).
    if obj.contains_key("firstName") && obj.contains_key("lastName") {
        return Some(render_users);
    }
    // Role rows carry `role` (the role name string) + `permissions`.
    if obj.contains_key("role") && obj.contains_key("permissions") {
        return Some(render_roles);
    }
    // Monitor/monitoring-test rows carry `checkResultStatus` or `checkStatus`.
    if obj.contains_key("checkResultStatus") || obj.contains_key("checkStatus") {
        return Some(render_monitors);
    }
    // Audit rows carry `auditType` or `frameworkType`.
    if obj.contains_key("auditType") || obj.contains_key("frameworkType") {
        return Some(render_audits);
    }
    // Event rows carry `requestDescription` or `testName` (both unique to events).
    if obj.contains_key("requestDescription") || obj.contains_key("testName") {
        return Some(render_events);
    }
    // Security review rows carry `reviewDeadlineAt` (unique to the security-review DTO).
    if obj.contains_key("reviewDeadlineAt") {
        return Some(render_security_reviews);
    }
    None
}

// ---------------------------------------------------------------------------
// Per-resource renderers
// ---------------------------------------------------------------------------

fn render_vendors(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "CATEGORY", "RISK", "STATUS", "RENEWAL"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "name"),
            |r| str_field(r, "category"),
            |r| str_field(r, "risk"),
            |r| str_field(r, "status"),
            |r| str_field(r, "renewalDate"),
        ],
        width,
    )
}

fn render_questionnaires(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "TITLE", "RECIPIENT", "COMPLETED", "DATE_SENT"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "title"),
            |r| str_field(r, "recipientEmail"),
            |r| bool_field(r, "isCompleted"),
            |r| str_field(r, "dateSent"),
        ],
        width,
    )
}

fn render_risks(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "TITLE", "TREATMENT", "IMPACT", "LIKELIHOOD", "STATUS"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "title"),
            |r| str_field(r, "treatmentPlan"),
            |r| scalar_field(r, "impact"),
            |r| scalar_field(r, "likelihood"),
            |r| str_field(r, "status"),
        ],
        width,
    )
}

fn render_controls(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "CODE", "NAME", "CREATED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "code"),
            |r| str_field(r, "name"),
            |r| str_field(r, "createdAt"),
        ],
        width,
    )
}

fn render_devices(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "MODEL", "SERIAL", "OS", "COMPLIANT", "LAST_CHECKED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "model"),
            |r| str_field(r, "serialNumber"),
            |r| str_field(r, "osVersion"),
            |r| bool_field(r, "isDeviceCompliant"),
            |r| str_field(r, "lastCheckedAt"),
        ],
        width,
    )
}

fn render_personnel(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "EMAIL", "STATUS", "STARTED", "SEPARATED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| {
                r.get("user")
                    .and_then(|u| u.get("email"))
                    .and_then(|e| e.as_str())
                    .map(String::from)
                    .unwrap_or_default()
            },
            |r| str_field(r, "employmentStatus"),
            |r| str_field(r, "startedAt"),
            |r| str_field(r, "separatedAt"),
        ],
        width,
    )
}

fn render_policies(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "STATUS", "RENEWAL", "PUBLISHED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "name"),
            |r| str_field(r, "status"),
            |r| str_field(r, "renewalDate"),
            |r| str_field(r, "publishedAt"),
        ],
        width,
    )
}

fn render_evidence(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "TEMPLATE_CODE", "CREATED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "name"),
            |r| str_field(r, "evidenceTemplateCode"),
            |r| str_field(r, "createdAt"),
        ],
        width,
    )
}

fn render_frameworks(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "SHORT_NAME", "ENABLED", "CONTROLS"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "name"),
            |r| str_field(r, "shortName"),
            |r| bool_field(r, "isEnabled"),
            |r| scalar_field(r, "numInScopeControls"),
        ],
        width,
    )
}

fn render_assets(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "TYPE", "PROVIDER", "CREATED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "name"),
            |r| str_field(r, "assetType"),
            |r| str_field(r, "assetProvider"),
            |r| str_field(r, "createdAt"),
        ],
        width,
    )
}

fn render_workspaces(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "PRIMARY", "CREATED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "name"),
            |r| bool_field(r, "primary"),
            |r| str_field(r, "createdAt"),
        ],
        width,
    )
}

fn render_registers(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "DESCRIPTION", "CREATED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "name"),
            |r| str_field(r, "description"),
            |r| str_field(r, "createdAt"),
        ],
        width,
    )
}

fn render_users(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "EMAIL", "FIRST_NAME", "LAST_NAME", "JOB_TITLE"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "email"),
            |r| str_field(r, "firstName"),
            |r| str_field(r, "lastName"),
            |r| str_field(r, "jobTitle"),
        ],
        width,
    )
}

fn render_roles(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "ROLE", "CREATED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "role"),
            |r| str_field(r, "createdAt"),
        ],
        width,
    )
}

fn render_monitors(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "CHECK_STATUS", "RESULT_STATUS", "LAST_PASSED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "name"),
            |r| str_field(r, "checkStatus"),
            |r| str_field(r, "checkResultStatus"),
            |r| str_field(r, "lastPassedAt"),
        ],
        width,
    )
}

fn render_audits(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "FRAMEWORK", "AUDIT_TYPE", "STATUS", "START", "END"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "frameworkType"),
            |r| str_field(r, "auditType"),
            |r| str_field(r, "status"),
            |r| str_field(r, "startDate"),
            |r| str_field(r, "endDate"),
        ],
        width,
    )
}

fn render_events(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "TYPE", "CATEGORY", "SOURCE", "CREATED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "type"),
            |r| str_field(r, "category"),
            |r| str_field(r, "source"),
            |r| str_field(r, "createdAt"),
        ],
        width,
    )
}

fn render_security_reviews(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "TITLE", "STATUS", "TYPE", "DECISION", "DEADLINE"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "title"),
            |r| str_field(r, "status"),
            |r| str_field(r, "type"),
            |r| str_field(r, "decision"),
            |r| str_field(r, "reviewDeadlineAt"),
        ],
        width,
    )
}

// ---------------------------------------------------------------------------
// Generic table rendering
// ---------------------------------------------------------------------------

type FieldFn = fn(&Value) -> String;

/// Render a table, shrinking the widest column one char at a time until it fits
/// `width`. Two spaces between columns.
fn render_table(headers: &[&str], rows: &[Value], fields: &[FieldFn], width: usize) -> String {
    let mut grid: Vec<Vec<String>> = Vec::with_capacity(rows.len() + 1);
    grid.push(headers.iter().map(|s| s.to_string()).collect());
    for row in rows {
        grid.push(fields.iter().map(|f| f(row)).collect());
    }

    let cols = headers.len();
    let mut widths = vec![0usize; cols];
    for r in &grid {
        for (i, cell) in r.iter().enumerate() {
            widths[i] = widths[i].max(cell.chars().count());
        }
    }

    let sep = "  ";
    let sep_total = sep.len() * cols.saturating_sub(1);
    loop {
        let total: usize = widths.iter().sum::<usize>() + sep_total;
        if total <= width {
            break;
        }
        let widest = widths
            .iter()
            .enumerate()
            .filter(|(_, w)| **w > 1)
            .max_by_key(|(_, w)| **w)
            .map(|(i, _)| i);
        match widest {
            Some(i) => widths[i] -= 1,
            None => break,
        }
    }

    let mut out = String::new();
    for r in &grid {
        for (i, cell) in r.iter().enumerate() {
            let truncated = truncate(cell, widths[i]);
            let pad = widths[i].saturating_sub(truncated.chars().count());
            if i > 0 {
                out.push_str(sep);
            }
            let _ = write!(out, "{}{}", truncated, " ".repeat(pad));
        }
        out.push('\n');
    }
    out
}

/// Char-boundary-safe truncation with a trailing ellipsis.
fn truncate(s: &str, max: usize) -> String {
    let count = s.chars().count();
    if count <= max {
        return s.to_string();
    }
    if max <= 1 {
        return "…".repeat(max);
    }
    let mut result: String = s.chars().take(max - 1).collect();
    result.push('…');
    result
}

fn str_field(row: &Value, key: &str) -> String {
    row.get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

fn bool_field(row: &Value, key: &str) -> String {
    row.get(key)
        .and_then(|v| v.as_bool())
        .map(|b| b.to_string())
        .unwrap_or_default()
}

/// Render any scalar (number/string/bool) as a string. Drata IDs are numbers.
fn scalar_field(row: &Value, key: &str) -> String {
    match row.get(key) {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Bool(b)) => b.to_string(),
        _ => String::new(),
    }
}

#[cfg(test)]
mod tests;
