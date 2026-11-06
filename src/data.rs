//! The shared `--data` reader for `raw` and every curated `--data` flag.
//!
//! A `--data` value is one of:
//! - `@path`: read the JSON body from the file at `path`,
//! - `-`: read the JSON body from stdin,
//! - anything else: inline JSON.
//!
//! `DataSource::FileOrStdinOnly` rejects the inline form before reading
//! anything, for bodies that carry secrets (cloud credentials): inline JSON
//! would land in shell history and the process table.

use eyre::{Context, Result, bail};
use serde_json::Value;
use std::io::Read;
use tracing::debug;

/// Which `--data` forms a command accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataSource {
    /// Inline JSON, `@file` or `-`.
    Any,
    /// `@file` or `-` only; inline JSON is an error.
    FileOrStdinOnly,
}

/// Resolve a `--data` argument into a JSON value, honoring `source`.
pub fn read_data(data: &str, source: DataSource) -> Result<Value> {
    let raw = if data == "-" {
        debug!("reading --data from stdin");
        let mut buf = String::new();
        std::io::stdin()
            .read_to_string(&mut buf)
            .context("Failed to read JSON from stdin")?;
        buf
    } else if let Some(file) = data.strip_prefix('@') {
        debug!(file, "reading --data from file");
        std::fs::read_to_string(file).with_context(|| format!("Failed to read --data file `{}`", file))?
    } else {
        if source == DataSource::FileOrStdinOnly {
            bail!(
                "--data here accepts only @file or - (stdin): the body carries credentials, \
                 and inline JSON would leave them in shell history"
            );
        }
        // Inline JSON. Length only, never the body.
        debug!(bytes = data.len(), "parsing inline --data JSON");
        data.to_string()
    };

    serde_json::from_str(&raw).context("--data is not valid JSON")
}

#[cfg(test)]
mod tests;
