//! `--limit` on `asset list` and `event list` (v0.2.0 shakedown: an unbounded
//! list on a large tenant fetched ~1,900 pages and tripped Drata's rate limit).
#![allow(clippy::unwrap_used)]
use clap::Parser;
use drata_cli::cli::{AssetAction, Cli, Commands, EventAction};

fn parse(argv: &[&str]) -> Result<Cli, clap::Error> {
    Cli::try_parse_from(std::iter::once("drata").chain(argv.iter().copied()))
}

#[test]
fn asset_and_event_list_take_a_limit() {
    let cli = parse(&["asset", "list", "--limit", "25"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Asset {
            action: AssetAction::List { limit: Some(25), .. }
        }
    ));
    let cli = parse(&["event", "list", "--all", "--limit", "5"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Event {
            action: EventAction::List {
                all: true,
                limit: Some(5)
            }
        }
    ));
}

#[test]
fn a_zero_or_missing_limit_is_rejected() {
    assert!(parse(&["asset", "list", "--limit", "0"]).is_err());
    assert!(parse(&["event", "list", "--limit"]).is_err());
}

#[test]
fn list_without_limit_still_fetches_everything() {
    let cli = parse(&["event", "list"]).unwrap();
    assert!(matches!(
        cli.command,
        Commands::Event {
            action: EventAction::List {
                all: false,
                limit: None
            }
        }
    ));
}
