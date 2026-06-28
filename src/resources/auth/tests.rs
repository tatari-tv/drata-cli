#![allow(clippy::unwrap_used)]
use super::*;
use crate::config::TokenSource;

fn diag(source: TokenSource) -> AuthDiagnostic {
    AuthDiagnostic {
        profile: "default".to_string(),
        region: Some("us".to_string()),
        allow_writes: false,
        token_source: source,
        credentials_path: None,
        known_profiles: vec![],
    }
}

#[test]
fn whoami_runs_with_each_source() {
    assert!(whoami(&diag(TokenSource::CliFlag)).is_ok());
    assert!(whoami(&diag(TokenSource::EnvVar)).is_ok());
    assert!(whoami(&diag(TokenSource::Profile("default".to_string()))).is_ok());
}

#[test]
fn auth_runs_on_fresh_install() {
    // Regression: `drata auth` must work where no key is configured.
    assert!(auth(&diag(TokenSource::NotFound)).is_ok());
}
