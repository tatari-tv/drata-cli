#![deny(clippy::unwrap_used)]
#![deny(dead_code)]
#![deny(unused_variables)]

use clap::Parser;
use eyre::{Context, Result};
use std::fs;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt;
use tracing_subscriber::prelude::*;

use drata_cli::cli::Cli;
use drata_cli::config::{AuthDiagnostic, Config};

const PROJECT: &str = "drata";

fn setup_tracing(log_level: &str) -> Result<()> {
    // xdg_data_dir() (not dirs::data_local_dir()) so macOS also honors XDG and
    // logs land in ~/.local/share - matching the path advertised in `--help`.
    let log_dir = drata_cli::config::xdg_data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(PROJECT)
        .join("logs");

    fs::create_dir_all(&log_dir).context("Failed to create log directory")?;

    let log_file = log_dir.join(format!("{}.log", PROJECT));

    let file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_file)
        .context("Failed to open log file")?;

    // Level comes from --log-level, never RUST_LOG.
    let filter = EnvFilter::try_new(log_level).unwrap_or_else(|_| EnvFilter::new("warn"));

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(file)
                .with_ansi(false)
                .with_target(true)
                .with_file(true)
                .with_line_number(true),
        )
        .with(filter)
        .init();

    info!(log_path = %log_file.display(), "tracing initialized");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // --example prints a skeleton and exits. It never touches the API, so bypass
    // config/auth before Config::load demands a key.
    if let Some(skeleton) = drata_cli::example_if_requested(&cli) {
        print!("{}", skeleton);
        return Ok(());
    }

    // Auth/onboarding commands must work on a fresh install with no key, so they
    // dispatch before Config::load would fail with the missing-key error.
    if drata_cli::is_auth_command(&cli) {
        let diag = AuthDiagnostic::load(&cli).context("Failed to load auth diagnostic")?;
        return drata_cli::run_auth(&cli, &diag);
    }

    let config = Config::load(&cli).context("Failed to load configuration")?;

    setup_tracing(&config.log_level).context("Failed to setup tracing")?;

    drata_cli::run(&cli, &config).await.context("Command failed")?;

    Ok(())
}
