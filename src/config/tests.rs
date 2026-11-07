#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{Cli, Commands};
use clap::Parser;
use std::sync::{Mutex, MutexGuard};
use tempfile::TempDir;

// xdg_config_dir / xdg_data_dir resolve to the XDG layout on EVERY platform -
// including macOS, where the dirs crate would return ~/Library/... and ignore the
// env vars. These tests assert the env-honoring behavior and the $HOME fallback,
// not a platform path.

// Serialize all env-var-touching tests to prevent parallel races.
static ENV_LOCK: Mutex<()> = Mutex::new(());

/// RAII test fixture: holds the ENV_LOCK (serializing env mutation) and a fresh
/// temp dir pointed at by XDG_CONFIG_HOME. Both stay alive for the test scope and
/// are released on drop, so the temp credentials dir is not deleted mid-test.
struct Env {
    _lock: MutexGuard<'static, ()>,
    dir: TempDir,
}

impl Env {
    fn new() -> Self {
        let lock = ENV_LOCK.lock().unwrap();
        let dir = TempDir::new().unwrap();
        // SAFETY: serialized by ENV_LOCK; no concurrent env mutation.
        unsafe {
            std::env::set_var("XDG_CONFIG_HOME", dir.path());
            std::env::remove_var("DRATA_API_KEY");
            std::env::remove_var("DRATA_REGION");
            std::env::remove_var("DRATA_PROFILE");
            std::env::remove_var("SAFEBASE_API_KEY");
        }
        Self { _lock: lock, dir }
    }

    fn path(&self) -> &std::path::Path {
        self.dir.path()
    }
}

impl Drop for Env {
    /// Clear the key env vars a test set, while the lock is still held, so
    /// none outlives the test.
    fn drop(&mut self) {
        // SAFETY: serialized by ENV_LOCK, which `self` still holds.
        unsafe {
            std::env::remove_var("DRATA_API_KEY");
            std::env::remove_var("DRATA_REGION");
            std::env::remove_var("SAFEBASE_API_KEY");
        }
    }
}

fn make_cli(api_key: Option<&str>) -> Cli {
    Cli {
        config: None,
        api_key: api_key.map(|s| s.to_string()),
        region: None,
        profile: None,
        allow_writes: false,
        yes: false,
        output: None,
        log_level: None,
        command: Commands::Auth,
    }
}

#[test]
fn test_xdg_config_dir_honors_env_and_falls_back() {
    let lock = ENV_LOCK.lock().unwrap();
    let prior = std::env::var("XDG_CONFIG_HOME").ok();

    let dir = TempDir::new().unwrap();
    unsafe { std::env::set_var("XDG_CONFIG_HOME", dir.path()) };
    assert_eq!(xdg_config_dir().as_deref(), Some(dir.path()));

    unsafe { std::env::remove_var("XDG_CONFIG_HOME") };
    let fallback = xdg_config_dir().unwrap();
    assert!(
        fallback.ends_with(".config"),
        "fallback should be ~/.config, got {}",
        fallback.display()
    );

    match prior {
        Some(v) => unsafe { std::env::set_var("XDG_CONFIG_HOME", v) },
        None => unsafe { std::env::remove_var("XDG_CONFIG_HOME") },
    }
    drop(lock);
}

#[test]
fn test_xdg_data_dir_honors_env_and_falls_back() {
    let lock = ENV_LOCK.lock().unwrap();
    let prior = std::env::var("XDG_DATA_HOME").ok();

    let dir = TempDir::new().unwrap();
    unsafe { std::env::set_var("XDG_DATA_HOME", dir.path()) };
    assert_eq!(xdg_data_dir().as_deref(), Some(dir.path()));

    unsafe { std::env::remove_var("XDG_DATA_HOME") };
    let fallback = xdg_data_dir().unwrap();
    assert!(
        fallback.ends_with(".local/share"),
        "fallback should be ~/.local/share, got {}",
        fallback.display()
    );

    match prior {
        Some(v) => unsafe { std::env::set_var("XDG_DATA_HOME", v) },
        None => unsafe { std::env::remove_var("XDG_DATA_HOME") },
    }
    drop(lock);
}

#[test]
fn test_config_from_cli_key() {
    let env = Env::new();
    let config = Config::load(&make_cli(Some("cli-key")), Backend::Drata).unwrap();
    assert_eq!(config.api_key, "cli-key");
    assert_eq!(config.token_source, TokenSource::CliFlag);
    drop(env);
}

#[test]
fn test_config_from_env_key() {
    let env = Env::new();
    unsafe { std::env::set_var("DRATA_API_KEY", "env-key") };
    let config = Config::load(&make_cli(None), Backend::Drata).unwrap();
    assert_eq!(config.api_key, "env-key");
    assert_eq!(config.token_source, TokenSource::EnvVar);
    unsafe { std::env::remove_var("DRATA_API_KEY") };
    drop(env);
}

#[test]
fn test_config_missing_key_errors() {
    let env = Env::new();
    let result = Config::load(&make_cli(None), Backend::Drata);
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(msg.contains("DRATA_API_KEY"));
    assert!(msg.contains("drata login"));
    drop(env);
}

#[test]
fn test_config_reads_profile_from_file() {
    let env = Env::new();
    save_drata(
        "default",
        DrataBlock {
            api_key: "file-key".to_string(),
            region: "eu".to_string(),
            allow_writes: true,
        },
    )
    .unwrap();

    let config = Config::load(&make_cli(None), Backend::Drata).unwrap();
    assert_eq!(config.api_key, "file-key");
    assert_eq!(config.region.as_deref(), Some("eu"));
    assert!(config.allow_writes);
    assert_eq!(config.token_source, TokenSource::Profile("default".to_string()));
    drop(env);
}

#[test]
fn test_cli_flag_beats_env_and_file() {
    let env = Env::new();
    save_drata(
        "default",
        DrataBlock {
            api_key: "file-key".to_string(),
            region: "us".to_string(),
            allow_writes: false,
        },
    )
    .unwrap();
    unsafe { std::env::set_var("DRATA_API_KEY", "env-key") };

    let config = Config::load(&make_cli(Some("cli-key")), Backend::Drata).unwrap();
    assert_eq!(config.api_key, "cli-key");
    assert_eq!(config.token_source, TokenSource::CliFlag);
    unsafe { std::env::remove_var("DRATA_API_KEY") };
    drop(env);
}

#[test]
fn test_write_guard_off_by_default() {
    let env = Env::new();
    // A CLI/env key with no write-enabled source must NOT enable writes.
    let config = Config::load(&make_cli(Some("k")), Backend::Drata).unwrap();
    assert!(!config.allow_writes, "writes must fail closed by default");
    drop(env);
}

#[test]
fn test_allow_writes_only_from_write_enabled_profile() {
    let env = Env::new();
    save_drata(
        "default",
        DrataBlock {
            api_key: "k".to_string(),
            region: "us".to_string(),
            allow_writes: false,
        },
    )
    .unwrap();
    // A read-only profile keeps writes off.
    let config = Config::load(&make_cli(None), Backend::Drata).unwrap();
    assert!(!config.allow_writes);
    drop(env);
}

#[test]
fn test_credentials_legacy_migration() {
    let (creds, migrated) = Credentials::parse(r#"{"api-key":"legacy-key","region":"apac"}"#).unwrap();
    assert!(migrated, "legacy single-key file should migrate");
    let p = creds.profiles["default"]
        .drata
        .as_ref()
        .expect("default profile gets a drata block");
    assert_eq!(p.api_key, "legacy-key");
    assert_eq!(p.region, "apac");
    assert!(!p.allow_writes);
}

#[test]
fn test_credentials_legacy_migration_snake_case() {
    // The real upstream/legacy files are snake_case (`api_key`, `allow_writes`).
    // serde aliases must accept them, or a genuine legacy file would parse to
    // empty credentials and never migrate.
    let json = r#"{"api_key":"legacy-key","region":"eu","allow_writes":true}"#;
    let (creds, migrated) = Credentials::parse(json).unwrap();
    assert!(migrated, "snake_case legacy single-key file should migrate");
    let p = creds.profiles["default"]
        .drata
        .as_ref()
        .expect("default profile gets a drata block");
    assert_eq!(p.api_key, "legacy-key");
    assert_eq!(p.region, "eu");
    assert!(p.allow_writes, "snake_case allow_writes should carry over");
}

#[test]
fn test_credentials_profiles_shape_not_migrated() {
    let json = r#"{"profiles":{"prod":{"api-key":"k","region":"us","allow-writes":true}}}"#;
    let (creds, migrated) = Credentials::parse(json).unwrap();
    assert!(!migrated);
    let p = creds.profiles["prod"].drata.as_ref().unwrap();
    assert!(p.allow_writes);
    assert!(creds.profiles["prod"].safebase.is_none());
    assert_eq!(creds.shape, Shape::Flat);
}

#[test]
fn test_save_load_roundtrip_and_permissions() {
    let env = Env::new();
    save_drata(
        "prod",
        DrataBlock {
            api_key: "secret".to_string(),
            region: "us".to_string(),
            allow_writes: true,
        },
    )
    .unwrap();

    let path = credentials_path().unwrap();
    assert!(path.exists());
    // The credentials file lives under the isolated XDG config dir.
    assert!(path.starts_with(env.path()));

    use std::os::unix::fs::PermissionsExt;
    let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o600, "credentials must be 0600, got {:o}", mode);

    let creds = Credentials::load().unwrap();
    assert_eq!(creds.profiles["prod"].drata.as_ref().unwrap().api_key, "secret");
    drop(env);
}

#[test]
fn test_remove_profile() {
    let env = Env::new();
    save_drata(
        "temp",
        DrataBlock {
            api_key: "k".to_string(),
            region: "us".to_string(),
            allow_writes: false,
        },
    )
    .unwrap();
    // The profile's only block goes, and the profile with it.
    assert_eq!(remove_block("temp", Backend::Drata).unwrap(), Removal::Profile);
    assert!(!Credentials::load().unwrap().profiles.contains_key("temp"));
    // Removing again finds nothing.
    assert_eq!(remove_block("temp", Backend::Drata).unwrap(), Removal::NotFound);
    drop(env);
}

#[test]
fn test_auth_diagnostic_detects_sources() {
    let env = Env::new();

    let diag = AuthDiagnostic::load(&make_cli(Some("k"))).unwrap();
    assert_eq!(diag.drata.token_source, TokenSource::CliFlag);

    let diag = AuthDiagnostic::load(&make_cli(None)).unwrap();
    assert_eq!(diag.drata.token_source, TokenSource::NotFound);

    save_drata(
        "default",
        DrataBlock {
            api_key: "k".to_string(),
            region: "us".to_string(),
            allow_writes: false,
        },
    )
    .unwrap();
    let diag = AuthDiagnostic::load(&make_cli(None)).unwrap();
    assert_eq!(diag.drata.token_source, TokenSource::Profile("default".to_string()));
    assert!(diag.known_profiles.contains(&"default".to_string()));
    drop(env);
}

#[test]
fn test_auth_diagnostic_write_status_matches_resolved_source() {
    let env = Env::new();

    // A write-enabled profile exists, but the token resolves from a CLI key.
    // The diagnostic must NOT claim writes are enabled (runtime Config::load
    // fails them closed), so the two views agree.
    save_drata(
        "default",
        DrataBlock {
            api_key: "profile-key".to_string(),
            region: "us".to_string(),
            allow_writes: true,
        },
    )
    .unwrap();

    let diag = AuthDiagnostic::load(&make_cli(Some("cli-key"))).unwrap();
    assert_eq!(diag.drata.token_source, TokenSource::CliFlag);
    assert!(
        !diag.drata.allow_writes,
        "CLI-key token must not inherit the profile's write flag"
    );

    // Resolving from the profile DOES report its write flag.
    let diag = AuthDiagnostic::load(&make_cli(None)).unwrap();
    assert_eq!(diag.drata.token_source, TokenSource::Profile("default".to_string()));
    assert!(
        diag.drata.allow_writes,
        "profile-resolved token reports the profile write flag"
    );
    drop(env);
}

// ---------------------------------------------------------------------------
// Per-backend blocks (SafeBase)
// ---------------------------------------------------------------------------

/// A pre-SafeBase flat file: two profiles, one write-enabled.
const FLAT: &str = r#"{
  "profiles": {
    "default": {
      "api-key": "drata-key",
      "region": "eu",
      "allow-writes": true
    },
    "ro": {
      "api-key": "ro-key"
    }
  }
}"#;

/// A nested file with both blocks on `default`.
const NESTED: &str = r#"{
  "profiles": {
    "default": {
      "drata": { "api-key": "drata-key", "region": "apac", "allow-writes": true },
      "safebase": { "api-key": "sb-key", "allow-writes": true }
    }
  }
}"#;

fn write_creds(content: &str) -> std::path::PathBuf {
    let path = credentials_path().unwrap();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, content).unwrap();
    path
}

/// Pin the file's mtime far in the past, so any rewrite shows as a new mtime
/// regardless of filesystem timestamp granularity.
fn age(path: &std::path::Path) -> std::time::SystemTime {
    let old = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_000_000);
    std::fs::File::options()
        .write(true)
        .open(path)
        .unwrap()
        .set_modified(old)
        .unwrap();
    std::fs::metadata(path).unwrap().modified().unwrap()
}

fn cli(argv: &[&str]) -> Cli {
    Cli::try_parse_from(std::iter::once("drata").chain(argv.iter().copied())).unwrap()
}

/// Run an auth command the way `main` does.
fn run(argv: &[&str]) -> Result<()> {
    let cli = cli(argv);
    let diag = AuthDiagnostic::load(&cli)?;
    crate::run_auth(&cli, &diag)
}

fn mode(path: &std::path::Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path).unwrap().permissions().mode() & 0o777
}

#[test]
fn flat_credentials_load_unchanged_and_file_untouched() {
    let env = Env::new();
    let path = write_creds(FLAT);
    let mtime = age(&path);

    let creds = Credentials::load().unwrap();
    let default = &creds.profiles["default"];
    assert_eq!(
        default.drata,
        Some(DrataBlock {
            api_key: "drata-key".to_string(),
            region: "eu".to_string(),
            allow_writes: true,
        })
    );
    assert_eq!(default.safebase, None);
    let ro = creds.profiles["ro"].drata.as_ref().unwrap();
    assert_eq!(
        (ro.api_key.as_str(), ro.region.as_str(), ro.allow_writes),
        ("ro-key", "us", false)
    );

    // Every read path: Drata config, SafeBase config (a no-key error), auth.
    let config = Config::load(&make_cli(None), Backend::Drata).unwrap();
    assert_eq!(config.api_key, "drata-key");
    assert_eq!(config.region.as_deref(), Some("eu"));
    assert!(config.allow_writes);
    assert_eq!(config.token_source, TokenSource::Profile("default".to_string()));
    assert!(Config::load(&make_cli(None), Backend::Safebase).is_err());
    AuthDiagnostic::load(&make_cli(None)).unwrap();

    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        FLAT,
        "a read rewrote the bytes"
    );
    assert_eq!(
        std::fs::metadata(&path).unwrap().modified().unwrap(),
        mtime,
        "a read touched the file"
    );
    assert!(!backup_path(&path).exists(), "a read wrote a backup");
    drop(env);
}

#[test]
fn nested_credentials_read_is_untouched_too() {
    let env = Env::new();
    let path = write_creds(NESTED);
    let mtime = age(&path);
    let config = Config::load(&make_cli(None), Backend::Safebase).unwrap();
    assert_eq!(config.api_key, "sb-key");
    assert_eq!(std::fs::read_to_string(&path).unwrap(), NESTED);
    assert_eq!(std::fs::metadata(&path).unwrap().modified().unwrap(), mtime);
    drop(env);
}

#[test]
fn drata_login_preserves_safebase_block() {
    let env = Env::new();
    write_creds(NESTED);
    let before = Credentials::load().unwrap().profiles["default"].safebase.clone();

    run(&["login", "--api-key", "new-drata", "--region", "us"]).unwrap();

    let profile = Credentials::load().unwrap().profiles["default"].clone();
    assert_eq!(profile.safebase, before, "drata login changed the safebase block");
    assert_eq!(
        profile.drata,
        Some(DrataBlock {
            api_key: "new-drata".to_string(),
            region: "us".to_string(),
            allow_writes: false,
        })
    );
    drop(env);
}

#[test]
fn safebase_logout_preserves_drata_block() {
    let env = Env::new();
    write_creds(NESTED);
    let before = Credentials::load().unwrap().profiles["default"].drata.clone();

    run(&["safebase", "logout"]).unwrap();

    let creds = Credentials::load().unwrap();
    let profile = &creds.profiles["default"];
    assert_eq!(profile.drata, before, "safebase logout changed the drata block");
    assert_eq!(profile.safebase, None);
    drop(env);
}

#[test]
fn safebase_login_preserves_drata_block_and_drata_logout_keeps_safebase() {
    let env = Env::new();
    write_creds(NESTED);
    run(&["safebase", "login", "--api-key", "sb-new"]).unwrap();
    let profile = Credentials::load().unwrap().profiles["default"].clone();
    assert_eq!(profile.drata.as_ref().unwrap().api_key, "drata-key");
    assert_eq!(
        profile.safebase,
        Some(SafebaseBlock {
            api_key: "sb-new".to_string(),
            allow_writes: false,
        })
    );

    run(&["logout"]).unwrap();
    let profile = Credentials::load().unwrap().profiles["default"].clone();
    assert_eq!(profile.drata, None);
    assert_eq!(profile.safebase.unwrap().api_key, "sb-new");
    drop(env);
}

#[test]
fn logout_of_last_block_removes_profile() {
    let env = Env::new();
    write_creds(NESTED);
    assert_eq!(remove_block("default", Backend::Safebase).unwrap(), Removal::Block);
    assert_eq!(remove_block("default", Backend::Safebase).unwrap(), Removal::NotFound);
    assert_eq!(remove_block("default", Backend::Drata).unwrap(), Removal::Profile);
    assert!(Credentials::load().unwrap().profiles.is_empty());
    drop(env);
}

#[test]
fn logout_with_nothing_to_remove_leaves_the_file_alone() {
    let env = Env::new();
    let path = write_creds(FLAT);
    let mtime = age(&path);
    run(&["safebase", "logout"]).unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), FLAT);
    assert_eq!(std::fs::metadata(&path).unwrap().modified().unwrap(), mtime);
    drop(env);
}

#[test]
fn first_shape_changing_save_backs_up_the_flat_file_once() {
    let env = Env::new();
    let path = write_creds(FLAT);

    run(&["safebase", "login", "--api-key", "sb-key", "--allow-writes"]).unwrap();

    let backup = backup_path(&path);
    assert!(backup.ends_with("credentials.json.pre-safebase"));
    assert_eq!(
        std::fs::read_to_string(&backup).unwrap(),
        FLAT,
        "backup must hold the original bytes"
    );
    assert_eq!(mode(&backup), 0o600);
    assert_eq!(mode(&path), 0o600);

    // The rewritten file is nested; the flat Drata credentials carried over.
    let written: Value = serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(written["profiles"]["default"]["drata"]["api-key"], "drata-key");
    assert_eq!(written["profiles"]["default"]["drata"]["region"], "eu");
    assert_eq!(written["profiles"]["default"]["safebase"]["api-key"], "sb-key");
    assert_eq!(written["profiles"]["ro"]["drata"]["api-key"], "ro-key");
    assert!(written["profiles"]["default"].get("api-key").is_none());

    // A later save of the now-nested file leaves the backup alone.
    let backup_mtime = age(&backup);
    run(&["logout", "--profile", "ro"]).unwrap();
    assert_eq!(std::fs::read_to_string(&backup).unwrap(), FLAT);
    assert_eq!(std::fs::metadata(&backup).unwrap().modified().unwrap(), backup_mtime);
    drop(env);
}

#[test]
fn legacy_migration_backs_up_the_single_key_file() {
    let env = Env::new();
    let legacy = r#"{"api_key":"legacy-key","region":"eu"}"#;
    let path = write_creds(legacy);
    let creds = Credentials::load().unwrap();
    assert_eq!(creds.profiles["default"].drata.as_ref().unwrap().api_key, "legacy-key");
    assert_eq!(std::fs::read_to_string(backup_path(&path)).unwrap(), legacy);
    // The persisted file reads back nested, so a further save is no shape change.
    let (reread, migrated) = Credentials::parse(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert!(!migrated);
    assert_eq!(reread.shape, Shape::Nested);
    drop(env);
}

#[test]
fn mixed_flat_and_nested_profile_is_an_error_naming_it() {
    let json = r#"{"profiles":{
        "good":{"drata":{"api-key":"k"}},
        "mixed":{"api-key":"k","safebase":{"api-key":"s"}}
    }}"#;
    let err = Credentials::parse(json).unwrap_err().to_string();
    assert!(err.contains("\"mixed\""), "{err}");
    assert!(err.contains("mixes flat fields"), "{err}");
}

#[test]
fn profile_with_no_credential_is_an_error_naming_it() {
    let err = Credentials::parse(r#"{"profiles":{"empty":{}}}"#)
        .unwrap_err()
        .to_string();
    assert!(err.contains("\"empty\""), "{err}");
}

#[test]
fn missing_block_never_falls_back_to_the_other_backend() {
    let env = Env::new();
    write_creds(r#"{"profiles":{"default":{"safebase":{"api-key":"sb-key"}}}}"#);
    let err = Config::load(&make_cli(None), Backend::Drata).unwrap_err().to_string();
    assert!(err.contains("No Drata API key found for profile \"default\""), "{err}");

    write_creds(r#"{"profiles":{"default":{"drata":{"api-key":"drata-key"}}}}"#);
    let err = Config::load(&make_cli(None), Backend::Safebase)
        .unwrap_err()
        .to_string();
    assert!(
        err.contains("No SafeBase API key found for profile \"default\""),
        "{err}"
    );
    assert!(err.contains("SAFEBASE_API_KEY"), "{err}");
    assert!(err.contains("drata safebase login --profile default"), "{err}");
    // The Drata env var does not stand in for SafeBase either.
    unsafe { std::env::set_var("DRATA_API_KEY", "drata-env") };
    assert!(Config::load(&make_cli(None), Backend::Safebase).is_err());
    drop(env);
}

#[test]
fn safebase_key_precedence_flag_env_profile() {
    let env = Env::new();
    write_creds(NESTED);

    let config = Config::load(&make_cli(None), Backend::Safebase).unwrap();
    assert_eq!(config.api_key, "sb-key");
    assert_eq!(config.token_source, TokenSource::Profile("default".to_string()));
    assert_eq!(config.region, None);
    assert!(config.allow_writes, "profile key carries its block's write flag");

    unsafe { std::env::set_var("SAFEBASE_API_KEY", "sb-env") };
    let config = Config::load(&make_cli(None), Backend::Safebase).unwrap();
    assert_eq!(config.api_key, "sb-env");
    assert_eq!(config.token_source, TokenSource::EnvVar);
    assert!(
        !config.allow_writes,
        "an env key never inherits the profile's write flag"
    );

    let config = Config::load(&make_cli(Some("sb-flag")), Backend::Safebase).unwrap();
    assert_eq!(config.api_key, "sb-flag");
    assert_eq!(config.token_source, TokenSource::CliFlag);
    assert!(!config.allow_writes);

    let mut with_writes = make_cli(Some("sb-flag"));
    with_writes.allow_writes = true;
    assert!(Config::load(&with_writes, Backend::Safebase).unwrap().allow_writes);
    drop(env);
}

#[test]
fn region_is_drata_only() {
    let env = Env::new();
    unsafe { std::env::set_var("SAFEBASE_API_KEY", "sb-env") };
    // DRATA_REGION is exported for Drata; SafeBase ignores it.
    unsafe { std::env::set_var("DRATA_REGION", "eu") };
    let config = Config::load(&make_cli(None), Backend::Safebase).unwrap();
    assert_eq!(config.region, None);

    // The --region flag on a SafeBase command is an error.
    let mut with_region = make_cli(None);
    with_region.region = Some("eu".to_string());
    let err = Config::load(&with_region, Backend::Safebase).unwrap_err().to_string();
    assert!(err.contains("--region applies to Drata commands only"), "{err}");
    let err = run(&["safebase", "login", "--api-key", "k", "--region", "eu"])
        .unwrap_err()
        .to_string();
    assert!(err.contains("--region applies to Drata commands only"), "{err}");
    assert!(
        !credentials_path().unwrap().exists(),
        "a rejected login wrote credentials"
    );
    drop(env);
}

#[test]
fn auth_flags_apply_to_drata_lines_only() {
    let env = Env::new();
    write_creds(NESTED);
    let mut cli = make_cli(Some("drata-flag"));
    cli.allow_writes = true;
    let diag = AuthDiagnostic::load(&cli).unwrap();
    assert_eq!(diag.drata.token_source, TokenSource::CliFlag);
    assert!(diag.drata.allow_writes);
    assert_eq!(diag.drata.region.as_deref(), Some("apac"));
    // SafeBase still resolves from its own block, with its own write flag.
    assert_eq!(diag.safebase.token_source, TokenSource::Profile("default".to_string()));
    assert!(diag.safebase.allow_writes);

    unsafe { std::env::set_var("SAFEBASE_API_KEY", "sb-env") };
    let diag = AuthDiagnostic::load(&cli).unwrap();
    assert_eq!(diag.safebase.token_source, TokenSource::EnvVar);
    assert!(!diag.safebase.allow_writes, "--allow-writes is Drata's on auth/whoami");
    drop(env);
}
