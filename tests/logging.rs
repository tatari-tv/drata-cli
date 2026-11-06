//! What the debug log records about a command: its subcommand path, never an
//! argument value. A rejected inline `--data` credential body must not reach
//! `drata.log` through a span field.

#[allow(dead_code)]
#[path = "commands/table.rs"]
mod table;

use clap::{CommandFactory, Parser};
use drata_cli::cli::{Cli, OutputFormat};
use drata_cli::config::{Config, TokenSource};
use drata_cli::confirm::default_confirm;
use std::io::Write;
use std::sync::{Arc, Mutex};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::prelude::*;

const SECRET: &str = "PROBESECRET-not-a-real-key";

/// A `MakeWriter` into a shared buffer, so a test reads what the fmt layer wrote.
#[derive(Clone, Default)]
struct Captured(Arc<Mutex<Vec<u8>>>);

impl Captured {
    fn text(&self) -> String {
        String::from_utf8(self.0.lock().unwrap().clone()).unwrap()
    }
}

impl Write for Captured {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for Captured {
    type Writer = Captured;
    fn make_writer(&'a self) -> Self::Writer {
        self.clone()
    }
}

fn test_config() -> Config {
    Config {
        api_key: "test-key".to_string(),
        region: "us".to_string(),
        allow_writes: true,
        profile: "test".to_string(),
        output_format: OutputFormat::Json,
        token_source: TokenSource::CliFlag,
    }
}

fn parse(argv: &[&str]) -> Cli {
    Cli::try_parse_from(
        std::iter::once("drata")
            .chain(argv.iter().copied())
            .chain(std::iter::once("--yes")),
    )
    .expect("argv parses")
}

/// Run `drata <argv> --yes` through `run` (the binary's entry) with a
/// DEBUG-level fmt layer on this thread, the layer `main` installs. Returns
/// the command's result and the formatted log. The inline bodies here are
/// rejected before any request, so the client's real base URL is never hit.
async fn run_logged(argv: &[&str]) -> (eyre::Result<()>, String) {
    let captured = Captured::default();
    let subscriber = tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(captured.clone())
                .with_ansi(false),
        )
        .with(tracing_subscriber::EnvFilter::new("debug"));
    let _guard = tracing::subscriber::set_default(subscriber);
    let cli = parse(argv);
    let config = test_config();
    let result = drata_cli::run(&cli, &config, default_confirm(cli.yes)).await;
    (result, captured.text())
}

#[tokio::test]
async fn a_rejected_inline_credential_never_reaches_the_log() {
    let body = format!(r#"{{"secretAccessKey":"{SECRET}"}}"#);
    let cases: [(Vec<&str>, &str); 2] = [
        (
            vec!["connection", "aws", "create", "--data", &body],
            "command=connection aws create",
        ),
        (
            vec![
                "vendor",
                "security-review",
                "provide-trust-center-access-link",
                "1",
                "2",
                "--data",
                &body,
            ],
            "command=vendor security-review provide-trust-center-access-link",
        ),
    ];
    for (argv, span) in cases {
        let (result, log) = run_logged(&argv).await;
        assert!(result.is_err(), "{argv:?}: inline --data is rejected");
        assert!(!log.contains(SECRET), "{argv:?}: the secret reached the log:\n{log}");
        // The span is in the log, so the check above read real span output.
        assert!(log.contains(span), "{argv:?}: `{span}` missing from\n{log}");
    }
}

#[test]
fn command_path_names_the_subcommands_only() {
    let body = format!(r#"{{"secretAccessKey":"{SECRET}"}}"#);
    let cli = parse(&["connection", "aws", "create", "--data", &body]);
    let path = drata_cli::command_path(&cli.command);
    assert_eq!(path, "connection aws create");
    // The parsed command does carry the secret: the path is what drops it.
    assert!(format!("{:?}", cli.command).contains(SECRET));
}

/// The subcommand tokens at the head of `argv`, read off the clap tree: the
/// path `command_path` must report, found without the `Debug` text it walks.
fn argv_path(argv: &[&str]) -> String {
    let root = Cli::command();
    let mut current = &root;
    let mut path = Vec::new();
    for token in argv {
        match current.find_subcommand(token) {
            Some(sub) => {
                path.push(sub.get_name().to_string());
                current = sub;
            }
            None => break,
        }
    }
    path.join(" ")
}

#[test]
fn command_path_matches_the_argv_of_every_command_table_row() {
    let mut wrong = Vec::new();
    for row in table::rows() {
        let cli = parse(row.argv);
        let want = argv_path(row.argv);
        let got = drata_cli::command_path(&cli.command);
        if got != want {
            wrong.push(format!("{:?}: want `{want}`, got `{got}`", row.argv));
        }
    }
    assert!(wrong.is_empty(), "command_path drift:\n{}", wrong.join("\n"));
}
