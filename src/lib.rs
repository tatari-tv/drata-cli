#![deny(clippy::unwrap_used)]
#![deny(dead_code)]
#![deny(unused_variables)]

pub mod backend;
pub mod cli;
pub mod client;
pub mod config;
pub mod confirm;
pub mod data;
pub mod expand;
pub mod filter;
pub mod output;
pub mod query;
pub mod raw;
pub mod resources;
pub mod spec;
pub mod verify;

use backend::Backend;
use cli::{Cli, Commands, SafebaseAction};
use client::DrataClient;
use config::{AuthDiagnostic, Config};
use confirm::ConfirmFn;
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
        Commands::Vendor { action } => resources::vendor::example_if_requested(action),
        Commands::Risk { action } => resources::risk::example_if_requested(action),
        Commands::Control { action } => resources::control::example_if_requested(action),
        Commands::Policy { action } => resources::policy::example_if_requested(action),
        Commands::Evidence { action } => resources::evidence::example_if_requested(action),
        Commands::EvidenceItem { action } => resources::evidence::item::example_if_requested(action),
        Commands::AccessReview { action } => resources::access::example_if_requested(action),
        Commands::Audit { action } => resources::audit::example_if_requested(action),
        Commands::Framework { action } => resources::framework::example_if_requested(action),
        Commands::Asset { action } => resources::asset::example_if_requested(action),
        Commands::Register { action } => resources::register::example_if_requested(action),
        Commands::Monitor { action } => resources::monitor::example_if_requested(action),
        Commands::ComplianceCheckExclusion { action } => resources::compliance::example_if_requested(action),
        Commands::CustomField { action } => resources::custom::example_if_requested(action),
        Commands::InfrastructureIdentity { action } => resources::identity::example_if_requested_infrastructure(action),
        Commands::VersionControlIdentity { action } => {
            resources::identity::example_if_requested_version_control(action)
        }
        Commands::Connection { action } => resources::connection::example_if_requested(action),
        Commands::Raw(args) => raw::example_if_requested(Backend::Drata, args),
        Commands::Safebase {
            action: SafebaseAction::Raw(args),
        } => raw::example_if_requested(Backend::Safebase, args),
        _ => None,
    }
}

/// The subcommand path of a parsed command, e.g. `connection aws create`, the
/// one field the `run` and `dispatch` spans record. Built only from names in
/// the clap tree, never from an argument: the span field lands on every DEBUG
/// line under it, and a `--data` body can carry a credential.
///
/// The parsed enum keeps no subcommand names, so this walks its `Debug` text:
/// each variant identifier, kebab-cased, that names a child of the clap
/// command reached so far extends the path. Argument values never match (a
/// string prints quoted, a leaf has no children), and a non-match is dropped.
pub fn command_path(command: &Commands) -> String {
    use clap::CommandFactory;
    let debug = format!("{command:?}");
    let root = Cli::command();
    let mut current = &root;
    let mut path: Vec<&str> = Vec::new();
    for ident in variant_identifiers(&debug) {
        if let Some(sub) = current.find_subcommand(kebab(ident)) {
            path.push(sub.get_name());
            current = sub;
        }
    }
    path.join(" ")
}

/// The identifiers in `Debug` text that start with an uppercase letter (enum
/// variants, `Some`, `None`), skipping everything inside string and char
/// literals.
fn variant_identifiers(debug: &str) -> Vec<&str> {
    let bytes = debug.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            quote @ (b'"' | b'\'') => {
                i += 1;
                while i < bytes.len() && bytes[i] != quote {
                    i += if bytes[i] == b'\\' { 2 } else { 1 };
                }
                i += 1;
            }
            b if b.is_ascii_alphanumeric() || b == b'_' => {
                let start = i;
                while i < bytes.len() && (bytes[i].is_ascii_alphanumeric() || bytes[i] == b'_') {
                    i += 1;
                }
                if bytes[start].is_ascii_uppercase() {
                    out.push(&debug[start..i]);
                }
            }
            _ => i += 1,
        }
    }
    out
}

/// `CreateWithFile` -> `create-with-file`, clap's derive naming for a variant.
fn kebab(ident: &str) -> String {
    let mut out = String::with_capacity(ident.len() + 4);
    let mut prev_lower_or_digit = false;
    for c in ident.chars() {
        if c.is_ascii_uppercase() && prev_lower_or_digit {
            out.push('-');
        }
        prev_lower_or_digit = c.is_ascii_lowercase() || c.is_ascii_digit();
        out.push(c.to_ascii_lowercase());
    }
    out
}

/// True if the command is an auth/onboarding command that must run without a
/// configured key (so we bypass `Config::load`).
pub fn is_auth_command(cli: &Cli) -> bool {
    matches!(
        cli.command,
        Commands::Login { .. }
            | Commands::Logout
            | Commands::Whoami
            | Commands::Auth
            | Commands::Safebase {
                action: SafebaseAction::Login { .. } | SafebaseAction::Logout
            }
    )
}

/// The API a command calls: SafeBase for everything under `drata safebase`,
/// Drata otherwise. Picks whose credential `Config::load` resolves, so a
/// command never needs the other backend's key.
pub fn command_backend(command: &Commands) -> Backend {
    match command {
        Commands::Safebase { .. } => Backend::Safebase,
        _ => Backend::Drata,
    }
}

/// Dispatch the auth/onboarding commands. Safe to call when no key is
/// configured. `drata login`/`logout` touch only the profile's Drata block,
/// `drata safebase login`/`logout` only its SafeBase block.
pub fn run_auth(cli: &Cli, diag: &AuthDiagnostic) -> Result<()> {
    config::check_region_scope(cli, command_backend(&cli.command))?;
    let profile = config::selected_profile(cli);
    match &cli.command {
        Commands::Login {
            api_key,
            region,
            allow_writes,
        } => resources::auth::login(&profile, api_key, region, *allow_writes),
        Commands::Logout => resources::auth::logout(&profile, Backend::Drata),
        Commands::Safebase { action } => match action {
            SafebaseAction::Login { api_key, allow_writes } => {
                resources::auth::safebase_login(&profile, api_key, *allow_writes)
            }
            SafebaseAction::Logout => resources::auth::logout(&profile, Backend::Safebase),
            SafebaseAction::Raw(_)
            | SafebaseAction::Account { .. }
            | SafebaseAction::Member { .. }
            | SafebaseAction::Document { .. }
            | SafebaseAction::Request { .. }
            | SafebaseAction::Kb { .. } => Err(eyre::eyre!("run_auth called on non-auth command")),
        },
        Commands::Whoami => resources::auth::whoami(diag),
        Commands::Auth => resources::auth::auth(diag),
        _ => Err(eyre::eyre!("run_auth called on non-auth command")),
    }
}

/// Build the client for the command's backend from its resolved credential,
/// then dispatch. `config` is resolved per command (`Config::load(cli,
/// command_backend(..))`), so no command demands a key it does not use.
#[instrument(skip_all, fields(command = %command_path(&cli.command)))]
pub async fn run(cli: &Cli, config: &Config, confirm: ConfirmFn) -> Result<()> {
    let client = DrataClient::new(
        config.backend,
        config.api_key.clone(),
        config.region.as_deref(),
        config.allow_writes,
    )?;
    dispatch(cli, &client, config, &confirm).await
}

/// Route a parsed command to its handler over an already-built client. Split
/// from `run` so the command table (`tests/commands.rs`) can dispatch every
/// curated command against a wiremock base URL.
#[instrument(skip_all, fields(command = %command_path(&cli.command)))]
pub async fn dispatch(cli: &Cli, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match &cli.command {
        Commands::Vendor { action } => {
            resources::vendor::handle(action, client, config, confirm).await?;
        }
        Commands::VendorCriteria { action } => {
            resources::vendor::reference::criteria(action, client, config).await?;
        }
        Commands::VendorCriteriaCreation { action } => {
            resources::vendor::reference::criteria_creation(action, client, config).await?;
        }
        Commands::VendorQuestionnaireTemplate { action } => {
            resources::vendor::reference::template(action, client, config).await?;
        }
        Commands::VendorRiskLevel { action } => {
            resources::vendor::reference::risk_level(action, client, config).await?;
        }
        Commands::VendorSecurityReview { action } => {
            resources::vendor::reference::security_reviews(action, client, config).await?;
        }
        Commands::Risk { action } => {
            resources::risk::handle(action, client, config, confirm).await?;
        }
        Commands::Control { action } => {
            resources::control::handle(action, client, config, confirm).await?;
        }
        Commands::Device { action } => {
            resources::device::handle(action, client, config, confirm).await?;
        }
        Commands::Personnel { action } => {
            resources::personnel::handle(action, client, config, confirm).await?;
        }
        Commands::Group { action } => {
            resources::personnel::group::handle(action, client, config).await?;
        }
        Commands::ComplianceCheckExclusion { action } => {
            resources::compliance::handle(action, client, config, confirm).await?;
        }
        Commands::CustomField { action } => {
            resources::custom::handle_field(action, client, config, confirm).await?;
        }
        Commands::InfrastructureIdentity { action } => {
            resources::identity::handle_infrastructure(action, client, config, confirm).await?;
        }
        Commands::VersionControlIdentity { action } => {
            resources::identity::handle_version_control(action, client, config, confirm).await?;
        }
        Commands::Connection { action } => {
            resources::connection::handle(action, client, config, confirm).await?;
        }
        Commands::CustomConnection { action } => {
            resources::custom::handle_connection(action, client, config).await?;
        }
        Commands::Policy { action } => {
            resources::policy::handle(action, client, config, confirm).await?;
        }
        Commands::PolicyLanguage { action } => {
            resources::policy::language::handle(action, client, config).await?;
        }
        Commands::Evidence { action } => {
            resources::evidence::handle(action, client, config, confirm).await?;
        }
        Commands::EvidenceItem { action } => {
            resources::evidence::item::handle(action, client, config, confirm).await?;
        }
        Commands::AccessReview { action } => {
            resources::access::handle(action, client, config, confirm).await?;
        }
        Commands::Framework { action } => {
            resources::framework::handle(action, client, config, confirm).await?;
        }
        Commands::Asset { action } => {
            resources::asset::handle(action, client, config, confirm).await?;
        }
        Commands::Company { action } => {
            resources::company::handle(action, client, config).await?;
        }
        Commands::Workspace { action } => {
            resources::workspace::handle(action, client, config).await?;
        }
        Commands::Register { action } => {
            resources::register::handle(action, client, config, confirm).await?;
        }
        Commands::User { action } => {
            resources::user::handle(action, client, config).await?;
        }
        Commands::Monitor { action } => {
            resources::monitor::handle(action, client, config, confirm).await?;
        }
        Commands::Audit { action } => {
            resources::audit::handle(action, client, config, confirm).await?;
        }
        Commands::Event { action } => {
            resources::event::handle(action, client, config).await?;
        }
        Commands::UploadUrl { action } => {
            resources::upload::handle(action, client, config, confirm).await?;
        }
        Commands::Raw(args) => {
            raw::handle(args, client, config, confirm).await?;
        }
        Commands::Verify => {
            // Live create -> verify -> delete cycle against the real tenant.
            // The harness only ever touches a throwaway `zzz-clitest-` vendor,
            // but it does mutate, so confirm once before starting (--yes skips).
            if !confirm("POST", "/vendors (create + delete a throwaway zzz-clitest- vendor)")? {
                return Err(eyre::eyre!("aborted"));
            }
            let result = verify::run(client).await?;
            let json = serde_json::json!({
                "vendorId": result.vendor_id,
                "created": result.created,
                "verifiedList": result.verified_list,
                "verifiedGet": result.verified_get,
                "deleted": result.deleted,
                "verifiedDeleted": result.verified_deleted,
            });
            output::print_value(&json, &config.output_format);
        }
        // Auth commands normally take the no-key bypass in main; if they reach
        // `run`, a key is configured. Re-derive the diagnostic to report source.
        Commands::Safebase {
            action: SafebaseAction::Raw(args),
        } => {
            raw::handle(args, client, config, confirm).await?;
        }
        Commands::Safebase {
            action:
                action @ (SafebaseAction::Account { .. }
                | SafebaseAction::Member { .. }
                | SafebaseAction::Document { .. }
                | SafebaseAction::Request { .. }
                | SafebaseAction::Kb { .. }),
        } => {
            resources::safebase::handle(action, client, config).await?;
        }
        Commands::Login { .. }
        | Commands::Logout
        | Commands::Whoami
        | Commands::Auth
        | Commands::Safebase {
            action: SafebaseAction::Login { .. } | SafebaseAction::Logout,
        } => {
            let diag = AuthDiagnostic::load(cli)?;
            run_auth(cli, &diag)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests;
