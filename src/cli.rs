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
}

#[derive(Subcommand, Debug)]
pub enum VendorAction {
    /// List vendors (optional name patterns: exact -> starts-with -> contains)
    List {
        /// Zero or more name patterns
        patterns: Vec<String>,
    },
    /// Get a vendor by ID
    Get { id: String },
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
