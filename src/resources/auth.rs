//! `drata login` / `logout` / `whoami` / `auth`, and `drata safebase login` /
//! `logout` - credential onboarding.
//!
//! These run without a fully resolved `Config`: `login`/`auth` work on a fresh
//! install where no key is set yet, so the dispatcher in `lib::run_auth` hands
//! them an `AuthDiagnostic` (or the raw CLI) instead of demanding a key. Each
//! login/logout touches only its own backend's block of the profile.

use crate::backend::Backend;
use crate::config::{self, AuthDiagnostic, CredentialStatus, DrataBlock, Removal, SafebaseBlock, TokenSource};
use eyre::Result;
use tracing::{debug, instrument};

/// `drata login --api-key ... [--region ...] [--allow-writes]`. Replaces the
/// profile's Drata block in credentials.json (0600, atomic); its SafeBase
/// block is kept.
#[instrument(skip(api_key), fields(profile, region, allow_writes))]
pub fn login(profile: &str, api_key: &str, region: &str, allow_writes: bool) -> Result<()> {
    debug!(profile, region, allow_writes, "login");
    config::save_drata(
        profile,
        DrataBlock {
            api_key: api_key.to_string(),
            region: region.to_string(),
            allow_writes,
        },
    )?;
    println!(
        "Saved profile {:?} (region {}, writes {}).",
        profile,
        region,
        if allow_writes { "enabled" } else { "disabled" }
    );
    Ok(())
}

/// `drata safebase login --api-key ... [--allow-writes]`. Replaces the
/// profile's SafeBase block; its Drata block is kept.
#[instrument(skip(api_key), fields(profile, allow_writes))]
pub fn safebase_login(profile: &str, api_key: &str, allow_writes: bool) -> Result<()> {
    debug!(profile, allow_writes, "safebase login");
    config::save_safebase(
        profile,
        SafebaseBlock {
            api_key: api_key.to_string(),
            allow_writes,
        },
    )?;
    println!(
        "Saved SafeBase credential to profile {:?} (writes {}).",
        profile,
        if allow_writes { "enabled" } else { "disabled" }
    );
    Ok(())
}

/// `drata logout` / `drata safebase logout` - remove the profile's block for
/// `backend`, and the profile once it holds no block.
#[instrument]
pub fn logout(profile: &str, backend: Backend) -> Result<()> {
    debug!(profile, %backend, "logout");
    match config::remove_block(profile, backend)? {
        Removal::Profile => println!("Removed the {backend} credential and profile {profile:?}."),
        Removal::Block => println!("Removed the {backend} credential from profile {profile:?}."),
        Removal::NotFound => println!("No {backend} credential in profile {profile:?} to remove."),
    }
    Ok(())
}

/// `drata whoami` - report the active credentials and where they resolved
/// from, without printing a key.
pub fn whoami(diag: &AuthDiagnostic) -> Result<()> {
    print!("{}", render_diagnostic(diag, true));
    Ok(())
}

/// `drata auth` - same diagnostic view, framed as onboarding help; works on a
/// fresh install with no key.
pub fn auth(diag: &AuthDiagnostic) -> Result<()> {
    print!("{}", render_diagnostic(diag, false));
    Ok(())
}

/// Where a key resolved from, as the `source:` value.
fn source_text(status: &CredentialStatus) -> String {
    match &status.token_source {
        TokenSource::CliFlag => "--api-key flag".to_string(),
        TokenSource::EnvVar => format!("{} env var", config::api_key_env_var(status.backend)),
        TokenSource::Profile(p) => format!("profile {:?}", p),
        TokenSource::NotFound => "(none found)".to_string(),
    }
}

fn found(status: &CredentialStatus) -> bool {
    !matches!(status.token_source, TokenSource::NotFound)
}

fn key(status: &CredentialStatus) -> &'static str {
    if found(status) { "found" } else { "not found" }
}

fn writes(status: &CredentialStatus) -> &'static str {
    if status.allow_writes { "enabled" } else { "disabled" }
}

/// The diagnostic text. The Drata lines come first, worded as they were
/// before SafeBase (`tatari-skills` vendor-eval reads `writes:`); the SafeBase
/// lines sit below under their own indented heading.
fn render_diagnostic(diag: &AuthDiagnostic, terse: bool) -> String {
    let drata = &diag.drata;
    let safebase = &diag.safebase;
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(&s);
        out.push('\n');
    };

    line(format!("key:       {}", key(drata)));
    line(format!("source:    {}", source_text(drata)));
    line(format!("profile:   {}", diag.profile));
    let region = drata.region.as_deref().unwrap_or("(default us)");
    line(format!("region:    {}", region));
    line(format!("writes:    {}", writes(drata)));
    if let Some(p) = &diag.credentials_path {
        line(format!("creds:     {}", p.display()));
    }
    if !diag.known_profiles.is_empty() {
        line(format!("profiles:  {}", diag.known_profiles.join(", ")));
    }

    line("safebase:".to_string());
    line(format!("  key:     {}", key(safebase)));
    line(format!("  source:  {}", source_text(safebase)));
    line(format!("  writes:  {}", writes(safebase)));

    if !terse && !found(drata) {
        line(String::new());
        line("To configure a key:".to_string());
        line("  drata login --api-key <your-key> --region us".to_string());
        line("or set an env var:".to_string());
        line("  export DRATA_API_KEY=<your-key>".to_string());
    }
    if !terse && !found(safebase) {
        line(String::new());
        line("To configure a SafeBase key:".to_string());
        line("  drata safebase login --api-key <your-key>".to_string());
        line("or set an env var:".to_string());
        line("  export SAFEBASE_API_KEY=<your-key>".to_string());
    }
    out
}

#[cfg(test)]
mod tests;
