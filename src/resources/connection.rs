//! `drata connection aws | azure | gcp | gitlab ...`: the Standard Connections.
//! One implementation, parameterized by `Provider`: the four specs differ only
//! in the path prefix and the request body.
//!
//! | Command | Method | Path |
//! |---------|--------|------|
//! | `list` | GET | `/<provider>-connections` |
//! | `get` | GET | `/<provider>-connections/{connectionId}` |
//! | `create` | POST | `/<provider>-connections` |
//! | `update` | PUT | `/<provider>-connections/{connectionId}` |
//! | `remove` | DELETE | `/<provider>-connections/{connectionId}` |
//!
//! `create` and `update` bodies carry cloud credentials (an Azure client
//! secret, a GCP service account key, a GitLab token), so they come only from
//! `--data @file` or `--data -`, and nothing here logs, prints or echoes a body.
//! The spec's responses carry no credential field: every credential property is
//! documented "Never returned in any response". The one sensitive-looking field
//! a read returns is the AWS `externalId`, which the operator needs to write the
//! role's trust policy.

use crate::cli::{
    ConnectionAction, ConnectionBodyArgs, ConnectionExpand, ConnectionGetArgs, ConnectionListArgs, ConnectionVerb,
};
use crate::client::{DrataClient, encode_path_segment};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use crate::spec;
use eyre::{Result, bail};
use serde_json::{Value, json};
use tracing::{debug, instrument};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Provider {
    Aws,
    Azure,
    Gcp,
    Gitlab,
}

impl Provider {
    /// The collection path, also the spec path template of `list` and `create`.
    pub fn collection(self) -> &'static str {
        match self {
            Provider::Aws => "/aws-connections",
            Provider::Azure => "/azure-connections",
            Provider::Gcp => "/gcp-connections",
            Provider::Gitlab => "/gitlab-connections",
        }
    }

    /// The command noun, for error messages.
    fn noun(self) -> &'static str {
        match self {
            Provider::Aws => "connection aws",
            Provider::Azure => "connection azure",
            Provider::Gcp => "connection gcp",
            Provider::Gitlab => "connection gitlab",
        }
    }

    /// The spec path template of the item ops, for `--example` on `update`.
    fn item_template(self) -> String {
        format!("{}/{{connectionId}}", self.collection())
    }
}

/// `.../{connectionId}`. Pure.
pub fn item_path(provider: Provider, connection_id: u64) -> String {
    format!(
        "{}/{}",
        provider.collection(),
        encode_path_segment(&connection_id.to_string())
    )
}

/// `GET` collection with `expand[]`. Pure.
pub fn list_path(provider: Provider, expand: &[ConnectionExpand]) -> String {
    let mut q = QueryPath::new(provider.collection().to_string());
    q.push_each("expand[]", expand.iter().map(wire_name));
    q.finish()
}

/// `GET` one connection with `expand[]`. Pure.
pub fn get_path(provider: Provider, args: &ConnectionGetArgs) -> String {
    let mut q = QueryPath::new(item_path(provider, args.connection_id));
    q.push_each("expand[]", args.expand.iter().map(wire_name));
    q.finish()
}

/// The `--data` body for `create` or `update`: `@file` or `-` only, and a JSON
/// object. An error names the problem, never a value from the body. Reading
/// happens before the confirm and before any request.
pub fn read_body(data: &str) -> Result<Value> {
    let body = read_data(data, DataSource::FileOrStdinOnly)?;
    if !body.is_object() {
        bail!("--data must be a JSON object, got {}", json_kind(&body));
    }
    Ok(body)
}

fn json_kind(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::Number(_) => "a number",
        Value::String(_) => "a string",
        Value::Array(_) => "an array",
        Value::Object(_) => "an object",
    }
}

fn example(method: &str, template: &str) -> Result<String> {
    spec::example_for_operation(method, template)?
        .ok_or_else(|| eyre::eyre!("operation `{method} {template}` has no JSON request body"))
}

fn provider_verb(action: &ConnectionAction) -> (Provider, &ConnectionVerb) {
    match action {
        ConnectionAction::Aws { verb } => (Provider::Aws, verb),
        ConnectionAction::Azure { verb } => (Provider::Azure, verb),
        ConnectionAction::Gcp { verb } => (Provider::Gcp, verb),
        ConnectionAction::Gitlab { verb } => (Provider::Gitlab, verb),
    }
}

pub fn example_if_requested(action: &ConnectionAction) -> Option<Result<String>> {
    let (provider, verb) = provider_verb(action);
    match verb {
        ConnectionVerb::Create(ConnectionBodyArgs { example: true, .. }) => {
            Some(example("POST", provider.collection()))
        }
        ConnectionVerb::Update {
            body: ConnectionBodyArgs { example: true, .. },
            ..
        } => Some(example("PUT", &provider.item_template())),
        _ => None,
    }
}

#[instrument(skip_all)]
pub async fn handle(
    action: &ConnectionAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    let (provider, verb) = provider_verb(action);
    match verb {
        ConnectionVerb::List(ConnectionListArgs { expand }) => {
            let path = list_path(provider, expand);
            debug!(path, noun = provider.noun(), "connection list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        ConnectionVerb::Get(args) => {
            let path = get_path(provider, args);
            debug!(path, noun = provider.noun(), "connection get");
            print_value(&client.get(&path).await?, &config.output_format);
        }
        ConnectionVerb::Create(args) => {
            let Some(data) = args.data.as_deref() else {
                bail!(
                    "`{} create` requires --data @file or - (or use --example)",
                    provider.noun()
                );
            };
            let body = read_body(data)?;
            let path = provider.collection();
            debug!(path, noun = provider.noun(), "connection create");
            if !confirm("POST", path)? {
                bail!("aborted");
            }
            print_value(&client.post(path, body).await?, &config.output_format);
        }
        ConnectionVerb::Update { connection_id, body } => {
            let (Some(connection_id), Some(data)) = (connection_id, body.data.as_deref()) else {
                bail!(
                    "`{} update` requires <connection_id> and --data @file or - (or use --example)",
                    provider.noun()
                );
            };
            let payload = read_body(data)?;
            let path = item_path(provider, *connection_id);
            debug!(path, noun = provider.noun(), "connection update");
            if !confirm("PUT", &path)? {
                bail!("aborted");
            }
            print_value(&client.put(&path, payload).await?, &config.output_format);
        }
        ConnectionVerb::Remove { connection_id } => {
            let path = item_path(provider, *connection_id);
            debug!(path, noun = provider.noun(), "connection remove");
            if !confirm("DELETE", &path)? {
                bail!("aborted");
            }
            print_value(&client.delete(&path).await?, &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
