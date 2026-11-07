//! `drata policy version create | replace-file`.
//!
//! | Command | Method | Path | Body |
//! |---------|--------|------|------|
//! | `create` | POST | `/policies/{policyId}/policy-versions` | multipart (UPLOADED) or JSON (EXTERNAL) |
//! | `replace-file` | PUT | `/policies/{policyId}/policy-versions/{policyVersionId}/file` | multipart: `file`, optional `language` |
//!
//! Multipart field names are the spec's property names, verbatim.

use crate::cli::{PolicySourceType, PolicyVersionAction, PolicyVersionCreateArgs};
use crate::client::{DrataClient, Multipart, encode_path_segment};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::print_value;
use crate::spec;
use eyre::{Result, bail, eyre};
use serde_json::{Map, Value};
use std::path::Path;
use tracing::{debug, instrument};

/// The request body `create` sends: a file part forces multipart, anything else is JSON.
#[derive(Debug)]
pub enum Payload {
    Multipart(Multipart),
    Json(Value),
}

/// `POST /policies/{policyId}/policy-versions`. Pure.
pub fn create_path(policy_id: &str) -> String {
    format!("/policies/{}/policy-versions", encode_path_segment(policy_id))
}

/// `PUT /policies/{policyId}/policy-versions/{policyVersionId}/file`. Pure.
pub fn replace_file_path(policy_id: &str, version_id: &str) -> String {
    format!(
        "/policies/{}/policy-versions/{}/file",
        encode_path_segment(policy_id),
        encode_path_segment(version_id)
    )
}

fn source_type_value(source: &PolicySourceType) -> &'static str {
    match source {
        PolicySourceType::Uploaded => "UPLOADED",
        PolicySourceType::External => "EXTERNAL",
    }
}

/// The `create` body from flags. An UPLOADED version needs `--file` (multipart);
/// an EXTERNAL one needs `--external-file-id` and takes no file (JSON). A flag
/// that belongs to the other source type is rejected before the confirm and the
/// request. `--data` is the whole body, sent as JSON. Pure up to reading `--data`.
pub fn build_create(args: &PolicyVersionCreateArgs) -> Result<Payload> {
    if let Some(raw) = &args.data {
        return Ok(Payload::Json(read_data(raw, DataSource::Any)?));
    }
    let source = args
        .source_type
        .as_ref()
        .ok_or_else(|| eyre!("`policy version create` requires --source-type (or --data/--example)"))?;
    let owner_id = args
        .owner_id
        .ok_or_else(|| eyre!("`policy version create` requires --owner-id (or --data/--example)"))?;
    let material = args
        .material_change
        .ok_or_else(|| eyre!("`policy version create` requires --material-change true|false (or --data/--example)"))?;
    let renewal_date = args
        .renewal_date
        .as_deref()
        .ok_or_else(|| eyre!("`policy version create` requires --renewal-date (or --data/--example)"))?;
    match (source, &args.file, args.external_file_id) {
        (PolicySourceType::Uploaded, None, _) => bail!("--source-type UPLOADED requires --file"),
        (PolicySourceType::Uploaded, Some(_), Some(_)) => {
            bail!("--external-file-id is only valid with --source-type EXTERNAL")
        }
        (PolicySourceType::External, Some(_), _) => bail!("--file is only valid with --source-type UPLOADED"),
        (PolicySourceType::External, None, None) => bail!("--source-type EXTERNAL requires --external-file-id"),
        _ => {}
    }

    let mut fields: Vec<(&str, String)> = vec![
        ("sourceType", source_type_value(source).to_string()),
        ("ownerId", owner_id.to_string()),
        ("isMaterialChange", material.to_string()),
        ("renewalDate", renewal_date.to_string()),
    ];
    let optional = [
        ("requiresApproval", args.requires_approval.map(|b| b.to_string())),
        ("externalFileId", args.external_file_id.map(|n| n.to_string())),
        ("fileName", args.file_name.clone()),
        ("description", args.description.clone()),
        ("changeSummary", args.change_summary.clone()),
        ("changesExplanation", args.changes_explanation.clone()),
        (
            "requiresAcknowledgement",
            args.requires_acknowledgement.map(|b| b.to_string()),
        ),
    ];
    fields.extend(optional.into_iter().filter_map(|(k, v)| v.map(|v| (k, v))));

    if let Some(path) = &args.file {
        let mut form = Multipart::single("file", path);
        for (name, value) in fields {
            form.add_field(name, value);
        }
        return Ok(Payload::Multipart(form));
    }
    let mut body = Map::new();
    for (name, value) in fields {
        body.insert(name.to_string(), typed(name, value));
    }
    Ok(Payload::Json(Value::Object(body)))
}

/// A JSON body keeps numbers and booleans typed; the text form carries them as strings.
fn typed(name: &str, value: String) -> Value {
    match name {
        "ownerId" | "externalFileId" => value.parse::<u64>().map(Value::from).unwrap_or(Value::String(value)),
        "isMaterialChange" | "requiresApproval" | "requiresAcknowledgement" => {
            value.parse::<bool>().map(Value::from).unwrap_or(Value::String(value))
        }
        _ => Value::String(value),
    }
}

/// The `replace-file` multipart body: the spec's `file` part and optional `language`. Pure.
pub fn build_replace_file(file: &Path, language: Option<&str>) -> Multipart {
    let mut form = Multipart::single("file", file);
    form.add_opt_field("language", language);
    form
}

pub fn example_if_requested(action: &PolicyVersionAction) -> Option<Result<String>> {
    match action {
        PolicyVersionAction::Create(args) if args.example => Some(
            spec::example_for_operation("POST", "/policies/{policyId}/policy-versions")
                .and_then(|e| e.ok_or_else(|| eyre!("`POST /policies/{{policyId}}/policy-versions` has no JSON body"))),
        ),
        _ => None,
    }
}

#[instrument(skip_all)]
pub async fn handle(
    action: &PolicyVersionAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        PolicyVersionAction::Create(args) => {
            let policy_id = args
                .policy_id
                .as_deref()
                .ok_or_else(|| eyre!("`policy version create` requires <policy_id> (or --example)"))?;
            let path = create_path(policy_id);
            let payload = build_create(args)?;
            debug!(
                path,
                multipart = matches!(payload, Payload::Multipart(_)),
                "policy version create"
            );
            if !confirm("POST", &path)? {
                bail!("aborted");
            }
            let result = match payload {
                Payload::Multipart(form) => client.post_multipart(&path, &form).await?,
                Payload::Json(body) => client.post(&path, body).await?,
            };
            print_value(&result, &config.output_format);
        }
        PolicyVersionAction::ReplaceFile {
            policy_id,
            version_id,
            file,
            language,
        } => {
            let path = replace_file_path(policy_id, version_id);
            let form = build_replace_file(file, language.as_deref());
            debug!(path, has_language = language.is_some(), "policy version replace-file");
            if !confirm("PUT", &path)? {
                bail!("aborted");
            }
            print_value(&client.put_multipart(&path, &form).await?, &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
