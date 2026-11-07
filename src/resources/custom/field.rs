//! Custom field definition bodies and paths. Pure.

use crate::cli::{CustomFieldEntityType, CustomFieldType};
use crate::client::encode_path_segment;
use crate::query::{QueryPath, wire_name};
use eyre::{Result, bail};
use serde_json::{Value, json};

pub const COLLECTION: &str = "/custom-field-definitions";

/// The spec's `maxLength` on `name`.
const MAX_NAME: usize = 191;
/// The spec's `maxLength` on `description`.
const MAX_DESCRIPTION: usize = 30000;
/// The spec's `minItems` / `maxItems` on `options`.
const OPTIONS: std::ops::RangeInclusive<usize> = 2..=100;

/// `GET /custom-field-definitions` with the `entityType` and `isHidden` filters.
pub fn list_path(entity_type: Option<&CustomFieldEntityType>, hidden: Option<bool>) -> String {
    let mut q = QueryPath::new(COLLECTION);
    q.push_opt("entityType", entity_type.map(wire_name))
        .push_opt("isHidden", hidden);
    q.finish()
}

/// `GET /custom-field-definitions/{customFieldId}`, the id one encoded
/// segment (`name:Regulatory Owner` holds a space).
pub fn get_path(custom_field_id: &str) -> String {
    format!("{COLLECTION}/{}", encode_path_segment(custom_field_id))
}

/// `POST /custom-field-definitions` body from flags. `isHidden` and
/// `isRequired` are required by the spec, so they are always sent. `--option`
/// builds the OPTIONS list in the order given (positions from 1, none hidden);
/// the spec requires 2 to 100 options for an OPTIONS field and defines none for
/// any other type.
pub fn build_create_body(
    name: Option<&str>,
    description: Option<&str>,
    field_type: Option<&CustomFieldType>,
    hidden: bool,
    required: bool,
    options: &[String],
) -> Result<Value> {
    let name = name.ok_or_else(|| eyre::eyre!("requires --name (or use --data/--example)"))?;
    let field_type = field_type.ok_or_else(|| eyre::eyre!("requires --field-type (or use --data/--example)"))?;
    if name.chars().count() > MAX_NAME {
        bail!("--name takes at most {MAX_NAME} characters");
    }
    if description.is_some_and(|d| d.chars().count() > MAX_DESCRIPTION) {
        bail!("--description takes at most {MAX_DESCRIPTION} characters");
    }
    let mut body = json!({
        "name": name,
        "fieldType": wire_name(field_type),
        "isHidden": hidden,
        "isRequired": required,
    });
    if let Some(d) = description {
        body["description"] = json!(d);
    }
    match (field_type, options.is_empty()) {
        (CustomFieldType::Options, false) => {
            if !OPTIONS.contains(&options.len()) {
                bail!(
                    "--option takes {} to {} values for an OPTIONS field, got {}",
                    OPTIONS.start(),
                    OPTIONS.end(),
                    options.len()
                );
            }
            body["options"] = options
                .iter()
                .enumerate()
                .map(|(i, value)| json!({ "value": value, "position": i + 1, "isHidden": false }))
                .collect();
        }
        (CustomFieldType::Options, true) => {
            bail!(
                "an OPTIONS field requires --option with {} to {} values",
                OPTIONS.start(),
                OPTIONS.end()
            )
        }
        (_, false) => bail!("--option is only for an OPTIONS field (numeric options and currency go through --data)"),
        (_, true) => {}
    }
    Ok(body)
}

#[cfg(test)]
mod tests;
