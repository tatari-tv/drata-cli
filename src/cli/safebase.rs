//! clap types for `drata safebase`, the SafeBase Trust Center API namespace.

use super::RawArgs;
use clap::{Subcommand, ValueEnum};

#[derive(Subcommand, Debug)]
pub enum SafebaseAction {
    /// Save the profile's SafeBase credential (api key + write flag); its
    /// Drata credential is left as it is
    Login {
        /// SafeBase API key to store
        #[arg(long)]
        api_key: String,
        /// Mark this credential write-enabled (allows non-GET requests)
        #[arg(long = "allow-writes")]
        allow_writes: bool,
    },
    /// Remove the profile's SafeBase credential; its Drata credential stays
    Logout,
    /// Generic passthrough to any SafeBase operation: `raw <METHOD> <path> ...`
    /// (same flags as `drata raw`; GET, POST, PUT, PATCH, DELETE)
    Raw(RawArgs),
    /// Trust center accounts (the companies with access)
    Account {
        #[command(subcommand)]
        action: SafebaseAccountAction,
    },
    /// Account members
    Member {
        #[command(subcommand)]
        action: SafebaseMemberAction,
    },
    /// Trust center documents
    Document {
        #[command(subcommand)]
        action: SafebaseDocumentAction,
    },
    /// Access requests
    Request {
        #[command(subcommand)]
        action: SafebaseRequestAction,
    },
    /// Knowledge base
    Kb {
        #[command(subcommand)]
        action: SafebaseKbAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum SafebaseAccountAction {
    /// List accounts (every page by default)
    List {
        /// Stream every account as NDJSON, one per line
        #[arg(long)]
        all: bool,
    },
    /// Show one account
    Get {
        /// Account id (UUID)
        id: String,
    },
    /// Print the trust center URL for one account
    Url {
        /// Account id (UUID)
        id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum SafebaseMemberAction {
    /// List account members, optionally filtered
    List {
        /// Only members with this email
        #[arg(long)]
        email: Option<String>,
        /// Only members of the account with this Salesforce account id
        #[arg(long)]
        sfdc_account_id: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum SafebaseDocumentAction {
    /// List documents (every page by default)
    List {
        /// Stream every document as NDJSON, one per line
        #[arg(long)]
        all: bool,
        /// Free-text filter on documents
        #[arg(long)]
        query: Option<String>,
    },
    /// Show one document
    Get {
        /// Document id, or with --external the external id (UUID)
        id: String,
        /// Look the document up by its external id
        #[arg(long)]
        external: bool,
    },
    /// List the document types
    Types,
}

/// Access request status filter for `request list` (spec `status` enum).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lower")]
pub enum SafebaseRequestStatus {
    Pending,
    Approved,
    Declined,
}

#[derive(Subcommand, Debug)]
pub enum SafebaseRequestAction {
    /// List access requests
    List {
        /// Only requests in this status
        #[arg(long, value_enum)]
        status: Option<SafebaseRequestStatus>,
    },
}

#[derive(Subcommand, Debug)]
pub enum SafebaseKbAction {
    /// Search the knowledge base (every page by default)
    Search {
        /// Search text; omit to list entries
        query: Option<String>,
        /// Stream every entry as NDJSON, one per line
        #[arg(long)]
        all: bool,
    },
}
