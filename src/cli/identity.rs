//! clap types for `drata infrastructure-identity ...` and
//! `drata version-control-identity ...`. The two nouns share their argument
//! structs; only the `--action` enum differs, because the two specs list
//! different actions.

use clap::{Args, Subcommand, ValueEnum};

/// Sub-collections an identity read can expand (spec `UserIdentityExpandEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum IdentityExpand {
    User,
    ServiceAccountDesignator,
    Connection,
}

/// Actions on an Infrastructure User Identity (spec `InfrastructureUserIdentityActionEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum InfrastructureIdentityActionKind {
    LinkUser,
    UnlinkUser,
    MarkServiceAccount,
    UnmarkServiceAccount,
}

/// Actions on a Version Control User Identity (spec `VersionControlUserIdentityActionEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum VersionControlIdentityActionKind {
    LinkUser,
    UnlinkUser,
    MarkServiceAccount,
    UnmarkServiceAccount,
    AcknowledgeWriteAccess,
    RetractWriteAccess,
    AcknowledgePushProductionCodeAccess,
    RetractPushProductionCodeAccess,
}

/// The identity a verb addresses.
#[derive(Args, Clone, Debug)]
pub struct IdentityRef {
    /// Workspace ID
    pub workspace_id: String,
    /// Identity ID (the numeric `id` from `list`, not the external `identityId`)
    pub identity_id: String,
}

/// The same ids, optional in clap only so `--example` runs without them.
#[derive(Args, Clone, Debug)]
pub struct ExampleIdentityRef {
    /// Workspace ID
    #[arg(required_unless_present = "example")]
    pub workspace_id: Option<String>,
    /// Identity ID (the numeric `id` from `list`, not the external `identityId`)
    #[arg(required_unless_present = "example")]
    pub identity_id: Option<String>,
}

#[derive(Args, Clone, Debug)]
pub struct IdentityListArgs {
    /// Workspace ID
    pub workspace_id: String,
    /// Only the identity with this external service ID
    #[arg(long)]
    pub identity_id: Option<String>,
    /// Only this external username
    #[arg(long)]
    pub username: Option<String>,
    /// Only this external email
    #[arg(long)]
    pub email: Option<String>,
    /// Only identities synced from this client type (the spec's ClientTypeEnum, e.g. GITHUB)
    #[arg(long, value_parser = client_type)]
    pub client_type: Option<String>,
    /// Only identities synced from this connection
    #[arg(long)]
    pub connection_id: Option<u64>,
    /// Sub-collections to expand (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub expand: Vec<IdentityExpand>,
}

#[derive(Args, Clone, Debug)]
pub struct IdentityGetArgs {
    #[command(flatten)]
    pub target: IdentityRef,
    /// Sub-collections to expand (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub expand: Vec<IdentityExpand>,
}

/// The `run-action` fields every kind shares, plus the full-body escape hatch.
#[derive(Args, Clone, Debug)]
pub struct IdentityBodyArgs {
    /// Drata user ID to link (linkUser only)
    #[arg(long, conflicts_with = "data")]
    pub user_id: Option<u64>,
    /// Why this identity is a service account (markServiceAccount only, max 30000 chars)
    #[arg(long, conflicts_with = "data")]
    pub reason: Option<String>,
    /// Full request body: inline JSON, @file, or - for stdin (overrides individual flags)
    #[arg(long)]
    pub data: Option<String>,
    /// Print the request body skeleton from the spec and exit
    #[arg(long)]
    pub example: bool,
}

/// Parse `--client-type` against the spec's `ClientTypeEnum`, case-insensitively.
/// The enum runs to hundreds of values and grows with each Drata release, so the
/// embedded spec is the list rather than a hand-copied clap enum.
pub fn client_type(raw: &str) -> Result<String, String> {
    let values = crate::spec::enum_values("ClientTypeEnum").map_err(|e| e.to_string())?;
    values
        .into_iter()
        .find(|v| v.eq_ignore_ascii_case(raw))
        .ok_or_else(|| format!("`{raw}` is not a ClientTypeEnum value in the spec (e.g. GITHUB, AWS, GITLAB)"))
}

#[derive(Subcommand, Debug)]
pub enum InfrastructureIdentityAction {
    /// List Infrastructure User Identities in a workspace
    List(IdentityListArgs),
    /// Get one Infrastructure User Identity
    Get(IdentityGetArgs),
    /// List the actions available on an Infrastructure User Identity now
    Actions(IdentityRef),
    /// Link or unlink a Drata user, or mark or unmark a service account
    RunAction {
        #[command(flatten)]
        target: ExampleIdentityRef,
        /// Action to run (required unless --data/--example)
        #[arg(long, value_enum, ignore_case = true, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        action: Option<InfrastructureIdentityActionKind>,
        #[command(flatten)]
        body: IdentityBodyArgs,
    },
}

#[derive(Subcommand, Debug)]
pub enum VersionControlIdentityAction {
    /// List Version Control User Identities in a workspace
    List(IdentityListArgs),
    /// Get one Version Control User Identity
    Get(IdentityGetArgs),
    /// List the actions available on a Version Control User Identity now
    Actions(IdentityRef),
    /// Link or unlink a Drata user, mark or unmark a service account, or acknowledge or retract access
    RunAction {
        #[command(flatten)]
        target: ExampleIdentityRef,
        /// Action to run (required unless --data/--example)
        #[arg(long, value_enum, ignore_case = true, conflicts_with = "data", required_unless_present_any = ["data", "example"])]
        action: Option<VersionControlIdentityActionKind>,
        #[command(flatten)]
        body: IdentityBodyArgs,
    },
}
