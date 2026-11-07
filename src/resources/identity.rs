//! `drata infrastructure-identity ...` and `drata version-control-identity ...`.
//! One implementation, parameterized by `Kind`: the two specs differ only in
//! the path prefix and the action list.
//!
//! | Command | Method | Path |
//! |---------|--------|------|
//! | `list` | GET | `/workspaces/{w}/<kind>-user-identities` |
//! | `get` | GET | `/workspaces/{w}/<kind>-user-identities/{id}` |
//! | `actions` | GET | `/workspaces/{w}/<kind>-user-identities/{id}/actions` |
//! | `run-action` | POST | `/workspaces/{w}/<kind>-user-identities/{id}/actions` |

use crate::cli::{
    IdentityBodyArgs, IdentityGetArgs, IdentityListArgs, IdentityRef, InfrastructureIdentityAction,
    VersionControlIdentityAction,
};
use crate::client::{DrataClient, encode_path_segment};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::{Table, print_value, print_value_as};
use crate::query::{QueryPath, wire_name};
use crate::spec;
use eyre::{Result, bail};
use serde_json::{Value, json};
use tracing::{debug, instrument};

/// The spec's `maxLength` on `reason`.
const MAX_REASON: usize = 30000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Infrastructure,
    VersionControl,
}

impl Kind {
    /// The path segment after `/workspaces/{w}/`.
    fn segment(self) -> &'static str {
        match self {
            Kind::Infrastructure => "infrastructure-user-identities",
            Kind::VersionControl => "version-control-user-identities",
        }
    }

    /// The command noun, for error messages.
    fn noun(self) -> &'static str {
        match self {
            Kind::Infrastructure => "infrastructure-identity",
            Kind::VersionControl => "version-control-identity",
        }
    }

    /// The spec path template of the actions op, for `--example`.
    fn actions_template(self) -> String {
        format!("/workspaces/{{workspaceId}}/{}/{{identityId}}/actions", self.segment())
    }
}

/// `/workspaces/{w}/<kind>-user-identities`. Pure.
pub fn collection_path(kind: Kind, workspace_id: &str) -> String {
    format!("/workspaces/{}/{}", encode_path_segment(workspace_id), kind.segment())
}

/// `.../{identityId}`. Pure.
pub fn item_path(kind: Kind, target: &IdentityRef) -> String {
    format!(
        "{}/{}",
        collection_path(kind, &target.workspace_id),
        encode_path_segment(&target.identity_id)
    )
}

/// `.../{identityId}/actions`, the list and the run path. Pure.
pub fn actions_path(kind: Kind, target: &IdentityRef) -> String {
    format!("{}/actions", item_path(kind, target))
}

/// `GET` collection with every filter the op takes. Pure.
pub fn list_path(kind: Kind, args: &IdentityListArgs) -> String {
    let mut q = QueryPath::new(collection_path(kind, &args.workspace_id));
    q.push_opt("identityId", args.identity_id.as_deref())
        .push_opt("username", args.username.as_deref())
        .push_opt("email", args.email.as_deref())
        .push_opt("clientType", args.client_type.as_deref())
        .push_opt("connectionId", args.connection_id)
        .push_each("expand[]", args.expand.iter().map(wire_name));
    q.finish()
}

/// `GET` one identity with `expand[]`. Pure.
pub fn get_path(kind: Kind, args: &IdentityGetArgs) -> String {
    let mut q = QueryPath::new(item_path(kind, &args.target));
    q.push_each("expand[]", args.expand.iter().map(wire_name));
    q.finish()
}

/// The `run-action` body from flags. The spec requires `userId` with `linkUser`
/// and `reason` with `markServiceAccount`; a field paired with any other action
/// is rejected here, before the confirm and the request. Pure.
pub fn build_run_action_body(action: &str, user_id: Option<u64>, reason: Option<&str>) -> Result<Value> {
    let mut body = json!({ "action": action });
    match (action, user_id) {
        ("linkUser", Some(id)) => body["userId"] = json!(id),
        ("linkUser", None) => bail!("--action linkUser requires --user-id"),
        (_, Some(_)) => bail!("--user-id is only valid with --action linkUser"),
        (_, None) => {}
    }
    match (action, reason) {
        ("markServiceAccount", Some(r)) => {
            if r.chars().count() > MAX_REASON {
                bail!("--reason takes at most {MAX_REASON} characters");
            }
            body["reason"] = json!(r);
        }
        ("markServiceAccount", None) => bail!("--action markServiceAccount requires --reason"),
        (_, Some(_)) => bail!("--reason is only valid with --action markServiceAccount"),
        (_, None) => {}
    }
    Ok(body)
}

/// One verb, kind-independent: both clap enums map onto this.
enum Verb<'a> {
    List(&'a IdentityListArgs),
    Get(&'a IdentityGetArgs),
    Actions(&'a IdentityRef),
    RunAction {
        target: (Option<&'a str>, Option<&'a str>),
        action: Option<String>,
        body: &'a IdentityBodyArgs,
    },
}

fn example(kind: Kind) -> Result<String> {
    let template = kind.actions_template();
    spec::example_for_operation(crate::backend::Backend::Drata, "POST", &template)?
        .ok_or_else(|| eyre::eyre!("operation `POST {template}` has no JSON request body"))
}

pub fn example_if_requested_infrastructure(action: &InfrastructureIdentityAction) -> Option<Result<String>> {
    match action {
        InfrastructureIdentityAction::RunAction { body, .. } if body.example => Some(example(Kind::Infrastructure)),
        _ => None,
    }
}

pub fn example_if_requested_version_control(action: &VersionControlIdentityAction) -> Option<Result<String>> {
    match action {
        VersionControlIdentityAction::RunAction { body, .. } if body.example => Some(example(Kind::VersionControl)),
        _ => None,
    }
}

#[instrument(skip_all)]
pub async fn handle_infrastructure(
    action: &InfrastructureIdentityAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    let verb = match action {
        InfrastructureIdentityAction::List(args) => Verb::List(args),
        InfrastructureIdentityAction::Get(args) => Verb::Get(args),
        InfrastructureIdentityAction::Actions(target) => Verb::Actions(target),
        InfrastructureIdentityAction::RunAction { target, action, body } => Verb::RunAction {
            target: (target.workspace_id.as_deref(), target.identity_id.as_deref()),
            action: action.as_ref().map(wire_name),
            body,
        },
    };
    handle(Kind::Infrastructure, verb, client, config, confirm).await
}

#[instrument(skip_all)]
pub async fn handle_version_control(
    action: &VersionControlIdentityAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    let verb = match action {
        VersionControlIdentityAction::List(args) => Verb::List(args),
        VersionControlIdentityAction::Get(args) => Verb::Get(args),
        VersionControlIdentityAction::Actions(target) => Verb::Actions(target),
        VersionControlIdentityAction::RunAction { target, action, body } => Verb::RunAction {
            target: (target.workspace_id.as_deref(), target.identity_id.as_deref()),
            action: action.as_ref().map(wire_name),
            body,
        },
    };
    handle(Kind::VersionControl, verb, client, config, confirm).await
}

async fn handle(kind: Kind, verb: Verb<'_>, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match verb {
        Verb::List(args) => {
            let path = list_path(kind, args);
            debug!(path, noun = kind.noun(), "identity list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        Verb::Get(args) => {
            let path = get_path(kind, args);
            debug!(path, noun = kind.noun(), "identity get");
            print_value(&client.get(&path).await?, &config.output_format);
        }
        Verb::Actions(target) => {
            let path = actions_path(kind, target);
            debug!(path, noun = kind.noun(), "identity actions");
            print_value_as(&client.get(&path).await?, &config.output_format, Table::IdentityActions);
        }
        Verb::RunAction { target, action, body } => {
            let (Some(workspace_id), Some(identity_id)) = target else {
                bail!(
                    "`{} run-action` requires <workspace_id> <identity_id>; use --example to print the body skeleton",
                    kind.noun()
                );
            };
            let path = actions_path(
                kind,
                &IdentityRef {
                    workspace_id: workspace_id.to_string(),
                    identity_id: identity_id.to_string(),
                },
            );
            let payload = match (&body.data, action) {
                (Some(raw), _) => read_data(raw, DataSource::Any)?,
                (None, Some(action)) => build_run_action_body(&action, body.user_id, body.reason.as_deref())?,
                (None, None) => bail!("requires --action (or use --data/--example)"),
            };
            debug!(path, noun = kind.noun(), "identity run-action");
            if !confirm("POST", &path)? {
                bail!("aborted");
            }
            print_value(&client.post(&path, payload).await?, &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
