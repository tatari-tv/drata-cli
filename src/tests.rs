#![allow(clippy::unwrap_used)]
use super::*;
use cli::{Cli, Commands, VendorAction};

fn cli_with(command: Commands) -> Cli {
    Cli {
        config: None,
        api_key: None,
        region: None,
        profile: None,
        allow_writes: false,
        yes: false,
        output: None,
        log_level: None,
        command,
    }
}

#[test]
fn is_auth_command_detects_onboarding() {
    assert!(is_auth_command(&cli_with(Commands::Auth)));
    assert!(is_auth_command(&cli_with(Commands::Whoami)));
    assert!(is_auth_command(&cli_with(Commands::Logout)));
    assert!(is_auth_command(&cli_with(Commands::Login {
        api_key: "k".to_string(),
        region: "us".to_string(),
        allow_writes: false,
    })));
    assert!(!is_auth_command(&cli_with(Commands::Vendor {
        action: VendorAction::List {
            patterns: vec![],
            all: false,
            expand: vec![]
        }
    })));
}

#[test]
fn example_if_requested_for_vendor_create() {
    let cli = cli_with(Commands::Vendor {
        action: VendorAction::Create {
            name: None,
            category: None,
            risk: None,
            status: None,
            url: None,
            notes: None,
            example: true,
        },
    });
    assert!(example_if_requested(&cli).is_some());
}

#[test]
fn example_if_requested_none_for_list() {
    let cli = cli_with(Commands::Vendor {
        action: VendorAction::List {
            patterns: vec![],
            all: false,
            expand: vec![],
        },
    });
    assert!(example_if_requested(&cli).is_none());
}

#[test]
fn example_bypasses_required_path_positional() {
    use clap::Parser;
    // `--example` must work without the workspace/register positional...
    let cli = Cli::try_parse_from(["drata", "control", "create", "--example"]).expect("--example should parse");
    assert!(example_if_requested(&cli).is_some());

    // ...but omitting both the positional AND --example is a parse error.
    assert!(
        Cli::try_parse_from(["drata", "control", "create"]).is_err(),
        "workspace_id is required unless --example"
    );

    // Same contract for risk/framework/evidence create.
    for argv in [
        ["drata", "risk", "create", "--example"],
        ["drata", "framework", "create", "--example"],
        ["drata", "evidence", "create", "--example"],
    ] {
        assert!(
            Cli::try_parse_from(argv).is_ok(),
            "{argv:?} should parse with --example"
        );
    }
}

#[test]
fn log_level_parses_case_insensitively() {
    use clap::Parser;
    let cli = Cli::try_parse_from(["drata", "-l", "DEBUG", "company", "get"]).expect("DEBUG should parse");
    assert_eq!(cli.log_level.map(|l| l.as_str()), Some("debug"));
    // An invalid level is rejected.
    assert!(Cli::try_parse_from(["drata", "--log-level", "bogus", "company", "get"]).is_err());
}
