//! The command table: one row per curated operation.
//!
//! Each row names a spec operation (`method`, `path` template), the argv of the
//! curated command that reaches it, and the canned wiremock response. Shared by
//! two test crates through `#[path]`:
//!
//! - `tests/commands.rs` parses every argv, dispatches it against wiremock and
//!   asserts exactly one request with this method and a path matching the
//!   template, and that every leaf subcommand has a row.
//! - `tests/coverage.rs` derives its curated set from these rows, so the
//!   coverage count is the number of commands proven to reach their op.
//!
//! One row per op: a second row for the same op would let a deleted row hide
//! behind its twin, so `tests/commands.rs` rejects duplicates.

/// The canned response the mock server returns for a row.
#[derive(Debug, Clone, Copy)]
pub enum Response {
    /// `200` with an empty cursor page: `{"data": [], "pagination": {"cursor": null}}`.
    Page,
    /// `200` with a single object: `{"id": 1}`.
    Object,
    /// `204 No Content`.
    Empty,
    /// `202 Accepted` with the agent review's acceptance body:
    /// `{"securityReviewId": 2, "workflowId": 1, "action": "start_assessment"}`.
    Accepted,
    /// Any status with a literal JSON body, for ops whose exit rule reads the
    /// response (per-item results, recipe validation).
    Json(u16, &'static str),
}

/// One excluded item: the per-item results body the exclusion ops answer.
pub const ITEM_RESULTS: &str = r#"{"data": [{"statusCode": 201, "outcome": "EXCLUDED", "data": {"id": "sg-1", "connectionId": 1, "exclusionIds": [10]}}]}"#;

/// One personnel excluded: the per-target results body the compliance-check
/// exclusion create op answers.
pub const EXCLUDE_RESULTS: &str = r#"{"results": [{"targetId": "1", "outcome": "EXCLUDED", "exclusionIds": [10]}], "successCount": 1, "skippedCount": 0}"#;

/// One personnel's exclusions deleted: the body the delete op answers.
pub const DELETE_RESULTS: &str = r#"{"results": [{"targetId": "1", "outcome": "DELETED", "exclusionIds": [10]}], "successCount": 1, "skippedCount": 0}"#;

/// A valid recipe: the validation body `validate-recipe` exits 0 on.
pub const RECIPE_VALID: &str = r#"{"isValid": true, "errors": []}"#;

/// A recipe for the `--recipe` rows.
pub const RECIPE: &str = r#"{"assertion": "nofail", "allConnections": true, "providers": []}"#;

/// The evidence-files upload answer: `upload` prints its `fileKey`.
pub const EVIDENCE_FILE: &str = r#"{"fileKey": "acct/evidence-library/1/f.txt", "originalFilename": "f.txt", "mimeType": "text/plain", "fileSize": 1}"#;

#[derive(Debug)]
pub struct Row {
    pub method: &'static str,
    pub path: &'static str,
    pub argv: &'static [&'static str],
    pub response: Response,
}

/// A readable file for the multipart upload rows.
pub const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE");

/// `--data` for `provide-trust-center-access-link`, which takes its body from a
/// file or stdin only.
pub const ACCESS_LINK_DATA: &str = concat!("@", env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/access-link.json");

const fn row(method: &'static str, path: &'static str, argv: &'static [&'static str], response: Response) -> Row {
    Row {
        method,
        path,
        argv,
        response,
    }
}

use Response::{Accepted, Empty, Json, Object, Page};

#[path = "vendor.rs"]
mod vendor;

/// Every row: the vendor rows (`tests/commands/vendor.rs`) and the rest. Split
/// across two files only to stay under the 1500-line limit.
pub fn rows() -> impl Iterator<Item = &'static Row> {
    vendor::VENDOR_ROWS.iter().chain(NON_VENDOR_ROWS)
}

/// Every row whose command is not under `vendor`, `vendor-*` or `vendor security-review`.
const NON_VENDOR_ROWS: &[Row] = &[
    // risks
    row(
        "GET",
        "/risk-registers/{riskRegisterId}/risks",
        &["risk", "list", "1"],
        Page,
    ),
    row(
        "POST",
        "/risk-registers/{riskRegisterId}/risks",
        &["risk", "create", "1", "--title", "t", "--description", "d"],
        Object,
    ),
    row(
        "GET",
        "/risk-registers/{riskRegisterId}/risks/{riskId}",
        &["risk", "get", "1", "2"],
        Object,
    ),
    row(
        "PUT",
        "/risk-registers/{riskRegisterId}/risks/{riskId}",
        &["risk", "update", "1", "2", "--title", "t"],
        Object,
    ),
    row(
        "GET",
        "/risk-registers/{riskRegisterId}/insights",
        &["risk", "insights", "1"],
        Object,
    ),
    row(
        "POST",
        "/risk-registers/{riskRegisterId}/risks/{riskId}/documents",
        &["risk", "upload", "1", "2", "--file", FIXTURE],
        Object,
    ),
    // controls
    row(
        "GET",
        "/workspaces/{workspaceId}/controls",
        &["control", "list", "1"],
        Page,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/controls",
        &[
            "control",
            "create",
            "1",
            "--name",
            "n",
            "--description",
            "d",
            "--code",
            "C-1",
        ],
        Object,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/controls/{controlId}",
        &["control", "get", "1", "2"],
        Object,
    ),
    row(
        "PUT",
        "/workspaces/{workspaceId}/controls/{controlId}",
        &["control", "update", "1", "2", "--name", "n"],
        Object,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/controls/{controlId}/requirements",
        &["control", "requirements", "1", "2"],
        Page,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/controls-requirement-comparison",
        &["control", "compare", "1", "--control-ids", "2", "3"],
        Object,
    ),
    // devices
    row("GET", "/devices", &["device", "list"], Page),
    row("GET", "/devices/{deviceId}", &["device", "get", "1"], Object),
    row(
        "GET",
        "/personnel/{personnelId}/devices",
        &["device", "for-personnel", "1"],
        Page,
    ),
    row("GET", "/devices/{deviceId}/apps", &["device", "apps", "1"], Page),
    row(
        "POST",
        "/devices/{deviceId}/documents",
        &[
            "device",
            "upload",
            "1",
            "--file",
            FIXTURE,
            "--type",
            "PASSWORD_MANAGER_EVIDENCE",
        ],
        Object,
    ),
    // personnel
    row("GET", "/personnel", &["personnel", "list"], Page),
    row("GET", "/personnel/{personnelId}", &["personnel", "get", "1"], Object),
    row(
        "PUT",
        "/personnel/{personnelId}",
        &["personnel", "update", "1", "--started-at", "2026-01-01"],
        Object,
    ),
    row("GET", "/personnel-search", &["personnel", "search"], Page),
    row(
        "GET",
        "/workspaces/{workspaceId}/personnel-search",
        &["personnel", "search", "--workspace-id", "1"],
        Page,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/scoped-personnel-counts",
        &["personnel", "group", "counts", "1"],
        Object,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/scoped-personnel-groups",
        &["personnel", "group", "list", "1"],
        Page,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/scoped-personnel-groups",
        &["personnel", "group", "add", "1", "42"],
        Object,
    ),
    row(
        "PUT",
        "/workspaces/{workspaceId}/scoped-personnel-groups",
        &["personnel", "group", "replace", "1", "--group-id", "1", "2"],
        Empty,
    ),
    row(
        "DELETE",
        "/workspaces/{workspaceId}/scoped-personnel-groups/{groupId}",
        &["personnel", "group", "remove", "1", "42"],
        Empty,
    ),
    // groups
    row("GET", "/groups", &["group", "list"], Page),
    // compliance-check exclusions
    row(
        "GET",
        "/workspaces/{workspaceId}/compliance-check-exclusions",
        &["compliance-check-exclusion", "list", "1"],
        Page,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/compliance-check-exclusions",
        &[
            "compliance-check-exclusion",
            "create",
            "1",
            "--target-id",
            "1",
            "--compliance-check",
            "offboarding",
            "--reason",
            "left",
            "--start-date",
            "2026-07-01T00:00:00Z",
        ],
        Json(201, EXCLUDE_RESULTS),
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/compliance-check-exclusions/action-delete",
        &[
            "compliance-check-exclusion",
            "remove",
            "1",
            "--target-id",
            "1",
            "--compliance-check",
            "offboarding",
        ],
        Json(200, DELETE_RESULTS),
    ),
    // custom fields
    row("GET", "/custom-field-definitions", &["custom-field", "list"], Page),
    row(
        "GET",
        "/custom-field-definitions/{customFieldId}",
        &["custom-field", "get", "1"],
        Object,
    ),
    row(
        "POST",
        "/custom-field-definitions",
        &["custom-field", "create", "--name", "n", "--field-type", "text"],
        Object,
    ),
    // custom connections
    row(
        "GET",
        "/workspaces/{workspaceId}/custom-connections/{connectionId}/resources/{resourceId}/attributes",
        &["custom-connection", "resource-attribute", "1", "2", "3"],
        Object,
    ),
    // policies
    row("GET", "/policies", &["policy", "list"], Page),
    row(
        "POST",
        "/policies",
        &[
            "policy",
            "create",
            "--name",
            "n",
            "--owner-id",
            "1",
            "--source-type",
            "EXTERNAL",
            "--description",
            "d",
            "--renewal-date",
            "2027-01-01",
        ],
        Object,
    ),
    row("GET", "/policies/{policyId}", &["policy", "get", "1"], Object),
    row(
        "PUT",
        "/policies/{policyId}",
        &["policy", "update", "1", "--name", "n"],
        Object,
    ),
    row("GET", "/policies/{policyId}/actions", &["policy", "actions", "1"], Page),
    row(
        "GET",
        "/policies/{policyId}/policy-versions",
        &["policy", "versions", "1"],
        Page,
    ),
    row(
        "GET",
        "/policies/{policyId}/policy-versions/{policyVersionId}",
        &["policy", "version", "1", "2"],
        Object,
    ),
    row(
        "POST",
        "/policies/{policyId}/policy-versions",
        &[
            "policy",
            "version",
            "create",
            "1",
            "--source-type",
            "EXTERNAL",
            "--external-file-id",
            "5",
            "--owner-id",
            "1",
            "--material-change",
            "false",
            "--renewal-date",
            "2027-01-01",
        ],
        Object,
    ),
    row(
        "PUT",
        "/policies/{policyId}/policy-versions/{policyVersionId}/file",
        &["policy", "version", "replace-file", "1", "2", "--file", FIXTURE],
        Object,
    ),
    row(
        "GET",
        "/policies/{policyId}/policy-versions/{policyVersionId}/languages",
        &["policy-language", "list", "1", "2"],
        Page,
    ),
    row(
        "GET",
        "/policy-language-settings",
        &["policy-language", "settings"],
        Object,
    ),
    row(
        "GET",
        "/policy-language-versions",
        &["policy-language", "versions"],
        Page,
    ),
    row(
        "GET",
        "/policy-language-versions/{policyLanguageVersionId}",
        &["policy-language", "get", "1"],
        Object,
    ),
    // evidence library
    row(
        "GET",
        "/workspaces/{workspaceId}/evidence-library",
        &["evidence", "list", "1"],
        Page,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/evidence-library",
        &["evidence", "create", "1", "--name", "n"],
        Object,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}",
        &["evidence", "get", "1", "2"],
        Object,
    ),
    row(
        "PUT",
        "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}",
        &["evidence", "update", "1", "2", "--name", "n"],
        Object,
    ),
    row(
        "DELETE",
        "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}",
        &["evidence", "remove", "1", "2"],
        Empty,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}/versions/{versionId}",
        &["evidence", "get-version", "1", "2", "3"],
        Object,
    ),
    // frameworks
    row(
        "GET",
        "/workspaces/{workspaceId}/frameworks",
        &["framework", "list", "1"],
        Page,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/frameworks",
        &[
            "framework",
            "create",
            "1",
            "--name",
            "n",
            "--short-name",
            "N",
            "--description",
            "d",
        ],
        Object,
    ),
    row(
        "PUT",
        "/workspaces/{workspaceId}/frameworks/{frameworkId}",
        &["framework", "update", "1", "2", "--name", "n"],
        Object,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements",
        &["framework", "requirements", "1", "2"],
        Page,
    ),
    // assets
    row("GET", "/assets", &["asset", "list"], Page),
    row(
        "POST",
        "/assets",
        &[
            "asset",
            "create",
            "--name",
            "n",
            "--description",
            "d",
            "--asset-type",
            "VIRTUAL",
            "--asset-class-types",
            "SOFTWARE",
            "--owner-id",
            "1",
        ],
        Object,
    ),
    row("GET", "/assets/{assetId}", &["asset", "get", "1"], Object),
    row(
        "PUT",
        "/assets/{assetId}",
        &["asset", "update", "1", "--name", "n"],
        Object,
    ),
    row("DELETE", "/assets/{assetId}", &["asset", "remove", "1"], Empty),
    // company and workspaces
    row("GET", "/company", &["company", "get"], Object),
    row("GET", "/workspaces", &["workspace", "list"], Page),
    // risk registers
    row("GET", "/risk-registers", &["register", "list"], Page),
    row(
        "POST",
        "/risk-registers",
        &["register", "create", "--name", "n"],
        Object,
    ),
    row(
        "GET",
        "/risk-registers/{riskRegisterId}",
        &["register", "get", "1"],
        Object,
    ),
    row(
        "PUT",
        "/risk-registers/{riskRegisterId}",
        &["register", "update", "1", "--name", "n"],
        Object,
    ),
    row(
        "DELETE",
        "/risk-registers/{riskRegisterId}",
        &["register", "remove", "1"],
        Empty,
    ),
    // users and roles
    row("GET", "/users", &["user", "list"], Page),
    row("GET", "/users/{userId}", &["user", "get", "1"], Object),
    row("GET", "/roles", &["user", "roles"], Page),
    row("GET", "/roles/{roleId}", &["user", "role", "1"], Object),
    row("GET", "/roles/{roleId}/users", &["user", "role-users", "1"], Page),
    // monitoring tests
    row(
        "GET",
        "/workspaces/{workspaceId}/monitoring-tests",
        &["monitor", "list", "1"],
        Page,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}",
        &["monitor", "get", "1", "2"],
        Object,
    ),
    row(
        "PUT",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}",
        &["monitor", "update", "1", "2", "--name", "n"],
        Object,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions",
        &["monitor", "exclusions", "1", "2"],
        Page,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/failures",
        &["monitor", "failures", "1", "2"],
        Page,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/passes",
        &["monitor", "passes", "1", "2"],
        Page,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/monitoring-tests",
        &["monitor", "create", "1", "--name", "n", "--recipe", RECIPE],
        Object,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/monitoring-tests/action-validate-recipe",
        &["monitor", "validate-recipe", "1", "--recipe", RECIPE],
        Json(200, RECIPE_VALID),
    ),
    row(
        "PUT",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/recipe",
        &["monitor", "recipe", "1", "2", "--recipe", RECIPE],
        Object,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/action-retest",
        &["monitor", "retest", "1", "2"],
        Object,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions",
        &[
            "monitor",
            "exclusion",
            "create",
            "1",
            "2",
            "--reason",
            "r",
            "--failure",
            "sg-1",
            "1",
        ],
        Json(201, ITEM_RESULTS),
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions/action-delete",
        &["monitor", "exclusion", "remove", "1", "2", "--exclusion-id", "10"],
        Json(200, ITEM_RESULTS),
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions",
        &["monitor", "codebase-exclusion", "list", "1", "2"],
        Page,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions",
        &[
            "monitor",
            "codebase-exclusion",
            "create",
            "1",
            "2",
            "--reason",
            "r",
            "--finding-id",
            "f",
        ],
        Json(201, ITEM_RESULTS),
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions/action-delete",
        &[
            "monitor",
            "codebase-exclusion",
            "remove",
            "1",
            "2",
            "--exclusion-id",
            "10",
        ],
        Json(200, ITEM_RESULTS),
    ),
    // audits
    row("GET", "/workspaces/{workspaceId}/audits", &["audit", "list", "1"], Page),
    row(
        "GET",
        "/workspaces/{workspaceId}/audits/{auditId}",
        &["audit", "get", "1", "2"],
        Object,
    ),
    // events
    row("GET", "/events", &["event", "list"], Page),
    row("GET", "/events/{eventId}", &["event", "get", "1"], Object),
    // uploads
    row(
        "POST",
        "/upload-urls",
        &["upload-url", "create", "--purpose", "evidence", "--workspace", "1"],
        Object,
    ),
    // tickets, risk remove and search, framework requirement controls
    row(
        "GET",
        "/workspaces/{workspaceId}/controls/{controlId}/tickets",
        &["control", "ticket", "list", "1", "2"],
        Page,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/controls/{controlId}/tickets",
        &[
            "control",
            "ticket",
            "link",
            "1",
            "2",
            "--ticket-url",
            "https://x.test/T-1",
        ],
        Object,
    ),
    row(
        "DELETE",
        "/workspaces/{workspaceId}/controls/{controlId}/tickets/{ticketId}",
        &["control", "ticket", "unlink", "1", "2", "3"],
        Empty,
    ),
    row(
        "GET",
        "/risk-registers/{riskRegisterId}/risks/{riskId}/tickets",
        &["risk", "ticket", "list", "1", "2"],
        Page,
    ),
    row(
        "POST",
        "/risk-registers/{riskRegisterId}/risks/{riskId}/tickets",
        &["risk", "ticket", "link", "1", "2", "--ticket-url", "https://x.test/T-1"],
        Object,
    ),
    row(
        "POST",
        "/risk-registers/{riskRegisterId}/risks/{riskId}/tickets/action-create",
        &[
            "risk",
            "ticket",
            "create",
            "1",
            "2",
            "--connection-id",
            "3",
            "--field",
            "summary",
            "string",
            "s",
        ],
        Object,
    ),
    row(
        "DELETE",
        "/risk-registers/{riskRegisterId}/risks/{riskId}/tickets/{ticketId}",
        &["risk", "ticket", "unlink", "1", "2", "3"],
        Empty,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets",
        &["monitor", "ticket", "list", "1", "2"],
        Page,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets",
        &[
            "monitor",
            "ticket",
            "link",
            "1",
            "2",
            "--ticket-url",
            "https://x.test/T-1",
        ],
        Object,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets/action-create",
        &[
            "monitor",
            "ticket",
            "create",
            "1",
            "2",
            "--connection-id",
            "3",
            "--field",
            "summary",
            "string",
            "s",
        ],
        Object,
    ),
    row(
        "DELETE",
        "/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets/{ticketId}",
        &["monitor", "ticket", "unlink", "1", "2", "3"],
        Empty,
    ),
    row(
        "DELETE",
        "/risk-registers/{riskRegisterId}/risks/{riskId}",
        &["risk", "remove", "1", "2"],
        Empty,
    ),
    row("GET", "/risks-search", &["risk", "search"], Page),
    row(
        "GET",
        "/risk-registers/{riskRegisterId}/risks-search",
        &["risk", "search", "--register-id", "1"],
        Page,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements/{requirementId}/controls",
        &["framework", "requirement", "controls", "1", "2", "3"],
        Page,
    ),
    // infrastructure and version control identities
    row(
        "GET",
        "/workspaces/{workspaceId}/infrastructure-user-identities",
        &["infrastructure-identity", "list", "1"],
        Page,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/infrastructure-user-identities/{identityId}",
        &["infrastructure-identity", "get", "1", "2"],
        Object,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/infrastructure-user-identities/{identityId}/actions",
        &["infrastructure-identity", "actions", "1", "2"],
        Object,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/infrastructure-user-identities/{identityId}/actions",
        &[
            "infrastructure-identity",
            "run-action",
            "1",
            "2",
            "--action",
            "markServiceAccount",
            "--reason",
            "r",
        ],
        Object,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/version-control-user-identities",
        &["version-control-identity", "list", "1"],
        Page,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/version-control-user-identities/{identityId}",
        &["version-control-identity", "get", "1", "2"],
        Object,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/version-control-user-identities/{identityId}/actions",
        &["version-control-identity", "actions", "1", "2"],
        Object,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/version-control-user-identities/{identityId}/actions",
        &[
            "version-control-identity",
            "run-action",
            "1",
            "2",
            "--action",
            "retractWriteAccess",
        ],
        Object,
    ),
    // evidence items (the Evidence tag; not the Evidence Library)
    row(
        "GET",
        "/workspaces/{workspaceId}/evidence",
        &["evidence-item", "list", "1"],
        Page,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/evidence",
        &[
            "evidence-item",
            "create",
            "1",
            "--name",
            "x",
            "--url",
            "https://example.com",
            "--filed-at",
            "2026-11-01",
        ],
        Object,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/evidence-files",
        &["evidence-item", "upload", "1", "--file", FIXTURE],
        Json(201, EVIDENCE_FILE),
    ),
    row(
        "DELETE",
        "/workspaces/{workspaceId}/evidence/{evidenceId}",
        &["evidence-item", "remove", "1", "2"],
        Empty,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/evidence/{evidenceId}",
        &["evidence-item", "get", "1", "2"],
        Object,
    ),
    row(
        "PUT",
        "/workspaces/{workspaceId}/evidence/{evidenceId}",
        &["evidence-item", "update", "1", "2", "--name", "y"],
        Object,
    ),
    row(
        "GET",
        "/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts",
        &["evidence-item", "artifact", "list", "1", "2"],
        Page,
    ),
    row(
        "DELETE",
        "/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}",
        &["evidence-item", "artifact", "remove", "1", "2", "3"],
        Empty,
    ),
    row(
        "PUT",
        "/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}",
        &["evidence-item", "artifact", "update", "1", "2", "3", "--name", "z"],
        Object,
    ),
    row(
        "POST",
        "/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}/actions",
        &[
            "evidence-item",
            "artifact",
            "run-action",
            "1",
            "2",
            "3",
            "--action",
            "archive",
        ],
        Object,
    ),
    // access reviews
    row(
        "GET",
        "/access-review-applications",
        &["access-review", "app", "list"],
        Page,
    ),
    row(
        "POST",
        "/access-review-applications",
        &["access-review", "app", "create", "--name", "AWS", "--reviewer-id", "1"],
        Object,
    ),
    row(
        "DELETE",
        "/access-review-applications/{applicationId}",
        &["access-review", "app", "remove", "1"],
        Empty,
    ),
    row(
        "GET",
        "/access-review-applications/{applicationId}",
        &["access-review", "app", "get", "1"],
        Object,
    ),
    row(
        "PUT",
        "/access-review-applications/{applicationId}",
        &["access-review", "app", "update", "1", "--name", "GCP"],
        Object,
    ),
    row(
        "GET",
        "/access-review-applications/{applicationId}/users",
        &["access-review", "app", "users", "list", "1"],
        Page,
    ),
    row(
        "PUT",
        "/access-review-applications/{applicationId}/users",
        &[
            "access-review",
            "app",
            "users",
            "set",
            "1",
            "--user",
            "a:jane:jane@acme.com",
        ],
        Object,
    ),
    row(
        "GET",
        "/access-review-periods",
        &["access-review", "period", "list"],
        Page,
    ),
    row(
        "POST",
        "/access-review-periods",
        &[
            "access-review",
            "period",
            "create",
            "--starting-date",
            "2026-11-01T00:00:00Z",
            "--ending-date",
            "2026-12-01T00:00:00Z",
            "--application",
            "1:2",
        ],
        Object,
    ),
    row(
        "GET",
        "/access-review-periods/{periodId}",
        &["access-review", "period", "get", "1"],
        Object,
    ),
    row(
        "POST",
        "/access-review-periods/{periodId}/actions",
        &["access-review", "period", "run-action", "1", "--action", "complete"],
        Object,
    ),
    row(
        "GET",
        "/access-review-periods/{periodId}/applications/{reviewAppId}",
        &["access-review", "period", "app", "get", "1", "2"],
        Object,
    ),
    row(
        "PUT",
        "/access-review-periods/{periodId}/applications/{reviewAppId}",
        &[
            "access-review",
            "period",
            "app",
            "update",
            "1",
            "2",
            "--reviewer-id",
            "3",
        ],
        Object,
    ),
    row(
        "POST",
        "/access-review-periods/{periodId}/applications/{reviewAppId}/actions",
        &[
            "access-review",
            "period",
            "app",
            "run-action",
            "1",
            "2",
            "--action",
            "complete",
        ],
        Object,
    ),
    row(
        "GET",
        "/access-review-periods/{periodId}/applications/{reviewAppId}/users",
        &["access-review", "period", "app", "users", "1", "2"],
        Page,
    ),
    row(
        "GET",
        "/access-review-periods/{periodId}/evidence-package",
        &["access-review", "period", "evidence-package", "1"],
        Object,
    ),
];
