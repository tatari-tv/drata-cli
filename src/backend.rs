//! The two Drata-company APIs drata-cli talks to, and everything that differs
//! between them.
//!
//! - **Drata** (the GRC public API v2): region-selected base URL,
//!   `Authorization: Bearer <key>`, reqwest's default redirect policy (reqwest
//!   strips `Authorization` on a cross-host hop).
//! - **SafeBase** (the Trust Center REST API): one fixed base URL, the key in
//!   `x-sb-api-key`, and same-origin redirects only. reqwest strips
//!   `Authorization` on a cross-host redirect but not a custom header, so a
//!   cross-origin redirect would hand the SafeBase key to another host.
//!
//! Each backend also owns its embedded OpenAPI spec, parsed once.

use eyre::{Context, Result, bail};
use reqwest::header::{AUTHORIZATION, HeaderName, HeaderValue};
use reqwest::redirect;
use serde_json::Value;
use std::fmt;
use std::sync::OnceLock;
use tracing::debug;

/// Drata region base URLs from the spec `servers` block. Default is US.
pub const DRATA_US_BASE_URL: &str = "https://public-api.drata.com/public/v2";
pub const DRATA_EU_BASE_URL: &str = "https://public-api.eu.drata.com/public/v2";
pub const DRATA_APAC_BASE_URL: &str = "https://public-api.apac.drata.com/public/v2";

/// SafeBase's single base URL (the spec's one `servers` entry).
pub const SAFEBASE_BASE_URL: &str = "https://app.safebase.io/api/ext/v1/rest";

/// The header SafeBase reads its API key from.
pub const SAFEBASE_KEY_HEADER: &str = "x-sb-api-key";

/// Redirect hops a SafeBase request may follow, all within one origin.
/// Harvested from atlassian-cli `client.rs`.
const MAX_REDIRECTS: usize = 10;

/// Which API a client, spec lookup or command targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Backend {
    Drata,
    Safebase,
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Backend::Drata => "Drata",
            Backend::Safebase => "SafeBase",
        })
    }
}

impl Backend {
    /// The base URL requests are sent under. Drata resolves `region`
    /// (us/eu/apac, default us) and rejects an unknown one, so the key is never
    /// sent to an unintended endpoint. SafeBase has one region and rejects any.
    pub fn base_url(self, region: Option<&str>) -> Result<&'static str> {
        match self {
            Backend::Drata => drata_base_url(region.unwrap_or("us")),
            Backend::Safebase => match region {
                None => Ok(SAFEBASE_BASE_URL),
                Some(r) => bail!("SafeBase has a single region; a region ({:?}) applies to Drata only", r),
            },
        }
    }

    /// The one auth header this backend reads: `Authorization: Bearer <key>`
    /// for Drata, `x-sb-api-key: <key>` for SafeBase. Marked sensitive so it
    /// never shows in reqwest's debug output. The error never echoes the key.
    pub fn auth_header(self, key: &str) -> Result<(HeaderName, HeaderValue)> {
        let (name, raw) = match self {
            Backend::Drata => (AUTHORIZATION, format!("Bearer {}", key)),
            Backend::Safebase => (HeaderName::from_static(SAFEBASE_KEY_HEADER), key.to_string()),
        };
        let mut value = match HeaderValue::from_str(&raw) {
            Ok(v) => v,
            Err(_) => bail!("the {} API key holds characters not allowed in an HTTP header", self),
        };
        value.set_sensitive(true);
        Ok((name, value))
    }

    /// The redirect policy for this backend's API client. Drata keeps
    /// reqwest's default. SafeBase follows a redirect only when it stays on
    /// the origin the request started at.
    pub fn redirect_policy(self) -> redirect::Policy {
        match self {
            Backend::Drata => redirect::Policy::default(),
            Backend::Safebase => same_origin_redirects(),
        }
    }

    /// This backend's embedded OpenAPI spec, parsed on first use and cached in
    /// one `OnceLock` per backend.
    pub fn spec(self) -> Result<&'static Value> {
        static DRATA: OnceLock<Value> = OnceLock::new();
        static SAFEBASE: OnceLock<Value> = OnceLock::new();
        let (cell, json) = match self {
            Backend::Drata => (&DRATA, crate::spec::DRATA_SPEC_JSON),
            Backend::Safebase => (&SAFEBASE, crate::spec::SAFEBASE_SPEC_JSON),
        };
        if let Some(v) = cell.get() {
            return Ok(v);
        }
        let parsed: Value = serde_json::from_str(json)
            .with_context(|| format!("Failed to parse the embedded {} OpenAPI spec", self))?;
        debug!(backend = %self, "parsed embedded spec");
        Ok(cell.get_or_init(|| parsed))
    }
}

/// Resolve a Drata region string to its base URL. `Err` for an unknown region.
fn drata_base_url(region: &str) -> Result<&'static str> {
    match region.to_lowercase().as_str() {
        "us" => Ok(DRATA_US_BASE_URL),
        "eu" => Ok(DRATA_EU_BASE_URL),
        "apac" => Ok(DRATA_APAC_BASE_URL),
        other => bail!(
            "Unknown region {:?}. Valid values: us, eu, apac. \
             Set the correct region in your profile (`drata login --region <region>`) \
             or via --region / DRATA_REGION.",
            other
        ),
    }
}

/// Follow redirects only within the origin the request started at, never
/// across origins, where the SafeBase key would follow. Harvested from
/// atlassian-cli `same_origin_redirects`; the origin comes from the chain's
/// first URL rather than a captured base URL, so a test base URL is covered too.
fn same_origin_redirects() -> redirect::Policy {
    redirect::Policy::custom(|attempt| {
        let start = attempt.previous().first().map(|u| u.origin());
        if attempt.previous().len() >= MAX_REDIRECTS {
            attempt.error(format!("more than {MAX_REDIRECTS} redirects"))
        } else if start.as_ref() != Some(&attempt.url().origin()) {
            let target = attempt.url().to_string();
            attempt.error(format!("refused cross-origin redirect to {target}"))
        } else {
            attempt.follow()
        }
    })
}

#[cfg(test)]
mod tests;
