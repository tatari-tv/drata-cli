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
        action: VendorAction::List { patterns: vec![] }
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
        action: VendorAction::List { patterns: vec![] },
    });
    assert!(example_if_requested(&cli).is_none());
}
