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
    None
}

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
