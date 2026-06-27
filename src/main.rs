#![deny(clippy::unwrap_used)]
#![deny(dead_code)]
#![deny(unused_variables)]

use clap::Parser;
use colored::*;
use eyre::{Context, Result};
use log::info;
use std::fs;
use std::path::PathBuf;

use drata_cli::{Config, run};

mod cli;
use cli::Cli;

fn setup_logging() -> Result<()> {
    // xdg_data_dir (not the dirs data helper) so macOS also honors XDG and logs
    // land in ~/.local/share - matching the path advertised in `--help`.
    let log_dir = drata_cli::config::xdg_data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("drata-cli")
        .join("logs");

    fs::create_dir_all(&log_dir).context("Failed to create log directory")?;

    let log_file = log_dir.join("drata-cli.log");

    let target = Box::new(
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .context("Failed to open log file")?,
    );

    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(target))
        .init();

    info!("Logging initialized, writing to: {}", log_file.display());
    Ok(())
}

fn main() -> Result<()> {
    setup_logging().context("Failed to setup logging")?;

    let cli = Cli::parse();

    if cli.verbose {
        println!("{}", "🔍 Verbose mode enabled".yellow());
    }

    let config = Config::load(cli.config.as_ref()).context("Failed to load configuration")?;

    info!("Starting with config from: {:?}", cli.config);

    let result = run(&config).context("Application failed")?;
    for message in result.messages {
        println!("{}", message);
    }

    Ok(())
}
