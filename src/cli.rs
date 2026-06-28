//! All clap derive types. Zero logic lives here.
//!
//! Two-level command tree: `Commands` (one variant per tag/area) -> per-tag
//! `*Action` subcommand enum -> verbs. Auth onboarding (`login`/`logout`/
//! `whoami`/`auth`) sits at the top level since it gates everything else.

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "drata",
    about = "Drata compliance CLI",
    version = env!("GIT_DESCRIBE"),
    after_help = "Credentials: ~/.config/drata/credentials.json (profiles), \
                  or DRATA_API_KEY / DRATA_REGION / DRATA_PROFILE env vars, or --api-key.\n\
                  Logs: ~/.local/share/drata/logs/drata.log"
)]
pub struct Cli {
    /// Path to config file (reserved; credentials live in credentials.json)
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Drata API key (overrides env and credentials file)
    #[arg(long, global = true)]
    pub api_key: Option<String>,

    /// Drata region (us, eu, apac)
    #[arg(long, global = true)]
    pub region: Option<String>,

    /// Credentials profile to use
    #[arg(long, global = true)]
    pub profile: Option<String>,

    /// Allow mutating (non-GET) requests for this invocation
    #[arg(long = "allow-writes", global = true)]
    pub allow_writes: bool,

    /// Bypass confirmation prompts for mutating operations (POST/PUT/PATCH/DELETE)
    #[arg(long, global = true)]
    pub yes: bool,

    /// Output format
    #[arg(long, value_enum, global = true)]
    pub output: Option<OutputFormat>,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, global = true)]
    pub log_level: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "kebab-case")]
pub enum OutputFormat {
    Json,
    Table,
    Auto,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Save a credential profile (api key + region + write flag)
    Login {
        /// API key to store
        #[arg(long)]
        api_key: String,
        /// Region for this profile (us, eu, apac)
        #[arg(long, default_value = "us")]
        region: String,
        /// Mark this credential write-enabled (allows non-GET requests)
        #[arg(long = "allow-writes")]
        allow_writes: bool,
    },
    /// Remove a stored credential profile
    Logout,
    /// Show the active credential, region, and where the key resolved from
    Whoami,
    /// Show auth diagnostics (works without a configured key)
    Auth,
    /// Manage vendors and their questionnaires
    Vendor {
        #[command(subcommand)]
        action: VendorAction,
    },
    /// Manage risks in a risk register
    Risk {
        #[command(subcommand)]
        action: RiskAction,
    },
    /// Manage controls in a workspace
    Control {
        #[command(subcommand)]
        action: ControlAction,
    },
    /// Query devices (read-only; custom-connection devices via raw)
    Device {
        #[command(subcommand)]
        action: DeviceAction,
    },
    /// Manage personnel records
    Personnel {
        #[command(subcommand)]
        action: PersonnelAction,
    },
    /// Manage policies
    Policy {
        #[command(subcommand)]
        action: PolicyAction,
    },
    /// Manage evidence library items in a workspace
    Evidence {
        #[command(subcommand)]
        action: EvidenceAction,
    },
    /// Manage frameworks and requirements in a workspace
    Framework {
        #[command(subcommand)]
        action: FrameworkAction,
    },
    /// Manage assets
    Asset {
        #[command(subcommand)]
        action: AssetAction,
    },
    /// Get company information
    Company {
        #[command(subcommand)]
        action: CompanyAction,
    },
    /// List workspaces
    Workspace {
        #[command(subcommand)]
        action: WorkspaceAction,
    },
    /// Manage risk registers (list/get/create/update/delete)
    Register {
        #[command(subcommand)]
        action: RegisterAction,
    },
    /// List and inspect users and roles (read-only)
    User {
        #[command(subcommand)]
        action: UserAction,
    },
    /// Inspect monitoring tests in a workspace
    Monitor {
        #[command(subcommand)]
        action: MonitorAction,
    },
    /// List and inspect audits in a workspace (read-only)
    Audit {
        #[command(subcommand)]
        action: AuditAction,
    },
    /// List and inspect events (read-only)
    Event {
        #[command(subcommand)]
        action: EventAction,
    },
    /// Generic passthrough to any operation: `raw <METHOD> <path> ...`
    Raw(RawArgs),
}

/// Arguments for the generic `raw` passthrough namespace. Hits the active base
/// URL for any of the spec's operations. Non-GET is subject to the write
/// guardrail enforced in the client.
#[derive(clap::Args, Debug)]
pub struct RawArgs {
    /// HTTP method (GET, POST, PUT, DELETE; case-insensitive)
    pub method: String,
    /// Path template under the API base, e.g. /vendors or /vendors/123
    pub path: String,
    /// Query parameters as key=value (repeated or space-separated)
    #[arg(long, num_args = 1..)]
    pub query: Vec<String>,
    /// Request body: inline JSON, @file to read a file, or - for stdin
    #[arg(long)]
    pub data: Option<String>,
    /// Path to a file to upload (for multipart operations)
    #[arg(long)]
    pub file: Option<std::path::PathBuf>,
    /// Print the operation's request-body skeleton from the spec and exit
    #[arg(long)]
    pub example: bool,
}

// ---------------------------------------------------------------------------
// Vendors
// ---------------------------------------------------------------------------

#[derive(Subcommand, Debug)]
pub enum VendorAction {
    /// List vendors (optional name patterns: exact -> starts-with -> contains)
    List {
        /// Zero or more name patterns
        patterns: Vec<String>,
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get a vendor by ID
    Get {
        id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Create a vendor
    Create {
        /// Vendor name (required unless --example)
        #[arg(long)]
        name: Option<String>,
        /// Category (e.g. SECURITY, ENGINEERING)
        #[arg(long)]
        category: Option<String>,
        /// Risk level (NONE, LOW, MODERATE, HIGH)
        #[arg(long)]
        risk: Option<String>,
        /// Status (e.g. ACTIVE, PROSPECTIVE)
        #[arg(long)]
        status: Option<String>,
        /// Vendor homepage URL
        #[arg(long)]
        url: Option<String>,
        /// Free-form notes
        #[arg(long)]
        notes: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update an existing vendor
    Update {
        /// Vendor ID
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        category: Option<String>,
        #[arg(long)]
        risk: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        url: Option<String>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Remove a vendor by ID
    Remove { id: String },
    /// Upload a document for a vendor (multipart)
    Upload {
        /// Vendor ID
        vendor_id: String,
        /// Path to the file to upload
        #[arg(long)]
        file: std::path::PathBuf,
    },
    /// Manage a vendor's questionnaires
    Questionnaire {
        #[command(subcommand)]
        action: VendorQuestionnaireAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum VendorQuestionnaireAction {
    /// List questionnaires sent to a vendor
    List {
        /// Vendor ID
        vendor_id: String,
    },
    /// Get a single questionnaire by ID
    Get {
        /// Vendor ID
        vendor_id: String,
        /// Questionnaire ID
        questionnaire_id: String,
    },
    /// Send a questionnaire to a vendor
    Send {
        /// Vendor ID
        vendor_id: String,
        /// Recipient email
        #[arg(long)]
        email: String,
        /// Questionnaire template ID
        #[arg(long)]
        questionnaire_id: u64,
        /// Security review ID this questionnaire belongs to
        #[arg(long)]
        security_review_id: u64,
        /// Email body sent to the recipient
        #[arg(long)]
        email_content: String,
        /// Optional email subject
        #[arg(long)]
        email_subject: Option<String>,
    },
}

// ---------------------------------------------------------------------------
// Risks
// ---------------------------------------------------------------------------

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskTreatmentPlan {
    Untreated,
    Accept,
    Transfer,
    Avoid,
    Mitigate,
}

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskStatus {
    Active,
    Archived,
    Closed,
}

#[derive(Subcommand, Debug)]
pub enum RiskAction {
    /// List risks in a risk register
    List {
        /// Risk register ID
        register_id: String,
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get a single risk by ID
    Get {
        /// Risk register ID
        register_id: String,
        /// Risk ID
        risk_id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Create a risk in a register
    Create {
        /// Risk register ID
        register_id: String,
        /// Risk title
        #[arg(long)]
        title: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Treatment plan
        #[arg(long, value_enum, ignore_case = true)]
        treatment_plan: Option<RiskTreatmentPlan>,
        /// Impact score (numeric)
        #[arg(long)]
        impact: Option<f64>,
        /// Likelihood score (numeric)
        #[arg(long)]
        likelihood: Option<f64>,
        /// Status
        #[arg(long, value_enum, ignore_case = true)]
        status: Option<RiskStatus>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update a risk
    Update {
        /// Risk register ID
        register_id: String,
        /// Risk ID
        risk_id: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long, value_enum, ignore_case = true)]
        treatment_plan: Option<RiskTreatmentPlan>,
        #[arg(long)]
        impact: Option<f64>,
        #[arg(long)]
        likelihood: Option<f64>,
        #[arg(long, value_enum, ignore_case = true)]
        status: Option<RiskStatus>,
    },
    /// Get risk insights for a register
    Insights {
        /// Risk register ID
        register_id: String,
    },
    /// Upload a document to a risk (multipart)
    Upload {
        /// Risk register ID
        register_id: String,
        /// Risk ID
        risk_id: String,
        /// Path to the file to upload
        #[arg(long)]
        file: std::path::PathBuf,
    },
}

// ---------------------------------------------------------------------------
// Controls
// ---------------------------------------------------------------------------

#[derive(Subcommand, Debug)]
pub enum ControlAction {
    /// List controls in a workspace
    List {
        /// Workspace ID
        workspace_id: String,
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get a control by ID
    Get {
        /// Workspace ID
        workspace_id: String,
        /// Control ID
        control_id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Create a control in a workspace (supports multipart --file)
    Create {
        /// Workspace ID
        workspace_id: String,
        /// Control name
        #[arg(long)]
        name: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Guiding question
        #[arg(long)]
        question: Option<String>,
        /// Activity description
        #[arg(long)]
        activity: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update a control
    Update {
        /// Workspace ID
        workspace_id: String,
        /// Control ID
        control_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        question: Option<String>,
        #[arg(long)]
        activity: Option<String>,
    },
    /// List requirements mapped to a control
    Requirements {
        /// Workspace ID
        workspace_id: String,
        /// Control ID
        control_id: String,
    },
    /// Compare control-requirement mappings in a workspace
    Compare {
        /// Workspace ID
        workspace_id: String,
    },
}

// ---------------------------------------------------------------------------
// Devices
// ---------------------------------------------------------------------------

#[derive(Subcommand, Debug)]
pub enum DeviceAction {
    /// List all devices
    List {
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get a device by ID
    Get {
        /// Device ID
        device_id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// List devices for a personnel member
    ForPersonnel {
        /// Personnel ID
        personnel_id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// List apps installed on a device
    Apps {
        /// Device ID
        device_id: String,
    },
    /// Upload a document for a device (multipart)
    Upload {
        /// Device ID
        device_id: String,
        /// Path to the file to upload
        #[arg(long)]
        file: std::path::PathBuf,
    },
}

// ---------------------------------------------------------------------------
// Personnel
// ---------------------------------------------------------------------------

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
}

// ---------------------------------------------------------------------------
// Policies
// ---------------------------------------------------------------------------

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PolicySourceType {
    Uploaded,
    External,
}

#[derive(Subcommand, Debug)]
pub enum PolicyAction {
    /// List all policies
    List {
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get a policy by ID
    Get {
        /// Policy ID
        policy_id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Create a policy (supports multipart --file for uploaded policies)
    Create {
        /// Policy name
        #[arg(long)]
        name: Option<String>,
        /// Owner personnel ID
        #[arg(long)]
        owner_id: Option<u64>,
        /// Source type
        #[arg(long, value_enum, ignore_case = true)]
        source_type: Option<PolicySourceType>,
        /// Path to a file to upload (for UPLOADED source type; multipart)
        #[arg(long)]
        file: Option<std::path::PathBuf>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update a policy
    Update {
        /// Policy ID
        policy_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        owner_id: Option<u64>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        renewal_date: Option<String>,
    },
    /// List available actions on a policy
    Actions {
        /// Policy ID
        policy_id: String,
    },
    /// List policy versions
    Versions {
        /// Policy ID
        policy_id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get a specific policy version
    Version {
        /// Policy ID
        policy_id: String,
        /// Version ID
        version_id: String,
    },
}

// ---------------------------------------------------------------------------
// Evidence Library
// ---------------------------------------------------------------------------

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RenewalScheduleType {
    OneMonth,
    TwoMonths,
    ThreeMonths,
    SixMonths,
    OneYear,
    Custom,
    None,
}

#[derive(Subcommand, Debug)]
pub enum EvidenceAction {
    /// List evidence library items in a workspace
    List {
        /// Workspace ID
        workspace_id: String,
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get an evidence library item by ID
    Get {
        /// Workspace ID
        workspace_id: String,
        /// Evidence library item ID
        evidence_id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Create an evidence library item (supports multipart --file)
    Create {
        /// Workspace ID
        workspace_id: String,
        /// Item name
        #[arg(long)]
        name: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Renewal schedule type
        #[arg(long, value_enum, ignore_case = true)]
        renewal_schedule_type: Option<RenewalScheduleType>,
        /// Path to a file to upload (multipart)
        #[arg(long)]
        file: Option<std::path::PathBuf>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update an evidence library item (supports multipart --file)
    Update {
        /// Workspace ID
        workspace_id: String,
        /// Evidence library item ID
        evidence_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long, value_enum, ignore_case = true)]
        renewal_schedule_type: Option<RenewalScheduleType>,
        /// Path to a file to upload (multipart)
        #[arg(long)]
        file: Option<std::path::PathBuf>,
    },
    /// Remove an evidence library item
    Remove {
        /// Workspace ID
        workspace_id: String,
        /// Evidence library item ID
        evidence_id: String,
    },
    /// Get a specific version of an evidence library item
    GetVersion {
        /// Workspace ID
        workspace_id: String,
        /// Evidence library item ID
        evidence_id: String,
        /// Version ID
        version_id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
}

// ---------------------------------------------------------------------------
// Frameworks
// ---------------------------------------------------------------------------

#[derive(Subcommand, Debug)]
pub enum FrameworkAction {
    /// List frameworks in a workspace
    List {
        /// Workspace ID
        workspace_id: String,
    },
    /// Create a framework
    Create {
        /// Workspace ID
        workspace_id: String,
        /// Framework name
        #[arg(long)]
        name: Option<String>,
        /// Short name / acronym
        #[arg(long)]
        short_name: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update a framework
    Update {
        /// Workspace ID
        workspace_id: String,
        /// Framework ID
        framework_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    /// List requirements for a framework
    Requirements {
        /// Workspace ID
        workspace_id: String,
        /// Framework ID
        framework_id: String,
    },
}

// ---------------------------------------------------------------------------
// Assets
// ---------------------------------------------------------------------------

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetType {
    Physical,
    Virtual,
}

#[derive(Subcommand, Debug)]
pub enum AssetAction {
    /// List all assets
    List {
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get an asset by ID
    Get {
        /// Asset ID
        asset_id: String,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Create an asset
    Create {
        /// Asset name
        #[arg(long)]
        name: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Asset type (PHYSICAL or VIRTUAL)
        #[arg(long, value_enum, ignore_case = true)]
        asset_type: Option<AssetType>,
        /// Free-form notes
        #[arg(long)]
        notes: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update an asset
    Update {
        /// Asset ID
        asset_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long, value_enum, ignore_case = true)]
        asset_type: Option<AssetType>,
        #[arg(long)]
        notes: Option<String>,
    },
    /// Delete an asset by ID
    Remove {
        /// Asset ID
        asset_id: String,
    },
}

// ---------------------------------------------------------------------------
// Company
// ---------------------------------------------------------------------------

#[derive(Subcommand, Debug)]
pub enum CompanyAction {
    /// Get company information
    Get,
}

// ---------------------------------------------------------------------------
// Workspaces
// ---------------------------------------------------------------------------

#[derive(Subcommand, Debug)]
pub enum WorkspaceAction {
    /// List all workspaces
    List,
}

// ---------------------------------------------------------------------------
// Risk Registers
// ---------------------------------------------------------------------------

#[derive(Subcommand, Debug)]
pub enum RegisterAction {
    /// List all risk registers
    List,
    /// Get a risk register by ID
    Get {
        /// Risk register ID
        register_id: String,
    },
    /// Create a risk register
    Create {
        /// Register name
        #[arg(long)]
        name: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Owner personnel IDs (space-separated or repeated).
        /// Omit to leave unset; pass with no values (`--owner-ids`) to clear.
        #[arg(long, num_args = 0..)]
        owner_ids: Option<Vec<u64>>,
        /// Workspace IDs to associate (space-separated or repeated).
        /// Omit to leave unset; pass with no values (`--workspace-ids`) to clear.
        #[arg(long, num_args = 0..)]
        workspace_ids: Option<Vec<u64>>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update a risk register
    Update {
        /// Risk register ID
        register_id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
        /// Owner personnel IDs (space-separated or repeated).
        /// Omit to leave unset; pass with no values (`--owner-ids`) to clear all.
        #[arg(long, num_args = 0..)]
        owner_ids: Option<Vec<u64>>,
        /// Workspace IDs to associate (space-separated or repeated).
        /// Omit to leave unset; pass with no values (`--workspace-ids`) to clear all.
        #[arg(long, num_args = 0..)]
        workspace_ids: Option<Vec<u64>>,
    },
    /// Delete a risk register by ID
    Remove {
        /// Risk register ID
        register_id: String,
    },
}

// ---------------------------------------------------------------------------
// Users and Roles
// ---------------------------------------------------------------------------

#[derive(Subcommand, Debug)]
pub enum UserAction {
    /// List all users
    List {
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
    },
    /// Get a user by ID
    Get {
        /// User ID
        user_id: String,
    },
    /// List all roles
    Roles,
    /// Get a role by ID
    Role {
        /// Role ID
        role_id: String,
    },
    /// List users assigned to a role
    RoleUsers {
        /// Role ID
        role_id: String,
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
    },
}

// ---------------------------------------------------------------------------
// Monitoring Tests
// ---------------------------------------------------------------------------

#[derive(Subcommand, Debug)]
pub enum MonitorAction {
    /// List monitoring tests in a workspace
    List {
        /// Workspace ID
        workspace_id: String,
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
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
}

// ---------------------------------------------------------------------------
// Audits
// ---------------------------------------------------------------------------

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
    },
}

// ---------------------------------------------------------------------------
// Events
// ---------------------------------------------------------------------------

#[derive(Subcommand, Debug)]
pub enum EventAction {
    /// List all events
    List {
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
    },
    /// Get an event by ID
    Get {
        /// Event ID
        event_id: String,
    },
}
