//! Query-string building for curated list filters.
//!
//! Drata v2 array filters are bracketed keys repeated once per value
//! (`statuses[]=ACTIVE&statuses[]=FLAGGED`), never comma-joined. Keys and
//! values are percent-encoded through `encode_query`, so the brackets reach the
//! server as `%5B%5D`, the same as `expand[]` (`src/expand.rs`).

use crate::client::encode_query;
use clap::ValueEnum;
use std::fmt::Display;

/// The wire string of a clap `ValueEnum`: its possible-value name. Every
/// filter enum is declared with a `rename_all` that makes the clap name equal
/// the spec value (`SCREAMING_SNAKE_CASE` or `camelCase`).
pub fn wire_name<E: ValueEnum>(value: &E) -> String {
    value
        .to_possible_value()
        .map(|v| v.get_name().to_string())
        .expect("filter enums have no skipped variants")
}

/// A path that accumulates `key=value` query pairs.
#[derive(Debug, Clone)]
pub struct QueryPath {
    path: String,
    has_query: bool,
}

impl QueryPath {
    pub fn new(path: impl Into<String>) -> Self {
        let path = path.into();
        let has_query = path.contains('?');
        Self { path, has_query }
    }

    /// Append one `key=value` pair.
    pub fn push(&mut self, key: &str, value: impl Display) -> &mut Self {
        self.path.push(if self.has_query { '&' } else { '?' });
        self.path.push_str(&encode_query(key));
        self.path.push('=');
        self.path.push_str(&encode_query(&value.to_string()));
        self.has_query = true;
        self
    }

    /// Append `key=value` only when `value` is `Some`.
    pub fn push_opt(&mut self, key: &str, value: Option<impl Display>) -> &mut Self {
        if let Some(v) = value {
            self.push(key, v);
        }
        self
    }

    /// Append `key=value` once per value (the spec's array encoding).
    pub fn push_each<T: Display>(&mut self, key: &str, values: impl IntoIterator<Item = T>) -> &mut Self {
        for v in values {
            self.push(key, v);
        }
        self
    }

    pub fn finish(self) -> String {
        self.path
    }
}

#[cfg(test)]
mod tests;
