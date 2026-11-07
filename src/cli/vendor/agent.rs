//! clap types for the security-review agent workflow (all beta). Flattened
//! into `VendorSecurityReviewAction`, so every verb sits directly under
//! `drata vendor security-review`.
//!
//! Verb names are the spec's `AgentReviewActionEnum` values slugged
//! (`confirm_criteria` -> `confirm-criteria`) wherever the op has one, so the
//! `availableActions` of a 422 name the verb to run next. The rest take the
//! path's resource name (`workflow`, `residual-risk`, `follow-up-questionnaire`).

use super::{SecurityReviewDecision, SecurityReviewDocumentType};
use clap::{Args, Subcommand};
use std::path::PathBuf;

/// The two ids every agent verb addresses.
#[derive(Args, Clone, Debug)]
pub struct ReviewIds {
    /// Vendor ID
    pub vendor_id: String,
    /// Security review ID
    pub security_review_id: u64,
}

/// The same ids, optional in clap only so `--example` runs without them.
#[derive(Args, Clone, Debug)]
pub struct ExampleReviewIds {
    /// Vendor ID
    #[arg(required_unless_present = "example")]
    pub vendor_id: Option<String>,
    /// Security review ID
    #[arg(required_unless_present = "example")]
    pub security_review_id: Option<u64>,
}

#[derive(Subcommand, Clone, Debug)]
pub enum SecurityReviewAgentAction {
    /// Start the AI agent on a security review, or get the running agent's workflow ID (beta)
    StartAgent(ReviewIds),
    /// Save the vendor criteria the agent assesses; without --criterion-id, the default set (beta)
    ConfirmCriteria {
        #[command(flatten)]
        ids: ExampleReviewIds,
        /// Published vendor criterion IDs (space-separated)
        #[arg(long, num_args = 1.., conflicts_with = "data")]
        criterion_id: Vec<u64>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Start the assessment, or re-run it after one finished (beta)
    StartAssessment(ReviewIds),
    /// Stop the assessment that is running or preparing (beta)
    StopAssessment(ReviewIds),
    /// Collect the documents the vendor publishes on its Trust Center (beta)
    CollectPublicDocuments(ReviewIds),
    /// Read the form the vendor's Trust Center asks for before granting access (beta)
    AccessRequestRequirements(ReviewIds),
    /// Ask the vendor for access to its private Trust Center documents (beta)
    RequestTrustCenterAccess {
        #[command(flatten)]
        ids: ExampleReviewIds,
        /// The access request form as a JSON object, by field name: inline, @file, or - for stdin
        #[arg(long, conflicts_with = "data")]
        fields: Option<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Give the agent a link that opens the vendor's Trust Center documents (beta)
    ProvideTrustCenterAccessLink {
        #[command(flatten)]
        ids: ExampleReviewIds,
        /// The body `{"accessLink": "<url>"}` from @file or - (stdin) only; inline JSON is
        /// rejected, since anyone holding the link reads the documents
        #[arg(long, required_unless_present = "example")]
        data: Option<String>,
        /// Print the JSON body skeleton to fill in and pass as --data @file (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Stop waiting on a pending Trust Center access request (beta)
    SkipTrustCenterAccessRequest(ReviewIds),
    /// Upload 1 to 10 documents to the review and notify its agent (multipart) (beta)
    AddDocuments {
        #[command(flatten)]
        ids: ReviewIds,
        /// Path(s) to the document file(s) (space-separated, repeatable; 10 at most)
        #[arg(long, num_args = 1.., required = true)]
        file: Vec<PathBuf>,
        /// Vendor document type applied to every file (the server defaults to COMPLIANCE_REPORT)
        #[arg(long = "type", value_enum, ignore_case = true)]
        doc_type: Option<SecurityReviewDocumentType>,
    },
    /// Generate the follow-up questionnaire for the criteria the assessment did not meet (beta)
    GenerateFollowUpQuestionnaire(ReviewIds),
    /// Read the follow-up questionnaire the agent generated (beta)
    FollowUpQuestionnaire(ReviewIds),
    /// Email the vendor a link to the follow-up questionnaire (beta)
    SendFollowUpQuestionnaire {
        #[command(flatten)]
        ids: ExampleReviewIds,
        /// Recipient addresses, 1 to 5 (space-separated; required unless --data/--example)
        #[arg(long, num_args = 1.., conflicts_with = "data")]
        email: Vec<String>,
        /// Addresses copied, 5 at most (space-separated)
        #[arg(long, num_args = 1.., conflicts_with = "data")]
        cc: Vec<String>,
        /// Addresses blind copied, 5 at most (space-separated)
        #[arg(long, num_args = 1.., conflicts_with = "data")]
        bcc: Vec<String>,
        /// Email subject (default: the vendor email subject setting)
        #[arg(long, conflicts_with = "data")]
        email_subject: Option<String>,
        /// Email message (default: the email template's message)
        #[arg(long, conflicts_with = "data")]
        email_content: Option<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Get the review's recommended residual risk evaluation (beta)
    ResidualRisk(ReviewIds),
    /// Complete an agent review with a decision; base-tier reviews use run-action (beta)
    Finalize {
        #[command(flatten)]
        ids: ExampleReviewIds,
        /// Decision recorded on the review (required unless --data/--example; no default)
        #[arg(long, value_enum, ignore_case = true, conflicts_with = "data")]
        decision: Option<SecurityReviewDecision>,
        /// Reviewer note captured with the finalization
        #[arg(long, conflicts_with = "data")]
        note: Option<String>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Get the status of the review's summary PDF, with its download URL once READY (beta)
    Summary(ReviewIds),
    /// Get the agent workflow of a security review (null when none exists) (beta)
    Workflow(ReviewIds),
}
