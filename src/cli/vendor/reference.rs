//! clap types for the vendor-adjacent read nouns: `vendor-criteria`,
//! `vendor-criteria-creation`, `vendor-questionnaire-template`,
//! `vendor-risk-level` and the cross-vendor `vendor-security-review`.

use super::{ImpactLevel, SecurityReviewDecision, SecurityReviewStatus, SecurityReviewType, VendorRisk};
use clap::{Subcommand, ValueEnum};

/// Severity (spec `severities[]` items, criteria and assessments).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Severity {
    Low,
    Medium,
    High,
}

/// Category for the questionnaire-template filter (spec `categories[]` items
/// on `/vendor-questionnaire-templates`). Not `VendorCategory`: templates have
/// no `NONE`.
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TemplateCategory {
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
}

/// Sub-collections `vendor-security-review list` can expand (spec `expand[]`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum ReviewListExpand {
    User,
    RequesterUser,
    Vendor,
}

/// Criterion origin (spec `criterionTypes[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CriterionType {
    Custom,
    Default,
    AiGenerated,
}

/// Sub-collections `vendor-criteria list` can expand (spec `expand[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum CriteriaExpand {
    VendorTypes,
    ImpactLevels,
    QuestionnaireFile,
}

/// Criteria creation job status (spec `statuses[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CriteriaCreationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Questionnaire template status (spec `status` param).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TemplateStatus {
    Active,
    Draft,
}

#[derive(Subcommand, Debug)]
pub enum VendorCriteriaAction {
    /// List vendor assessment criteria
    List {
        /// Match criteria whose name contains this text
        #[arg(long)]
        name: Option<String>,
        /// Impact levels to match (space-separated)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        impact_level: Vec<ImpactLevel>,
        /// Vendor risk level IDs to match (space-separated)
        #[arg(long, num_args = 1..)]
        vendor_risk_level_id: Vec<u64>,
        /// Vendor type IDs to match (space-separated)
        #[arg(long, num_args = 1..)]
        vendor_type_id: Vec<u64>,
        /// Match on whether the criterion is a draft (true or false)
        #[arg(long)]
        draft: Option<bool>,
        /// Only criteria selected for this security review
        #[arg(long)]
        security_review_id: Option<u64>,
        /// Criterion origins to match (space-separated)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        criterion_type: Vec<CriterionType>,
        /// Severities to match (space-separated)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        severity: Vec<Severity>,
        /// Sub-collections to expand (space-separated)
        #[arg(long, value_enum, num_args = 1..)]
        expand: Vec<CriteriaExpand>,
    },
}

#[derive(Subcommand, Debug)]
pub enum VendorCriteriaCreationAction {
    /// List vendor criteria creation jobs for a workspace
    List {
        /// Workspace ID
        workspace_id: String,
        /// Job statuses to match (space-separated)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        status: Vec<CriteriaCreationStatus>,
    },
}

#[derive(Subcommand, Debug)]
pub enum VendorQuestionnaireTemplateAction {
    /// List vendor questionnaire templates
    List {
        /// Match templates whose title contains this text
        #[arg(long)]
        title: Option<String>,
        /// Template status to match
        #[arg(long, value_enum, ignore_case = true)]
        status: Option<TemplateStatus>,
        /// Risk levels to match (space-separated)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        risk_level: Vec<VendorRisk>,
        /// Categories to match (space-separated)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        category: Vec<TemplateCategory>,
    },
    /// List the questions of a questionnaire template
    Questions {
        /// Questionnaire template ID
        questionnaire_id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum VendorRiskLevelAction {
    /// List vendor risk levels
    List,
}

#[derive(Subcommand, Debug)]
pub enum VendorSecurityReviewsAction {
    /// List security reviews across every vendor
    List {
        /// Statuses to match (space-separated)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        status: Vec<SecurityReviewStatus>,
        /// Types to match (space-separated)
        #[arg(long = "type", value_enum, ignore_case = true, num_args = 1..)]
        review_type: Vec<SecurityReviewType>,
        /// Decisions to match (space-separated)
        #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
        decision: Vec<SecurityReviewDecision>,
        /// Only reviews created on or after this date (YYYY-MM-DD)
        #[arg(long)]
        created_from: Option<String>,
        /// Only reviews created on or before this date (YYYY-MM-DD)
        #[arg(long)]
        created_to: Option<String>,
        /// Only reviews updated on or after this date (YYYY-MM-DD)
        #[arg(long)]
        updated_from: Option<String>,
        /// Only reviews updated on or before this date (YYYY-MM-DD)
        #[arg(long)]
        updated_to: Option<String>,
        /// Only reviews of these vendor IDs (space-separated)
        #[arg(long, num_args = 1..)]
        vendor_id: Vec<u64>,
        /// Sub-collections to expand (space-separated)
        #[arg(long, value_enum, num_args = 1..)]
        expand: Vec<ReviewListExpand>,
    },
}
