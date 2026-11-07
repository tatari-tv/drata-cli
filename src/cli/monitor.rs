//! clap types for `drata monitor ...`: monitoring tests, their custom-test
//! recipes, retests, exclusions and codebase exclusions.

use super::MonitorTicketAction;
use clap::{Args, Subcommand, ValueEnum};

/// The two ids every per-test verb addresses.
#[derive(Args, Clone, Debug)]
pub struct MonitorTestIds {
    /// Workspace ID
    pub workspace_id: String,
    /// Test ID
    pub test_id: String,
}

/// The same ids, optional in clap only so `--example` runs without them.
#[derive(Args, Clone, Debug)]
pub struct ExampleMonitorTestIds {
    /// Workspace ID
    #[arg(required_unless_present = "example")]
    pub workspace_id: Option<String>,
    /// Test ID
    #[arg(required_unless_present = "example")]
    pub test_id: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum MonitorAction {
    /// List monitoring tests in a workspace
    List {
        /// Workspace ID
        workspace_id: String,
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
        /// Filter on draft state (true or false)
        #[arg(long)]
        draft: Option<bool>,
        /// Filter by test name
        #[arg(long)]
        name: Option<String>,
    },
    /// Get a monitoring test by ID
    Get {
        /// Workspace ID
        workspace_id: String,
        /// Test ID
        test_id: String,
    },
    /// Update a monitoring test
    Update {
        /// Workspace ID
        workspace_id: String,
        /// Test ID
        test_id: String,
        /// Display name for the test
        #[arg(long)]
        name: Option<String>,
        /// Enable or disable the test
        #[arg(long)]
        enabled: Option<bool>,
        /// Description
        #[arg(long)]
        description: Option<String>,
    },
    /// List exclusions for a monitoring test
    Exclusions {
        /// Workspace ID
        workspace_id: String,
        /// Test ID
        test_id: String,
    },
    /// List recent failures for a monitoring test
    Failures {
        /// Workspace ID
        workspace_id: String,
        /// Test ID
        test_id: String,
    },
    /// List recent passes for a monitoring test
    Passes {
        /// Workspace ID
        workspace_id: String,
        /// Test ID
        test_id: String,
    },
    /// Tickets linked to a monitoring test
    Ticket {
        #[command(subcommand)]
        action: MonitorTicketAction,
    },
    /// Create a custom monitoring test (saved as a draft) over Custom Connection data
    Create {
        /// Workspace ID
        #[arg(required_unless_present = "example")]
        workspace_id: Option<String>,
        /// Test name (required unless --data/--example)
        #[arg(long, conflicts_with = "data")]
        name: Option<String>,
        /// What the test checks, saved as the description of the evidence it collects
        #[arg(long, conflicts_with = "data")]
        description: Option<String>,
        /// Custom Connection IDs the test runs against when the recipe's allConnections is false (space-separated)
        #[arg(long, num_args = 1.., conflicts_with = "data")]
        connection_id: Vec<u64>,
        /// The recipe (test logic) as a JSON object: inline, @file, or - for stdin (required unless --data/--example)
        #[arg(long, conflicts_with = "data")]
        recipe: Option<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Check a custom test recipe against the workspace's Custom Connections; saves nothing, exits 1 when invalid
    ValidateRecipe {
        /// Workspace ID
        #[arg(required_unless_present = "example")]
        workspace_id: Option<String>,
        /// The recipe as a JSON object: inline, @file, or - for stdin (required unless --data/--example)
        #[arg(long, conflicts_with = "data")]
        recipe: Option<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides --recipe)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Replace the recipe of a custom test draft; the test ID may be `name:<exact draft name>`
    Recipe {
        #[command(flatten)]
        ids: ExampleMonitorTestIds,
        /// Custom Connection IDs the test runs against when the recipe's allConnections is false (space-separated)
        #[arg(long, num_args = 1.., conflicts_with = "data")]
        connection_id: Vec<u64>,
        /// The replacement recipe as a JSON object: inline, @file, or - for stdin (required unless --data/--example)
        #[arg(long, conflicts_with = "data")]
        recipe: Option<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Run a monitoring test now; fails without retrying on 409 (a run in flight) or 429 (post-run cooldown)
    Retest(MonitorTestIds),
    /// Create and delete exclusions of a monitoring test's failures
    Exclusion {
        #[command(subcommand)]
        action: MonitorExclusionAction,
    },
    /// List, create and delete the codebase exclusions of a codebase monitoring test
    CodebaseExclusion {
        #[command(subcommand)]
        action: MonitorCodebaseExclusionAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum MonitorExclusionAction {
    /// Exclude failures of a monitoring test, with a reason; prints every item, exits 1 if any failed
    Create {
        #[command(flatten)]
        ids: ExampleMonitorTestIds,
        /// Why the failures are excluded, applied to every one (required unless --data/--example)
        #[arg(long, conflicts_with = "data")]
        reason: Option<String>,
        /// A failure as two values: <id> <connection-id> from `monitor failures`; `null` when its
        /// connectionId is null. Repeatable, 300 at most
        #[arg(
            long = "failure",
            num_args = 2,
            value_names = ["ID", "CONNECTION_ID"],
            action = clap::ArgAction::Append,
            conflicts_with = "data"
        )]
        failures: Vec<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Delete exclusions by ID so their failures count again; prints every item, exits 1 if any failed
    Remove {
        #[command(flatten)]
        ids: MonitorTestIds,
        /// Exclusion IDs from `monitor exclusions` (space-separated, 300 at most)
        #[arg(long, num_args = 1.., required = true)]
        exclusion_id: Vec<u64>,
    },
}

/// Sub-collections `monitor codebase-exclusion list` can expand (spec
/// `MonitorCodebaseExclusionExpandEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum CodebaseExclusionExpand {
    ExclusionDesignator,
}

#[derive(Subcommand, Debug)]
pub enum MonitorCodebaseExclusionAction {
    /// List the codebase exclusions of a codebase monitoring test
    List {
        #[command(flatten)]
        ids: MonitorTestIds,
        /// Filter by Code Repository name (e.g. example-org/example-repo)
        #[arg(long)]
        repository_name: Option<String>,
        /// Sub-collections to expand (space-separated)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
        expand: Vec<CodebaseExclusionExpand>,
    },
    /// Exclude findings of a codebase monitoring test, with a reason; prints every item, exits 1 if any failed
    Create {
        #[command(flatten)]
        ids: ExampleMonitorTestIds,
        /// Why the findings are excluded, applied to every one (required unless --data/--example)
        #[arg(long, conflicts_with = "data")]
        reason: Option<String>,
        /// findingId values from `monitor failures` (space-separated, 300 at most)
        #[arg(long, num_args = 1.., conflicts_with = "data")]
        finding_id: Vec<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Delete codebase exclusions by ID so their findings count again; prints every item, exits 1 if any failed
    Remove {
        #[command(flatten)]
        ids: MonitorTestIds,
        /// Codebase exclusion IDs from `monitor codebase-exclusion list` (space-separated, 300 at most)
        #[arg(long, num_args = 1.., required = true)]
        exclusion_id: Vec<u64>,
    },
}
