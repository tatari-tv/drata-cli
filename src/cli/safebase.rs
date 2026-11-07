//! clap types for `drata safebase`, the SafeBase Trust Center API namespace.

use super::RawArgs;
use clap::{Args, Subcommand, ValueEnum};
use std::path::PathBuf;

/// SafeBase's email on a member add or a request decline is opt-in. Neither
/// flag omits `message` from the body, so SafeBase sends no email.
#[derive(Args, Debug, Clone, Default)]
pub struct EmailNotice {
    /// Send SafeBase's email with this custom message
    #[arg(long, conflicts_with = "notify")]
    pub message: Option<String>,
    /// Send SafeBase's email with no custom message
    #[arg(long)]
    pub notify: bool,
}

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
    /// List the members with an email, optionally within one Salesforce account
    List {
        /// Members with this email (SafeBase requires it, observed 2026-11-07)
        #[arg(long, required = true)]
        email: Option<String>,
        /// Only members of the account with this Salesforce account id
        #[arg(long)]
        sfdc_account_id: Option<String>,
    },
    /// Add one member (with names) to an account. Bulk adds go through raw
    Add {
        /// Account id (UUID)
        account_id: String,
        /// Member email
        #[arg(long)]
        email: String,
        /// Member first name
        #[arg(long)]
        first_name: String,
        /// Member last name
        #[arg(long)]
        last_name: String,
        #[command(flatten)]
        notice: EmailNotice,
    },
    /// Remove members from an account by email
    Remove {
        /// Account id (UUID)
        account_id: String,
        /// Emails of the members to remove (space-separated or repeated)
        #[arg(long, required = true, num_args = 1..)]
        email: Vec<String>,
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
    /// Upload a file as a document: request an upload, send the bytes to
    /// SafeBase's signed URL, then create (or replace) the document
    Upload(UploadArgs),
}

/// `drata safebase document upload`.
#[derive(Args, Debug)]
pub struct UploadArgs {
    /// File to upload (at most 75 MiB; its type must be one SafeBase accepts)
    pub file: PathBuf,
    /// Document name shown in the trust center
    #[arg(long)]
    pub name: String,
    /// Document type (see `drata safebase document types`)
    #[arg(long = "type")]
    pub doc_type: Option<String>,
    /// Replace the file of this existing document (document id). Observed
    /// 2026-11-07: the document keeps its id; name, externalId, checksum and
    /// size take the new values; type, expiresAt, aiOptOut and products
    /// survive; tags untested
    #[arg(long)]
    pub replace: Option<String>,
    /// External id to store on the document (a UUID; default: a fresh v4).
    /// Sent in lowercase hyphenated form; `document get --external` finds it
    #[arg(long)]
    pub external_id: Option<String>,
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
    /// Approve an access request into an existing account. Approving into a
    /// new account goes through raw
    Approve {
        /// Request id (UUID)
        request_id: String,
        /// Account to grant access through (UUID; see `account list`)
        #[arg(long)]
        account_id: String,
    },
    /// Decline an access request
    Decline {
        /// Request id (UUID)
        request_id: String,
        #[command(flatten)]
        notice: EmailNotice,
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
    /// Add a knowledge base entry (`question` required; `answer`, `comment`,
    /// `accessLevel` optional)
    Add {
        /// Entry JSON: inline, @file, or - for stdin
        #[arg(long)]
        data: String,
    },
    /// Update a knowledge base entry (fields to change; null clears
    /// `answer` / `comment`)
    Update {
        /// KB entry id
        id: String,
        /// Fields JSON: inline, @file, or - for stdin
        #[arg(long)]
        data: String,
    },
}
