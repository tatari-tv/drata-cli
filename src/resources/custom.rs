//! `drata custom-field ...` and `drata custom-connection ...`.
//!
//! | Command | Method | Path |
//! |---------|--------|------|
//! | `custom-field list` | GET | `/custom-field-definitions` |
//! | `custom-field get` | GET | `/custom-field-definitions/{customFieldId}` (id or `name:<name>`) |
//! | `custom-field create` | POST | `/custom-field-definitions` |
//! | `custom-connection resource-attribute` | GET | `/workspaces/{w}/custom-connections/{c}/resources/{r}/attributes` |

pub mod connection;
pub mod field;

use crate::cli::{CustomConnectionAction, CustomFieldAction};
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::print_value;
use crate::spec;
use eyre::{Result, bail};
use serde_json::json;
use tracing::{debug, instrument};

pub fn example_if_requested(action: &CustomFieldAction) -> Option<Result<String>> {
    match action {
        CustomFieldAction::Create { example: true, .. } => Some(
            spec::example_for_operation(crate::backend::Backend::Drata, "POST", "/custom-field-definitions").and_then(
                |s| s.ok_or_else(|| eyre::eyre!("operation `POST /custom-field-definitions` has no JSON request body")),
            ),
        ),
        _ => None,
    }
}

#[instrument(skip_all)]
pub async fn handle_field(
    action: &CustomFieldAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        CustomFieldAction::List { entity_type, hidden } => {
            let path = field::list_path(entity_type.as_ref(), *hidden);
            debug!(path, "custom-field list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        CustomFieldAction::Get { custom_field_id } => {
            let path = field::get_path(custom_field_id);
            debug!(path, "custom-field get");
            print_value(&client.get(&path).await?, &config.output_format);
        }
        CustomFieldAction::Create {
            name,
            description,
            field_type,
            hidden,
            required,
            option,
            data,
            example: _,
        } => {
            let body = match data {
                Some(raw) => read_data(raw, DataSource::Any)?,
                None => field::build_create_body(
                    name.as_deref(),
                    description.as_deref(),
                    field_type.as_ref(),
                    *hidden,
                    *required,
                    option,
                )?,
            };
            debug!("custom-field create");
            if !confirm("POST", field::COLLECTION)? {
                bail!("aborted");
            }
            print_value(&client.post(field::COLLECTION, body).await?, &config.output_format);
        }
    }
    Ok(())
}

#[instrument(skip_all)]
pub async fn handle_connection(action: &CustomConnectionAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        CustomConnectionAction::ResourceAttribute {
            workspace_id,
            connection_id,
            resource_id,
        } => {
            let path = connection::attributes_path(workspace_id, connection_id, resource_id);
            debug!(path, "custom-connection resource-attribute");
            print_value(&client.get(&path).await?, &config.output_format);
            Ok(())
        }
    }
}
