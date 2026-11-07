//! Tables for the SafeBase result rows. Named by command (`Table::Safebase*`),
//! never sniffed: SafeBase rows share keys (`id`, `name`, `type`) with Drata's.

use super::{bool_field, joined_strings, render_table, scalar_field, str_field};
use serde_json::Value;

pub(super) fn render_accounts(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "DOMAINS", "MEMBERS", "NDA", "LAST SEEN"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "name"),
            |r| joined_strings(r, "domains"),
            |r| scalar_field(r, "numMembers"),
            |r| bool_field(r, "isNdaDone"),
            |r| str_field(r, "lastSeen"),
        ],
        width,
    )
}

pub(super) fn render_members(rows: &[Value], width: usize) -> String {
    render_table(
        &["ACCOUNT", "NAME", "EMAIL", "LAST SEEN"],
        rows,
        &[
            |r| scalar_field(r, "accountId"),
            full_name,
            |r| str_field(r, "email"),
            |r| str_field(r, "lastSeen"),
        ],
        width,
    )
}

pub(super) fn render_documents(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "TYPE", "SIZE", "EXPIRES", "STATUS"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "name"),
            |r| str_field(r, "type"),
            |r| scalar_field(r, "size"),
            |r| str_field(r, "expiresAt"),
            |r| str_field(r, "uploadStatus"),
        ],
        width,
    )
}

pub(super) fn render_document_types(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME"],
        rows,
        &[|r| scalar_field(r, "id"), |r| str_field(r, "name")],
        width,
    )
}

pub(super) fn render_requests(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "NAME", "EMAIL", "COMPANY", "STATUS", "CREATED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            full_name,
            |r| str_field(r, "email"),
            |r| str_field(r, "companyName"),
            |r| str_field(r, "status"),
            |r| str_field(r, "createdAt"),
        ],
        width,
    )
}

pub(super) fn render_kb_entries(rows: &[Value], width: usize) -> String {
    render_table(
        &["ID", "SOURCE", "ACCESS", "CONTENT", "MODIFIED"],
        rows,
        &[
            |r| scalar_field(r, "id"),
            |r| str_field(r, "source"),
            |r| str_field(r, "accessLevel"),
            one_line_content,
            |r| str_field(r, "modifiedAt"),
        ],
        width,
    )
}

/// `firstName lastName`, falling back to the deprecated `name`.
fn full_name(row: &Value) -> String {
    let joined = [str_field(row, "firstName"), str_field(row, "lastName")]
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");
    if joined.is_empty() { str_field(row, "name") } else { joined }
}

/// KB `content` is free text with newlines; a table cell is one line.
fn one_line_content(row: &Value) -> String {
    str_field(row, "content")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
