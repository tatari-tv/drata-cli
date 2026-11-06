//! clap types for `drata compliance-check-exclusion ...`: the personnel
//! compliance-check exclusions of a workspace.

use clap::{Subcommand, ValueEnum};

/// Spec `ComplianceCheckTypeEnum`.
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComplianceCheckType {
    FullCompliance,
    AcceptedPolicies,
    IdentityMfa,
    BgCheck,
    AgentInstalled,
    PasswordManager,
    HddEncryption,
    Antivirus,
    AutoUpdates,
    LockScreen,
    SecurityTraining,
    LocationServices,
    HipaaTraining,
    Offboarding,
    NistAiTraining,
}

/// Sub-collections `compliance-check-exclusion list` can expand (spec
/// `ComplianceCheckExclusionExpandEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum ComplianceCheckExclusionExpand {
    ComplianceCheckTypes,
    CreatedBy,
}

#[derive(Subcommand, Debug)]
pub enum ComplianceCheckExclusionAction {
    /// List the personnel exclusions in force in a workspace, newest first
    List {
        /// Workspace ID
        workspace_id: String,
        /// Only exclusions covering this compliance check
        #[arg(long, value_enum, ignore_case = true)]
        compliance_check: Option<ComplianceCheckType>,
        /// Sub-collections to expand (space-separated)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
        expand: Vec<ComplianceCheckExclusionExpand>,
    },
    /// Exclude personnel from compliance checks; prints every target, exits 1 if any failed
    Create {
        /// Workspace ID
        #[arg(required_unless_present = "example")]
        workspace_id: Option<String>,
        /// Personnel IDs to exclude (space-separated, 500 at most; required unless --data/--example)
        #[arg(long, num_args = 1.., conflicts_with = "data")]
        target_id: Vec<String>,
        /// Compliance checks to exclude them from (space-separated; required unless --data/--example)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true, conflicts_with = "data")]
        compliance_check: Vec<ComplianceCheckType>,
        /// Why they are excluded, applied to every target (required unless --data/--example)
        #[arg(long, conflicts_with = "data")]
        reason: Option<String>,
        /// When the exclusion takes effect, ISO 8601 (required unless --data/--example)
        #[arg(long, conflicts_with = "data")]
        start_date: Option<String>,
        /// When the exclusion expires, ISO 8601 and after --start-date; omit for open-ended
        #[arg(long, conflicts_with = "data")]
        end_date: Option<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Delete personnel exclusions for the given checks so they count again; prints every target, exits 1 if any failed
    Remove {
        /// Workspace ID
        workspace_id: String,
        /// Personnel IDs whose exclusions are deleted (space-separated, 500 at most)
        #[arg(long, num_args = 1.., required = true)]
        target_id: Vec<String>,
        /// Compliance checks to delete the exclusions for (space-separated)
        #[arg(long, num_args = 1.., required = true, value_enum, ignore_case = true)]
        compliance_check: Vec<ComplianceCheckType>,
    },
}
