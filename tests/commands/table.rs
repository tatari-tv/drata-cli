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
}

#[derive(Debug)]
pub struct Row {
    pub method: &'static str,
    pub path: &'static str,
    pub argv: &'static [&'static str],
    pub response: Response,
}

/// A readable file for the multipart upload rows.
pub const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/LICENSE");

const fn row(method: &'static str, path: &'static str, argv: &'static [&'static str], response: Response) -> Row {
    Row {
        method,
        path,
        argv,
        response,
    }
}

use Response::{Empty, Object, Page};

pub const ROWS: &[Row] = &[
    // vendors
    row("GET", "/vendors", &["vendor", "list"], Page),
    row("POST", "/vendors", &["vendor", "create", "--name", "x"], Object),
    row("GET", "/vendors/{vendorId}", &["vendor", "get", "1"], Object),
    row(
        "PUT",
        "/vendors/{vendorId}",
        &["vendor", "update", "1", "--name", "x"],
        Object,
    ),
    row("DELETE", "/vendors/{vendorId}", &["vendor", "remove", "1"], Empty),
    row(
        "POST",
        "/vendors/{vendorId}/documents",
        &["vendor", "upload", "1", "--file", FIXTURE],
        Object,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/questionnaires",
        &["vendor", "questionnaire", "list", "1"],
        Page,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/questionnaires/{questionnaireId}",
        &["vendor", "questionnaire", "get", "1", "2"],
        Object,
    ),
    row(
        "POST",
        "/vendors/{vendorId}/questionnaires",
        &[
            "vendor",
            "questionnaire",
            "send",
            "1",
            "--email",
            "a@example.com",
            "--questionnaire-id",
            "2",
            "--security-review-id",
            "3",
            "--email-content",
            "hello",
        ],
        Object,
    ),
    // vendor security reviews
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews",
        &["vendor", "security-review", "list", "1"],
        Page,
    ),
    row(
        "POST",
        "/vendors/{vendorId}/security-reviews",
        &[
            "vendor",
            "security-review",
            "create",
            "1",
            "--review-deadline-at",
            "2026-12-31",
            "--status",
            "NOT_YET_STARTED",
            "--type",
            "SECURITY",
        ],
        Object,
    ),
    row(
        "POST",
        "/vendors/{vendorId}/security-reviews/with-file",
        &[
            "vendor",
            "security-review",
            "create-with-file",
            "1",
            "--file",
            FIXTURE,
            "--title",
            "t",
            "--review-deadline-at",
            "2026-12-31",
            "--status",
            "NOT_YET_STARTED",
            "--type",
            "SECURITY",
            "--document-type",
            "COMPLIANCE_REPORT",
        ],
        Object,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}",
        &["vendor", "security-review", "get", "1", "2"],
        Object,
    ),
    row(
        "PUT",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}",
        &["vendor", "security-review", "update", "1", "2", "--title", "t"],
        Object,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/actions",
        &["vendor", "security-review", "actions", "1", "2"],
        Object,
    ),
    row(
        "POST",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/actions",
        &[
            "vendor",
            "security-review",
            "run-action",
            "1",
            "2",
            "--action",
            "finalize",
        ],
        Object,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires",
        &["vendor", "security-review", "questionnaires", "1", "2"],
        Page,
    ),
    row(
        "POST",
        "/vendors/{vendorId}/security-questionnaires",
        &[
            "vendor",
            "security-review",
            "upload-questionnaire",
            "1",
            "--file",
            FIXTURE,
        ],
        Object,
    ),
    row(
        "POST",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires",
        &[
            "vendor",
            "security-review",
            "upload-questionnaire-to-review",
            "1",
            "2",
            "--file",
            FIXTURE,
        ],
        Object,
    ),
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
    // vendor-adjacent reads, notes, observations, assessments
    row("GET", "/vendors-search", &["vendor", "search"], Page),
    row(
        "GET",
        "/vendors/{vendorId}/trust-overview",
        &["vendor", "trust-overview", "1"],
        Object,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/questionnaires/{questionnaireId}/questions",
        &["vendor", "questionnaire", "questions", "1", "2"],
        Page,
    ),
    row("GET", "/vendor-criteria", &["vendor-criteria", "list"], Page),
    row(
        "GET",
        "/workspaces/{workspaceId}/vendor-criteria-creations",
        &["vendor-criteria-creation", "list", "1"],
        Page,
    ),
    row(
        "GET",
        "/vendor-questionnaire-templates",
        &["vendor-questionnaire-template", "list"],
        Page,
    ),
    row(
        "GET",
        "/vendor-questionnaire-templates/{questionnaireId}/questions",
        &["vendor-questionnaire-template", "questions", "1"],
        Page,
    ),
    row("GET", "/vendor-risk-levels", &["vendor-risk-level", "list"], Page),
    row(
        "GET",
        "/vendor-security-reviews",
        &["vendor-security-review", "list"],
        Page,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/notes",
        &["vendor", "security-review", "note", "list", "1", "2"],
        Page,
    ),
    row(
        "POST",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/notes",
        &["vendor", "security-review", "note", "create", "1", "2", "--note", "x"],
        Object,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}",
        &["vendor", "security-review", "note", "get", "1", "2", "3"],
        Object,
    ),
    row(
        "PUT",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}",
        &[
            "vendor",
            "security-review",
            "note",
            "update",
            "1",
            "2",
            "3",
            "--note",
            "x",
        ],
        Object,
    ),
    row(
        "DELETE",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}",
        &["vendor", "security-review", "note", "remove", "1", "2", "3"],
        Empty,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/observations",
        &["vendor", "security-review", "observation", "list", "1", "2"],
        Page,
    ),
    row(
        "POST",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/observations",
        &[
            "vendor",
            "security-review",
            "observation",
            "create",
            "1",
            "2",
            "--observation",
            "x",
        ],
        Object,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}",
        &["vendor", "security-review", "observation", "get", "1", "2", "3"],
        Object,
    ),
    row(
        "PUT",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}",
        &[
            "vendor",
            "security-review",
            "observation",
            "update",
            "1",
            "2",
            "3",
            "--observation",
            "x",
        ],
        Object,
    ),
    row(
        "DELETE",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}",
        &["vendor", "security-review", "observation", "remove", "1", "2", "3"],
        Empty,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments",
        &["vendor", "security-review", "assessment", "list", "1", "2"],
        Page,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments/{assessmentId}",
        &["vendor", "security-review", "assessment", "get", "1", "2", "3"],
        Object,
    ),
    row(
        "GET",
        "/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments-stats",
        &["vendor", "security-review", "assessment", "stats", "1", "2"],
        Object,
    ),
];
