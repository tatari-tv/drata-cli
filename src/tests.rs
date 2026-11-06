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
            expand: vec![],
            status: vec![],
            category: vec![],
            vendor_type: vec![],
            vendor_type_id: vec![],
            vendor_user_id: vec![],
            sub_processor: None,
            action_required: None,
            review_deadline_status: vec![],
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
            status: vec![],
            category: vec![],
            vendor_type: vec![],
            vendor_type_id: vec![],
            vendor_user_id: vec![],
            sub_processor: None,
            action_required: None,
            review_deadline_status: vec![],
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

    // Same contract for risk/framework/evidence create: parses without the
    // positional AND actually yields a skeleton via example_if_requested.
    for argv in [
        ["drata", "risk", "create", "--example"],
        ["drata", "framework", "create", "--example"],
        ["drata", "evidence", "create", "--example"],
    ] {
        let cli = Cli::try_parse_from(argv).unwrap_or_else(|e| panic!("{argv:?} should parse with --example: {e}"));
        let skeleton = example_if_requested(&cli)
            .unwrap_or_else(|| panic!("{argv:?} should produce a skeleton"))
            .unwrap_or_else(|e| panic!("{argv:?} skeleton generation failed: {e}"));
        assert!(!skeleton.is_empty(), "{argv:?} skeleton must be non-empty");
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

#[test]
fn variant_identifiers_skip_string_and_char_literals() {
    let debug = r#"Vendor { action: Create { name: Some("Evil \"Quoted\" Name"), c: 'X' } }"#;
    assert_eq!(variant_identifiers(debug), vec!["Vendor", "Create", "Some"]);
}

#[test]
fn kebab_matches_the_clap_derive_names() {
    assert_eq!(kebab("CreateWithFile"), "create-with-file");
    assert_eq!(kebab("Vendor"), "vendor");
    assert_eq!(kebab("Base64Files"), "base64-files");
}

#[test]
fn command_path_stops_at_the_leaf_and_drops_values() {
    use cli::VendorAction;
    let command = Commands::Vendor {
        action: VendorAction::Get {
            id: "Raw".to_string(),
            expand: vec![],
        },
    };
    assert_eq!(command_path(&command), "vendor get");
    assert_eq!(
        command_path(&Commands::Company {
            action: cli::CompanyAction::Get
        }),
        "company get"
    );
}
