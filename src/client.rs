//! Drata HTTP client.
//!
//! `serde_json::Value` is the wire currency: every verb returns `Value`,
//! request bodies are built with `json!`. Adapted from `pagerduty-cli`'s
//! `client.rs`, with two Drata-specific changes:
//!
//! - **Cursor pagination (Drata v2).** Drata list endpoints return
//!   `{ "data": [...], "pagination": { "cursor": <string|null> } }`. We loop
//!   sending `?cursor=<c>`, accumulate `data[]`, and stop when
//!   `pagination.cursor` is null. Hardened against a misbehaving server:
//!   a repeated cursor (no forward progress) aborts, and a max-page bound caps
//!   total requests.
//! - **Write guardrail.** Any non-GET request fails closed unless the client
//!   was built write-enabled (`allow_writes: true` on the resolved credential).
//!   This is a property of the key, not the profile name.
//! - **Multipart uploads.** `post_multipart` sends a `multipart/form-data`
//!   request (for the 10 ops that require it) through the same write guardrail
//!   and instrumentation machinery.
//! - **NDJSON streaming.** `stream_all` drains cursor-paginated results,
//!   writing one JSON object per line to a `Write` impl instead of buffering.

use eyre::{Context, Result, bail};
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use reqwest::{Client, Method, StatusCode};
use serde_json::Value;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;
use tracing::{debug, instrument, trace, warn};

/// Typed Drata API error. Lets callers (like `try_get`) pattern-match on HTTP
/// status via `eyre::Report::downcast_ref::<ApiError>()`, while still printing
/// the same human-readable message the `bail!` path produced.
#[derive(Debug, Error)]
#[error("{formatted}")]
pub struct ApiError {
    pub status: StatusCode,
    pub body: String,
    pub formatted: String,
}

/// Raised when a non-GET request is attempted on a client whose resolved
/// credential is not write-enabled. Fails closed: writes are off by default.
#[derive(Debug, Error)]
#[error(
    "write blocked: the active Drata credential is not write-enabled.\n\
     Non-GET requests ({method} {path}) require a credential with `allow-writes: true`.\n\
     Set it on the active profile in credentials.json, or pass --allow-writes."
)]
pub struct WriteGuardError {
    pub method: String,
    pub path: String,
}

/// A single named file part of a multipart/form-data body. `field` is the form
/// field name the endpoint expects (`file`, `files`, `externalEvidence`, ...);
/// `path` is the local file to read and send.
#[derive(Debug, Clone)]
pub struct FilePart {
    pub field: String,
    pub path: PathBuf,
}

/// A multipart/form-data body: named file parts plus scalar text fields.
///
/// Drata's upload endpoints vary in both the file field name and the required
/// scalar fields, so each caller builds the exact shape for its endpoint rather
/// than relying on a single hard-coded `file` part.
#[derive(Debug, Default)]
pub struct Multipart {
    pub files: Vec<FilePart>,
    pub fields: Vec<(String, String)>,
}

impl Multipart {
    pub fn new() -> Self {
        Self::default()
    }

    /// Build a body with a single file part under `field`.
    pub fn single(field: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        let mut form = Self::new();
        form.add_file(field, path);
        form
    }

    /// Append a file part under the form field name `field`.
    pub fn add_file(&mut self, field: impl Into<String>, path: impl Into<PathBuf>) -> &mut Self {
        self.files.push(FilePart {
            field: field.into(),
            path: path.into(),
        });
        self
    }

    /// Append a scalar text field.
    pub fn add_field(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.fields.push((name.into(), value.into()));
        self
    }

    /// Append a scalar text field only when `value` is `Some`.
    pub fn add_opt_field(&mut self, name: impl Into<String>, value: Option<impl Into<String>>) -> &mut Self {
        if let Some(v) = value {
            self.fields.push((name.into(), v.into()));
        }
        self
    }

    /// True when the body carries no files and no fields.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty() && self.fields.is_empty()
    }
}

/// A file part with its bytes read into memory, ready to (re)build a reqwest
/// `Part` on each retry attempt.
struct LoadedPart {
    field: String,
    file_name: String,
    mime: String,
    bytes: Vec<u8>,
}

/// Characters that must be percent-encoded in query parameter values.
/// RFC 3986 §3.4 reserved set plus `+`/`=`/`&` which carry parser meaning.
const QUERY_ENCODE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}')
    .add(b'&')
    .add(b'+')
    .add(b'=');

/// Percent-encode a string for safe use as a URL query parameter value.
pub fn encode_query(value: &str) -> String {
    utf8_percent_encode(value, QUERY_ENCODE_SET).to_string()
}

/// Region base URLs from the spec `servers` block. Selected by credential
/// region; default is US.
const US_BASE_URL: &str = "https://public-api.drata.com/public/v2";
const EU_BASE_URL: &str = "https://public-api.eu.drata.com/public/v2";
const APAC_BASE_URL: &str = "https://public-api.apac.drata.com/public/v2";

/// Per-page size requested from cursor-paginated list endpoints.
const PAGINATION_SIZE: u32 = 50;
const MAX_RETRY_ATTEMPTS: u32 = 3;
const DEFAULT_RETRY_DELAY_SECS: u64 = 5;
/// Upper bound on a server-provided `Retry-After`. A hostile or buggy server
/// could otherwise stall the CLI for minutes/hours despite bounded attempts.
const MAX_RETRY_DELAY_SECS: u64 = 60;
const REQUEST_TIMEOUT_SECS: u64 = 30;
/// Hard cap on pages drained by cursor pagination, so a server that never
/// returns a null cursor cannot loop forever.
const MAX_PAGES: u32 = 10_000;

/// Resolve a region string to its base URL. Returns `Err` for unknown regions
/// so the bearer token is never sent to an unintended endpoint.
pub fn base_url_for_region(region: &str) -> Result<&'static str> {
    match region.to_lowercase().as_str() {
        "us" => Ok(US_BASE_URL),
        "eu" => Ok(EU_BASE_URL),
        "apac" => Ok(APAC_BASE_URL),
        other => {
            bail!(
                "Unknown region {:?}. Valid values: us, eu, apac. \
                 Set the correct region in your profile (`drata login --region <region>`) \
                 or via --region / DRATA_REGION.",
                other
            )
        }
    }
}

pub struct DrataClient {
    http: Client,
    base_url: String,
    api_key: String,
    /// Whether the resolved credential is write-enabled. When false, every
    /// non-GET request fails closed via the write guardrail.
    allow_writes: bool,
}

impl DrataClient {
    /// Build a client for the given region. `allow_writes` is a property of the
    /// resolved credential: a non-GET request on a `false` client fails closed.
    /// Returns `Err` if `region` is not a known value (us/eu/apac).
    #[instrument(skip(api_key), fields(region, allow_writes))]
    pub fn new(api_key: String, region: &str, allow_writes: bool) -> Result<Self> {
        debug!(region, allow_writes, "building DrataClient");
        let base_url = base_url_for_region(region)?;
        let http = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            http,
            base_url: base_url.to_string(),
            api_key,
            allow_writes,
        })
    }

    /// Override the base URL. The wiremock test seam.
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    /// Whether this client may issue mutating requests.
    pub fn allow_writes(&self) -> bool {
        self.allow_writes
    }

    /// Shared request machinery: builds the request, sends it, retries 429
    /// with `Retry-After` (bounded), maps 204 to `Value::Null`, and parses
    /// error bodies through `format_api_error`. The write guardrail is enforced
    /// here so every mutating path is covered, including `raw`.
    #[instrument(skip(self, body), fields(%method, %path))]
    async fn send_inner(&self, method: Method, path: &str, body: Option<Value>) -> Result<Value> {
        if method != Method::GET && !self.allow_writes {
            warn!(%method, %path, "write blocked: credential not write-enabled");
            return Err(WriteGuardError {
                method: method.to_string(),
                path: path.to_string(),
            }
            .into());
        }

        let url = format!("{}{}", self.base_url, path);
        let mut attempts = 0u32;

        loop {
            attempts += 1;

            let mut req = self
                .http
                .request(method.clone(), &url)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key));

            if let Some(ref b) = body {
                req = req.json(b);
            }

            let resp = req.send().await.context("HTTP request failed")?;
            let status = resp.status();

            if status == StatusCode::TOO_MANY_REQUESTS {
                if attempts > MAX_RETRY_ATTEMPTS {
                    bail!("Rate limited after {} attempts", MAX_RETRY_ATTEMPTS);
                }
                let delay = resp
                    .headers()
                    .get("Retry-After")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(DEFAULT_RETRY_DELAY_SECS)
                    .min(MAX_RETRY_DELAY_SECS);
                warn!(delay, attempts, max = MAX_RETRY_ATTEMPTS, "rate limited, retrying");
                sleep(Duration::from_secs(delay)).await;
                continue;
            }

            // 204 No Content (e.g. DELETE /vendors/{id}) has an empty body.
            if status == StatusCode::NO_CONTENT {
                debug!("204 No Content");
                return Ok(Value::Null);
            }

            if !status.is_success() {
                let error_body = resp.text().await.unwrap_or_default();
                // debug! (not warn!): this path also fires for the 404s that
                // `try_get` deliberately swallows as "not found", so warning here
                // would cry wolf on every expected-miss lookup.
                debug!(%method, %url, %status, body = %error_body, "API request failed");
                return Err(ApiError {
                    status,
                    formatted: format_api_error(&method, &url, status, &error_body),
                    body: error_body,
                }
                .into());
            }

            let json: Value = resp.json().await.context("Failed to parse response JSON")?;
            debug!(%status, "request succeeded");
            return Ok(json);
        }
    }

    /// Send a multipart/form-data request. Used for the upload operations
    /// identified by `spec::Operation.multipart`. The write guardrail applies:
    /// the client must be write-enabled.
    ///
    /// Drata's upload endpoints differ in both the file field name (`file`,
    /// `files`, `externalEvidence`) and the scalar fields they require, so the
    /// caller supplies the exact shape via `Multipart` rather than a single
    /// hard-coded `file` part. Retries on 429 with `Retry-After` (bounded),
    /// matching `send_inner`'s contract.
    #[instrument(skip(self, form), fields(%method, %path))]
    pub async fn send_multipart(&self, method: Method, path: &str, form: &Multipart) -> Result<Value> {
        debug!(%method, path, files = form.files.len(), fields = form.fields.len(), "multipart request");

        if !self.allow_writes {
            warn!(%method, %path, "write blocked: credential not write-enabled");
            return Err(WriteGuardError {
                method: method.to_string(),
                path: path.to_string(),
            }
            .into());
        }

        // Read every file part upfront. reqwest consumes the `Form` on send, so
        // the retry loop rebuilds it each attempt from these buffered bytes.
        let mut loaded: Vec<LoadedPart> = Vec::with_capacity(form.files.len());
        for fp in &form.files {
            let bytes = std::fs::read(&fp.path)
                .with_context(|| format!("Failed to read upload file `{}`", fp.path.display()))?;
            let file_name = fp
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("upload")
                .to_string();
            let mime = mime_guess::from_path(&fp.path).first_or_octet_stream().to_string();
            trace!(field = %fp.field, file = %fp.path.display(), bytes = bytes.len(), %mime, "loaded multipart file part");
            loaded.push(LoadedPart {
                field: fp.field.clone(),
                file_name,
                mime,
                bytes,
            });
        }

        let url = format!("{}{}", self.base_url, path);
        let mut attempts = 0u32;

        loop {
            attempts += 1;

            let mut multipart = reqwest::multipart::Form::new();
            for (name, value) in &form.fields {
                multipart = multipart.text(name.clone(), value.clone());
            }
            for part in &loaded {
                let file_part = reqwest::multipart::Part::bytes(part.bytes.clone())
                    .file_name(part.file_name.clone())
                    .mime_str(&part.mime)
                    .context("Failed to set MIME type on multipart part")?;
                multipart = multipart.part(part.field.clone(), file_part);
            }

            let resp = self
                .http
                .request(method.clone(), &url)
                .header("Accept", "application/json")
                .header("Authorization", format!("Bearer {}", self.api_key))
                .multipart(multipart)
                .send()
                .await
                .context("Multipart HTTP request failed")?;

            let status = resp.status();

            if status == StatusCode::TOO_MANY_REQUESTS {
                if attempts > MAX_RETRY_ATTEMPTS {
                    bail!("Rate limited after {} attempts (multipart)", MAX_RETRY_ATTEMPTS);
                }
                let delay = resp
                    .headers()
                    .get("Retry-After")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|s| s.parse::<u64>().ok())
                    .unwrap_or(DEFAULT_RETRY_DELAY_SECS)
                    .min(MAX_RETRY_DELAY_SECS);
                warn!(
                    delay,
                    attempts,
                    max = MAX_RETRY_ATTEMPTS,
                    "rate limited on multipart, retrying"
                );
                sleep(Duration::from_secs(delay)).await;
                continue;
            }

            if status == StatusCode::NO_CONTENT {
                debug!("multipart 204 No Content");
                return Ok(Value::Null);
            }

            if !status.is_success() {
                let error_body = resp.text().await.unwrap_or_default();
                debug!(%method, %url, %status, body = %error_body, "multipart API request failed");
                return Err(ApiError {
                    status,
                    formatted: format_api_error(&method, &url, status, &error_body),
                    body: error_body,
                }
                .into());
            }

            let json: Value = resp.json().await.context("Failed to parse multipart response JSON")?;
            debug!(%status, "multipart request succeeded");
            return Ok(json);
        }
    }

    /// POST a multipart/form-data request. Convenience wrapper around
    /// `send_multipart` for upload operations that use POST.
    #[instrument(skip(self, form), fields(%path))]
    pub async fn post_multipart(&self, path: &str, form: &Multipart) -> Result<Value> {
        self.send_multipart(Method::POST, path, form).await
    }

    /// PUT a multipart/form-data request. Used for update operations where the
    /// spec specifies a PUT with `multipart/form-data` (e.g. evidence update).
    #[instrument(skip(self, form), fields(%path))]
    pub async fn put_multipart(&self, path: &str, form: &Multipart) -> Result<Value> {
        self.send_multipart(Method::PUT, path, form).await
    }

    #[instrument(skip(self))]
    pub async fn get(&self, path: &str) -> Result<Value> {
        self.send_inner(Method::GET, path, None).await
    }

    /// GET a resource, returning `Ok(None)` on HTTP 404. All other errors
    /// propagate. Used for fallback lookups where a missing resource is a
    /// legitimate signal.
    #[instrument(skip(self))]
    pub async fn try_get(&self, path: &str) -> Result<Option<Value>> {
        match self.send_inner(Method::GET, path, None).await {
            Ok(v) => Ok(Some(v)),
            Err(e) => match e.downcast_ref::<ApiError>() {
                Some(api_err) if api_err.status == StatusCode::NOT_FOUND => Ok(None),
                _ => Err(e),
            },
        }
    }

    #[instrument(skip(self, body))]
    pub async fn post(&self, path: &str, body: Value) -> Result<Value> {
        self.send_inner(Method::POST, path, Some(body)).await
    }

    #[instrument(skip(self, body))]
    pub async fn put(&self, path: &str, body: Value) -> Result<Value> {
        self.send_inner(Method::PUT, path, Some(body)).await
    }

    #[instrument(skip(self))]
    pub async fn delete(&self, path: &str) -> Result<Value> {
        self.send_inner(Method::DELETE, path, None).await
    }

    /// Generic passthrough for the `raw` namespace. Non-GET is subject to the
    /// same write guardrail as the typed verbs.
    #[instrument(skip(self, body))]
    pub async fn raw(&self, method: &str, path: &str, body: Option<Value>) -> Result<Value> {
        let m = Method::from_str(&method.to_uppercase()).map_err(|_| eyre::eyre!("Invalid HTTP method: {}", method))?;
        self.send_inner(m, path, body).await
    }

    /// Paginate through all results for a Drata v2 cursor-paginated list
    /// endpoint. `path` may already carry query parameters. Accumulates the
    /// `data[]` array; stops when `pagination.cursor` is null.
    ///
    /// Hardened: aborts if the server returns a cursor it already gave us (no
    /// forward progress), and caps total pages at `MAX_PAGES`.
    #[instrument(skip(self))]
    pub async fn get_all(&self, path: &str) -> Result<Vec<Value>> {
        let mut all = Vec::new();
        let mut cursor: Option<String> = None;
        let mut seen_cursors: std::collections::HashSet<String> = std::collections::HashSet::new();
        let sep = if path.contains('?') { '&' } else { '?' };
        let mut pages = 0u32;

        loop {
            pages += 1;
            if pages > MAX_PAGES {
                bail!(
                    "cursor pagination exceeded MAX_PAGES ({}) for path {}; \
                     the server may not be returning a terminal null cursor. \
                     Use --all for streaming instead of buffering.",
                    MAX_PAGES,
                    path
                );
            }

            let paginated = match &cursor {
                Some(c) => format!("{}{}size={}&cursor={}", path, sep, PAGINATION_SIZE, encode_query(c)),
                None => format!("{}{}size={}", path, sep, PAGINATION_SIZE),
            };
            let resp = self.get(&paginated).await?;

            if let Some(items) = resp.get("data").and_then(|v| v.as_array()) {
                trace!(page = pages, items = items.len(), "accumulated page");
                all.extend(items.clone());
            }

            let next = resp
                .get("pagination")
                .and_then(|p| p.get("cursor"))
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .map(String::from);

            match next {
                Some(n) => {
                    // No-progress guard: a server that echoes the same cursor
                    // forever would otherwise loop until MAX_PAGES.
                    if !seen_cursors.insert(n.clone()) {
                        bail!(
                            "repeated cursor {:?} detected on path {}; \
                             server is not making forward progress. Aborting to avoid \
                             returning silently truncated results.",
                            n,
                            path
                        );
                    }
                    cursor = Some(n);
                }
                None => break,
            }
        }

        debug!(total = all.len(), pages, "cursor pagination complete");
        Ok(all)
    }

    /// Stream all results from a cursor-paginated list endpoint as NDJSON.
    /// Writes one JSON object per line to `writer` instead of buffering an
    /// unbounded `Vec<Value>`. Same hardening as `get_all` (repeated-cursor
    /// abort, max-page bound).
    ///
    /// This is the `--all` streaming path for large tenants where buffering the
    /// full result set would be impractical.
    #[instrument(skip(self, writer))]
    pub async fn stream_all<W: Write>(&self, path: &str, writer: &mut W) -> Result<u64> {
        let mut cursor: Option<String> = None;
        let mut seen_cursors: std::collections::HashSet<String> = std::collections::HashSet::new();
        let sep = if path.contains('?') { '&' } else { '?' };
        let mut pages = 0u32;
        let mut total_items: u64 = 0;

        loop {
            pages += 1;
            if pages > MAX_PAGES {
                bail!(
                    "stream_all exceeded MAX_PAGES ({}) for path {}; \
                     the server may not be returning a terminal null cursor.",
                    MAX_PAGES,
                    path
                );
            }

            let paginated = match &cursor {
                Some(c) => format!("{}{}size={}&cursor={}", path, sep, PAGINATION_SIZE, encode_query(c)),
                None => format!("{}{}size={}", path, sep, PAGINATION_SIZE),
            };
            let resp = self.get(&paginated).await?;

            if let Some(items) = resp.get("data").and_then(|v| v.as_array()) {
                trace!(page = pages, items = items.len(), "streaming page");
                for item in items {
                    let line = serde_json::to_string(item).context("Failed to serialize item as NDJSON")?;
                    writeln!(writer, "{}", line).context("Failed to write NDJSON line")?;
                    total_items += 1;
                }
            }

            let next = resp
                .get("pagination")
                .and_then(|p| p.get("cursor"))
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty())
                .map(String::from);

            match next {
                Some(n) => {
                    if !seen_cursors.insert(n.clone()) {
                        bail!(
                            "repeated cursor {:?} detected in stream_all on path {}; \
                             server is not making forward progress.",
                            n,
                            path
                        );
                    }
                    cursor = Some(n);
                }
                None => break,
            }
        }

        debug!(total_items, pages, "stream_all complete");
        Ok(total_items)
    }
}

/// Format a Drata API error response into a human-readable message. Drata
/// returns structured JSON errors; this names the failing `METHOD url` and
/// never collapses to "Unknown error" - the raw body always survives.
fn format_api_error(method: &Method, url: &str, status: StatusCode, body: &str) -> String {
    // Always name the request that failed. The key lives in a header, not the
    // URL, so this leaks nothing - and "which call broke" is the first question.
    let target = format!("{} {}", method, url);

    if let Ok(parsed) = serde_json::from_str::<Value>(body) {
        // Drata's envelope: `{ "message": "...", "errors": [...] }`. Also try
        // a nested `error.message` and a bare-string `error` for robustness.
        let message: Option<String> = parsed
            .get("message")
            .and_then(|m| m.as_str())
            .map(|s| s.to_string())
            .or_else(|| {
                parsed
                    .get("error")
                    .and_then(|e| e.get("message"))
                    .and_then(|m| m.as_str())
                    .map(|s| s.to_string())
            })
            .or_else(|| parsed.get("error").and_then(|e| e.as_str()).map(|s| s.to_string()));

        let details: Vec<String> = parsed
            .get("errors")
            .and_then(|e| e.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|v| v.as_str().map(|s| s.to_string()).unwrap_or_else(|| v.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let msg = match message {
            Some(m) => {
                let mut s = format!("API error {} on {}: {}", status, target, m);
                if !details.is_empty() {
                    s.push_str(&format!("\nDetails: {}", details.join("; ")));
                }
                s
            }
            // No message at the expected path: surface the raw body verbatim so
            // the real Drata error is never hidden behind "Unknown error".
            None => format!("API error {} on {}: {}", status, target, body.trim()),
        };

        return msg;
    }

    format!("API error {} on {}: {}", status, target, body)
}

#[cfg(test)]
mod tests;
