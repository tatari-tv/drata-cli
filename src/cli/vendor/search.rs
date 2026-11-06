//! clap types for `drata vendor search` (`GET /vendors-search`).

use super::{ReviewDeadlineStatus, SecurityReviewDecision, SecurityReviewStatus, VendorCategory, VendorStatus};
use clap::{Args, ValueEnum};

/// Vendor risk level (spec `risk` param and `riskLevels[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VendorRisk {
    None,
    Low,
    Moderate,
    High,
}

/// Impact level (spec `impactLevel` param and `impactLevels[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImpactLevel {
    Insignificant,
    Minor,
    Moderate,
    Major,
    Critical,
    Unscored,
}

/// Sub-collections `vendor search` can expand (spec `expand[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum VendorSearchExpand {
    CustomFields,
    Documents,
    LatestSecurityReviews,
    VendorUser,
    LastQuestionnaire,
}

/// Facets `vendor search` can count (spec `facets[]` items).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum VendorSearchFacet {
    Status,
    Category,
    VendorTypeId,
    Risk,
    ImpactLevel,
    IsSubProcessor,
    SecurityReviewStatus,
    SecurityReviewDecision,
    ScheduledQuestionnaireStatus,
}

#[derive(Args, Debug)]
pub struct VendorSearchArgs {
    /// Free-text query
    #[arg(long)]
    pub q: Option<String>,
    /// Vendor statuses to match (space-separated)
    #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
    pub status: Vec<VendorStatus>,
    /// Categories to match (space-separated)
    #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
    pub category: Vec<VendorCategory>,
    /// Custom vendor type IDs to match (space-separated)
    #[arg(long, num_args = 1..)]
    pub vendor_type_id: Vec<u64>,
    /// Vendor owner user IDs to match (space-separated)
    #[arg(long, num_args = 1..)]
    pub vendor_user_id: Vec<u64>,
    /// Risk level to match
    #[arg(long, value_enum, ignore_case = true)]
    pub risk: Option<VendorRisk>,
    /// Impact level to match
    #[arg(long, value_enum, ignore_case = true)]
    pub impact_level: Option<ImpactLevel>,
    /// Match on whether the vendor is a subprocessor (true or false)
    #[arg(long)]
    pub sub_processor: Option<bool>,
    /// Match on whether the vendor needs action (true or false)
    #[arg(long)]
    pub action_required: Option<bool>,
    /// Renewal date on or after this date (YYYY-MM-DD; spec param `renewalDateGte`)
    #[arg(long)]
    pub renewal_from: Option<String>,
    /// Renewal date on or before this date (YYYY-MM-DD; spec param `renewalDateLte`)
    #[arg(long)]
    pub renewal_to: Option<String>,
    /// Next review deadline status (space-separated)
    #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
    pub review_deadline_status: Vec<ReviewDeadlineStatus>,
    /// Latest security review status (space-separated)
    #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
    pub security_review_status: Vec<SecurityReviewStatus>,
    /// Latest security review decision (space-separated)
    #[arg(long, value_enum, ignore_case = true, num_args = 1..)]
    pub security_review_decision: Vec<SecurityReviewDecision>,
    /// Sub-collections to expand (space-separated)
    #[arg(long, value_enum, num_args = 1..)]
    pub expand: Vec<VendorSearchExpand>,
    /// Count these facets: fetches one page and prints the whole
    /// `{data, pagination, facets}` envelope as JSON (space-separated)
    #[arg(long, value_enum, num_args = 1..)]
    pub facet: Vec<VendorSearchFacet>,
}
