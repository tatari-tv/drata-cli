//! clap types for `drata personnel ...` (records, search, scoped groups) and
//! `drata group ...` (the account's personnel groups).

use clap::{Args, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmploymentStatus {
    CurrentEmployee,
    FormerEmployee,
    CurrentContractor,
    FormerContractor,
    OutOfScope,
    Unknown,
    SpecialFormerEmployee,
    SpecialFormerContractor,
    FutureHire,
    ServiceAccount,
}

/// A personnel compliance check's status (spec `ComplianceCheckStatusPublicV2Enum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ComplianceCheckStatus {
    Pass,
    Fail,
    Misconfigured,
    Excluded,
    OutOfScope,
    NotAssessed,
}

/// Spec `PersonnelSyncStatusPublicV2Enum`.
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonnelSyncStatus {
    Synced,
    NotSynced,
}

/// Sub-collections `personnel search` can expand (spec `PersonnelSearchExpandEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum PersonnelSearchExpand {
    CustomFields,
    ComplianceChecks,
    ReasonProvider,
    User,
    Groups,
    IdpConnection,
    HrisConnection,
}

/// Facets `personnel search` can count (spec `PersonnelSearchFacetEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum PersonnelSearchFacet {
    EmploymentStatus,
    ComplianceStatus,
    SyncStatus,
    GroupIds,
    AcceptedPoliciesStatus,
    BgCheckStatus,
    IdentityMfaStatus,
    SecurityTrainingStatus,
    HipaaTrainingStatus,
    AiAwarenessTrainingStatus,
    OffboardingStatus,
    AgentInstalledStatus,
    PasswordManagerStatus,
    DiskEncryptionStatus,
    AntivirusStatus,
    AutoUpdatesStatus,
    LockScreenStatus,
}

#[derive(Args, Debug)]
pub struct PersonnelSearchArgs {
    /// Search one workspace; omit to search across all workspaces
    #[arg(long)]
    pub workspace_id: Option<String>,
    /// Free-text query over name, email and job title (max 191 chars)
    #[arg(long)]
    pub q: Option<String>,
    /// First-name prefix (max 191 chars)
    #[arg(long)]
    pub first_name: Option<String>,
    /// Last-name prefix (max 191 chars)
    #[arg(long)]
    pub last_name: Option<String>,
    /// Employment statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub employment_status: Vec<EmploymentStatus>,
    /// Overall compliance statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub compliance_status: Vec<ComplianceCheckStatus>,
    /// Accepted-policies check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub accepted_policies_status: Vec<ComplianceCheckStatus>,
    /// Background-check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub bg_check_status: Vec<ComplianceCheckStatus>,
    /// Identity MFA check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub identity_mfa_status: Vec<ComplianceCheckStatus>,
    /// Security-training check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub security_training_status: Vec<ComplianceCheckStatus>,
    /// HIPAA-training check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub hipaa_training_status: Vec<ComplianceCheckStatus>,
    /// AI-awareness-training check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub ai_awareness_training_status: Vec<ComplianceCheckStatus>,
    /// Offboarding check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub offboarding_status: Vec<ComplianceCheckStatus>,
    /// Agent-installed check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub agent_installed_status: Vec<ComplianceCheckStatus>,
    /// Password-manager check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub password_manager_status: Vec<ComplianceCheckStatus>,
    /// Disk-encryption check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub disk_encryption_status: Vec<ComplianceCheckStatus>,
    /// Antivirus check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub antivirus_status: Vec<ComplianceCheckStatus>,
    /// Auto-updates check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub auto_updates_status: Vec<ComplianceCheckStatus>,
    /// Lock-screen check statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub lock_screen_status: Vec<ComplianceCheckStatus>,
    /// Group IDs to match, from `group list` (space-separated)
    #[arg(long, num_args = 1..)]
    pub group_id: Vec<u64>,
    /// Identity-connection sync status
    #[arg(long, value_enum, ignore_case = true)]
    pub sync_status: Option<PersonnelSyncStatus>,
    /// Identity-provider connection IDs to match (space-separated)
    #[arg(long, num_args = 1..)]
    pub idp_connection_id: Vec<u64>,
    /// HRIS connection IDs to match (space-separated)
    #[arg(long, num_args = 1..)]
    pub hris_connection_id: Vec<u64>,
    /// Custom field equality filters as `<id|name>=<value>` (space-separated; a numeric
    /// left side is a customFieldId, anything else a field name; AND-ed)
    #[arg(long, num_args = 1..)]
    pub custom_field: Vec<String>,
    /// Custom field range filters as `<id|name>=<gte>..<lte>` (space-separated; either bound
    /// may be empty, one is required; AND-ed)
    #[arg(long, num_args = 1..)]
    pub custom_range_field: Vec<String>,
    /// Sub-collections to expand (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub expand: Vec<PersonnelSearchExpand>,
    /// Count these facets: fetches one page and prints the whole
    /// {data, pagination, facets} envelope as JSON (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub facet: Vec<PersonnelSearchFacet>,
}

#[derive(Subcommand, Debug)]
pub enum PersonnelAction {
    /// List all personnel
    List {
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get a personnel record by ID
    Get {
        /// Personnel ID
        personnel_id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Update a personnel record
    Update {
        /// Personnel ID
        personnel_id: String,
        /// Employment status
        #[arg(long, value_enum, ignore_case = true)]
        employment_status: Option<EmploymentStatus>,
        /// Start date (ISO 8601)
        #[arg(long)]
        started_at: Option<String>,
        /// Separation date (ISO 8601)
        #[arg(long)]
        separated_at: Option<String>,
        /// Reason if not a human (e.g. SERVICE_ACCOUNT)
        #[arg(long)]
        not_human_reason: Option<String>,
    },
    /// Search personnel (all pages as rows; --facet counts facets on one page)
    Search(Box<PersonnelSearchArgs>),
    /// A workspace's personnel scope: the personnel groups it covers (beta)
    Group {
        #[command(subcommand)]
        action: PersonnelGroupAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum PersonnelGroupAction {
    /// List the personnel groups in a workspace's scope (beta)
    List {
        /// Workspace ID
        workspace_id: String,
    },
    /// Count the personnel the workspace's scoped groups reach, and the tenant total (beta)
    Counts {
        /// Workspace ID
        workspace_id: String,
    },
    /// Add one personnel group to a workspace's scope (beta)
    Add {
        /// Workspace ID
        workspace_id: String,
        /// Group ID from `group list`
        group_id: u64,
    },
    /// Replace a workspace's whole scope with exactly these groups; groups not listed are removed (beta)
    Replace {
        /// Workspace ID
        workspace_id: String,
        /// The groups the scope holds afterwards (space-separated)
        #[arg(long, num_args = 1.., required_unless_present = "clear", conflicts_with = "clear")]
        group_id: Vec<u64>,
        /// Clear the scope entirely (sends an empty group list)
        #[arg(long)]
        clear: bool,
    },
    /// Remove one personnel group from a workspace's scope (beta)
    Remove {
        /// Workspace ID
        workspace_id: String,
        /// Group ID from `personnel group list`
        group_id: u64,
    },
}

#[derive(Subcommand, Debug)]
pub enum GroupAction {
    /// List personnel groups, optionally filtered by a name substring
    List {
        /// Case-insensitive name substring (max 191 chars)
        #[arg(long)]
        name: Option<String>,
    },
}
