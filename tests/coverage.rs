//! Coverage test: every spec operation must be reachable.
//!
//! Reachability has two sources:
//! - **curated** typed commands, counted from the command table
//!   (`tests/commands/table.rs`), and
//! - the **`raw`** namespace, which reaches ANY method+path pair.
//!
//! Because `raw` routes by method+path, an operation is reachable as long as it
//! is routable (has a non-empty, upper-cased HTTP method and an absolute path).
//! This test enumerates all operations from the spec, asserts each is routable
//! (so the curated/raw split genuinely covers every op), and reports the
//! curated-coverage percentage. It FAILS if a future op becomes unroutable or if
//! a curated entry stops matching a real operation - the early-warning the
//! design doc asks for.

use drata_cli::backend::Backend;
use drata_cli::spec::{self, Operation};

/// Operation count of the committed spec: a tripwire that fails on every spec refresh,
/// so a refresh is a deliberate, reviewed change.
const EXPECTED_OPERATIONS: usize = 308;

/// Operation count of the committed SafeBase spec (`spec/safebase-openapi-v1.json`).
const EXPECTED_SAFEBASE_OPERATIONS: usize = 47;

/// The command table (`tests/commands/table.rs`). Only `method` and `path`
/// are read here; `tests/commands.rs` reads the argv and response of each row.
#[allow(dead_code)]
#[path = "commands/table.rs"]
mod table;

/// Curated ops floor: the ops the curated commands reach as of the latest
/// phase of `docs/plan/2026-11-05-drata-v2-endpoint-coverage.md`. A floor, not
/// an equality: each phase raises it.
const CURATED_FLOOR: usize = 227;

/// The curated set, derived from the command table: every `(method, path)`
/// here is reached by a command that `tests/commands.rs` dispatches and
/// observes on the wire.
fn curated() -> Vec<(&'static str, &'static str)> {
    table::rows().map(|r| (r.method, r.path)).collect()
}

fn is_curated(op: &Operation) -> bool {
    curated().iter().any(|(m, p)| *m == op.method && *p == op.path)
}

/// An operation is reachable via `raw` iff it is routable: a known upper-cased
/// method and an absolute path template.
fn raw_reachable(op: &Operation) -> bool {
    matches!(op.method.as_str(), "GET" | "POST" | "PUT" | "PATCH" | "DELETE") && op.path.starts_with('/')
}

#[test]
fn every_operation_is_reachable() {
    let ops = spec::operations(Backend::Drata).expect("spec operations parse");
    assert_eq!(
        ops.len(),
        EXPECTED_OPERATIONS,
        "expected {EXPECTED_OPERATIONS} operations"
    );

    let unreachable: Vec<&Operation> = ops.iter().filter(|op| !is_curated(op) && !raw_reachable(op)).collect();

    assert!(
        unreachable.is_empty(),
        "these operations are reachable by neither a curated command nor raw: {:#?}",
        unreachable
            .iter()
            .map(|op| format!("{} {}", op.method, op.path))
            .collect::<Vec<_>>()
    );
}

/// SafeBase has no curated commands yet: `drata safebase raw` is the only
/// route, so every op must be routable through it. Unlike Drata's, one op
/// (`PATCH /accounts/{id}`) has no `operationId`, so none is required.
#[test]
fn every_safebase_operation_is_reachable_through_raw() {
    let ops = spec::operations(Backend::Safebase).expect("spec operations parse");
    assert_eq!(
        ops.len(),
        EXPECTED_SAFEBASE_OPERATIONS,
        "expected {EXPECTED_SAFEBASE_OPERATIONS} SafeBase operations"
    );
    let unreachable: Vec<String> = ops
        .iter()
        .filter(|op| !raw_reachable(op))
        .map(|op| format!("{} {}", op.method, op.path))
        .collect();
    assert!(
        unreachable.is_empty(),
        "unreachable SafeBase operations: {unreachable:#?}"
    );
    assert!(
        ops.iter().any(|op| op.method == "PATCH"),
        "SafeBase has PATCH operations; raw must reach them"
    );
}

#[test]
fn safebase_questionnaire_upload_matches_the_documented_multipart_fields() {
    let spec = Backend::Safebase.spec().expect("spec parses");
    let schema = &spec["paths"]["/questionnaires"]["post"]["requestBody"]["content"]["multipart/form-data"]["schema"]["properties"];
    for field in ["qnr", "accountId", "productId", "dueDate", "notes"] {
        assert!(schema.get(field).is_some(), "POST /questionnaires lacks field {field}");
    }
}

#[test]
fn every_curated_entry_matches_a_real_operation() {
    let ops = spec::operations(Backend::Drata).expect("spec operations parse");
    for (method, path) in curated() {
        let found = ops.iter().any(|op| op.method == method && op.path == path);
        assert!(
            found,
            "curated entry `{} {}` has no matching spec operation",
            method, path
        );
    }
}

#[test]
fn reports_curated_coverage_percentage() {
    let ops = spec::operations(Backend::Drata).expect("spec operations parse");
    let total = ops.len();
    let curated = ops.iter().filter(|op| is_curated(op)).count();
    let pct = (curated as f64 / total as f64) * 100.0;

    // Print before asserting so a failing run still shows the count.
    println!(
        "curated coverage: {}/{} operations ({:.1}%); raw covers all {}",
        curated, total, pct, total
    );

    assert!(
        curated >= CURATED_FLOOR,
        "curated coverage dropped below the floor of {} ops (got {})",
        CURATED_FLOOR,
        curated
    );
    assert_eq!(total, EXPECTED_OPERATIONS);
}

#[test]
fn operation_level_tags_match_the_committed_spec() {
    let tags = spec::operation_tags(Backend::Drata).expect("operation tags parse");
    assert_eq!(tags.len(), 51);
    assert!(tags.contains(&"Audit Requests".to_string()));
    assert!(tags.contains(&"Procurement Connection Mappings".to_string()));
}
