#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn always_yes_confirms() {
    let f = always_yes();
    assert!(f("POST", "/vendors").unwrap());
    assert!(f("DELETE", "/vendors/1").unwrap());
}

#[test]
fn always_no_aborts() {
    let f = always_no();
    assert!(!f("POST", "/vendors").unwrap());
    assert!(!f("PUT", "/vendors/1").unwrap());
}

#[test]
fn fail_closed_errors_without_tty() {
    let f = fail_closed();
    let err = f("POST", "/vendors").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("not a TTY"), "got: {msg}");
    assert!(msg.contains("POST"), "got: {msg}");
    assert!(msg.contains("/vendors"), "got: {msg}");
}

#[test]
fn fail_closed_includes_path_in_message() {
    let f = fail_closed();
    let err = f("DELETE", "/assets/42").unwrap_err();
    let msg = err.to_string();
    assert!(msg.contains("DELETE"), "got: {msg}");
    assert!(msg.contains("/assets/42"), "got: {msg}");
}
