//! clap types for `drata audit ...`: audits, and under `audit request` the
//! beta DCF suggestions the spec tags "Audit Requests".

use clap::{Args, Subcommand, ValueEnum};

/// Confidence levels a suggestion carries and a list or accept can narrow to
/// (spec `ConfidenceLevelEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DcfConfidenceLevel {
    High,
    Medium,
    Low,
}

/// Decision statuses a suggestion list can filter on (spec
/// `AuditRequestDcfSuggestionStatusFilterEnum`; rejected suggestions are never listed).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DcfSuggestionStatus {
    Pending,
    Accepted,
}

/// Fields a suggestion list sorts by (spec `SortTypeLimitedEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum DcfSuggestionSort {
    CreatedAt,
    UpdatedAt,
}

/// Sort directions (spec `SortDirectionEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DcfSortDirection {
    Asc,
    Desc,
}

/// Decisions on one suggestion (spec `AuditRequestDcfSuggestionActionEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DcfSuggestionDecision {
    Accept,
    Reject,
}

/// An audit, addressed by workspace and audit id.
#[derive(Args, Clone, Debug)]
pub struct AuditRef {
    /// Workspace ID
    pub workspace_id: String,
    /// Audit ID
    pub audit_id: String,
}

/// The same ids, optional in clap only so `--example` runs without them.
#[derive(Args, Clone, Debug)]
pub struct ExampleAuditRef {
    /// Workspace ID (required unless --example)
    #[arg(required_unless_present = "example")]
    pub workspace_id: Option<String>,
    /// Audit ID (required unless --example)
    #[arg(required_unless_present = "example")]
    pub audit_id: Option<String>,
}

/// One audit request within an audit.
#[derive(Args, Clone, Debug)]
pub struct AuditRequestRef {
    /// Workspace ID
    pub workspace_id: String,
    /// Audit ID
    pub audit_id: String,
    /// Audit request ID
    pub request_id: u64,
}

/// One DCF suggestion on an audit request.
#[derive(Args, Clone, Debug)]
pub struct DcfSuggestionRef {
    /// Workspace ID
    pub workspace_id: String,
    /// Audit ID
    pub audit_id: String,
    /// Audit request ID
    pub request_id: u64,
    /// DCF suggestion ID
    pub suggestion_id: u64,
}

/// The same ids, optional in clap only so `--example` runs without them.
#[derive(Args, Clone, Debug)]
pub struct ExampleDcfSuggestionRef {
    /// Workspace ID (required unless --example)
    #[arg(required_unless_present = "example")]
    pub workspace_id: Option<String>,
    /// Audit ID (required unless --example)
    #[arg(required_unless_present = "example")]
    pub audit_id: Option<String>,
    /// Audit request ID (required unless --example)
    #[arg(required_unless_present = "example")]
    pub request_id: Option<u64>,
    /// DCF suggestion ID (required unless --example)
    #[arg(required_unless_present = "example")]
    pub suggestion_id: Option<u64>,
}

#[derive(Subcommand, Debug)]
pub enum AuditAction {
    /// List audits in a workspace
    List {
        /// Workspace ID
        workspace_id: String,
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
    },
    /// Get an audit by ID
    Get {
        /// Workspace ID
        workspace_id: String,
        /// Audit ID
        audit_id: String,
        /// Sub-collections to expand (space-separated, e.g. dcfSuggestionsSummary)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Audit requests
    Request {
        #[command(subcommand)]
        action: AuditRequestAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum AuditRequestAction {
    /// AI-suggested controls (DCFs) for audit requests (beta)
    DcfSuggestion {
        #[command(subcommand)]
        action: DcfSuggestionAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum DcfSuggestionAction {
    /// Start generating suggestions for audit requests; answers 202, status is in `audit get --expand dcfSuggestionsSummary` (beta)
    Generate {
        #[command(flatten)]
        audit: ExampleAuditRef,
        /// Audit request IDs to generate for, 1 to 100 (space-separated)
        #[arg(long = "request-id", num_args = 1.., conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        request_ids: Vec<u64>,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Accept pending suggestions on audit requests, by request or on every request in the audit (beta)
    Accept {
        #[command(flatten)]
        audit: ExampleAuditRef,
        /// Audit request IDs whose pending suggestions to accept, 1 to 500 (space-separated)
        #[arg(long = "request-id", num_args = 1.., conflicts_with_all = ["all_requests", "data"])]
        request_ids: Vec<u64>,
        /// Accept on every request in the audit with a matching pending suggestion (up to 500)
        #[arg(long, conflicts_with = "data")]
        all_requests: bool,
        /// Accept only suggestions at these confidence levels (space-separated; default: every level)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true, conflicts_with = "data")]
        confidence_level: Vec<DcfConfidenceLevel>,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// List the suggestions on an audit request (beta)
    List {
        #[command(flatten)]
        request: AuditRequestRef,
        /// Only suggestions with these decision statuses (space-separated)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
        status: Vec<DcfSuggestionStatus>,
        /// Only suggestions at these confidence levels (space-separated)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
        confidence_level: Vec<DcfConfidenceLevel>,
        /// Field to sort by
        #[arg(long, value_enum, ignore_case = true)]
        sort: Option<DcfSuggestionSort>,
        /// Sort direction
        #[arg(long, value_enum, ignore_case = true)]
        sort_dir: Option<DcfSortDirection>,
    },
    /// Get one suggestion (beta)
    Get(DcfSuggestionRef),
    /// Accept or reject one suggestion (beta)
    RunAction {
        #[command(flatten)]
        target: ExampleDcfSuggestionRef,
        /// Decision to make (required unless --data/--example)
        #[arg(long, value_enum, ignore_case = true, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        action: Option<DcfSuggestionDecision>,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
}
