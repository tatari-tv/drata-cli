#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{Cli, Commands};
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
        }
        Self { _lock: lock, dir }
    }

    fn path(&self) -> &std::path::Path {
        self.dir.path()
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
    let config = Config::load(&make_cli(Some("cli-key"))).unwrap();
    assert_eq!(config.api_key, "cli-key");
    assert_eq!(config.token_source, TokenSource::CliFlag);
    drop(env);
}

#[test]
fn test_config_from_env_key() {
    let env = Env::new();
    unsafe { std::env::set_var("DRATA_API_KEY", "env-key") };
    let config = Config::load(&make_cli(None)).unwrap();
    assert_eq!(config.api_key, "env-key");
    assert_eq!(config.token_source, TokenSource::EnvVar);
    unsafe { std::env::remove_var("DRATA_API_KEY") };
    drop(env);
}

#[test]
fn test_config_missing_key_errors() {
    let env = Env::new();
    let result = Config::load(&make_cli(None));
    assert!(result.is_err());
    let msg = format!("{}", result.unwrap_err());
    assert!(msg.contains("DRATA_API_KEY"));
    assert!(msg.contains("drata login"));
    drop(env);
}

#[test]
fn test_config_reads_profile_from_file() {
    let env = Env::new();
    save_profile(
        "default",
        Profile {
            api_key: "file-key".to_string(),
            region: "eu".to_string(),
            allow_writes: true,
        },
    )
    .unwrap();

    let config = Config::load(&make_cli(None)).unwrap();
    assert_eq!(config.api_key, "file-key");
    assert_eq!(config.region, "eu");
    assert!(config.allow_writes);
    assert_eq!(config.token_source, TokenSource::Profile("default".to_string()));
    drop(env);
}

#[test]
fn test_cli_flag_beats_env_and_file() {
    let env = Env::new();
    save_profile(
        "default",
        Profile {
            api_key: "file-key".to_string(),
            region: "us".to_string(),
            allow_writes: false,
        },
    )
    .unwrap();
    unsafe { std::env::set_var("DRATA_API_KEY", "env-key") };

    let config = Config::load(&make_cli(Some("cli-key"))).unwrap();
    assert_eq!(config.api_key, "cli-key");
    assert_eq!(config.token_source, TokenSource::CliFlag);
    unsafe { std::env::remove_var("DRATA_API_KEY") };
    drop(env);
}

#[test]
fn test_write_guard_off_by_default() {
    let env = Env::new();
    // A CLI/env key with no write-enabled source must NOT enable writes.
    let config = Config::load(&make_cli(Some("k"))).unwrap();
    assert!(!config.allow_writes, "writes must fail closed by default");
    drop(env);
}

#[test]
fn test_allow_writes_only_from_write_enabled_profile() {
    let env = Env::new();
    save_profile(
        "default",
        Profile {
            api_key: "k".to_string(),
            region: "us".to_string(),
            allow_writes: false,
        },
    )
    .unwrap();
    // A read-only profile keeps writes off.
    let config = Config::load(&make_cli(None)).unwrap();
    assert!(!config.allow_writes);
    drop(env);
}

#[test]
fn test_credentials_legacy_migration() {
    let (creds, migrated) = Credentials::parse(r#"{"api-key":"legacy-key","region":"apac"}"#).unwrap();
    assert!(migrated, "legacy single-key file should migrate");
    let p = creds.profiles.get("default").expect("default profile created");
    assert_eq!(p.api_key, "legacy-key");
    assert_eq!(p.region, "apac");
    assert!(!p.allow_writes);
}

#[test]
fn test_credentials_profiles_shape_not_migrated() {
    let json = r#"{"profiles":{"prod":{"api-key":"k","region":"us","allow-writes":true}}}"#;
    let (creds, migrated) = Credentials::parse(json).unwrap();
    assert!(!migrated);
    let p = creds.profiles.get("prod").unwrap();
    assert!(p.allow_writes);
}

#[test]
fn test_save_load_roundtrip_and_permissions() {
    let env = Env::new();
    save_profile(
        "prod",
        Profile {
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
    assert_eq!(creds.profiles.get("prod").unwrap().api_key, "secret");
    drop(env);
}

#[test]
fn test_remove_profile() {
    let env = Env::new();
    save_profile(
        "temp",
        Profile {
            api_key: "k".to_string(),
            region: "us".to_string(),
            allow_writes: false,
        },
    )
    .unwrap();
    assert!(remove_profile("temp").unwrap());
    // Removing again returns false (already gone).
    assert!(!remove_profile("temp").unwrap());
    drop(env);
}

#[test]
fn test_auth_diagnostic_detects_sources() {
    let env = Env::new();

    let diag = AuthDiagnostic::load(&make_cli(Some("k"))).unwrap();
    assert_eq!(diag.token_source, TokenSource::CliFlag);

    let diag = AuthDiagnostic::load(&make_cli(None)).unwrap();
    assert_eq!(diag.token_source, TokenSource::NotFound);

    save_profile(
        "default",
        Profile {
            api_key: "k".to_string(),
            region: "us".to_string(),
            allow_writes: false,
        },
    )
    .unwrap();
    let diag = AuthDiagnostic::load(&make_cli(None)).unwrap();
    assert_eq!(diag.token_source, TokenSource::Profile("default".to_string()));
    assert!(diag.known_profiles.contains(&"default".to_string()));
    drop(env);
}
