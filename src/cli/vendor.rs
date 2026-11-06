//! Clap types for `drata vendor ...`, including the questionnaire and
//! security-review groups. Zero logic lives here.

use clap::{Subcommand, ValueEnum};

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
        /// Filter by vendor status (space-separated; spec param `statuses[]`)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        status: Vec<VendorStatus>,
        /// Filter by category (space-separated; spec param `categories[]`)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        category: Vec<VendorCategory>,
        /// Filter by vendor type (space-separated; spec param `types[]`)
        #[arg(long = "type", value_enum, ignore_case = true, num_args = 1..)]
        vendor_type: Vec<VendorType>,
        /// Filter by custom vendor type ID (space-separated; spec param `vendorTypeIds[]`)
        #[arg(long, num_args = 1..)]
        vendor_type_id: Vec<u64>,
        /// Filter by vendor user ID (space-separated; spec param `vendorUserIds[]`)
        #[arg(long, num_args = 1..)]
        vendor_user_id: Vec<u64>,
        /// Filter on whether the vendor is a subprocessor (true or false)
        #[arg(long)]
        sub_processor: Option<bool>,
        /// Filter on whether the vendor needs action (true or false)
        #[arg(long)]
        action_required: Option<bool>,
        /// Filter by next review deadline status (space-separated; spec param
        /// `nextReviewDeadlineStatus[]`)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        review_deadline_status: Vec<ReviewDeadlineStatus>,
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
        /// Custom vendor type ID (honored only when custom vendor types are enabled)
        #[arg(long)]
        vendor_type_id: Option<u64>,
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
        /// Vendor document type (optional, e.g. COMPLIANCE_REPORT)
        #[arg(long = "type")]
        doc_type: Option<String>,
        /// Associate with a security review by ID (optional)
        #[arg(long)]
        security_review_id: Option<u64>,
    },
    /// Manage a vendor's questionnaires
    Questionnaire {
        #[command(subcommand)]
        action: VendorQuestionnaireAction,
    },
    /// Manage a vendor's security reviews
    SecurityReview {
        #[command(subcommand)]
        action: VendorSecurityReviewAction,
    },
}

/// Vendor status filter for `vendor list` (spec `statuses[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VendorStatus {
    Prospective,
    Active,
    Archived,
    Approved,
    Rejected,
    Flagged,
    OnHold,
    Offboarded,
    UnderReview,
    None,
    Current,
}

/// Vendor category filter for `vendor list` (spec `categories[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VendorCategory {
    Engineering,
    Product,
    Marketing,
    Cs,
    Sales,
    Finance,
    Hr,
    Administrative,
    Security,
    Legal,
    InformationTechnology,
    None,
}

/// Vendor type filter for `vendor list` (spec `types[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VendorType {
    Vendor,
    Supplier,
    Contractor,
    Partner,
    Other,
    None,
}

/// Next-review-deadline filter for `vendor list` (spec
/// `nextReviewDeadlineStatus[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewDeadlineStatus {
    NoRenewal,
    DueSoon,
    Overdue,
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

/// Status filter / create field for security reviews.
/// Serializes to SCREAMING_SNAKE_CASE per the Drata spec
/// (`VendorSecurityReviewStatusEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityReviewStatus {
    NotYetStarted,
    InProgress,
    Completed,
    NotRequired,
}

/// Type filter / create field for security reviews.
/// Serializes to SCREAMING_SNAKE_CASE per the Drata spec
/// (`VendorSecurityReviewTypeEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityReviewType {
    Security,
    SocReport,
    UploadReport,
}

/// Action for `vendor security-review run-action`.
/// Serializes to lowercase per the Drata spec
/// (`SecurityReviewActionEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lowercase")]
pub enum SecurityReviewAction {
    Finalize,
    Reopen,
}

/// Decision filter for `vendor security-review list`.
/// Serializes to SCREAMING_SNAKE_CASE per the Drata spec
/// (`VendorSecurityReviewDecisionEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityReviewDecision {
    Pending,
    Approved,
    ApprovedWithConditions,
    Rejected,
}

/// Document type for `vendor security-review create-with-file`.
/// Serializes to SCREAMING_SNAKE_CASE per the Drata spec
/// (`VendorSecurityReviewWithFileCreateRequestPublicV2Dto.documentType`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityReviewDocumentType {
    ComplianceReport,
    ComplianceReportReview,
    BridgeLetter,
    UploadedComplianceReportReview,
    QuestionnaireAttachment,
    SocDocument,
    QuestionnaireReport,
    TrustCenterPage,
}

#[derive(Subcommand, Clone, Debug)]
pub enum VendorSecurityReviewAction {
    /// List security reviews for a vendor
    List {
        /// Vendor ID
        vendor_id: String,
        /// Filter by status (space-separated, repeatable; spec param `status[]`)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        status: Vec<SecurityReviewStatus>,
        /// Filter by type (space-separated, repeatable; spec param `type[]`)
        #[arg(long = "type", value_enum, ignore_case = true, num_args = 1..)]
        review_type: Vec<SecurityReviewType>,
        /// Filter by decision (space-separated, repeatable; spec param `decision[]`)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        decision: Vec<SecurityReviewDecision>,
        /// Only reviews created on or after this date (YYYY-MM-DD; spec param `createdAtFrom`)
        #[arg(long)]
        created_from: Option<String>,
        /// Only reviews created on or before this date (YYYY-MM-DD; spec param `createdAtTo`)
        #[arg(long)]
        created_to: Option<String>,
        /// Only reviews updated on or after this date (YYYY-MM-DD; spec param `updatedAtFrom`)
        #[arg(long)]
        updated_from: Option<String>,
        /// Only reviews updated on or before this date (YYYY-MM-DD; spec param `updatedAtTo`)
        #[arg(long)]
        updated_to: Option<String>,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
        /// Stream all pages as NDJSON instead of buffering
        #[arg(long)]
        all: bool,
    },
    /// Create a security review for a vendor
    Create {
        /// Vendor ID
        vendor_id: String,
        /// Review deadline (ISO 8601, e.g. 2026-12-31). Required unless --data/--example.
        #[arg(long)]
        review_deadline_at: Option<String>,
        /// Review status. Required unless --data/--example.
        #[arg(long, value_enum, ignore_case = true)]
        status: Option<SecurityReviewStatus>,
        /// Review type. Required unless --data/--example.
        #[arg(long = "type", value_enum, ignore_case = true)]
        review_type: Option<SecurityReviewType>,
        /// Optional title
        #[arg(long)]
        title: Option<String>,
        /// Optional note
        #[arg(long)]
        note: Option<String>,
        /// Requested-at timestamp (ISO 8601)
        #[arg(long)]
        requested_at: Option<String>,
        /// Requester user ID
        #[arg(long)]
        requester_user_id: Option<u64>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Get a single security review by ID
    Get {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Sub-collections to expand (space-separated, repeatable)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Update a security review (title, soc-form, updated-at)
    Update {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// New title
        #[arg(long)]
        title: Option<String>,
        /// SOC form as a JSON object (per spec `SocReviewFormSaveRequestPublicV2Dto`)
        #[arg(long)]
        soc_form: Option<String>,
        /// Override the last-updated timestamp (ISO 8601 date-time); the only
        /// field the spec allows on a completed review
        #[arg(long)]
        updated_at: Option<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// List available actions for a security review
    Actions {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
    },
    /// Run a lifecycle action (finalize or reopen) on a security review
    RunAction {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Action to run
        #[arg(long, value_enum, ignore_case = true)]
        action: SecurityReviewAction,
        /// Finalization decision (finalize only; the server defaults to APPROVED)
        #[arg(long, value_enum, ignore_case = true)]
        decision: Option<SecurityReviewDecision>,
        /// Reviewer note captured with the finalization (finalize only)
        #[arg(long)]
        note: Option<String>,
        /// Reset the decision to PENDING as part of the reopen (reopen only)
        #[arg(long)]
        reset_decision: bool,
    },
    /// List security questionnaires attached to a security review
    Questionnaires {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
    },
    /// Create a security review with an attached file (multipart upload)
    CreateWithFile {
        /// Vendor ID
        vendor_id: String,
        /// Path to the file to upload (single `file` part)
        #[arg(long)]
        file: std::path::PathBuf,
        /// Review title (required for create-with-file)
        #[arg(long)]
        title: String,
        /// Review deadline (ISO 8601, e.g. 2026-12-31)
        #[arg(long)]
        review_deadline_at: String,
        /// Review status
        #[arg(long, value_enum, ignore_case = true)]
        status: SecurityReviewStatus,
        /// Review type
        #[arg(long = "type", value_enum, ignore_case = true)]
        review_type: SecurityReviewType,
        /// Document type of the uploaded file
        #[arg(long, value_enum, ignore_case = true)]
        document_type: SecurityReviewDocumentType,
        /// Optional note
        #[arg(long)]
        note: Option<String>,
        /// Requested-at timestamp (ISO 8601)
        #[arg(long)]
        requested_at: Option<String>,
        /// Requester user ID
        #[arg(long)]
        requester_user_id: Option<u64>,
    },
    /// Upload one or more completed questionnaire files for a vendor (multipart)
    UploadQuestionnaire {
        /// Vendor ID
        vendor_id: String,
        /// Path(s) to questionnaire file(s) (space-separated, repeatable)
        #[arg(long, num_args = 1.., required = true)]
        file: Vec<std::path::PathBuf>,
    },
    /// Upload one or more completed questionnaire files to a security review (multipart)
    UploadQuestionnaireToReview {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Path(s) to questionnaire file(s) (space-separated, repeatable)
        #[arg(long, num_args = 1.., required = true)]
        file: Vec<std::path::PathBuf>,
    },
}
