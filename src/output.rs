//! Output rendering: `print_value(value, format)` emits JSON or a table.
//!
//! - `Json`: always pretty JSON.
//! - `Table`: dispatch on the response shape; fall back to JSON for unknown
//!   shapes.
//! - `Auto`: JSON when stdout is piped, table when interactive.
//!
//! `$PAGER` (default `less -RFX`) is used only when stdout is a TTY and the
//! rendered output overflows the terminal height. Adapted from `pagerduty-cli`.

use crate::cli::OutputFormat;
use serde_json::Value;
use std::io::{IsTerminal, Write};
use std::process::{Command, Stdio};

mod table;

/// Threshold below which we skip the pager even on a TTY.
const PAGER_MIN_LINES: usize = 2;

pub fn print_value(value: &Value, format: &OutputFormat) {
    let as_json = match format {
        OutputFormat::Json => true,
        OutputFormat::Table => false,
        OutputFormat::Auto => !std::io::stdout().is_terminal(),
    };

    let (width, height) = detect_terminal_size();

    let rendered = if as_json {
        serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string())
    } else {
        match table::render(value, width) {
            Some(t) => t.trim_end_matches('\n').to_string(),
            None => serde_json::to_string_pretty(value).unwrap_or_else(|_| value.to_string()),
        }
    };

    write_with_optional_pager(&rendered, height);
}

/// Print through `$PAGER` when stdout is a TTY AND the rendered output would
/// scroll past the terminal height. Otherwise print directly.
fn write_with_optional_pager(content: &str, height: usize) {
    let stdout_is_tty = std::io::stdout().is_terminal();
    let line_count = content.lines().count();
    let should_page = stdout_is_tty && line_count > height.max(PAGER_MIN_LINES);

    if should_page && try_write_to_pager(content) {
        return;
    }

    println!("{}", content);
}

fn try_write_to_pager(content: &str) -> bool {
    let (cmd, args) = pager_command();
    let mut child = match Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    if let Some(stdin) = child.stdin.as_mut() {
        // Ignore SIGPIPE-style errors: `less` may exit before we finish writing
        // (e.g. user hits q). That is not a failure.
        let _ = stdin.write_all(content.as_bytes());
        let _ = stdin.write_all(b"\n");
    }

    let _ = child.wait();
    true
}

fn pager_command() -> (String, Vec<String>) {
    if let Ok(env) = std::env::var("PAGER") {
        let trimmed = env.trim();
        if !trimmed.is_empty() {
            let mut parts = trimmed.split_whitespace();
            if let Some(cmd) = parts.next() {
                return (cmd.to_string(), parts.map(String::from).collect());
            }
        }
    }
    ("less".to_string(), vec!["-R".into(), "-F".into(), "-X".into()])
}

fn detect_terminal_size() -> (usize, usize) {
    use terminal_size::{Height, Width, terminal_size};
    match terminal_size() {
        Some((Width(w), Height(h))) => (w as usize, h as usize),
        None => (table::DEFAULT_WIDTH, usize::MAX),
    }
}

#[cfg(test)]
mod tests;
