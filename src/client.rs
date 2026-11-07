//! HTTP client for the Drata-company APIs: the Drata GRC API and SafeBase.
//!
//! `serde_json::Value` is the wire currency: every verb returns `Value`,
//! request bodies are built with `json!`. Adapted from `pagerduty-cli`'s
//! `client.rs`. What differs per API (base URL, auth header, redirect policy)
//! comes from `Backend`. The rest:
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
//! - **SafeBase envelope.** A SafeBase 2xx must carry `{"ok": true, ...}`; a
//!   2xx with `ok: false` or no boolean `ok` is an `EnvelopeError` carrying
//!   the body. The response is returned whole: callers read `result` only
//!   when it is present (nine ops answer `{"ok": true}` alone).

use crate::backend::Backend;
use eyre::{Context, Result, bail};
use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use reqwest::header::{HeaderName, HeaderValue};
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

/// A SafeBase 2xx whose body breaks the `{ok, result}` envelope: `ok` is
/// `false`, or there is no boolean `ok`. Never read as success.
#[derive(Debug, Error)]
#[error("{formatted}")]
pub struct EnvelopeError {
    pub status: StatusCode,
    pub body: String,
    pub formatted: String,
}

/// Raised when a non-GET request is attempted on a client whose resolved
/// credential is not write-enabled. Fails closed: writes are off by default.
/// Names the backend whose credential blocked the write.
#[derive(Debug, Error)]
#[error(
    "write blocked: the active {backend} credential is not write-enabled.\n\
     Non-GET requests ({method} {path}) require a credential with `allow-writes: true`.\n\
     Set it on the active profile in credentials.json, or pass --allow-writes."
)]
pub struct WriteGuardError {
    pub backend: Backend,
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

/// Characters that must be percent-encoded in one path segment: the query set
/// plus the separators `/` and `?` and the escape `%`. `:` stays literal, so a
/// `name:<draft name>` test id reads as the spec writes it.
const PATH_SEGMENT_ENCODE_SET: &AsciiSet = &QUERY_ENCODE_SET.add(b'/').add(b'?').add(b'%');

/// Percent-encode a string for use as one URL path segment, so a free-text id
/// cannot add segments or start the query string.
pub fn encode_path_segment(value: &str) -> String {
    utf8_percent_encode(value, PATH_SEGMENT_ENCODE_SET).to_string()
}

/// Per-page size requested from cursor-paginated list endpoints: the spec's
/// `size` maximum on every cursor endpoint. The server default of 50 put the
/// page cap at 500k items, below a real tenant's `/events` count (~945k).
const PAGINATION_SIZE: u32 = 500;
const MAX_RETRY_ATTEMPTS: u32 = 3;
const DEFAULT_RETRY_DELAY_SECS: u64 = 5;
/// Upper bound on a server-provided `Retry-After`. A hostile or buggy server
/// could otherwise stall the CLI for hours despite bounded attempts. 60s was too
/// low: the v0.2.0 shakedown saw Drata's edge send `Retry-After: 162`, so every
/// capped retry landed inside the block and all three attempts were wasted.
const MAX_RETRY_DELAY_SECS: u64 = 300;
const REQUEST_TIMEOUT_SECS: u64 = 30;
/// Minimum spacing between page requests in one cursor drain: 400/min, under
/// Drata's 500/min per-IP limit with room for other calls.
const MIN_PAGE_INTERVAL: Duration = Duration::from_millis(150);
/// Hard cap on pages drained by cursor pagination, so a server that never
/// returns a null cursor cannot loop forever. At `PAGINATION_SIZE` this is 5M items.
const MAX_PAGES: u32 = 10_000;

/// Whether `send_inner` retries an HTTP 429.
///
/// `Default` retries up to `MAX_RETRY_ATTEMPTS` times, honoring `Retry-After`.
/// `Never` sends once and returns the 429 as an `ApiError` carrying the
/// server's body: for ops where 429 means "not now" (a post-run cooldown), not
/// "slow down".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Retry429 {
    Default,
    Never,
}

/// The client for both Drata-company APIs: the Drata GRC API and SafeBase.
/// `Backend` picks which one, and with it the base URL, the auth header and
/// the redirect policy. The name stays `DrataClient` because SafeBase is a
/// Drata product and the binary is `drata`.
pub struct DrataClient {
    http: Client,
    backend: Backend,
    base_url: String,
    /// The backend's auth header, built once from the key. The only auth
    /// header any request carries.
    auth: (HeaderName, HeaderValue),
    /// Whether the resolved credential is write-enabled. When false, every
    /// non-GET request fails closed via the write guardrail.
    allow_writes: bool,
    /// Minimum spacing between page requests in one cursor drain.
    page_interval: Duration,
}

impl DrataClient {
    /// Build a client for `backend`. `region` selects the Drata base URL
    /// (default us); SafeBase has one and rejects any `region`. `allow_writes`
    /// is a property of the resolved credential: a non-GET request on a
    /// `false` client fails closed. Returns `Err` for an unknown region or a
    /// key that cannot be sent as a header.
    #[instrument(skip(api_key))]
    pub fn new(backend: Backend, api_key: String, region: Option<&str>, allow_writes: bool) -> Result<Self> {
        debug!(%backend, ?region, allow_writes, "building DrataClient");
        let base_url = backend.base_url(region)?;
        let auth = backend.auth_header(&api_key)?;
        let http = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .redirect(backend.redirect_policy())
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            http,
            backend,
            base_url: base_url.to_string(),
            auth,
            allow_writes,
            page_interval: MIN_PAGE_INTERVAL,
        })
    }

    /// Override the base URL. The wiremock test seam.
    pub fn with_base_url(mut self, url: String) -> Self {
        self.base_url = url;
        self
    }

    /// Override the page pacing interval. A test seam: a long interval makes
    /// pacing measurable over local request latency.
    #[cfg(test)]
    pub(crate) fn with_page_interval(mut self, interval: Duration) -> Self {
        self.page_interval = interval;
        self
    }

    /// The API this client talks to.
    pub fn backend(&self) -> Backend {
        self.backend
    }

    /// Whether this client may issue mutating requests.
    pub fn allow_writes(&self) -> bool {
        self.allow_writes
    }

    /// Shared request machinery: builds the request, sends it, retries 429
    /// with `Retry-After` (bounded) unless `retry` is `Retry429::Never`, maps
    /// 204 to `Value::Null`, and parses error bodies through
    /// `format_api_error`. The write guardrail is enforced here so every
    /// mutating path is covered, including `raw`.
    #[instrument(skip(self, body), fields(%method, %path, ?retry))]
    async fn send_inner(&self, method: Method, path: &str, body: Option<Value>, retry: Retry429) -> Result<Value> {
        if method != Method::GET && !self.allow_writes {
            warn!(backend = %self.backend, %method, %path, "write blocked: credential not write-enabled");
            return Err(WriteGuardError {
                backend: self.backend,
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
                .header(self.auth.0.clone(), self.auth.1.clone());

            if let Some(ref b) = body {
                req = req.json(b);
            }

            let resp = req.send().await.context("HTTP request failed")?;
            let status = resp.status();

            if status == StatusCode::TOO_MANY_REQUESTS && retry == Retry429::Default {
                if attempts > MAX_RETRY_ATTEMPTS {
                    bail!("Rate limited after {} attempts", MAX_RETRY_ATTEMPTS);
                }
                let delay = retry_delay(resp.headers().get("Retry-After").and_then(|v| v.to_str().ok()));
                announce_retry(delay, attempts);
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
            return self.check_envelope(&method, &url, status, json);
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
            warn!(backend = %self.backend, %method, %path, "write blocked: credential not write-enabled");
            return Err(WriteGuardError {
                backend: self.backend,
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
                .header(self.auth.0.clone(), self.auth.1.clone())
                .multipart(multipart)
                .send()
                .await
                .context("Multipart HTTP request failed")?;

            let status = resp.status();

            if status == StatusCode::TOO_MANY_REQUESTS {
                if attempts > MAX_RETRY_ATTEMPTS {
                    bail!("Rate limited after {} attempts (multipart)", MAX_RETRY_ATTEMPTS);
                }
                let delay = retry_delay(resp.headers().get("Retry-After").and_then(|v| v.to_str().ok()));
                announce_retry(delay, attempts);
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
            return self.check_envelope(&method, &url, status, json);
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
        self.send_inner(Method::GET, path, None, Retry429::Default).await
    }

    /// GET a resource, returning `Ok(None)` on HTTP 404. All other errors
    /// propagate. Used for fallback lookups where a missing resource is a
    /// legitimate signal.
    #[instrument(skip(self))]
    pub async fn try_get(&self, path: &str) -> Result<Option<Value>> {
        match self.send_inner(Method::GET, path, None, Retry429::Default).await {
            Ok(v) => Ok(Some(v)),
            Err(e) => match e.downcast_ref::<ApiError>() {
                Some(api_err) if api_err.status == StatusCode::NOT_FOUND => Ok(None),
                _ => Err(e),
            },
        }
    }

    #[instrument(skip(self, body))]
    pub async fn post(&self, path: &str, body: Value) -> Result<Value> {
        self.send_inner(Method::POST, path, Some(body), Retry429::Default).await
    }

    /// POST with no body, sent exactly once: a 429 comes back as an `ApiError`
    /// carrying the server's body instead of being retried. For ops where 429
    /// means "not now" (a post-run cooldown), not "slow down".
    #[instrument(skip(self))]
    pub async fn post_once(&self, path: &str) -> Result<Value> {
        self.send_inner(Method::POST, path, None, Retry429::Never).await
    }

    #[instrument(skip(self, body))]
    pub async fn put(&self, path: &str, body: Value) -> Result<Value> {
        self.send_inner(Method::PUT, path, Some(body), Retry429::Default).await
    }

    #[instrument(skip(self, body))]
    pub async fn patch(&self, path: &str, body: Value) -> Result<Value> {
        self.send_inner(Method::PATCH, path, Some(body), Retry429::Default)
            .await
    }

    /// DELETE, with a JSON body for the ops whose spec requires one (Drata's
    /// agent review `assessment-runs/current` takes `{}`; SafeBase
    /// `DELETE /accounts/{id}/members` takes the emails to remove).
    #[instrument(skip(self, body))]
    pub async fn delete(&self, path: &str, body: Option<Value>) -> Result<Value> {
        self.send_inner(Method::DELETE, path, body, Retry429::Default).await
    }

    /// Generic passthrough for the `raw` namespace. Non-GET is subject to the
    /// same write guardrail as the typed verbs.
    #[instrument(skip(self, body))]
    pub async fn raw(&self, method: &str, path: &str, body: Option<Value>) -> Result<Value> {
        let m = Method::from_str(&method.to_uppercase()).map_err(|_| eyre::eyre!("Invalid HTTP method: {}", method))?;
        self.send_inner(m, path, body, Retry429::Default).await
    }

    /// Paginate through all results for a Drata v2 cursor-paginated list
    /// endpoint. `path` may already carry query parameters. Accumulates the
    /// `data[]` array; stops when `pagination.cursor` is null.
    #[instrument(skip(self))]
    pub async fn get_all(&self, path: &str) -> Result<Vec<Value>> {
        self.get_all_limited(path, None).await
    }

    /// `get_all`, stopping once `limit` items are collected.
    #[instrument(skip(self))]
    pub async fn get_all_limited(&self, path: &str, limit: Option<usize>) -> Result<Vec<Value>> {
        let mut all = Vec::new();
        let (items, pages) = self
            .drain_pages(path, limit, |item| {
                all.push(item.clone());
                Ok(())
            })
            .await?;
        debug!(total = items, pages, "cursor pagination complete");
        Ok(all)
    }

    /// Stream all results from a cursor-paginated list endpoint as NDJSON.
    /// Writes one JSON object per line to `writer` instead of buffering an
    /// unbounded `Vec<Value>`. This is the `--all` path for large tenants.
    #[instrument(skip(self, writer))]
    pub async fn stream_all<W: Write>(&self, path: &str, writer: &mut W) -> Result<u64> {
        self.stream_all_limited(path, None, writer).await
    }

    /// `stream_all`, stopping once `limit` items are written.
    #[instrument(skip(self, writer))]
    pub async fn stream_all_limited<W: Write>(&self, path: &str, limit: Option<usize>, writer: &mut W) -> Result<u64> {
        let (items, pages) = self
            .drain_pages(path, limit, |item| {
                let line = serde_json::to_string(item).context("Failed to serialize item as NDJSON")?;
                writeln!(writer, "{}", line).context("Failed to write NDJSON line")?;
                Ok(())
            })
            .await?;
        debug!(total_items = items, pages, "stream_all complete");
        Ok(items)
    }

    /// The one cursor loop behind `get_all*` and `stream_all*`. Hands each item
    /// to `sink`, stops at a null cursor or at `limit` items, and returns
    /// (items, pages).
    ///
    /// Hardened: aborts if the server returns a cursor it already gave us (no
    /// forward progress), and caps total pages at `MAX_PAGES`.
    ///
    /// Paced: page requests are spaced at least `MIN_PAGE_INTERVAL` apart, so a
    /// large tenant (~495k assets, ~945k events) cannot outrun Drata's 500/min
    /// limit and trip the edge's multi-minute 429 block (v0.2.0 shakedown).
    async fn drain_pages<F>(&self, path: &str, limit: Option<usize>, mut sink: F) -> Result<(u64, u32)>
    where
        F: FnMut(&Value) -> Result<()>,
    {
        let mut cursor: Option<String> = None;
        let mut seen_cursors: std::collections::HashSet<String> = std::collections::HashSet::new();
        let sep = if path.contains('?') { '&' } else { '?' };
        let size = match limit {
            Some(n) => n.clamp(1, PAGINATION_SIZE as usize) as u32,
            None => PAGINATION_SIZE,
        };
        let mut pages = 0u32;
        let mut items: u64 = 0;
        let mut last_page: Option<tokio::time::Instant> = None;

        loop {
            if limit.is_some_and(|n| items >= n as u64) {
                break;
            }
            pages += 1;
            if pages > MAX_PAGES {
                bail!(
                    "cursor pagination hit the {}-page cap on {} after {} items (size {}) \
                     without a null cursor; the result set is larger than the cap allows",
                    MAX_PAGES,
                    path,
                    items,
                    size
                );
            }

            if let Some(prev) = last_page {
                let wait = self.page_interval.saturating_sub(prev.elapsed());
                if !wait.is_zero() {
                    trace!(wait_ms = wait.as_millis() as u64, "pacing page request");
                    sleep(wait).await;
                }
            }
            last_page = Some(tokio::time::Instant::now());

            let paginated = match &cursor {
                Some(c) => format!("{}{}size={}&cursor={}", path, sep, size, encode_query(c)),
                None => format!("{}{}size={}", path, sep, size),
            };
            let resp = self.get(&paginated).await?;

            if let Some(page) = resp.get("data").and_then(|v| v.as_array()) {
                trace!(page = pages, items = page.len(), "draining page");
                for item in page {
                    if limit.is_some_and(|n| items >= n as u64) {
                        break;
                    }
                    sink(item)?;
                    items += 1;
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

        Ok((items, pages))
    }
}

impl DrataClient {
    /// SafeBase answers every success as `{"ok": true, ...}`. A 2xx with
    /// `ok: false`, or with no boolean `ok`, is an `EnvelopeError` carrying the
    /// body, never a success. Drata bodies pass through untouched.
    fn check_envelope(&self, method: &Method, url: &str, status: StatusCode, json: Value) -> Result<Value> {
        if self.backend != Backend::Safebase {
            return Ok(json);
        }
        let problem = match json.get("ok") {
            Some(Value::Bool(true)) => return Ok(json),
            Some(Value::Bool(false)) => "`ok` is false",
            _ => "no boolean `ok`",
        };
        let body = json.to_string();
        warn!(%method, %url, %status, problem, "SafeBase envelope rejected");
        Err(EnvelopeError {
            status,
            formatted: format!(
                "{} response on {} {} broke the envelope ({}): {}",
                self.backend, method, url, problem, body
            ),
            body,
        }
        .into())
    }
}

/// Seconds to wait after a 429: the server's `Retry-After` when it parses,
/// else the default, never past `MAX_RETRY_DELAY_SECS`.
fn retry_delay(retry_after: Option<&str>) -> u64 {
    retry_after
        .and_then(|s| s.trim().parse::<u64>().ok())
        .unwrap_or(DEFAULT_RETRY_DELAY_SECS)
        .min(MAX_RETRY_DELAY_SECS)
}

/// A 429 wait can run minutes; the `warn!` goes only to the log file, so say
/// it on stderr too or the terminal looks hung.
fn announce_retry(delay: u64, attempts: u32) {
    eprintln!(
        "drata: rate limited, waiting {}s before retry {}/{}",
        delay, attempts, MAX_RETRY_ATTEMPTS
    );
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
                s.push_str(&agent_guidance(&parsed));
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

/// The agent review's 422 (`AgentActionNotAvailableResponsePublicV2Dto`)
/// carries what to do next beside its `message`: one line per blocker
/// (`reason: message [resolution]`), the actions available now, and the
/// current step. Empty for any body without those keys.
fn agent_guidance(parsed: &Value) -> String {
    let mut out = String::new();
    if let Some(blockers) = parsed.get("blockers").and_then(|b| b.as_array()) {
        for b in blockers {
            let reason = b.get("reason").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
            let message = b.get("message").and_then(|v| v.as_str()).unwrap_or("");
            out.push_str(&format!("\nBlocker {}: {}", reason, message));
            let resolution = b.get("resolution");
            let r_type = resolution.and_then(|r| r.get("type")).and_then(|v| v.as_str());
            let r_action = resolution.and_then(|r| r.get("action")).and_then(|v| v.as_str());
            match (r_type, r_action) {
                (Some(t), Some(a)) => out.push_str(&format!(" [{}: {}]", t, a)),
                (Some(t), None) => out.push_str(&format!(" [{}]", t)),
                _ => {}
            }
        }
    }
    if let Some(actions) = parsed.get("availableActions").and_then(|a| a.as_array()) {
        let names: Vec<&str> = actions.iter().filter_map(|a| a.as_str()).collect();
        let shown = if names.is_empty() { "none".to_string() } else { names.join(", ") };
        out.push_str(&format!("\nAvailable actions: {}", shown));
    }
    if let Some(step) = parsed.get("currentStep").and_then(|s| s.as_str()) {
        out.push_str(&format!("\nCurrent step: {}", step));
    }
    out
}

pub mod presigned;

#[cfg(test)]
mod tests;
