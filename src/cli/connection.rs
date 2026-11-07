//! clap types for `drata connection aws | azure | gcp | gitlab ...`. The four
//! providers share one verb set and one argument shape; the provider is the
//! subcommand above the verb.

use clap::{Args, Subcommand, ValueEnum};

/// Sub-collections a connection read can expand (spec `StandardConnectionExpandEnum`).
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "camelCase")]
pub enum ConnectionExpand {
    Workspaces,
}

#[derive(Args, Clone, Debug)]
pub struct ConnectionListArgs {
    /// Sub-collections to expand (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub expand: Vec<ConnectionExpand>,
}

#[derive(Args, Clone, Debug)]
pub struct ConnectionGetArgs {
    /// Connection ID: numeric, or the provider's prefixed form (aws `roleArn:<role ARN>`,
    /// azure `subscriptionId:<subscription ID>`, gcp `projectId:<project ID>`; gitlab is
    /// numeric only). Pass an ARN unencoded: drata percent-encodes its `/`
    pub connection_id: String,
    /// Sub-collections to expand (space-separated)
    #[arg(long, num_args = 1.., value_enum, ignore_case = true)]
    pub expand: Vec<ConnectionExpand>,
}

/// The request body, readable only from a file or stdin: it carries cloud
/// credentials (a secret key, a token), which an inline argument would leave in
/// shell history and the process table.
#[derive(Args, Clone, Debug)]
pub struct ConnectionBodyArgs {
    /// Request body as @file or - for stdin; inline JSON is rejected because the body carries credentials
    #[arg(long, required_unless_present = "example")]
    pub data: Option<String>,
    /// Print the request body skeleton for this provider and exit
    #[arg(long)]
    pub example: bool,
}

#[derive(Subcommand, Debug)]
pub enum ConnectionVerb {
    /// List connections
    List(ConnectionListArgs),
    /// Get one connection
    Get(ConnectionGetArgs),
    /// Create a connection (--data @file or - only; the body carries credentials)
    Create(ConnectionBodyArgs),
    /// Update a connection (--data @file or - only; a new credential re-validates it)
    Update {
        /// Connection ID: numeric, or the provider's prefixed form (aws `roleArn:<role ARN>`,
        /// azure `subscriptionId:<subscription ID>`, gcp `projectId:<project ID>`; gitlab is
        /// numeric only). Pass an ARN unencoded: drata percent-encodes its `/`
        #[arg(required_unless_present = "example")]
        connection_id: Option<String>,
        #[command(flatten)]
        body: ConnectionBodyArgs,
    },
    /// Delete a connection
    Remove {
        /// Connection ID: numeric, or the provider's prefixed form (aws `roleArn:<role ARN>`,
        /// azure `subscriptionId:<subscription ID>`, gcp `projectId:<project ID>`; gitlab is
        /// numeric only). Pass an ARN unencoded: drata percent-encodes its `/`
        connection_id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConnectionAction {
    /// AWS connections (an IAM role Drata assumes)
    Aws {
        #[command(subcommand)]
        verb: ConnectionVerb,
    },
    /// Azure connections (an App Registration)
    Azure {
        #[command(subcommand)]
        verb: ConnectionVerb,
    },
    /// GCP connections (a service account key)
    Gcp {
        #[command(subcommand)]
        verb: ConnectionVerb,
    },
    /// GitLab connections (a personal access token)
    Gitlab {
        #[command(subcommand)]
        verb: ConnectionVerb,
    },
}
