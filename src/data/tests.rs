#![allow(clippy::unwrap_used)]
use super::*;
use std::io::Write;

fn body_file(contents: &str) -> (tempfile::TempDir, String) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("body.json");
    let mut f = std::fs::File::create(&path).unwrap();
    write!(f, "{contents}").unwrap();
    let arg = format!("@{}", path.display());
    (dir, arg)
}

#[test]
fn read_data_parses_inline_json() {
    let v = read_data(r#"{"name":"x"}"#, DataSource::Any).unwrap();
    assert_eq!(v["name"], "x");
}

#[test]
fn read_data_rejects_invalid_json() {
    let err = read_data("not json", DataSource::Any).unwrap_err();
    assert!(err.to_string().contains("not valid JSON"));
}

#[test]
fn read_data_reads_from_at_file() {
    let (dir, arg) = body_file(r#"{"k":42}"#);
    let v = read_data(&arg, DataSource::Any).unwrap();
    assert_eq!(v["k"], 42);
    drop(dir);
}

#[test]
fn read_data_missing_file_errors() {
    let err = read_data("@/no/such/file.json", DataSource::Any).unwrap_err();
    assert!(err.to_string().contains("Failed to read --data file"));
}

#[test]
fn file_or_stdin_only_rejects_inline_json() {
    let err = read_data(r#"{"secretAccessKey":"x"}"#, DataSource::FileOrStdinOnly).unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("only @file or -"), "{msg}");
    assert!(
        !msg.contains("secretAccessKey"),
        "the rejected body must not be echoed: {msg}"
    );
}

#[test]
fn file_or_stdin_only_reads_at_file() {
    let (dir, arg) = body_file(r#"{"k":1}"#);
    let v = read_data(&arg, DataSource::FileOrStdinOnly).unwrap();
    assert_eq!(v["k"], 1);
    drop(dir);
}
