//! Upload bytes to a presigned URL (S3 and its kin).
//!
//! A presigned URL carries its own authorization in the query string, so this
//! transport is deliberately separate from `DrataClient`: its own
//! `reqwest::Client`, no Drata credential, no Drata base URL, and no redirects
//! (a redirect would replay the signed request somewhere the caller never
//! named). The caller supplies every header. Drata's `/upload-urls` needs only
//! `Content-Type`, equal to the `contentType` it was asked to sign.
//!
//! The query string is the signature, so it never reaches an error message or
//! a log line: errors name the URL with its query and fragment removed, and
//! carry the server's raw body (S3 answers in XML, not Drata's JSON envelope).

use eyre::{Context, Result};
use reqwest::{Client, Method, StatusCode, redirect};
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, instrument};

/// Upper bound on a presigned upload. Longer than the API timeout: the body is
/// a whole file, not a JSON document.
const PRESIGNED_TIMEOUT_SECS: u64 = 300;

/// A non-2xx answer from a presigned URL. `url` is already redacted.
#[derive(Debug, Error)]
#[error("presigned upload failed: {status} on {method} {url}: {body}")]
pub struct PresignedError {
    pub status: StatusCode,
    pub method: String,
    pub url: String,
    pub body: String,
}

/// `url` with its query string and fragment removed, safe to print or log.
pub fn redact_url(url: &str) -> &str {
    url.split(['?', '#']).next().unwrap_or_default()
}

/// Send `bytes` to the presigned `url` with exactly `headers` and nothing
/// else: no `Authorization`, no redirects, no retries.
#[instrument(skip_all, fields(%method, url = redact_url(url), bytes = bytes.len()))]
pub async fn put_presigned(method: Method, url: &str, headers: &[(String, String)], bytes: Vec<u8>) -> Result<()> {
    let redacted = redact_url(url);
    debug!(%method, url = redacted, headers = headers.len(), "presigned upload");

    let http = Client::builder()
        .timeout(Duration::from_secs(PRESIGNED_TIMEOUT_SECS))
        .redirect(redirect::Policy::none())
        .build()
        .context("Failed to build presigned upload HTTP client")?;

    let mut req = http.request(method.clone(), url);
    for (name, value) in headers {
        req = req.header(name.as_str(), value.as_str());
    }

    // reqwest's error Display embeds the full URL, signature included.
    let resp = req
        .body(bytes)
        .send()
        .await
        .map_err(reqwest::Error::without_url)
        .with_context(|| format!("presigned upload request failed: {method} {redacted}"))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        debug!(%status, url = redacted, "presigned upload rejected");
        return Err(PresignedError {
            status,
            method: method.to_string(),
            url: redacted.to_string(),
            body: body.trim().to_string(),
        }
        .into());
    }

    debug!(%status, "presigned upload complete");
    Ok(())
}

#[cfg(test)]
mod tests;
