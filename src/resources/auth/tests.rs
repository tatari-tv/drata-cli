#![allow(clippy::unwrap_used)]
use super::*;
use crate::config::TokenSource;

fn status(backend: Backend, source: TokenSource, allow_writes: bool) -> CredentialStatus {
    CredentialStatus {
        backend,
        region: match backend {
            Backend::Drata => Some("us".to_string()),
            Backend::Safebase => None,
        },
        allow_writes,
        token_source: source,
    }
}

fn diag(drata: TokenSource, safebase: TokenSource) -> AuthDiagnostic {
    AuthDiagnostic {
        profile: "default".to_string(),
        credentials_path: None,
        known_profiles: vec![],
        drata: status(Backend::Drata, drata, false),
        safebase: status(Backend::Safebase, safebase, false),
    }
}

#[test]
fn whoami_runs_with_each_source() {
    assert!(whoami(&diag(TokenSource::CliFlag, TokenSource::NotFound)).is_ok());
    assert!(whoami(&diag(TokenSource::EnvVar, TokenSource::EnvVar)).is_ok());
    assert!(
        whoami(&diag(
            TokenSource::Profile("default".to_string()),
            TokenSource::NotFound
        ))
        .is_ok()
    );
}

#[test]
fn auth_runs_on_fresh_install() {
    // Regression: `drata auth` must work where no key is configured.
    assert!(auth(&diag(TokenSource::NotFound, TokenSource::NotFound)).is_ok());
}

#[test]
fn whoami_drata_lines_keep_their_wording_and_lead() {
    // tatari-skills vendor-eval reads `drata whoami` for the Drata write
    // status: the Drata lines must stay first and worded as before SafeBase.
    let mut d = diag(TokenSource::EnvVar, TokenSource::Profile("default".to_string()));
    d.drata.allow_writes = true;
    let text = render_diagnostic(&d, true);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(
        &lines[..5],
        &[
            "key:       found",
            "source:    DRATA_API_KEY env var",
            "profile:   default",
            "region:    us",
            "writes:    enabled",
        ]
    );
    // Exactly one unindented `writes:` line: the Drata one.
    assert_eq!(text.lines().filter(|l| l.starts_with("writes:")).count(), 1);
}

#[test]
fn whoami_reports_safebase_below_drata() {
    let mut d = diag(TokenSource::NotFound, TokenSource::EnvVar);
    d.safebase.allow_writes = true;
    let text = render_diagnostic(&d, true);
    let drata_writes = text.find("writes:    disabled").unwrap();
    let heading = text.find("safebase:\n").unwrap();
    assert!(
        heading > drata_writes,
        "SafeBase lines must sit below the Drata lines:\n{text}"
    );
    let below = &text[heading..];
    assert!(below.contains("  key:     found"), "{below}");
    assert!(below.contains("  source:  SAFEBASE_API_KEY env var"), "{below}");
    assert!(below.contains("  writes:  enabled"), "{below}");
}

#[test]
fn auth_prints_setup_help_per_missing_backend() {
    let both_missing = render_diagnostic(&diag(TokenSource::NotFound, TokenSource::NotFound), false);
    assert!(both_missing.contains("export DRATA_API_KEY=<your-key>"));
    assert!(both_missing.contains("drata safebase login --api-key <your-key>"));

    let safebase_set = render_diagnostic(&diag(TokenSource::NotFound, TokenSource::EnvVar), false);
    assert!(safebase_set.contains("export DRATA_API_KEY=<your-key>"));
    assert!(!safebase_set.contains("SAFEBASE_API_KEY=<your-key>"));

    // whoami is terse: no setup help even with nothing configured.
    let terse = render_diagnostic(&diag(TokenSource::NotFound, TokenSource::NotFound), true);
    assert!(!terse.contains("To configure"));
}
