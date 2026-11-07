//! clap types for `drata policy version create | replace-file` and
//! `drata policy-language ...`.
//!
//! `policy version <policy> <version>` already means "get one version", so the
//! `version` command keeps those two positionals and gains subcommands beside
//! them (`PolicyAction::Version` in `src/cli.rs`); the first token decides.

use super::PolicySourceType;
use clap::{Args, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Policy version statuses (spec `PolicyVersionStatusEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolicyVersionStatus {
    NeedsApproval,
    Approved,
    Published,
    Draft,
    Discarded,
}

/// Sub-collections a policy language version list can expand (spec `PolicyLanguageVersionListExpandEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum PolicyLanguageVersionListExpand {
    PolicyVersion,
    Policy,
}

/// Sub-collections one policy language version can expand (spec `PolicyLanguageVersionExpandEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum PolicyLanguageVersionExpand {
    PolicyVersion,
    Policy,
    DownloadUrl,
    DownloadPdfUrl,
}

/// Parse `--language` against the spec's `PolicyLanguageEnum` (BCP-47 codes),
/// case-insensitively. The embedded spec is the list: the organization's
/// languages come from `drata policy-language settings`, and Drata adds codes.
pub fn policy_language(raw: &str) -> Result<String, String> {
    let values =
        crate::spec::enum_values(crate::backend::Backend::Drata, "PolicyLanguageEnum").map_err(|e| e.to_string())?;
    values
        .into_iter()
        .find(|v| v.eq_ignore_ascii_case(raw))
        .ok_or_else(|| format!("`{raw}` is not a PolicyLanguageEnum value in the spec (e.g. en-US, fr-FR)"))
}

#[derive(Subcommand, Debug)]
pub enum PolicyVersionAction {
    /// Create a policy version (multipart with --file for UPLOADED, JSON for EXTERNAL)
    Create(PolicyVersionCreateArgs),
    /// Replace a policy version's file, or one language variant of it (multipart)
    ReplaceFile {
        /// Policy ID
        policy_id: String,
        /// Policy version ID
        version_id: String,
        /// Replacement file
        #[arg(long)]
        file: PathBuf,
        /// Replace only this language variant (BCP-47 code from `policy-language settings`)
        #[arg(long, value_parser = policy_language)]
        language: Option<String>,
    },
}

#[derive(Args, Debug)]
pub struct PolicyVersionCreateArgs {
    /// Policy ID
    #[arg(required_unless_present = "example")]
    pub policy_id: Option<String>,
    /// Where the content comes from: UPLOADED needs --file, EXTERNAL needs --external-file-id
    #[arg(long, value_enum, ignore_case = true, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
    pub source_type: Option<PolicySourceType>,
    /// Owner user ID
    #[arg(long, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
    pub owner_id: Option<u64>,
    /// Substantive change: bumps the major version and always needs approval (true|false)
    #[arg(long, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
    pub material_change: Option<bool>,
    /// Review/renewal date, YYYY-MM-DD, today or later
    #[arg(long, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
    pub renewal_date: Option<String>,
    /// A non-material version needs approval before it publishes (true|false)
    #[arg(long, conflicts_with = "data")]
    pub requires_approval: Option<bool>,
    /// Existing external file ID (EXTERNAL source)
    #[arg(long, conflicts_with = "data")]
    pub external_file_id: Option<u64>,
    /// File to upload (UPLOADED source)
    #[arg(long, conflicts_with = "data")]
    pub file: Option<PathBuf>,
    /// Display name for the file (max 191 chars)
    #[arg(long, conflicts_with = "data")]
    pub file_name: Option<String>,
    /// Short description (max 30000 chars)
    #[arg(long, conflicts_with = "data")]
    pub description: Option<String>,
    /// Short summary of the changes (max 30000 chars)
    #[arg(long, conflicts_with = "data")]
    pub change_summary: Option<String>,
    /// Detailed explanation of the changes (max 768 chars)
    #[arg(long, conflicts_with = "data")]
    pub changes_explanation: Option<String>,
    /// Users must acknowledge the version once published (true|false)
    #[arg(long, conflicts_with = "data")]
    pub requires_acknowledgement: Option<bool>,
    /// Full JSON body: inline, @file or - for stdin (no file part; EXTERNAL source)
    #[arg(long)]
    pub data: Option<String>,
    /// Print a JSON skeleton and exit (no API call)
    #[arg(long)]
    pub example: bool,
}

#[derive(Subcommand, Debug)]
pub enum PolicyLanguageAction {
    /// List the language variants of a policy version
    List {
        /// Policy ID
        policy_id: String,
        /// Policy version ID
        version_id: String,
        /// Only variants whose version has these statuses (space-separated)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
        status: Vec<PolicyVersionStatus>,
    },
    /// List the organization's configured policy languages
    Settings,
    /// List policy language versions across policies
    Versions {
        /// Only this policy version
        #[arg(long)]
        policy_version_id: Option<u64>,
        /// Only this policy
        #[arg(long)]
        policy_id: Option<u64>,
        /// Only variants whose version has these statuses (space-separated)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
        status: Vec<PolicyVersionStatus>,
        /// Sub-collections to expand (space-separated)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
        expand: Vec<PolicyLanguageVersionListExpand>,
    },
    /// Get one policy language version (JSON; the HTML representation is not curated)
    Get {
        /// Policy language version ID (the `id` from `list` or `versions`)
        id: String,
        /// Sub-collections to expand (space-separated)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
        expand: Vec<PolicyLanguageVersionExpand>,
    },
}
