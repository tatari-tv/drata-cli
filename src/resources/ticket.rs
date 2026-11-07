//! Tickets linked to a control, risk or monitoring test.
//!
//! One implementation, parameterized by the parent's `.../tickets` path:
//!
//! | Verb | Method | Path |
//! |------|--------|------|
//! | `list` | GET | `<parent>/tickets` |
//! | `link` | POST | `<parent>/tickets` |
//! | `unlink` | DELETE | `<parent>/tickets/{ticketId}` |
//! | `create` | POST | `<parent>/tickets/action-create` (risk and monitor only) |
//!
//! Controls have no `action-create` op, so `ControlTicketAction` has no
//! `Create` variant and `control_target` can never yield `TicketOp::Create`.

use crate::cli::{ControlTicketAction, MonitorTicketAction, RiskTicketAction, TicketCreateArgs};
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::print_value;
use crate::query::QueryPath;
use crate::spec;
use eyre::{Result, bail};
use serde_json::{Value, json};
use tracing::{debug, instrument};

/// Spec `TicketFieldTypePublicV2Enum`: the types `--field` accepts.
pub const FIELD_TYPES: &[&str] = &[
    "datetime",
    "date",
    "label",
    "labels",
    "user",
    "priority",
    "string",
    "text",
    "textarea",
    "number",
    "boolean",
    "array",
    "multiple",
    "multiselect",
    "option",
    "dropdown",
];

/// One ticket verb, with the ids already folded into the parent path.
#[derive(Debug)]
pub enum TicketOp<'a> {
    List { done: Option<bool> },
    Link { ticket_url: &'a str },
    Unlink { ticket_id: &'a str },
    Create(&'a TicketCreateArgs),
}

impl TicketOp<'_> {
    /// The verb name, for logs: `Create` carries the `--data` body, so the op
    /// itself is never a log field.
    pub fn verb(&self) -> &'static str {
        match self {
            TicketOp::List { .. } => "list",
            TicketOp::Link { .. } => "link",
            TicketOp::Unlink { .. } => "unlink",
            TicketOp::Create(_) => "create",
        }
    }
}

/// `(<parent>/tickets, op)` for a control.
pub fn control_target(action: &ControlTicketAction) -> (String, TicketOp<'_>) {
    match action {
        ControlTicketAction::List {
            workspace_id,
            control_id,
            done,
        } => (
            control_tickets(workspace_id, control_id),
            TicketOp::List { done: *done },
        ),
        ControlTicketAction::Link {
            workspace_id,
            control_id,
            ticket_url,
        } => (control_tickets(workspace_id, control_id), TicketOp::Link { ticket_url }),
        ControlTicketAction::Unlink {
            workspace_id,
            control_id,
            ticket_id,
        } => (
            control_tickets(workspace_id, control_id),
            TicketOp::Unlink { ticket_id },
        ),
    }
}

/// `(<parent>/tickets, op)` for a risk. `create` without ids (only legal with
/// `--example`, which never reaches a handler) is an error.
pub fn risk_target(action: &RiskTicketAction) -> Result<(String, TicketOp<'_>)> {
    Ok(match action {
        RiskTicketAction::List {
            register_id,
            risk_id,
            done,
        } => (risk_tickets(register_id, risk_id), TicketOp::List { done: *done }),
        RiskTicketAction::Link {
            register_id,
            risk_id,
            ticket_url,
        } => (risk_tickets(register_id, risk_id), TicketOp::Link { ticket_url }),
        RiskTicketAction::Unlink {
            register_id,
            risk_id,
            ticket_id,
        } => (risk_tickets(register_id, risk_id), TicketOp::Unlink { ticket_id }),
        RiskTicketAction::Create {
            register_id,
            risk_id,
            args,
        } => {
            let (Some(register_id), Some(risk_id)) = (register_id, risk_id) else {
                bail!("`drata risk ticket create` requires <register_id> <risk_id> (or use --example)");
            };
            (risk_tickets(register_id, risk_id), TicketOp::Create(args))
        }
    })
}

/// `(<parent>/tickets, op)` for a monitoring test.
pub fn monitor_target(action: &MonitorTicketAction) -> Result<(String, TicketOp<'_>)> {
    Ok(match action {
        MonitorTicketAction::List {
            workspace_id,
            test_id,
            done,
        } => (monitor_tickets(workspace_id, test_id), TicketOp::List { done: *done }),
        MonitorTicketAction::Link {
            workspace_id,
            test_id,
            ticket_url,
        } => (monitor_tickets(workspace_id, test_id), TicketOp::Link { ticket_url }),
        MonitorTicketAction::Unlink {
            workspace_id,
            test_id,
            ticket_id,
        } => (monitor_tickets(workspace_id, test_id), TicketOp::Unlink { ticket_id }),
        MonitorTicketAction::Create {
            workspace_id,
            test_id,
            args,
        } => {
            let (Some(workspace_id), Some(test_id)) = (workspace_id, test_id) else {
                bail!("`drata monitor ticket create` requires <workspace_id> <test_id> (or use --example)");
            };
            (monitor_tickets(workspace_id, test_id), TicketOp::Create(args))
        }
    })
}

fn control_tickets(workspace_id: &str, control_id: &str) -> String {
    format!("/workspaces/{}/controls/{}/tickets", workspace_id, control_id)
}

fn risk_tickets(register_id: &str, risk_id: &str) -> String {
    format!("/risk-registers/{}/risks/{}/tickets", register_id, risk_id)
}

fn monitor_tickets(workspace_id: &str, test_id: &str) -> String {
    format!("/workspaces/{}/monitoring-tests/{}/tickets", workspace_id, test_id)
}

/// The `--example` skeleton for a `create` action, if that is what was asked.
pub fn create_example(args: &TicketCreateArgs, spec_path: &str) -> Option<Result<String>> {
    if !args.example {
        return None;
    }
    Some(
        spec::example_for_operation(crate::backend::Backend::Drata, "POST", spec_path)
            .and_then(|s| s.ok_or_else(|| eyre::eyre!("operation `POST {}` has no JSON request body", spec_path))),
    )
}

pub const RISK_CREATE_SPEC_PATH: &str = "/risk-registers/{riskRegisterId}/risks/{riskId}/tickets/action-create";
pub const MONITOR_CREATE_SPEC_PATH: &str = "/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets/action-create";

/// `GET <tickets>` path with the optional `isDone` filter. Pure.
fn build_list_path(tickets: &str, done: Option<bool>) -> String {
    let mut q = QueryPath::new(tickets);
    q.push_opt("isDone", done);
    q.finish()
}

/// `POST <tickets>` body: the ticket URL. Pure.
fn build_link_body(ticket_url: &str) -> Value {
    json!({ "ticketUrl": ticket_url })
}

/// `DELETE <tickets>/{ticketId}` path. Pure.
fn build_unlink_path(tickets: &str, ticket_id: &str) -> String {
    format!("{}/{}", tickets, ticket_id)
}

/// `POST <tickets>/action-create` path. Pure.
fn build_create_path(tickets: &str) -> String {
    format!("{}/action-create", tickets)
}

/// Build the create body from flags. Pure. `connectionId` and at least one
/// field are required by the spec; each `--field` is `<field> <type> <value>`
/// with the type checked against the spec enum.
fn build_create_body(args: &TicketCreateArgs) -> Result<Value> {
    let connection_id = args
        .connection_id
        .ok_or_else(|| eyre::eyre!("ticket create requires --connection-id (or use --data/--example)"))?;
    if args.fields.is_empty() {
        bail!("ticket create requires at least one --field <field> <type> <value> (or use --data/--example)");
    }
    let mut fields = Vec::new();
    for triple in args.fields.chunks(3) {
        let [field, kind, value] = triple else {
            bail!("--field takes exactly three values: <field> <type> <value>");
        };
        if !FIELD_TYPES.contains(&kind.as_str()) {
            bail!("--field type `{}` is not one of: {}", kind, FIELD_TYPES.join(", "));
        }
        fields.push(json!({ "field": field, "type": kind, "value": value }));
    }
    let mut body = json!({ "connectionId": connection_id, "fields": fields });
    for (key, value) in [
        ("projectId", &args.project_id),
        ("issueTypeId", &args.issue_type_id),
        ("listId", &args.list_id),
        ("teamId", &args.team_id),
    ] {
        if let Some(v) = value {
            body[key] = json!(v);
        }
    }
    if args.use_rich_content {
        body["useRichContent"] = json!(true);
    }
    Ok(body)
}

/// Run one ticket verb against the parent's `<parent>/tickets` path.
#[instrument(skip(client, config, confirm, op))]
pub async fn handle(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    tickets: &str,
    op: TicketOp<'_>,
) -> Result<()> {
    debug!(tickets, verb = op.verb(), "ticket");
    match op {
        TicketOp::List { done } => {
            let items = client.get_all(&build_list_path(tickets, done)).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        TicketOp::Link { ticket_url } => {
            if !confirm("POST", tickets)? {
                bail!("aborted");
            }
            let result = client.post(tickets, build_link_body(ticket_url)).await?;
            print_value(&result, &config.output_format);
        }
        TicketOp::Unlink { ticket_id } => {
            let path = build_unlink_path(tickets, ticket_id);
            if !confirm("DELETE", &path)? {
                bail!("aborted");
            }
            let result = client.delete(&path, None).await?;
            print_value(&result, &config.output_format);
        }
        TicketOp::Create(args) => {
            let body = match &args.data {
                Some(raw) => read_data(raw, DataSource::Any)?,
                None => build_create_body(args)?,
            };
            let path = build_create_path(tickets);
            if !confirm("POST", &path)? {
                bail!("aborted");
            }
            let result = client.post(&path, body).await?;
            print_value(&result, &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
