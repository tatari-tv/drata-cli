//! clap types for the security-review notes, observations and assessments
//! (all beta). Notes and observations share one shape and one implementation,
//! so each verb carries the same flags under a different name.

use super::Severity;
use clap::{Subcommand, ValueEnum};

/// Assessment status (spec `statuses[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssessmentStatus {
    Met,
    NotMet,
    Inconclusive,
    #[value(name = "N_A")]
    NotApplicable,
    PartiallyMet,
}

#[derive(Subcommand, Clone, Debug)]
pub enum SecurityReviewNoteAction {
    /// List a security review's internal notes (beta)
    List {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Only notes related to this assessment
        #[arg(long)]
        assessment_id: Option<u64>,
        /// Sub-collections to expand: user (space-separated)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get one note (beta)
    Get {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Note ID
        note_id: u64,
        /// Sub-collections to expand: user (space-separated)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Create a note (beta)
    Create {
        /// Vendor ID
        #[arg(required_unless_present = "example")]
        vendor_id: Option<String>,
        /// Security review ID
        #[arg(required_unless_present = "example")]
        security_review_id: Option<u64>,
        /// Note text (required unless --data/--example)
        #[arg(long)]
        note: Option<String>,
        /// Where the note was drawn from
        #[arg(long)]
        source: Option<String>,
        /// Relate the note to this assessment of the review
        #[arg(long)]
        assessment_id: Option<u64>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update a note (beta)
    Update {
        /// Vendor ID
        #[arg(required_unless_present = "example")]
        vendor_id: Option<String>,
        /// Security review ID
        #[arg(required_unless_present = "example")]
        security_review_id: Option<u64>,
        /// Note ID
        #[arg(required_unless_present = "example")]
        note_id: Option<u64>,
        /// New note text
        #[arg(long)]
        note: Option<String>,
        /// New source
        #[arg(long, conflicts_with = "clear_source")]
        source: Option<String>,
        /// Clear the source (sends null)
        #[arg(long)]
        clear_source: bool,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Remove a note (beta)
    Remove {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Note ID
        note_id: u64,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum SecurityReviewObservationAction {
    /// List a security review's observations (beta)
    List {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Only observations related to this assessment
        #[arg(long)]
        assessment_id: Option<u64>,
        /// Sub-collections to expand: user (space-separated)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Get one observation (beta)
    Get {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Observation ID
        observation_id: u64,
        /// Sub-collections to expand: user (space-separated)
        #[arg(long, num_args = 1..)]
        expand: Vec<String>,
    },
    /// Create an observation (beta)
    Create {
        /// Vendor ID
        #[arg(required_unless_present = "example")]
        vendor_id: Option<String>,
        /// Security review ID
        #[arg(required_unless_present = "example")]
        security_review_id: Option<u64>,
        /// Observation text (required unless --data/--example)
        #[arg(long)]
        observation: Option<String>,
        /// Where the observation was drawn from
        #[arg(long)]
        source: Option<String>,
        /// Relate the observation to this assessment of the review
        #[arg(long)]
        assessment_id: Option<u64>,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update an observation (beta)
    Update {
        /// Vendor ID
        #[arg(required_unless_present = "example")]
        vendor_id: Option<String>,
        /// Security review ID
        #[arg(required_unless_present = "example")]
        security_review_id: Option<u64>,
        /// Observation ID
        #[arg(required_unless_present = "example")]
        observation_id: Option<u64>,
        /// New observation text
        #[arg(long)]
        observation: Option<String>,
        /// New source
        #[arg(long, conflicts_with = "clear_source")]
        source: Option<String>,
        /// Clear the source (sends null)
        #[arg(long)]
        clear_source: bool,
        /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Remove an observation (beta)
    Remove {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Observation ID
        observation_id: u64,
    },
}

#[derive(Subcommand, Clone, Debug)]
pub enum SecurityReviewAssessmentAction {
    /// List a security review's criteria assessments (beta)
    List {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Match assessments whose criteria name contains this text
        #[arg(long)]
        criteria_name: Option<String>,
        /// Statuses to match (space-separated)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        status: Vec<AssessmentStatus>,
        /// Severities to match (space-separated)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        severity: Vec<Severity>,
    },
    /// Get one assessment with its sources and requirements (beta)
    Get {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
        /// Assessment ID
        assessment_id: u64,
    },
    /// Count a security review's assessments by status (beta)
    Stats {
        /// Vendor ID
        vendor_id: String,
        /// Security review ID
        security_review_id: u64,
    },
}
