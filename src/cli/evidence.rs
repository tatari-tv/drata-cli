//! clap types for `drata evidence-item ...`: the spec's Evidence tag, whose
//! resource is the "Evidence Item" (`/workspaces/{w}/evidence`). The Evidence
//! Library (`drata evidence`, `/workspaces/{w}/evidence-library`) keeps its
//! `EvidenceAction` in `src/cli.rs`; the two nouns address different resources.

use super::RenewalScheduleType;
use clap::{Args, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Sub-collections an evidence item read can expand (spec `EvidenceExpandEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum EvidenceItemExpand {
    Owners,
    Artifacts,
    Controls,
}

/// Computed evidence statuses a list can filter on (spec `ComputedEvidenceStatusEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComputedEvidenceStatus {
    NeedsArtifact,
    Expired,
    ExpiringSoon,
    TestError,
    TestFailed,
    Ready,
    TestPassed,
    TestUnused,
    TestDisabled,
    TestDeleted,
}

/// Artifact storage types (spec `ArtifactTypeEnum` on the item list, `ArtifactType`
/// on the artifact list; the two carry the same values).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ArtifactType {
    Url,
    #[value(name = "S3_FILE")]
    S3File,
    TicketProvider,
    None,
    GoogleDrive,
    OneDrive,
    Box,
    Dropbox,
    SharePoint,
    TestResult,
}

/// Actions on one artifact (spec `EvidenceArtifactActionEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum ArtifactActionKind {
    Archive,
    Restore,
}

/// An evidence item, addressed by workspace and id.
#[derive(Args, Clone, Debug)]
pub struct EvidenceItemRef {
    /// Workspace ID
    pub workspace_id: String,
    /// Evidence item ID
    pub evidence_id: u64,
}

/// The same ids, optional in clap only so `--example` runs without them.
#[derive(Args, Clone, Debug)]
pub struct ExampleEvidenceItemRef {
    /// Workspace ID
    #[arg(required_unless_present = "example")]
    pub workspace_id: Option<String>,
    /// Evidence item ID
    #[arg(required_unless_present = "example")]
    pub evidence_id: Option<u64>,
}

/// One artifact, addressed by workspace, evidence item and artifact id.
#[derive(Args, Clone, Debug)]
pub struct ArtifactRef {
    /// Workspace ID
    pub workspace_id: String,
    /// Evidence item ID
    pub evidence_id: u64,
    /// Artifact ID (stable across the artifact's versions)
    pub artifact_id: u64,
}

/// The same ids, optional in clap only so `--example` runs without them.
#[derive(Args, Clone, Debug)]
pub struct ExampleArtifactRef {
    /// Workspace ID
    #[arg(required_unless_present = "example")]
    pub workspace_id: Option<String>,
    /// Evidence item ID
    #[arg(required_unless_present = "example")]
    pub evidence_id: Option<u64>,
    /// Artifact ID (stable across the artifact's versions)
    #[arg(required_unless_present = "example")]
    pub artifact_id: Option<u64>,
}

/// New artifacts, one per value, all filed on `--filed-at`. `create` sends them
/// as `artifacts`, `update` as `newArtifacts`.
#[derive(Args, Clone, Debug)]
pub struct NewArtifactArgs {
    /// URL artifacts (space-separated)
    #[arg(long = "url", num_args = 1.., conflicts_with = "data", requires = "filed_at")]
    pub urls: Vec<String>,
    /// Pre-uploaded file keys, from `evidence-item upload` (space-separated)
    #[arg(long = "file-key", num_args = 1.., conflicts_with = "data", requires = "filed_at")]
    pub file_keys: Vec<String>,
    /// Ticket URL artifacts (space-separated)
    #[arg(long = "ticket-url", num_args = 1.., conflicts_with = "data", requires = "filed_at")]
    pub ticket_urls: Vec<String>,
    /// Date the new artifacts were filed, YYYY-MM-DD (required with --url, --file-key or --ticket-url)
    #[arg(long, conflicts_with = "data")]
    pub filed_at: Option<String>,
}

/// The scalar fields `create` and `update` share.
#[derive(Args, Clone, Debug)]
pub struct EvidenceItemFields {
    /// Description (max 30000 chars)
    #[arg(long, conflicts_with = "data")]
    pub description: Option<String>,
    /// Guidance for implementing the evidence (max 30000 chars)
    #[arg(long, conflicts_with = "data")]
    pub implementation_guidance: Option<String>,
    /// Steps to reproduce or collect the evidence (max 30000 chars)
    #[arg(long, conflicts_with = "data")]
    pub steps_to_reproduce: Option<String>,
    /// Owner user ID
    #[arg(long, conflicts_with = "data")]
    pub owner_id: Option<u64>,
    /// Control IDs to link (space-separated; update only adds links)
    #[arg(long = "control-id", num_args = 1.., conflicts_with = "data")]
    pub control_ids: Vec<u64>,
    /// Renewal date, YYYY-MM-DD
    #[arg(long, conflicts_with = "data")]
    pub renewal_date: Option<String>,
    /// Renewal schedule type
    #[arg(long, value_enum, ignore_case = true, conflicts_with = "data")]
    pub renewal_schedule_type: Option<RenewalScheduleType>,
}

#[derive(Args, Clone, Debug)]
pub struct EvidenceItemListArgs {
    /// Workspace ID
    pub workspace_id: String,
    /// Only items with this name
    #[arg(long)]
    pub name: Option<String>,
    /// Only items owned by these user IDs (space-separated)
    #[arg(long = "owner-id", num_args = 1..)]
    pub owner_ids: Vec<u64>,
    /// Only items linked to these framework IDs (space-separated)
    #[arg(long = "framework-id", num_args = 1..)]
    pub framework_ids: Vec<u64>,
    /// Only items with these computed statuses (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub status: Vec<ComputedEvidenceStatus>,
    /// Only items with artifacts of these types (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub artifact_type: Vec<ArtifactType>,
    /// Sub-collections to expand (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub expand: Vec<EvidenceItemExpand>,
}

#[derive(Subcommand, Debug)]
pub enum EvidenceItemAction {
    /// List evidence items in a workspace
    List(EvidenceItemListArgs),
    /// Get one evidence item
    Get {
        #[command(flatten)]
        target: EvidenceItemRef,
        /// Sub-collections to expand (space-separated)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
        expand: Vec<EvidenceItemExpand>,
    },
    /// Create an evidence item with URL, ticket or pre-uploaded file artifacts
    Create {
        /// Workspace ID (required unless --example)
        #[arg(required_unless_present = "example")]
        workspace_id: Option<String>,
        /// Item name (max 191 chars)
        #[arg(long, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        name: Option<String>,
        #[command(flatten)]
        fields: EvidenceItemFields,
        #[command(flatten)]
        artifacts: NewArtifactArgs,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update an evidence item: fields, new artifacts, archive or restore artifacts
    Update {
        #[command(flatten)]
        target: ExampleEvidenceItemRef,
        /// New name (max 191 chars)
        #[arg(long, conflicts_with = "data")]
        name: Option<String>,
        #[command(flatten)]
        fields: EvidenceItemFields,
        #[command(flatten)]
        artifacts: NewArtifactArgs,
        /// Artifact IDs to archive, moving them to past artifacts (space-separated)
        #[arg(long = "archive-artifact", num_args = 1.., conflicts_with = "data")]
        archive_artifacts: Vec<u64>,
        /// Artifact IDs to restore, moving them back to current artifacts (space-separated)
        #[arg(long = "restore-artifact", num_args = 1.., conflicts_with = "data")]
        restore_artifacts: Vec<u64>,
        /// Full JSON body: inline, @file or - for stdin (replaceArtifacts goes here)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Permanently delete an evidence item and all of its artifacts
    Remove(EvidenceItemRef),
    /// Pre-upload an artifact file; prints the fileKey that --file-key takes (multipart)
    Upload {
        /// Workspace ID
        workspace_id: String,
        /// File to upload
        #[arg(long)]
        file: PathBuf,
    },
    /// List, update, archive, restore or delete an evidence item's artifacts
    Artifact {
        #[command(subcommand)]
        action: EvidenceArtifactAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum EvidenceArtifactAction {
    /// List an evidence item's artifacts (one row per artifact, not per version)
    List {
        #[command(flatten)]
        target: EvidenceItemRef,
        /// Only artifacts with this name
        #[arg(long)]
        name: Option<String>,
        /// Only artifacts of these types (space-separated)
        #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
        artifact_type: Vec<ArtifactType>,
        /// Only archived (true) or only current (false) artifacts
        #[arg(long)]
        archived: Option<bool>,
    },
    /// Rename an artifact or change its filed date, in place on its current version
    Update {
        #[command(flatten)]
        target: ExampleArtifactRef,
        /// New artifact name (max 191 chars)
        #[arg(long, conflicts_with_all = ["data", "clear_name"])]
        name: Option<String>,
        /// Clear the artifact name (sends null)
        #[arg(long, conflicts_with = "data")]
        clear_name: bool,
        /// New filed date, YYYY-MM-DD
        #[arg(long, conflicts_with_all = ["data", "clear_filed_at"])]
        filed_at: Option<String>,
        /// Clear the filed date (sends null)
        #[arg(long, conflicts_with = "data")]
        clear_filed_at: bool,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Permanently delete one artifact and its version history
    Remove(ArtifactRef),
    /// Archive an artifact (move to past) or restore it (move back to current)
    RunAction {
        #[command(flatten)]
        target: ExampleArtifactRef,
        /// Action to run (required unless --data/--example)
        #[arg(long, value_enum, ignore_case = true, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        action: Option<ArtifactActionKind>,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
}
