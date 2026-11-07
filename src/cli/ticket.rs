//! clap types for `drata control|risk|monitor ticket`.
//!
//! The three parents share verbs (`list | link | unlink`) and one
//! implementation (`src/resources/ticket.rs`). They differ in their positional
//! ids, so each parent has its own enum. Risks and monitoring tests also have
//! `create` (the `action-create` op); controls have no such op, so
//! `ControlTicketAction` has no `Create`.

use clap::{Args, Subcommand};

/// Flags for `ticket create`. `--data` carries the whole body and excludes the
/// field flags.
#[derive(Args, Debug)]
pub struct TicketCreateArgs {
    /// Connected ticketing-system connection ID
    #[arg(long, conflicts_with = "data")]
    pub connection_id: Option<u64>,
    /// Project ID in the ticketing system
    #[arg(long, conflicts_with = "data")]
    pub project_id: Option<String>,
    /// Issue type ID in the ticketing system
    #[arg(long, conflicts_with = "data")]
    pub issue_type_id: Option<String>,
    /// List ID in the ticketing system
    #[arg(long, conflicts_with = "data")]
    pub list_id: Option<String>,
    /// Team ID in the ticketing system
    #[arg(long, conflicts_with = "data")]
    pub team_id: Option<String>,
    /// A ticket field as three values: <field> <type> <value>. Repeatable
    #[arg(long = "field", num_args = 3, value_names = ["FIELD", "TYPE", "VALUE"], action = clap::ArgAction::Append, conflicts_with = "data")]
    pub fields: Vec<String>,
    /// Render the ticket description as rich content
    #[arg(long, conflicts_with = "data")]
    pub use_rich_content: bool,
    /// Full JSON body: inline, @file or - for stdin
    #[arg(long)]
    pub data: Option<String>,
    /// Print a JSON skeleton and exit (no API call)
    #[arg(long)]
    pub example: bool,
}

#[derive(Subcommand, Debug)]
pub enum ControlTicketAction {
    /// List tickets linked to a control
    List {
        /// Workspace ID
        workspace_id: String,
        /// Control ID
        control_id: String,
        /// Filter on completion state (true or false)
        #[arg(long)]
        done: Option<bool>,
    },
    /// Link an existing ticket to a control by URL
    Link {
        /// Workspace ID
        workspace_id: String,
        /// Control ID
        control_id: String,
        /// URL of the ticket in the connected ticketing system
        #[arg(long)]
        ticket_url: String,
    },
    /// Unlink a ticket from a control
    Unlink {
        /// Workspace ID
        workspace_id: String,
        /// Control ID
        control_id: String,
        /// Drata ticket ID
        ticket_id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum RiskTicketAction {
    /// List tickets linked to a risk
    List {
        /// Risk register ID
        register_id: String,
        /// Risk ID
        risk_id: String,
        /// Filter on completion state (true or false)
        #[arg(long)]
        done: Option<bool>,
    },
    /// Link an existing ticket to a risk by URL
    Link {
        /// Risk register ID
        register_id: String,
        /// Risk ID
        risk_id: String,
        /// URL of the ticket in the connected ticketing system
        #[arg(long)]
        ticket_url: String,
    },
    /// Unlink a ticket from a risk
    Unlink {
        /// Risk register ID
        register_id: String,
        /// Risk ID
        risk_id: String,
        /// Drata ticket ID
        ticket_id: String,
    },
    /// Create a ticket in the connected ticketing system and link it to a risk
    Create {
        /// Risk register ID (required unless --example)
        #[arg(required_unless_present = "example")]
        register_id: Option<String>,
        /// Risk ID (required unless --example)
        #[arg(required_unless_present = "example")]
        risk_id: Option<String>,
        #[command(flatten)]
        args: TicketCreateArgs,
    },
}

#[derive(Subcommand, Debug)]
pub enum MonitorTicketAction {
    /// List tickets linked to a monitoring test
    List {
        /// Workspace ID
        workspace_id: String,
        /// Test ID
        test_id: String,
        /// Filter on completion state (true or false)
        #[arg(long)]
        done: Option<bool>,
    },
    /// Link an existing ticket to a monitoring test by URL
    Link {
        /// Workspace ID
        workspace_id: String,
        /// Test ID
        test_id: String,
        /// URL of the ticket in the connected ticketing system
        #[arg(long)]
        ticket_url: String,
    },
    /// Unlink a ticket from a monitoring test
    Unlink {
        /// Workspace ID
        workspace_id: String,
        /// Test ID
        test_id: String,
        /// Drata ticket ID
        ticket_id: String,
    },
    /// Create a ticket in the connected ticketing system and link it to a monitoring test
    Create {
        /// Workspace ID (required unless --example)
        #[arg(required_unless_present = "example")]
        workspace_id: Option<String>,
        /// Test ID (required unless --example)
        #[arg(required_unless_present = "example")]
        test_id: Option<String>,
        #[command(flatten)]
        args: TicketCreateArgs,
    },
}
