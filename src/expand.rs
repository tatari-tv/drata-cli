//! Expand-parameter encoding for Drata v2 API.
//!
//! The spec uses `expand[]` as the query parameter name for sub-collection
//! expansion on list and get endpoints. Each value is a separate `expand[]`
//! key-value pair, NOT comma-delimited. For example:
//!
//! ```text
//! GET /assets?expand[]=asset&expand[]=complianceChecks
//! ```
//!
//! `append_expand` appends these to an existing path (which may already carry
//! a `?`), using `encode_query` to percent-encode the key and value.

use crate::client::encode_query;
use tracing::debug;

/// Append `expand[]=value` pairs to a path/query string.
/// `expand_vals` is empty -> returns `path` unchanged.
pub fn append_expand(path: &str, expand_vals: &[String]) -> String {
    if expand_vals.is_empty() {
        return path.to_string();
    }
    let mut out = String::from(path);
    let mut sep = if path.contains('?') { '&' } else { '?' };
    for val in expand_vals {
        // The parameter name is `expand[]`; the brackets must be encoded
        // in the key so the server sees `expand%5B%5D=<value>`.
        // Drata docs show `expand[]=controls` in examples; we encode the
        // brackets to be safe (RFC 3986 reserveds in query). Confirmed
        // against the spec: the parameter name is literally `expand[]`.
        out.push(sep);
        out.push_str(&encode_query("expand[]"));
        out.push('=');
        out.push_str(&encode_query(val));
        sep = '&';
    }
    debug!(count = expand_vals.len(), "appended expand[] params");
    out
}

#[cfg(test)]
mod tests;
