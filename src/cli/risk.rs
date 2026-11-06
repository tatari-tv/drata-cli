//! clap types for `drata risk search`.

use super::{RiskStatus, RiskTreatmentPlan};
use clap::{Args, ValueEnum};

/// Risk kind filter (spec `RiskVersionTypeEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RiskKind {
    Internal,
    External,
}

/// Sub-collections `risk search` can expand (spec `RiskSearchExpandEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum RiskSearchExpand {
    Owners,
    Reviewers,
    Controls,
    Categories,
    Documents,
    Notes,
    Tickets,
    Tasks,
    CustomFields,
}

/// Facets `risk search` can count (spec `RiskSearchFacetEnum`). `RiskRegisterId`
/// is valid only across all registers (`RiskSearchInRegisterFacetEnum` omits it).
#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
#[clap(rename_all = "camelCase")]
pub enum RiskSearchFacet {
    Status,
    TreatmentPlan,
    Type,
    Categories,
    Controls,
    OwnerEmails,
    ReviewerEmails,
    VendorNames,
    RiskRegisterId,
}

#[derive(Args, Debug)]
pub struct RiskSearchArgs {
    /// Search one risk register; omit to search every register
    #[arg(long)]
    pub register_id: Option<String>,
    /// Free-text query (max 191 chars)
    #[arg(long)]
    pub q: Option<String>,
    /// Statuses to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub status: Vec<RiskStatus>,
    /// Treatment plans to match (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub treatment_plan: Vec<RiskTreatmentPlan>,
    /// Risk kinds to match (space-separated)
    #[arg(long = "type", num_args = 1.., value_enum, ignore_case = true)]
    pub kinds: Vec<RiskKind>,
    /// Minimum inherent score
    #[arg(long)]
    pub inherent_score_gte: Option<f64>,
    /// Maximum inherent score
    #[arg(long)]
    pub inherent_score_lte: Option<f64>,
    /// Minimum residual score
    #[arg(long)]
    pub residual_score_gte: Option<f64>,
    /// Maximum residual score
    #[arg(long)]
    pub residual_score_lte: Option<f64>,
    /// Category names to match (space-separated)
    #[arg(long = "category", num_args = 1..)]
    pub categories: Vec<String>,
    /// Control codes to match (space-separated)
    #[arg(long = "control", num_args = 1..)]
    pub controls: Vec<String>,
    /// Owner emails to match (space-separated)
    #[arg(long = "owner-email", num_args = 1..)]
    pub owner_emails: Vec<String>,
    /// Reviewer emails to match (space-separated)
    #[arg(long = "reviewer-email", num_args = 1..)]
    pub reviewer_emails: Vec<String>,
    /// Vendor names to match (space-separated)
    #[arg(long = "vendor-name", num_args = 1..)]
    pub vendor_names: Vec<String>,
    /// Sub-collections to expand (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub expand: Vec<RiskSearchExpand>,
    /// Count these facets: fetches one page and prints the whole
    /// {data, pagination, facets} envelope as JSON (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub facet: Vec<RiskSearchFacet>,
}
