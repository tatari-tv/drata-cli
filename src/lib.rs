#![deny(clippy::unwrap_used)]
#![deny(dead_code)]
#![deny(unused_variables)]

pub mod cli;
pub mod client;
pub mod config;
pub mod filter;
pub mod output;
pub mod raw;
pub mod resources;
pub mod spec;

use cli::{Cli, Commands};
use client::DrataClient;
use config::{AuthDiagnostic, Config, DEFAULT_PROFILE};
use eyre::Result;
use tracing::instrument;

/// Returns the skeleton to print if the invoked command is a `--example`
/// request that should bypass API/auth setup, `None` otherwise.
///
/// Curated `--example` skeletons are hand-written and infallible; the `raw`
/// namespace derives its skeleton from the spec, which can fail (unknown
/// method/path, or an operation with no JSON body), hence the inner `Result`.
pub fn example_if_requested(cli: &Cli) -> Option<Result<String>> {
    match &cli.command {
        Commands::Vendor { action } => resources::vendor::example_if_requested(action).map(|s| Ok(s.to_string())),
        Commands::Raw(args) => raw::example_if_requested(args),
        _ => None,
    }
}

/// True if the command is an auth/onboarding command that must run without a
/// configured key (so we bypass `Config::load`).
pub fn is_auth_command(cli: &Cli) -> bool {
    matches!(
        cli.command,
        Commands::Login { .. } | Commands::Logout | Commands::Whoami | Commands::Auth
    )
}

/// The profile name the user selected (CLI > env > default). Used by auth
/// commands that mutate the credentials file.
fn selected_profile(cli: &Cli) -> String {
    cli.profile
        .clone()
        .or_else(|| std::env::var("DRATA_PROFILE").ok())
        .unwrap_or_else(|| DEFAULT_PROFILE.to_string())
}

/// Dispatch the auth/onboarding commands. Safe to call when no key is
/// configured.
pub fn run_auth(cli: &Cli, diag: &AuthDiagnostic) -> Result<()> {
    match &cli.command {
        Commands::Login {
            api_key,
            region,
            allow_writes,
        } => resources::auth::login(&selected_profile(cli), api_key, region, *allow_writes),
        Commands::Logout => resources::auth::logout(&selected_profile(cli)),
        Commands::Whoami => resources::auth::whoami(diag),
        Commands::Auth => resources::auth::auth(diag),
        _ => Err(eyre::eyre!("run_auth called on non-auth command")),
    }
}

#[instrument(skip_all, fields(command = ?cli.command))]
pub async fn run(cli: &Cli, config: &Config) -> Result<()> {
    let client = DrataClient::new(config.api_key.clone(), &config.region, config.allow_writes)?;

    match &cli.command {
        Commands::Vendor { action } => {
            resources::vendor::handle(action, &client, config).await?;
        }
        Commands::Raw(args) => {
            raw::handle(args, &client, config).await?;
        }
        // Auth commands normally take the no-key bypass in main; if they reach
        // `run`, a key is configured. Re-derive the diagnostic to report source.
        Commands::Login { .. } | Commands::Logout | Commands::Whoami | Commands::Auth => {
            let diag = AuthDiagnostic::load(cli)?;
            run_auth(cli, &diag)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
