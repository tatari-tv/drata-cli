//! clap types for `drata access-review ...`: the spec's Access Reviews tag.
//! `app` is an Access Review Application (a system whose user access is
//! reviewed, `/access-review-applications`); `period` is a review period
//! (`/access-review-periods`), which scopes applications in and assigns their
//! reviewers. Only one review period may be active on the account at a time.

use clap::{Args, Subcommand, ValueEnum};

/// Review period statuses a list can filter on (spec `AccessReviewPeriodStatusEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccessReviewPeriodStatus {
    Active,
    Completed,
}

/// Actions on a review period and on an application within one (spec
/// `AccessReviewPeriodActionEnum` and `AccessReviewPeriodApplicationActionEnum`;
/// the two carry the same values).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum AccessReviewActionKind {
    Complete,
}

/// One reviewable user for `app users set`, parsed from
/// `IDENTITY_ID:USERNAME:EMAIL[:ROLE,ROLE...]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviewUser {
    pub identity_id: String,
    pub username: String,
    pub email: String,
    pub roles: Vec<String>,
}

/// One application scoped into a new review period, parsed from
/// `APPLICATION_ID:REVIEWER_ID[,REVIEWER_ID...]`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReviewedApplication {
    pub application_id: u64,
    pub reviewer_ids: Vec<u64>,
}

/// Parse `--user IDENTITY_ID:USERNAME:EMAIL[:ROLE,ROLE...]`. The first three
/// parts are required and non-empty; roles are optional and comma-separated.
/// A role containing a comma, or an id containing a colon, goes through `--data`.
pub fn review_user(raw: &str) -> Result<ReviewUser, String> {
    let mut parts = raw.splitn(4, ':');
    let mut next = |what: &str| match parts.next() {
        Some(p) if !p.trim().is_empty() => Ok(p.trim().to_string()),
        _ => Err(format!(
            "`{raw}` has no {what}; expected IDENTITY_ID:USERNAME:EMAIL[:ROLE,ROLE...]"
        )),
    };
    let identity_id = next("identity id")?;
    let username = next("username")?;
    let email = next("email")?;
    let roles = parts
        .next()
        .map(|r| {
            r.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default();
    Ok(ReviewUser {
        identity_id,
        username,
        email,
        roles,
    })
}

/// Parse `--application APPLICATION_ID:REVIEWER_ID[,REVIEWER_ID...]`. Every
/// application in a period needs at least one reviewer (the spec requires
/// `reviewersIds`), so a bare id is rejected.
pub fn reviewed_application(raw: &str) -> Result<ReviewedApplication, String> {
    let usage = "expected APPLICATION_ID:REVIEWER_ID[,REVIEWER_ID...]";
    let (app, reviewers) = raw
        .split_once(':')
        .ok_or_else(|| format!("`{raw}` names no reviewers; {usage}"))?;
    let application_id = app
        .trim()
        .parse::<u64>()
        .map_err(|_| format!("`{app}` is not a numeric application id; {usage}"))?;
    let reviewer_ids = reviewers
        .split(',')
        .map(|r| {
            r.trim()
                .parse::<u64>()
                .map_err(|_| format!("`{r}` is not a numeric reviewer id; {usage}"))
        })
        .collect::<Result<Vec<u64>, String>>()?;
    Ok(ReviewedApplication {
        application_id,
        reviewer_ids,
    })
}

#[derive(Subcommand, Debug)]
pub enum AccessReviewAction {
    /// Manage Access Review Applications and their loaded users
    App {
        #[command(subcommand)]
        action: AccessReviewAppAction,
    },
    /// Start, inspect and complete review periods and the applications in them
    Period {
        #[command(subcommand)]
        action: AccessReviewPeriodAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum AccessReviewAppAction {
    /// List Access Review Applications
    List,
    /// Get one Access Review Application
    Get {
        /// Application ID
        application_id: u64,
    },
    /// Create a manually-managed Access Review Application (load users with `users set`)
    Create {
        /// Application name (max 191 chars)
        #[arg(long, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        name: Option<String>,
        /// Vendor URL (max 191 chars)
        #[arg(long, conflicts_with = "data")]
        website_url: Option<String>,
        /// Drata user IDs to assign as the default reviewers (space-separated)
        #[arg(long = "reviewer-id", num_args = 1.., conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        reviewer_ids: Vec<u64>,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Update an application's name, vendor URL or default reviewers (omitted fields unchanged)
    Update {
        /// Application ID (required unless --example)
        #[arg(required_unless_present = "example")]
        application_id: Option<u64>,
        /// New name (manually-managed applications only)
        #[arg(long, conflicts_with = "data")]
        name: Option<String>,
        /// New vendor URL (manually-managed applications only)
        #[arg(long, conflicts_with_all = ["data", "clear_website_url"])]
        website_url: Option<String>,
        /// Clear the vendor URL (sends null)
        #[arg(long, conflicts_with = "data")]
        clear_website_url: bool,
        /// Drata user IDs that replace the default reviewers (space-separated)
        #[arg(long = "reviewer-id", num_args = 1.., conflicts_with = "data")]
        reviewer_ids: Vec<u64>,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Delete a manually-managed application (also removed from periods it has not completed in)
    Remove {
        /// Application ID
        application_id: u64,
    },
    /// List or replace the users loaded into an application
    Users {
        #[command(subcommand)]
        action: AccessReviewAppUsersAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum AccessReviewAppUsersAction {
    /// List the users currently loaded into an application
    List {
        /// Application ID
        application_id: u64,
    },
    /// Replace the application's full user list: users omitted from it are removed
    Set {
        /// Application ID (required unless --example)
        #[arg(required_unless_present = "example")]
        application_id: Option<u64>,
        /// Users as IDENTITY_ID:USERNAME:EMAIL[:ROLE,ROLE...] (space-separated); the complete set
        #[arg(long = "user", value_name = "USER", num_args = 1.., value_parser = review_user, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        users: Vec<ReviewUser>,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
}

/// A review period's id, optional in clap only so `--example` runs without it.
#[derive(Args, Clone, Debug)]
pub struct ExamplePeriodRef {
    /// Review period ID
    #[arg(required_unless_present = "example")]
    pub period_id: Option<u64>,
}

/// An application within a review period.
#[derive(Args, Clone, Debug)]
pub struct PeriodAppRef {
    /// Review period ID
    pub period_id: u64,
    /// Review application ID (the period's own id for the application, not the application ID)
    pub review_app_id: u64,
}

/// The same ids, optional in clap only so `--example` runs without them.
#[derive(Args, Clone, Debug)]
pub struct ExamplePeriodAppRef {
    /// Review period ID
    #[arg(required_unless_present = "example")]
    pub period_id: Option<u64>,
    /// Review application ID (the period's own id for the application, not the application ID)
    #[arg(required_unless_present = "example")]
    pub review_app_id: Option<u64>,
}

#[derive(Subcommand, Debug)]
pub enum AccessReviewPeriodAction {
    /// List review periods
    List {
        /// Only active or only completed periods
        #[arg(long, value_enum, ignore_case = true)]
        status: Option<AccessReviewPeriodStatus>,
    },
    /// Get a review period and the applications scoped into it
    Get {
        /// Review period ID
        period_id: u64,
    },
    /// Start a review period (only one may be active on the account at a time)
    Create {
        /// Starting date-time, e.g. 2026-11-01T00:00:00Z
        #[arg(long, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        starting_date: Option<String>,
        /// Ending date-time, e.g. 2026-12-01T00:00:00Z
        #[arg(long, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        ending_date: Option<String>,
        /// Applications to review, as APPLICATION_ID:REVIEWER_ID[,REVIEWER_ID...] (space-separated)
        #[arg(long = "application", value_name = "APPLICATION", num_args = 1.., value_parser = reviewed_application, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        applications: Vec<ReviewedApplication>,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Complete a review period (every application in it must be completed first)
    RunAction {
        #[command(flatten)]
        target: ExamplePeriodRef,
        /// Action to run (required unless --data/--example)
        #[arg(long, value_enum, ignore_case = true, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        action: Option<AccessReviewActionKind>,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Inspect, reassign and complete one application within a review period
    App {
        #[command(subcommand)]
        action: AccessReviewPeriodAppAction,
    },
    /// Print the signed URL of a period's evidence package ZIP (not downloaded)
    EvidencePackage {
        /// Review period ID
        period_id: u64,
    },
}

#[derive(Subcommand, Debug)]
pub enum AccessReviewPeriodAppAction {
    /// Get an application in a review period: reviewers and user counts by decision
    Get(PeriodAppRef),
    /// Replace an application's reviewers in a review period (also added to its default reviewers)
    Update {
        #[command(flatten)]
        target: ExamplePeriodAppRef,
        /// Drata user IDs that replace the reviewers (space-separated)
        #[arg(long = "reviewer-id", num_args = 1.., conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        reviewer_ids: Vec<u64>,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// Mark an application in a review period as reviewed
    RunAction {
        #[command(flatten)]
        target: ExamplePeriodAppRef,
        /// Action to run (required unless --data/--example)
        #[arg(long, value_enum, ignore_case = true, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        action: Option<AccessReviewActionKind>,
        /// Mark every still NOT_REVIEWED user OUT_OF_SCOPE (without it, the server answers 409 while any remain)
        #[arg(long, conflicts_with = "data")]
        mark_unreviewed_out_of_scope: bool,
        /// Full JSON body: inline, @file or - for stdin
        #[arg(long)]
        data: Option<String>,
        /// Print a JSON skeleton and exit (no API call)
        #[arg(long)]
        example: bool,
    },
    /// List the users under review in an application, with each user's decision
    Users(PeriodAppRef),
}
