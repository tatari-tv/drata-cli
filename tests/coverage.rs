//! Coverage test: every spec operation must be reachable.
//!
//! Reachability has two sources:
//! - **curated** typed commands (Phase 1: the vendors vertical), and
//! - the **`raw`** namespace, which reaches ANY method+path pair.
//!
//! Because `raw` routes by method+path, an operation is reachable as long as it
//! is routable (has a non-empty, upper-cased HTTP method and an absolute path).
//! This test enumerates all operations from the spec, asserts each is routable
//! (so the curated/raw split genuinely covers all 167 ops), and reports the
//! curated-coverage percentage. It FAILS if a future op becomes unroutable or if
//! a curated entry stops matching a real operation - the early-warning the
//! design doc asks for.

use drata_cli::spec::{self, Operation};

/// The operations covered by curated typed commands as of Phase 3. Each entry is
/// `(METHOD, path-template)` and MUST correspond to a real operation in the spec
/// (the test asserts this, so a spec change that renames/removes one of these
/// paths fails loudly rather than silently overstating coverage).
const CURATED: &[(&str, &str)] = &[
    // Phase 1: vendors vertical
    ("GET", "/vendors"),
    ("POST", "/vendors"),
    ("GET", "/vendors/{vendorId}"),
    ("PUT", "/vendors/{vendorId}"),
    ("DELETE", "/vendors/{vendorId}"),
    ("GET", "/vendors/{vendorId}/questionnaires"),
    ("POST", "/vendors/{vendorId}/questionnaires"),
    ("GET", "/vendors/{vendorId}/questionnaires/{questionnaireId}"),
    // Phase 3: risks
    ("GET", "/risk-registers/{riskRegisterId}/risks"),
    ("POST", "/risk-registers/{riskRegisterId}/risks"),
    ("GET", "/risk-registers/{riskRegisterId}/risks/{riskId}"),
    ("PUT", "/risk-registers/{riskRegisterId}/risks/{riskId}"),
    ("GET", "/risk-registers/{riskRegisterId}/insights"),
    // Phase 3: controls
    ("GET", "/workspaces/{workspaceId}/controls"),
    ("POST", "/workspaces/{workspaceId}/controls"),
    ("GET", "/workspaces/{workspaceId}/controls/{controlId}"),
    ("PUT", "/workspaces/{workspaceId}/controls/{controlId}"),
    ("GET", "/workspaces/{workspaceId}/controls/{controlId}/requirements"),
    ("GET", "/workspaces/{workspaceId}/controls-requirement-comparison"),
    // Phase 3: devices
    ("GET", "/devices"),
    ("GET", "/devices/{deviceId}"),
    ("GET", "/personnel/{personnelId}/devices"),
    ("GET", "/devices/{deviceId}/apps"),
    // Phase 3: personnel
    ("GET", "/personnel"),
    ("GET", "/personnel/{personnelId}"),
    ("PUT", "/personnel/{personnelId}"),
    // Phase 3: policies
    ("GET", "/policies"),
    ("POST", "/policies"),
    ("GET", "/policies/{policyId}"),
    ("PUT", "/policies/{policyId}"),
    ("GET", "/policies/{policyId}/actions"),
    ("GET", "/policies/{policyId}/policy-versions"),
    ("GET", "/policies/{policyId}/policy-versions/{policyVersionId}"),
    // Phase 3: evidence library
    ("GET", "/workspaces/{workspaceId}/evidence-library"),
    ("POST", "/workspaces/{workspaceId}/evidence-library"),
    ("GET", "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}"),
    ("PUT", "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}"),
    (
        "DELETE",
        "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}",
    ),
    (
        "GET",
        "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}/versions/{versionId}",
    ),
    // Phase 3: frameworks
    ("GET", "/workspaces/{workspaceId}/frameworks"),
    ("POST", "/workspaces/{workspaceId}/frameworks"),
    ("PUT", "/workspaces/{workspaceId}/frameworks/{frameworkId}"),
    ("GET", "/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements"),
    // Phase 3: assets
    ("GET", "/assets"),
    ("POST", "/assets"),
    ("GET", "/assets/{assetId}"),
    ("PUT", "/assets/{assetId}"),
    ("DELETE", "/assets/{assetId}"),
    // Phase 3: company
    ("GET", "/company"),
    // Phase 3: workspaces
    ("GET", "/workspaces"),
];

fn is_curated(op: &Operation) -> bool {
    CURATED.iter().any(|(m, p)| *m == op.method && *p == op.path)
}

/// An operation is reachable via `raw` iff it is routable: a known upper-cased
/// method and an absolute path template.
fn raw_reachable(op: &Operation) -> bool {
    matches!(op.method.as_str(), "GET" | "POST" | "PUT" | "DELETE") && op.path.starts_with('/')
}

#[test]
fn every_operation_is_reachable() {
    let ops = spec::operations().expect("spec operations parse");
    assert_eq!(ops.len(), 167, "expected 167 operations");

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

#[test]
fn every_curated_entry_matches_a_real_operation() {
    let ops = spec::operations().expect("spec operations parse");
    for (method, path) in CURATED {
        let found = ops.iter().any(|op| op.method == *method && op.path == *path);
        assert!(
            found,
            "curated entry `{} {}` has no matching spec operation",
            method, path
        );
    }
}

#[test]
fn reports_curated_coverage_percentage() {
    let ops = spec::operations().expect("spec operations parse");
    let total = ops.len();
    let curated = ops.iter().filter(|op| is_curated(op)).count();
    let pct = (curated as f64 / total as f64) * 100.0;

    // Phase 3 raised the bar from 8 (Phase 1 baseline) to the Phase 3 set.
    // This is a floor, not an equality: later phases raise it further.
    assert!(
        curated >= 50,
        "curated coverage dropped below the Phase 3 baseline of 50 ops (got {})",
        curated
    );
    assert_eq!(total, 167);

    println!(
        "curated coverage: {}/{} operations ({:.1}%); raw covers all {}",
        curated, total, pct, total
    );
}

#[test]
fn operation_level_tags_number_thirty_five() {
    let tags = spec::operation_tags().expect("operation tags parse");
    assert_eq!(tags.len(), 35);
    assert!(tags.contains(&"Audit Requests".to_string()));
    assert!(tags.contains(&"Procurement Connection Mappings".to_string()));
}
