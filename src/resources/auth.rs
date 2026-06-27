//! `drata login` / `logout` / `whoami` / `auth` - credential onboarding.
//!
//! These run without a fully resolved `Config`: `login`/`auth` work on a fresh
//! install where no key is set yet, so the dispatcher in `lib::run_auth` hands
//! them an `AuthDiagnostic` (or the raw CLI) instead of demanding a key.

use crate::config::{self, AuthDiagnostic, Profile, TokenSource};
use eyre::Result;
use tracing::{debug, instrument};

/// `drata login --api-key ... [--region ...] [--allow-writes]`. Persists a
/// profile to credentials.json (0600, atomic).
#[instrument(skip(api_key), fields(profile, region, allow_writes))]
pub fn login(profile: &str, api_key: &str, region: &str, allow_writes: bool) -> Result<()> {
    debug!(profile, region, allow_writes, "login");
    config::save_profile(
        profile,
        Profile {
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

/// `drata logout` - remove the active profile from credentials.json.
#[instrument]
pub fn logout(profile: &str) -> Result<()> {
    debug!(profile, "logout");
    if config::remove_profile(profile)? {
        println!("Removed profile {:?}.", profile);
    } else {
        println!("No profile named {:?} to remove.", profile);
    }
    Ok(())
}

/// `drata whoami` - report the active credential and where it resolved from,
/// without printing the key.
pub fn whoami(diag: &AuthDiagnostic) -> Result<()> {
    print_diagnostic(diag, true);
    Ok(())
}

/// `drata auth` - same diagnostic view, framed as onboarding help; works on a
/// fresh install with no key.
pub fn auth(diag: &AuthDiagnostic) -> Result<()> {
    print_diagnostic(diag, false);
    Ok(())
}

fn print_diagnostic(diag: &AuthDiagnostic, terse: bool) {
    let token_found = !matches!(diag.token_source, TokenSource::NotFound);
    let source_line = match &diag.token_source {
        TokenSource::CliFlag => "source:    --api-key flag".to_string(),
        TokenSource::EnvVar => "source:    DRATA_API_KEY env var".to_string(),
        TokenSource::Profile(p) => format!("source:    profile {:?}", p),
        TokenSource::NotFound => "source:    (none found)".to_string(),
    };

    println!("key:       {}", if token_found { "found" } else { "not found" });
    println!("{}", source_line);
    println!("profile:   {}", diag.profile);
    println!("region:    {}", diag.region.as_deref().unwrap_or("(default us)"));
    println!("writes:    {}", if diag.allow_writes { "enabled" } else { "disabled" });
    if let Some(p) = &diag.credentials_path {
        println!("creds:     {}", p.display());
    }
    if !diag.known_profiles.is_empty() {
        println!("profiles:  {}", diag.known_profiles.join(", "));
    }

    if !token_found && !terse {
        println!();
        println!("To configure a key:");
        println!("  drata login --api-key <your-key> --region us");
        println!("or set an env var:");
        println!("  export DRATA_API_KEY=<your-key>");
    }
}

#[cfg(test)]
mod tests;
