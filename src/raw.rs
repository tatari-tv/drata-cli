//! `drata raw <METHOD> <path> ...` - the gated generic passthrough namespace.
//!
//! Reaches any of the 167 operations by HTTP method + path template, for the
//! long tail and power users. Non-GET requests flow through the same write
//! guardrail as the curated verbs (enforced in `client::send_inner`), so a
//! `raw POST` on a non-write-enabled credential fails closed exactly like a
//! curated create.
//!
//! `--data` accepts inline JSON, `@file` (read a file), or `-` (read stdin).
//! `--query k=v` (repeated or space-separated) appends percent-encoded query
//! parameters. `--example` prints the operation's request-body skeleton from
//! the spec and exits before any API/auth setup.

use crate::cli::RawArgs;
use crate::client::{DrataClient, encode_query};
use crate::config::Config;
use crate::output::print_value;
use crate::spec;
use eyre::{Context, Result, eyre};
use serde_json::Value;
use std::io::Read;
use tracing::{debug, instrument, warn};

/// Returns the spec-derived `--example` skeleton if this is a `raw --example`
/// request, else `None`. Checked before config/auth load (no key needed).
///
/// Unlike a curated `--example` (which is infallible), this can fail if the
/// method/path pair is unknown or has no JSON body; the caller surfaces that as
/// a plain message and exits.
pub fn example_if_requested(args: &RawArgs) -> Option<Result<String>> {
    if !args.example {
        return None;
    }
    Some(example_skeleton(&args.method, &args.path))
}

/// Resolve the `--example` skeleton for `method`+`path`, erroring with a clear
/// message when the operation is unknown or carries no JSON request body.
fn example_skeleton(method: &str, path: &str) -> Result<String> {
    match spec::example_for_operation(method, path)? {
        Some(skeleton) => Ok(skeleton),
        None => {
            // Distinguish "no such operation" from "operation has no JSON body".
            match spec::find_by_method_path(method, path)? {
                Some(_) => Err(eyre!(
                    "`{} {}` has no JSON request body (it may be a GET or a multipart upload)",
                    method.to_uppercase(),
                    path
                )),
                None => Err(eyre!(
                    "no operation `{} {}` in the spec; check the method and path template (e.g. /vendors/{{id}})",
                    method.to_uppercase(),
                    path
                )),
            }
        }
    }
}

/// Dispatch a `drata raw` invocation: build the path (with query params), read
/// the body if any, and send through the client's generic `raw` verb.
#[instrument(skip(client, config), fields(method = %args.method, path = %args.path))]
pub async fn handle(args: &RawArgs, client: &DrataClient, config: &Config) -> Result<()> {
    debug!(
        method = %args.method,
        path = %args.path,
        query_len = args.query.len(),
        has_data = args.data.is_some(),
        "raw request"
    );

    // Validate against the spec when possible. An unknown method+path is a warn,
    // not a hard error: the spec is an aid, and a power user may legitimately hit
    // a path the committed snapshot does not yet describe.
    if spec::find_by_method_path(&args.method, &args.path)?.is_none() {
        warn!(method = %args.method, path = %args.path, "method+path not found in spec; sending anyway");
    }

    let path = build_path(&args.path, &args.query)?;
    let body = match &args.data {
        Some(spec_data) => Some(read_data(spec_data).context("Failed to read --data")?),
        None => None,
    };

    let result = client.raw(&args.method, &path, body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

/// Append `key=value` query parameters (percent-encoded) to a path template.
/// Each entry must contain a single `=`. The path may already carry a `?`.
fn build_path(path: &str, query: &[String]) -> Result<String> {
    if query.is_empty() {
        return Ok(path.to_string());
    }
    let mut out = String::from(path);
    let mut sep = if path.contains('?') { '&' } else { '?' };
    for entry in query {
        let (key, value) = entry
            .split_once('=')
            .ok_or_else(|| eyre!("invalid --query `{}`: expected key=value", entry))?;
        out.push(sep);
        out.push_str(&encode_query(key));
        out.push('=');
        out.push_str(&encode_query(value));
        sep = '&';
    }
    debug!(query_count = query.len(), "built query string");
    Ok(out)
}

/// Resolve a `--data` argument into a JSON value. Accepts:
/// - `@path`  -> read the file at `path`,
/// - `-`      -> read stdin,
/// - anything else -> treat as inline JSON.
fn read_data(data: &str) -> Result<Value> {
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
        // Inline JSON. Preview length only, never the full body.
        debug!(bytes = data.len(), "parsing inline --data JSON");
        data.to_string()
    };

    serde_json::from_str(&raw).context("--data is not valid JSON")
}

#[cfg(test)]
mod tests;
