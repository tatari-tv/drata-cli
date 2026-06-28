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
use drata_cli::confirm;

const PROJECT: &str = "drata";

fn setup_tracing(log_level: &str) -> Result<()> {
    // Level comes from --log-level, never RUST_LOG.
    let filter = EnvFilter::try_new(log_level).unwrap_or_else(|_| EnvFilter::new("warn"));

    // xdg_data_dir() (not dirs::data_local_dir()) so macOS also honors XDG and
    // logs land in ~/.local/share - matching the path advertised in `--help`.
    let log_dir = drata_cli::config::xdg_data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(PROJECT)
        .join("logs");

    // Log setup degrades gracefully: if the log directory cannot be created or
    // the log file cannot be opened, fall back to stderr rather than aborting
    // the command. A missing log directory is not a reason to fail an API call.
    let maybe_file = fs::create_dir_all(&log_dir)
        .and_then(|_| {
            let log_file = log_dir.join(format!("{}.log", PROJECT));
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&log_file)
                .map(|f| (f, log_file))
        })
        .ok();

    match maybe_file {
        Some((file, log_file)) => {
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
        }
        None => {
            // Fallback: log to stderr so tracing macros still work.
            tracing_subscriber::registry()
                .with(fmt::layer().with_writer(std::io::stderr).with_ansi(false))
                .with(filter)
                .init();
            // Use eprintln here since tracing is just initializing.
            eprintln!(
                "warning: could not open log file under {}; logging to stderr",
                log_dir.display()
            );
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging FIRST so every code path writes to the advertised log file -
    // including --example, auth commands, and a failed Config::load (which used to
    // return before tracing was ever initialized). Level comes only from
    // --log-level (default warn); it is not a config or credential field.
    let log_level = cli.log_level.map(|l| l.as_str()).unwrap_or("warn");
    setup_tracing(log_level).context("Failed to setup tracing")?;

    // --example prints a skeleton and exits. It never touches the API, so bypass
    // config/auth before Config::load demands a key.
    if let Some(skeleton) = drata_cli::example_if_requested(&cli) {
        let skeleton = skeleton.context("Failed to generate --example skeleton")?;
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

    let confirm_fn = confirm::default_confirm(cli.yes);
    drata_cli::run(&cli, &config, confirm_fn)
        .await
        .context("Command failed")?;

    Ok(())
}
