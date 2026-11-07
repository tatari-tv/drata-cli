//! Configuration and credential management.
//!
//! Net-new relative to `pagerduty-cli`: credentials live in a JSON file with
//! named **profiles**. Each profile holds one block per backend, both optional,
//! at least one present: `drata` (`{api-key, region, allow-writes}`) and
//! `safebase` (`{api-key, allow-writes}`). Highlights:
//!
//! - Credentials at `xdg_config_dir()/drata/credentials.json`, mode `0600`.
//! - Atomic write (temp file in the same dir, fsync, rename) + permission
//!   enforcement on every write.
//! - A flat profile (`{api-key, region, allow-writes}`, the pre-SafeBase
//!   shape) reads as a `drata` block, in memory only: a read never rewrites
//!   it. Flat fields beside a nested block is a parse error naming the profile.
//! - Saves write the nested shape. Saving over a flat (or legacy) file copies
//!   it to `credentials.json.pre-safebase` (0600) first.
//! - Legacy single-key file (`{ "api_key": "...", "region": "..." }`) migrates
//!   to a `default` profile on load.
//! - Key precedence per backend: CLI flag > env (`DRATA_API_KEY` |
//!   `SAFEBASE_API_KEY`) > the profile's block for that backend. A missing
//!   block never falls back to the other backend's key. `DRATA_PROFILE` picks
//!   the profile for both. A `TokenSource` records which layer won.
//! - A token-free `AuthDiagnostic` view backs `drata auth` on fresh installs.

use crate::backend::Backend;
use crate::cli::{Cli, OutputFormat};
use eyre::{Context, Result, bail, eyre};
use serde::{Deserialize, Serialize};
use serde_json::Value;
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

/// Env var that overrides the profile's Drata key.
pub const DRATA_API_KEY_ENV: &str = "DRATA_API_KEY";
/// Env var that overrides the profile's SafeBase key.
pub const SAFEBASE_API_KEY_ENV: &str = "SAFEBASE_API_KEY";
/// Suffix of the copy taken before the first save that changes the file's shape.
const BACKUP_SUFFIX: &str = ".pre-safebase";

/// The env var that carries `backend`'s API key.
pub fn api_key_env_var(backend: Backend) -> &'static str {
    match backend {
        Backend::Drata => DRATA_API_KEY_ENV,
        Backend::Safebase => SAFEBASE_API_KEY_ENV,
    }
}

/// Where the pre-nested credentials file is copied before its first
/// shape-changing save: `credentials.json.pre-safebase` beside it. Rolling
/// back to a binary that reads only the flat shape is one `mv`.
pub fn backup_path(credentials: &Path) -> PathBuf {
    let mut name = credentials.as_os_str().to_os_string();
    name.push(BACKUP_SUFFIX);
    PathBuf::from(name)
}

/// A profile's Drata credential. `allow_writes` is a property of the key
/// itself: the write guardrail keys off this, not the profile name, so a stray
/// `DRATA_PROFILE=readwrite` cannot silently enable mutation.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct DrataBlock {
    pub api_key: String,
    #[serde(default = "default_region")]
    pub region: String,
    #[serde(default)]
    pub allow_writes: bool,
}

/// A profile's SafeBase credential. SafeBase has one region, so no `region`.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct SafebaseBlock {
    pub api_key: String,
    #[serde(default)]
    pub allow_writes: bool,
}

fn default_region() -> String {
    DEFAULT_REGION.to_string()
}

/// A named profile: one optional credential block per backend. Stored nested
/// (`{"drata": {...}, "safebase": {...}}`); a profile with neither is removed.
#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
pub struct Profile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drata: Option<DrataBlock>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safebase: Option<SafebaseBlock>,
}

/// The key and write flag of one backend's block, borrowed from a profile.
#[derive(Debug, Clone, Copy)]
struct BlockKey<'a> {
    api_key: &'a str,
    allow_writes: bool,
}

impl Profile {
    /// True when the profile holds no block for either backend.
    pub fn is_empty(&self) -> bool {
        self.drata.is_none() && self.safebase.is_none()
    }

    /// This profile's block for `backend`, if it has one. Never the other
    /// backend's block.
    fn block(&self, backend: Backend) -> Option<BlockKey<'_>> {
        match backend {
            Backend::Drata => self.drata.as_ref().map(|b| BlockKey {
                api_key: &b.api_key,
                allow_writes: b.allow_writes,
            }),
            Backend::Safebase => self.safebase.as_ref().map(|b| BlockKey {
                api_key: &b.api_key,
                allow_writes: b.allow_writes,
            }),
        }
    }
}

/// One profile as written on disk, before its shape is checked: the flat
/// pre-SafeBase fields and the nested blocks, all optional.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct ProfileOnDisk {
    api_key: Option<String>,
    region: Option<String>,
    allow_writes: Option<bool>,
    drata: Option<DrataBlock>,
    safebase: Option<SafebaseBlock>,
}

impl ProfileOnDisk {
    /// The in-memory profile, and whether it was stored flat. A flat profile
    /// becomes a `drata` block; flat fields beside a nested block, or no
    /// credential at all, is an error naming the profile.
    fn into_profile(self, name: &str) -> Result<(Profile, bool)> {
        let flat = self.api_key.is_some() || self.region.is_some() || self.allow_writes.is_some();
        let nested = self.drata.is_some() || self.safebase.is_some();
        match (flat, nested) {
            (true, true) => bail!(
                "profile {name:?} in credentials.json mixes flat fields (api-key, region, allow-writes) \
                 with a nested \"drata\"/\"safebase\" block. Move the flat fields into its \"drata\" block."
            ),
            (true, false) => {
                let api_key = self
                    .api_key
                    .ok_or_else(|| eyre!("profile {name:?} in credentials.json has no api-key"))?;
                let drata = DrataBlock {
                    api_key,
                    region: self.region.unwrap_or_else(default_region),
                    allow_writes: self.allow_writes.unwrap_or(false),
                };
                Ok((
                    Profile {
                        drata: Some(drata),
                        safebase: None,
                    },
                    true,
                ))
            }
            (false, true) => Ok((
                Profile {
                    drata: self.drata,
                    safebase: self.safebase,
                },
                false,
            )),
            (false, false) => bail!(
                "profile {name:?} in credentials.json holds no credential: it needs a \"drata\" or \"safebase\" block"
            ),
        }
    }
}

/// The shape the credentials file had when it was read. A save writes the
/// nested shape; saving over `Legacy` or `Flat` backs the file up first.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum Shape {
    /// No file, or a file with no profiles.
    #[default]
    Empty,
    /// The single-key file `{api_key, region}`.
    Legacy,
    /// At least one profile stored flat.
    Flat,
    /// Every profile stored as per-backend blocks.
    Nested,
}

/// On-disk credentials file: a map of profile name -> profile.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Credentials {
    pub profiles: BTreeMap<String, Profile>,
    #[serde(skip)]
    shape: Shape,
}

/// The file's top level, profiles left raw so each one parses (and fails)
/// under its own name.
#[derive(Debug, Deserialize)]
struct CredentialsOnDisk {
    #[serde(default)]
    profiles: BTreeMap<String, Value>,
}

/// Legacy single-key credential shape. Older installs (and the upstream TS CLI)
/// stored a flat `{ "api_key": ..., "region": ... }`. On load we detect this and
/// migrate it into a `default` profile.
///
/// The struct is kebab-case for consistency with the current shapes, but the
/// real legacy files are snake_case (`api_key`, `allow_writes`), so each field
/// carries a snake_case `alias` - otherwise a genuine legacy file would parse to
/// empty credentials and never migrate.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct LegacyCredentials {
    #[serde(alias = "api_key")]
    api_key: String,
    #[serde(default = "default_region")]
    region: String,
    #[serde(default, alias = "allow_writes")]
    allow_writes: bool,
}

impl Credentials {
    /// Parse credentials JSON. Flat profiles read as `drata` blocks; the
    /// legacy single-key shape migrates into a `default` profile. Returns
    /// `(creds, migrated)` where `migrated` is true only for the legacy file
    /// (so the caller may persist the new shape). A flat profile never sets
    /// `migrated`: a read does not rewrite it.
    pub fn parse(content: &str) -> Result<(Self, bool)> {
        let value: Value = serde_json::from_str(content).context("Failed to parse credentials.json")?;
        let has_profiles = value
            .get("profiles")
            .and_then(Value::as_object)
            .is_some_and(|m| !m.is_empty());
        if has_profiles {
            return Ok((Self::from_profiles(value)?, false));
        }

        if let Ok(legacy) = serde_json::from_value::<LegacyCredentials>(value.clone()) {
            debug!("migrating legacy single-key credentials into a `default` profile");
            let mut profiles = BTreeMap::new();
            profiles.insert(
                DEFAULT_PROFILE.to_string(),
                Profile {
                    drata: Some(DrataBlock {
                        api_key: legacy.api_key,
                        region: legacy.region,
                        allow_writes: legacy.allow_writes,
                    }),
                    safebase: None,
                },
            );
            return Ok((
                Credentials {
                    profiles,
                    shape: Shape::Legacy,
                },
                true,
            ));
        }

        // No profiles and not legacy: an empty set (a fresh install), or a
        // parse error for anything that is not a credentials object.
        Ok((Self::from_profiles(value)?, false))
    }

    /// Parse the profiles shape, profile by profile, recording whether any
    /// profile was stored flat.
    fn from_profiles(value: Value) -> Result<Self> {
        let on_disk: CredentialsOnDisk = serde_json::from_value(value).context("Failed to parse credentials.json")?;
        let mut profiles = BTreeMap::new();
        let mut any_flat = false;
        for (name, raw) in on_disk.profiles {
            let parsed: ProfileOnDisk = serde_json::from_value(raw)
                .with_context(|| format!("profile {name:?} in credentials.json is malformed"))?;
            let (profile, flat) = parsed.into_profile(&name)?;
            any_flat |= flat;
            profiles.insert(name, profile);
        }
        let shape = if profiles.is_empty() {
            Shape::Empty
        } else if any_flat {
            Shape::Flat
        } else {
            Shape::Nested
        };
        Ok(Credentials { profiles, shape })
    }

    /// Load credentials from `credentials_path()`, returning an empty set when
    /// no file exists. Never rewrites the file, except to persist a migrated
    /// legacy single-key file.
    pub fn load() -> Result<Self> {
        let path = match credentials_path() {
            Some(p) => p,
            None => return Ok(Credentials::default()),
        };
        if !path.exists() {
            return Ok(Credentials::default());
        }

        let content = fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
        let (mut creds, migrated) = Self::parse(&content)?;
        if migrated {
            debug!(path = %path.display(), "persisting migrated credentials");
            creds.save()?;
            creds.shape = Shape::Nested;
        }
        Ok(creds)
    }

    /// Persist credentials atomically with `0600` permissions, in the nested
    /// shape. Writes to a temp file in the same directory, fsyncs, then renames
    /// over the target so a crash never leaves a truncated credentials file.
    /// When the file on disk is flat or legacy, it is copied to
    /// `credentials.json.pre-safebase` first.
    pub fn save(&self) -> Result<()> {
        let path = credentials_path().ok_or_else(|| eyre!("Cannot resolve credentials path (no HOME?)"))?;
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir).with_context(|| format!("Failed to create {}", dir.display()))?;
        }
        if matches!(self.shape, Shape::Legacy | Shape::Flat) && path.exists() {
            back_up(&path)?;
        }
        let json = serde_json::to_string_pretty(self).context("Failed to serialize credentials")?;
        atomic_write_0600(&path, json.as_bytes())?;
        debug!(path = %path.display(), profiles = self.profiles.len(), "saved credentials");
        Ok(())
    }
}

/// Copy the credentials file, as it is on disk, to its `.pre-safebase` path
/// (0600, atomic). Runs before a save turns a flat or legacy file nested. An
/// existing backup is replaced: the file about to be rewritten is the newest
/// flat state, the one a rollback needs.
fn back_up(path: &Path) -> Result<()> {
    let bytes = fs::read(path).with_context(|| format!("Failed to read {}", path.display()))?;
    let backup = backup_path(path);
    atomic_write_0600(&backup, &bytes)?;
    warn!(backup = %backup.display(), "credentials.json changes shape; original copied");
    Ok(())
}

/// Atomically write `bytes` to `path` with mode `0600`. Temp file lives in the
/// target's own directory so the rename is same-filesystem.
fn atomic_write_0600(path: &Path, bytes: &[u8]) -> Result<()> {
    let dir = path
        .parent()
        .ok_or_else(|| eyre!("credentials path has no parent directory"))?;
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

/// The profile name the user selected: `--profile` > `DRATA_PROFILE` >
/// `default`. One profile serves both backends.
pub fn selected_profile(cli: &Cli) -> String {
    cli.profile
        .clone()
        .or_else(|| std::env::var("DRATA_PROFILE").ok())
        .unwrap_or_else(|| DEFAULT_PROFILE.to_string())
}

/// `--region` is Drata's. On a SafeBase command it is an error, not ignored.
pub fn check_region_scope(cli: &Cli, backend: Backend) -> Result<()> {
    if backend == Backend::Safebase && cli.region.is_some() {
        bail!("--region applies to Drata commands only; SafeBase has a single region");
    }
    Ok(())
}

/// Resolve `backend`'s key: `flag` > its env var > the profile's block for
/// that backend. `None` when none is set; never the other backend's key.
fn resolve_key(
    flag: Option<String>,
    backend: Backend,
    block: Option<BlockKey<'_>>,
    profile: &str,
) -> Option<(String, TokenSource)> {
    if let Some(k) = flag {
        return Some((k, TokenSource::CliFlag));
    }
    if let Ok(k) = std::env::var(api_key_env_var(backend)) {
        return Some((k, TokenSource::EnvVar));
    }
    block.map(|b| (b.api_key.to_string(), TokenSource::Profile(profile.to_string())))
}

/// The write flag is a property of the resolved credential. `--allow-writes`
/// opts in; the block's `allow-writes` counts only when the key came from that
/// block. A flag or env key without `--allow-writes` stays read-only.
fn write_enabled(flag: bool, source: &TokenSource, block: Option<BlockKey<'_>>) -> bool {
    flag || (matches!(source, TokenSource::Profile(_)) && block.is_some_and(|b| b.allow_writes))
}

/// Fully resolved runtime configuration for one backend. Carries a usable API
/// key plus the region (Drata only), write flag, and output settings.
#[derive(Debug)]
pub struct Config {
    pub backend: Backend,
    pub api_key: String,
    /// Drata region (us, eu, apac). `None` for SafeBase, which has one.
    pub region: Option<String>,
    pub allow_writes: bool,
    pub profile: String,
    pub output_format: OutputFormat,
    pub token_source: TokenSource,
}

impl Config {
    /// Resolve `backend`'s credential from CLI flags, env vars, and the
    /// credentials file, in that precedence order. Fails closed: no key for
    /// this backend is an error naming the backend and profile, even when the
    /// profile holds the other backend's key. SafeBase rejects `--region` and
    /// ignores `DRATA_REGION`.
    pub fn load(cli: &Cli, backend: Backend) -> Result<Self> {
        debug!(%backend, "Config::load");
        check_region_scope(cli, backend)?;
        let creds = Credentials::load()?;
        let profile_name = selected_profile(cli);
        let selected = creds.profiles.get(&profile_name);
        let block = selected.and_then(|p| p.block(backend));

        let (api_key, token_source) = resolve_key(cli.api_key.clone(), backend, block, &profile_name)
            .ok_or_else(|| eyre!("{}", no_key_error_message(backend, &profile_name)))?;

        let region = match backend {
            Backend::Drata => Some(
                cli.region
                    .clone()
                    .or_else(|| std::env::var("DRATA_REGION").ok())
                    .or_else(|| selected.and_then(|p| p.drata.as_ref()).map(|d| d.region.clone()))
                    .unwrap_or_else(|| DEFAULT_REGION.to_string()),
            ),
            Backend::Safebase => None,
        };

        let allow_writes = write_enabled(cli.allow_writes, &token_source, block);
        let output_format = cli.output.clone().unwrap_or(OutputFormat::Auto);

        debug!(
            %backend,
            profile = %profile_name,
            ?region,
            allow_writes,
            ?token_source,
            "config resolved"
        );

        Ok(Self {
            backend,
            api_key,
            region,
            allow_writes,
            profile: profile_name,
            output_format,
            token_source,
        })
    }
}

/// One backend's credential as `drata auth` reports it: where the key
/// resolved from and whether writes are on. Never the key itself.
#[derive(Debug)]
pub struct CredentialStatus {
    pub backend: Backend,
    pub token_source: TokenSource,
    /// Drata region (`None` = the default, us). Always `None` for SafeBase.
    pub region: Option<String>,
    pub allow_writes: bool,
}

/// Token-free diagnostic view for `drata auth` / `drata whoami`, covering both
/// backends. Constructs without a key so it works on a fresh install.
#[derive(Debug)]
pub struct AuthDiagnostic {
    pub profile: String,
    pub credentials_path: Option<PathBuf>,
    pub known_profiles: Vec<String>,
    /// Resolves from `--api-key`, `DRATA_API_KEY`, then the profile;
    /// `--region` and `--allow-writes` apply here.
    pub drata: CredentialStatus,
    /// Resolves from `SAFEBASE_API_KEY`, then the profile. The global flags
    /// on `auth`/`whoami` are Drata's, so none of them apply here.
    pub safebase: CredentialStatus,
}

impl AuthDiagnostic {
    pub fn load(cli: &Cli) -> Result<Self> {
        let creds = Credentials::load()?;
        let path = credentials_path().filter(|p| p.exists());
        let profile_name = selected_profile(cli);
        let selected = creds.profiles.get(&profile_name);

        // Mirror `Config::load`: a block's write flag only counts when the key
        // actually resolved from that block, so the diagnostic never claims
        // writes are enabled when the runtime would fail them closed.
        let drata_block = selected.and_then(|p| p.block(Backend::Drata));
        let drata_source = source_of(cli.api_key.clone(), Backend::Drata, drata_block, &profile_name);
        let drata = CredentialStatus {
            backend: Backend::Drata,
            region: cli
                .region
                .clone()
                .or_else(|| std::env::var("DRATA_REGION").ok())
                .or_else(|| selected.and_then(|p| p.drata.as_ref()).map(|d| d.region.clone())),
            allow_writes: write_enabled(cli.allow_writes, &drata_source, drata_block),
            token_source: drata_source,
        };

        let safebase_block = selected.and_then(|p| p.block(Backend::Safebase));
        let safebase_source = source_of(None, Backend::Safebase, safebase_block, &profile_name);
        let safebase = CredentialStatus {
            backend: Backend::Safebase,
            region: None,
            allow_writes: write_enabled(false, &safebase_source, safebase_block),
            token_source: safebase_source,
        };

        Ok(Self {
            profile: profile_name,
            credentials_path: path,
            known_profiles: creds.profiles.keys().cloned().collect(),
            drata,
            safebase,
        })
    }
}

/// Where `backend`'s key would resolve from, without keeping the key.
fn source_of(flag: Option<String>, backend: Backend, block: Option<BlockKey<'_>>, profile: &str) -> TokenSource {
    resolve_key(flag, backend, block, profile)
        .map(|(_, source)| source)
        .unwrap_or(TokenSource::NotFound)
}

/// Read, change one profile, and save. The other block on that profile and
/// every other profile stay as read.
fn update_profile(name: &str, change: impl FnOnce(&mut Profile)) -> Result<()> {
    let mut creds = Credentials::load()?;
    change(creds.profiles.entry(name.to_string()).or_default());
    creds.save()
}

/// Replace the profile's `drata` block, leaving its `safebase` block alone.
/// Used by `drata login`.
pub fn save_drata(name: &str, block: DrataBlock) -> Result<()> {
    debug!(profile = name, region = %block.region, allow_writes = block.allow_writes, "save_drata");
    update_profile(name, |p| p.drata = Some(block))
}

/// Replace the profile's `safebase` block, leaving its `drata` block alone.
/// Used by `drata safebase login`.
pub fn save_safebase(name: &str, block: SafebaseBlock) -> Result<()> {
    debug!(profile = name, allow_writes = block.allow_writes, "save_safebase");
    update_profile(name, |p| p.safebase = Some(block))
}

/// What a logout did to the profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Removal {
    /// The profile had no block for that backend; the file is untouched.
    NotFound,
    /// The block is gone; the profile keeps its other block.
    Block,
    /// The block was the profile's last, so the profile is gone too.
    Profile,
}

/// Remove one backend's block from a profile, and the profile with it when no
/// block is left. Used by `drata logout` and `drata safebase logout`. Saves
/// only when something was removed.
pub fn remove_block(name: &str, backend: Backend) -> Result<Removal> {
    debug!(profile = name, %backend, "remove_block");
    let mut creds = Credentials::load()?;
    let removed = match creds.profiles.get_mut(name) {
        Some(p) => match backend {
            Backend::Drata => p.drata.take().is_some(),
            Backend::Safebase => p.safebase.take().is_some(),
        },
        None => false,
    };
    if !removed {
        warn!(profile = name, %backend, "logout: no credential block to remove");
        return Ok(Removal::NotFound);
    }
    let outcome = if creds.profiles.get(name).is_some_and(Profile::is_empty) {
        creds.profiles.remove(name);
        Removal::Profile
    } else {
        Removal::Block
    };
    creds.save()?;
    Ok(outcome)
}

/// Error shown when no API key can be resolved for `backend` on the selected
/// profile.
pub fn no_key_error_message(backend: Backend, profile: &str) -> String {
    let login = match backend {
        Backend::Drata => format!("drata login --profile {profile} --api-key <your-key> --region us"),
        Backend::Safebase => format!("drata safebase login --profile {profile} --api-key <your-key>"),
    };
    format!(
        "No {backend} API key found for profile {profile:?}.\n\
         \n\
         To configure a key:\n\
         \x20\x20   {login}\n\
         \n\
         or set an env var:\n\
         \x20\x20   export {env}=<your-key>\n\
         \n\
         Run `drata auth` to verify detection.",
        env = api_key_env_var(backend),
    )
}

#[cfg(test)]
mod tests;
