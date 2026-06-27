//! Configuration and credential management.
//!
//! Net-new relative to `pagerduty-cli`: Drata credentials live in a JSON file
//! with named **profiles**, each carrying `{ api_key, region, allow_writes }`.
//! Highlights:
//!
//! - Credentials at `xdg_config_dir()/drata/credentials.json`, mode `0600`.
//! - Atomic write (temp file in the same dir, fsync, rename) + permission
//!   enforcement on every write.
//! - Legacy single-key file (`{ "api_key": "...", "region": "..." }`) migrates
//!   to a `default` profile on load.
//! - Precedence: CLI flags > env (`DRATA_API_KEY`/`DRATA_REGION`/`DRATA_PROFILE`)
//!   > credentials file. A `TokenSource` records which layer won.
//! - A token-free `AuthDiagnostic` view backs `drata auth` on fresh installs.

use crate::cli::{Cli, OutputFormat};
use eyre::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tracing::{debug, warn};

/// Default profile name used when none is selected and for legacy migration.
pub const DEFAULT_PROFILE: &str = "default";
/// Default region when a profile or env does not specify one.
pub const DEFAULT_REGION: &str = "us";

/// XDG config dir, honoring `$XDG_CONFIG_HOME` and falling back to `$HOME/.config`.
///
/// We deliberately do NOT use `dirs::config_dir()`: it honors `$XDG_CONFIG_HOME`
/// only on Linux. On macOS it resolves via system APIs and returns
/// `~/Library/Application Support`, ignoring the env var - which would put config
/// somewhere other than the XDG path advertised in `--help`. This resolves to the
/// same XDG layout on every platform.
pub fn xdg_config_dir() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("XDG_CONFIG_HOME") {
        let path = PathBuf::from(dir);
        if path.is_absolute() {
            return Some(path);
        }
    }
    dirs::home_dir().map(|h| h.join(".config"))
}

/// XDG data dir, honoring `$XDG_DATA_HOME` and falling back to `$HOME/.local/share`.
/// Same macOS rationale as `xdg_config_dir`.
pub fn xdg_data_dir() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("XDG_DATA_HOME") {
        let path = PathBuf::from(dir);
        if path.is_absolute() {
            return Some(path);
        }
    }
    dirs::home_dir().map(|h| h.join(".local").join("share"))
}

/// Path to the credentials file: `xdg_config_dir()/drata/credentials.json`.
pub fn credentials_path() -> Option<PathBuf> {
    xdg_config_dir().map(|d| d.join("drata").join("credentials.json"))
}

/// A single named credential. `allow_writes` is a property of the key itself:
/// the write guardrail keys off this, not the profile name, so a stray
/// `DRATA_PROFILE=readwrite` cannot silently enable mutation.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct Profile {
    pub api_key: String,
    #[serde(default = "default_region")]
    pub region: String,
    #[serde(default)]
    pub allow_writes: bool,
}

fn default_region() -> String {
    DEFAULT_REGION.to_string()
}

/// On-disk credentials file: a map of profile name -> profile.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Credentials {
    #[serde(default)]
    pub profiles: BTreeMap<String, Profile>,
}

/// Legacy single-key credential shape. Older installs (and the upstream TS CLI)
/// stored a flat `{ "api_key": ..., "region": ... }`. On load we detect this and
/// migrate it into a `default` profile.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LegacyCredentials {
    api_key: String,
    #[serde(default = "default_region")]
    region: String,
    #[serde(default)]
    allow_writes: bool,
}

impl Credentials {
    /// Parse credentials JSON, transparently migrating the legacy single-key
    /// shape into a `default` profile. Returns `(creds, migrated)` where
    /// `migrated` is true if a legacy file was upgraded (so the caller may
    /// persist the new shape).
    pub fn parse(content: &str) -> Result<(Self, bool)> {
        // Prefer the profiles shape; fall back to legacy on a shape mismatch.
        if let Ok(creds) = serde_json::from_str::<Credentials>(content)
            && !creds.profiles.is_empty()
        {
            return Ok((creds, false));
        }

        if let Ok(legacy) = serde_json::from_str::<LegacyCredentials>(content) {
            debug!("migrating legacy single-key credentials into a `default` profile");
            let mut profiles = BTreeMap::new();
            profiles.insert(
                DEFAULT_PROFILE.to_string(),
                Profile {
                    api_key: legacy.api_key,
                    region: legacy.region,
                    allow_writes: legacy.allow_writes,
                },
            );
            return Ok((Credentials { profiles }, true));
        }

        // Empty/unknown content parses to an empty profile set rather than an
        // error: a fresh install has no credentials yet.
        let creds: Credentials = serde_json::from_str(content).context("Failed to parse credentials.json")?;
        Ok((creds, false))
    }

    /// Load credentials from `credentials_path()`, returning an empty set when
    /// no file exists. Migrates legacy files and rewrites them atomically.
    pub fn load() -> Result<Self> {
        let path = match credentials_path() {
            Some(p) => p,
            None => return Ok(Credentials::default()),
        };
        if !path.exists() {
            return Ok(Credentials::default());
        }

        let content = fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
        let (creds, migrated) = Self::parse(&content)?;
        if migrated {
            debug!(path = %path.display(), "persisting migrated credentials");
            creds.save()?;
        }
        Ok(creds)
    }

    /// Persist credentials atomically with `0600` permissions. Writes to a temp
    /// file in the same directory, fsyncs, then renames over the target so a
    /// crash never leaves a truncated credentials file.
    pub fn save(&self) -> Result<()> {
        let path = credentials_path().ok_or_else(|| eyre::eyre!("Cannot resolve credentials path (no HOME?)"))?;
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).with_context(|| format!("Failed to create {}", dir.display()))?;
        }
        let json = serde_json::to_string_pretty(self).context("Failed to serialize credentials")?;
        atomic_write_0600(&path, json.as_bytes())?;
        debug!(path = %path.display(), profiles = self.profiles.len(), "saved credentials");
        Ok(())
    }
}

/// Atomically write `bytes` to `path` with mode `0600`. Temp file lives in the
/// target's own directory so the rename is same-filesystem.
fn atomic_write_0600(path: &Path, bytes: &[u8]) -> Result<()> {
    let dir = path
        .parent()
        .ok_or_else(|| eyre::eyre!("credentials path has no parent directory"))?;
    let tmp = dir.join(format!(".credentials.{}.tmp", std::process::id()));

    {
        let mut f = fs::OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&tmp)
            .with_context(|| format!("Failed to open temp file {}", tmp.display()))?;
        set_mode_0600(&f)?;
        f.write_all(bytes).context("Failed to write credentials temp file")?;
        f.sync_all().context("Failed to fsync credentials temp file")?;
    }

    fs::rename(&tmp, path).with_context(|| format!("Failed to rename {} -> {}", tmp.display(), path.display()))?;
    // Enforce mode on the final file too (rename preserves the temp's mode on
    // unix, but be explicit so a future non-unix path stays correct).
    enforce_mode_0600(path)?;
    Ok(())
}

fn set_mode_0600(f: &fs::File) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = f.metadata().context("Failed to stat temp file")?.permissions();
    perms.set_mode(0o600);
    f.set_permissions(perms).context("Failed to set 0600 on temp file")?;
    Ok(())
}

fn enforce_mode_0600(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path)
        .with_context(|| format!("Failed to stat {}", path.display()))?
        .permissions();
    if perms.mode() & 0o777 != 0o600 {
        perms.set_mode(0o600);
        fs::set_permissions(path, perms).with_context(|| format!("Failed to set 0600 on {}", path.display()))?;
    }
    Ok(())
}

/// Where an API key was resolved from. Surfaced by `drata auth` without leaking
/// the key value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSource {
    CliFlag,
    EnvVar,
    Profile(String),
    NotFound,
}

/// Fully resolved runtime configuration. Carries a usable API key plus the
/// region, write flag, and output/log settings.
#[derive(Debug)]
pub struct Config {
    pub api_key: String,
    pub region: String,
    pub allow_writes: bool,
    pub profile: String,
    pub output_format: OutputFormat,
    pub log_level: String,
    pub token_source: TokenSource,
}

impl Config {
    /// Resolve config from CLI flags, env vars, and the credentials file, in
    /// that precedence order. Fails closed: no key found is an error.
    pub fn load(cli: &Cli) -> Result<Self> {
        debug!("Config::load");
        let creds = Credentials::load()?;

        let profile_name = cli
            .profile
            .clone()
            .or_else(|| std::env::var("DRATA_PROFILE").ok())
            .unwrap_or_else(|| DEFAULT_PROFILE.to_string());

        let selected = creds.profiles.get(&profile_name);

        // Precedence: CLI flag > env var > profile.
        let (api_key, token_source) = if let Some(k) = cli.api_key.clone() {
            (k, TokenSource::CliFlag)
        } else if let Ok(k) = std::env::var("DRATA_API_KEY") {
            (k, TokenSource::EnvVar)
        } else if let Some(p) = selected {
            (p.api_key.clone(), TokenSource::Profile(profile_name.clone()))
        } else {
            return Err(eyre::eyre!("{}", no_key_error_message(&profile_name)));
        };

        let region = cli
            .region
            .clone()
            .or_else(|| std::env::var("DRATA_REGION").ok())
            .or_else(|| selected.map(|p| p.region.clone()))
            .unwrap_or_else(|| DEFAULT_REGION.to_string());

        // The write flag is a property of the resolved credential. A CLI flag
        // and the profile both opt in; env/CLI key without a write-enabled
        // source stays read-only (fail closed).
        let allow_writes = cli.allow_writes
            || match &token_source {
                TokenSource::Profile(_) => selected.map(|p| p.allow_writes).unwrap_or(false),
                _ => false,
            };

        let output_format = cli.output.clone().unwrap_or(OutputFormat::Auto);

        let log_level = cli.log_level.clone().unwrap_or_else(|| "warn".to_string());

        debug!(
            profile = %profile_name,
            region = %region,
            allow_writes,
            ?token_source,
            "config resolved"
        );

        Ok(Self {
            api_key,
            region,
            allow_writes,
            profile: profile_name,
            output_format,
            log_level,
            token_source,
        })
    }
}

/// Token-free diagnostic view for `drata auth`. Constructs without a key so it
/// works on a fresh install.
#[derive(Debug)]
pub struct AuthDiagnostic {
    pub profile: String,
    pub region: Option<String>,
    pub allow_writes: bool,
    pub token_source: TokenSource,
    pub credentials_path: Option<PathBuf>,
    pub known_profiles: Vec<String>,
}

impl AuthDiagnostic {
    pub fn load(cli: &Cli) -> Result<Self> {
        let creds = Credentials::load()?;
        let path = credentials_path().filter(|p| p.exists());

        let profile_name = cli
            .profile
            .clone()
            .or_else(|| std::env::var("DRATA_PROFILE").ok())
            .unwrap_or_else(|| DEFAULT_PROFILE.to_string());

        let selected = creds.profiles.get(&profile_name);

        let token_source = if cli.api_key.is_some() {
            TokenSource::CliFlag
        } else if std::env::var("DRATA_API_KEY").is_ok() {
            TokenSource::EnvVar
        } else if selected.is_some() {
            TokenSource::Profile(profile_name.clone())
        } else {
            TokenSource::NotFound
        };

        let region = cli
            .region
            .clone()
            .or_else(|| std::env::var("DRATA_REGION").ok())
            .or_else(|| selected.map(|p| p.region.clone()));

        let allow_writes = cli.allow_writes || selected.map(|p| p.allow_writes).unwrap_or(false);

        Ok(Self {
            profile: profile_name,
            region,
            allow_writes,
            token_source,
            credentials_path: path,
            known_profiles: creds.profiles.keys().cloned().collect(),
        })
    }
}

/// Persist or update a profile in the credentials file. Used by `drata login`.
pub fn save_profile(name: &str, profile: Profile) -> Result<()> {
    debug!(profile = name, region = %profile.region, allow_writes = profile.allow_writes, "save_profile");
    let mut creds = Credentials::load()?;
    creds.profiles.insert(name.to_string(), profile);
    creds.save()
}

/// Remove a profile from the credentials file. Used by `drata logout`. Returns
/// true if a profile was removed, false if it did not exist.
pub fn remove_profile(name: &str) -> Result<bool> {
    debug!(profile = name, "remove_profile");
    let mut creds = Credentials::load()?;
    let removed = creds.profiles.remove(name).is_some();
    if removed {
        creds.save()?;
    } else {
        warn!(profile = name, "logout: profile not found");
    }
    Ok(removed)
}

/// Error shown when no API key can be resolved for the selected profile.
pub fn no_key_error_message(profile: &str) -> String {
    format!(
        "No Drata API key found for profile {:?}.\n\
         \n\
         To configure a key:\n\
         \x20\x20   drata login --profile {} --api-key <your-key> --region us\n\
         \n\
         or set an env var:\n\
         \x20\x20   export DRATA_API_KEY=<your-key>\n\
         \n\
         Run `drata auth` to verify detection.",
        profile, profile
    )
}

#[cfg(test)]
mod tests;
