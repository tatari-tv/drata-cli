//! `drata evidence-item ...`: Evidence Items, the spec's Evidence tag. Not the
//! Evidence Library (`drata evidence`, `src/resources/evidence.rs`).
//!
//! | Command | Method | Path |
//! |---------|--------|------|
//! | `list` | GET | `/workspaces/{w}/evidence` |
//! | `get` | GET | `/workspaces/{w}/evidence/{id}` |
//! | `create` | POST | `/workspaces/{w}/evidence` |
//! | `update` | PUT | `/workspaces/{w}/evidence/{id}` |
//! | `remove` | DELETE | `/workspaces/{w}/evidence/{id}` (the item and all its artifacts) |
//! | `upload` | POST | `/workspaces/{w}/evidence-files` (multipart `file`) |
//! | `artifact list` | GET | `/workspaces/{w}/evidence/{id}/artifacts` |
//! | `artifact update` | PUT | `/workspaces/{w}/evidence/{id}/artifacts/{a}` |
//! | `artifact remove` | DELETE | `/workspaces/{w}/evidence/{id}/artifacts/{a}` |
//! | `artifact run-action` | POST | `/workspaces/{w}/evidence/{id}/artifacts/{a}/actions` |
//!
//! File artifacts are two steps, as the spec has them: `upload` prints a
//! `fileKey`, and `create --file-key` / `update --file-key` reference it.

use crate::cli::{
    ArtifactActionKind, ArtifactRef, ArtifactType, EvidenceArtifactAction, EvidenceItemAction, EvidenceItemExpand,
    EvidenceItemFields, EvidenceItemListArgs, EvidenceItemRef, NewArtifactArgs, RenewalScheduleType,
};
use crate::client::{DrataClient, Multipart, encode_path_segment};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use crate::spec;
use eyre::{Result, bail, eyre};
use serde_json::{Map, Value, json};
use std::path::Path;
use tracing::{debug, instrument};

const ITEM_TEMPLATE: &str = "/workspaces/{workspaceId}/evidence/{evidenceId}";
const ARTIFACT_TEMPLATE: &str = "/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}";

/// `/workspaces/{w}/evidence`. Pure.
pub fn collection_path(workspace_id: &str) -> String {
    format!("/workspaces/{}/evidence", encode_path_segment(workspace_id))
}

/// `/workspaces/{w}/evidence/{id}`. Pure.
pub fn item_path(workspace_id: &str, evidence_id: u64) -> String {
    format!("{}/{}", collection_path(workspace_id), evidence_id)
}

/// `/workspaces/{w}/evidence-files`, the artifact file pre-upload. Pure.
pub fn files_path(workspace_id: &str) -> String {
    format!("/workspaces/{}/evidence-files", encode_path_segment(workspace_id))
}

/// `/workspaces/{w}/evidence/{id}/artifacts`. Pure.
pub fn artifacts_path(target: &EvidenceItemRef) -> String {
    format!("{}/artifacts", item_path(&target.workspace_id, target.evidence_id))
}

/// `/workspaces/{w}/evidence/{id}/artifacts/{a}`. Pure.
pub fn artifact_path(target: &ArtifactRef) -> String {
    format!(
        "{}/artifacts/{}",
        item_path(&target.workspace_id, target.evidence_id),
        target.artifact_id
    )
}

/// `.../artifacts/{a}/actions`. Pure.
pub fn artifact_actions_path(target: &ArtifactRef) -> String {
    format!("{}/actions", artifact_path(target))
}

/// `GET` the item list with every filter the op takes. Pure.
pub fn list_path(args: &EvidenceItemListArgs) -> String {
    let mut q = QueryPath::new(collection_path(&args.workspace_id));
    q.push_opt("name", args.name.as_deref())
        .push_each("ownerIds[]", &args.owner_ids)
        .push_each("frameworkIds[]", &args.framework_ids)
        .push_each("evidenceStatuses[]", args.status.iter().map(wire_name))
        .push_each("artifactTypes[]", args.artifact_type.iter().map(wire_name))
        .push_each("expand[]", args.expand.iter().map(wire_name));
    q.finish()
}

/// `GET` one item with `expand[]`. Pure.
pub fn get_path(target: &EvidenceItemRef, expand: &[EvidenceItemExpand]) -> String {
    let mut q = QueryPath::new(item_path(&target.workspace_id, target.evidence_id));
    q.push_each("expand[]", expand.iter().map(wire_name));
    q.finish()
}

/// `GET` an item's artifacts with the op's filters. Pure.
pub fn artifact_list_path(
    target: &EvidenceItemRef,
    name: Option<&str>,
    types: &[ArtifactType],
    archived: Option<bool>,
) -> String {
    let mut q = QueryPath::new(artifacts_path(target));
    q.push_opt("name", name)
        .push_each("artifactTypes[]", types.iter().map(wire_name))
        .push_opt("isArchived", archived);
    q.finish()
}

/// The new artifacts from `--url`, `--file-key` and `--ticket-url`, each filed
/// on `--filed-at`. The create body names the type field `type`, the update
/// body's `newArtifacts` name it `artifactType`; `type_key` picks. `--filed-at`
/// with no artifact is rejected: it would be silently dropped. Pure.
pub fn build_new_artifacts(args: &NewArtifactArgs, type_key: &str) -> Result<Vec<Value>> {
    let sources = [
        ("URL", "url", &args.urls),
        ("S3_FILE", "fileKey", &args.file_keys),
        ("TICKET_PROVIDER", "ticketUrl", &args.ticket_urls),
    ];
    let count: usize = sources.iter().map(|(_, _, values)| values.len()).sum();
    let filed_at = match (count, args.filed_at.as_deref()) {
        (0, None) => return Ok(Vec::new()),
        (0, Some(_)) => bail!("--filed-at applies only to new artifacts (--url, --file-key or --ticket-url)"),
        (_, None) => bail!("--filed-at is required with --url, --file-key or --ticket-url"),
        (_, Some(date)) => date,
    };
    let mut artifacts = Vec::with_capacity(count);
    for (artifact_type, field, values) in sources {
        for value in values {
            let mut artifact = Map::new();
            artifact.insert(type_key.to_string(), json!(artifact_type));
            artifact.insert(field.to_string(), json!(value));
            artifact.insert("filedAt".to_string(), json!(filed_at));
            artifacts.push(Value::Object(artifact));
        }
    }
    Ok(artifacts)
}

/// Copy the shared scalar fields into a body. Pure.
fn put_fields(body: &mut Map<String, Value>, fields: &EvidenceItemFields) {
    let text = [
        ("description", &fields.description),
        ("implementationGuidance", &fields.implementation_guidance),
        ("stepsToReproduce", &fields.steps_to_reproduce),
        ("renewalDate", &fields.renewal_date),
    ];
    for (key, value) in text {
        if let Some(v) = value {
            body.insert(key.to_string(), json!(v));
        }
    }
    if let Some(id) = fields.owner_id {
        body.insert("ownerId".to_string(), json!(id));
    }
    if !fields.control_ids.is_empty() {
        body.insert("controlIds".to_string(), json!(fields.control_ids));
    }
    if let Some(schedule) = &fields.renewal_schedule_type {
        body.insert("renewalScheduleType".to_string(), json!(wire_name(schedule)));
    }
}

/// The `create` body. `artifacts` is required by the spec, so it is always
/// sent, empty when no artifact flag is given. Pure.
pub fn build_create_body(name: &str, fields: &EvidenceItemFields, artifacts: &NewArtifactArgs) -> Result<Value> {
    let mut body = Map::new();
    body.insert("name".to_string(), json!(name));
    put_fields(&mut body, fields);
    body.insert(
        "artifacts".to_string(),
        Value::Array(build_new_artifacts(artifacts, "type")?),
    );
    Ok(Value::Object(body))
}

/// What `update` changes beyond the shared fields.
#[derive(Debug)]
pub struct UpdateChanges<'a> {
    pub name: Option<&'a str>,
    pub fields: &'a EvidenceItemFields,
    pub artifacts: &'a NewArtifactArgs,
    pub archive: &'a [u64],
    pub restore: &'a [u64],
}

/// The `update` body: a partial update, so only what a flag names is sent. An
/// update that changes nothing, an artifact both archived and restored, or a
/// non-`NONE` schedule without `--renewal-date` (the spec: "required if
/// renewalScheduleType is not NONE") is rejected before the confirm. Pure.
pub fn build_update_body(changes: &UpdateChanges) -> Result<Value> {
    if let Some(id) = changes.archive.iter().find(|id| changes.restore.contains(id)) {
        bail!("artifact {id} is in both --archive-artifact and --restore-artifact");
    }
    let schedule_needs_date = changes
        .fields
        .renewal_schedule_type
        .as_ref()
        .is_some_and(|s| !matches!(s, RenewalScheduleType::None));
    if schedule_needs_date && changes.fields.renewal_date.is_none() {
        bail!("--renewal-schedule-type other than NONE requires --renewal-date");
    }
    let mut body = Map::new();
    if let Some(name) = changes.name {
        body.insert("name".to_string(), json!(name));
    }
    put_fields(&mut body, changes.fields);
    let new_artifacts = build_new_artifacts(changes.artifacts, "artifactType")?;
    if !new_artifacts.is_empty() {
        body.insert("newArtifacts".to_string(), Value::Array(new_artifacts));
    }
    if !changes.archive.is_empty() {
        body.insert("pastArtifacts".to_string(), json!(changes.archive));
    }
    if !changes.restore.is_empty() {
        body.insert("currentArtifacts".to_string(), json!(changes.restore));
    }
    if body.is_empty() {
        bail!("`evidence-item update` needs at least one field to change (or --data)");
    }
    Ok(Value::Object(body))
}

/// The artifact `update` body: `artifactName` and `filedAt`, each set, cleared
/// (`null`, the spec's documented clear) or left out. Pure.
pub fn build_artifact_update_body(
    name: Option<&str>,
    clear_name: bool,
    filed_at: Option<&str>,
    clear_filed_at: bool,
) -> Result<Value> {
    let mut body = Map::new();
    match (name, clear_name) {
        (Some(n), _) => {
            body.insert("artifactName".to_string(), json!(n));
        }
        (None, true) => {
            body.insert("artifactName".to_string(), Value::Null);
        }
        (None, false) => {}
    }
    match (filed_at, clear_filed_at) {
        (Some(d), _) => {
            body.insert("filedAt".to_string(), json!(d));
        }
        (None, true) => {
            body.insert("filedAt".to_string(), Value::Null);
        }
        (None, false) => {}
    }
    if body.is_empty() {
        bail!("`evidence-item artifact update` needs --name, --filed-at, --clear-name or --clear-filed-at (or --data)");
    }
    Ok(Value::Object(body))
}

/// The artifact `run-action` body. Pure.
pub fn build_artifact_action_body(action: &ArtifactActionKind) -> Value {
    json!({ "action": wire_name(action) })
}

/// The `upload` multipart body: the spec's `file` part. Pure.
pub fn build_upload_form(file: &Path) -> Multipart {
    Multipart::single("file", file)
}

/// What the confirm prompt says for `remove`: the item goes with every artifact. Pure.
pub fn remove_prompt(path: &str) -> String {
    format!("{path} (permanently deletes the evidence item and all of its artifacts)")
}

/// What the confirm prompt says for `artifact remove`. Pure.
pub fn artifact_remove_prompt(path: &str) -> String {
    format!("{path} (permanently deletes the artifact and its version history; `run-action --action archive` keeps it)")
}

/// A create/update response reports `artifactsCreated` of `artifactsRequested`;
/// fewer created means some artifacts failed while the item was still saved.
/// That fails the command after the response prints. A body without both
/// counts passes: the response is printed whole either way. Pure.
pub fn check_artifacts_created(resp: &Value) -> Result<()> {
    let count = |key: &str| resp.get(key).and_then(Value::as_u64);
    if let (Some(requested), Some(created)) = (count("artifactsRequested"), count("artifactsCreated"))
        && created < requested
    {
        let id = resp.get("id").map(Value::to_string).unwrap_or_else(|| "?".to_string());
        bail!(
            "evidence item {id} was saved, but only {created} of {requested} artifacts were created; {} failed",
            requested - created
        );
    }
    Ok(())
}

/// The `fileKey` an upload response carries, the value `--file-key` takes.
pub fn uploaded_file_key(resp: &Value) -> Result<&str> {
    resp.get("fileKey")
        .and_then(Value::as_str)
        .ok_or_else(|| eyre!("POST evidence-files response has no `fileKey` string"))
}

fn example(method: &str, path: &str) -> Result<String> {
    spec::example_for_operation(method, path)?
        .ok_or_else(|| eyre!("operation `{method} {path}` has no JSON request body"))
}

pub fn example_if_requested(action: &EvidenceItemAction) -> Option<Result<String>> {
    match action {
        EvidenceItemAction::Create { example: true, .. } => Some(example("POST", "/workspaces/{workspaceId}/evidence")),
        EvidenceItemAction::Update { example: true, .. } => Some(example("PUT", ITEM_TEMPLATE)),
        EvidenceItemAction::Artifact {
            action: EvidenceArtifactAction::Update { example: true, .. },
        } => Some(example("PUT", ARTIFACT_TEMPLATE)),
        EvidenceItemAction::Artifact {
            action: EvidenceArtifactAction::RunAction { example: true, .. },
        } => Some(example("POST", &format!("{ARTIFACT_TEMPLATE}/actions"))),
        _ => None,
    }
}

/// `--data` when given, otherwise the body the flags build.
fn body_or_data(data: Option<&str>, build: impl FnOnce() -> Result<Value>) -> Result<Value> {
    match data {
        Some(raw) => read_data(raw, DataSource::Any),
        None => build(),
    }
}

fn require<'a, T>(value: Option<&'a T>, what: &str, verb: &str) -> Result<&'a T>
where
    T: ?Sized,
{
    value.ok_or_else(|| eyre!("`evidence-item {verb}` requires {what} (or use --example)"))
}

#[instrument(skip_all)]
pub async fn handle(
    action: &EvidenceItemAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        EvidenceItemAction::List(args) => {
            let path = list_path(args);
            debug!(path, "evidence-item list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        EvidenceItemAction::Get { target, expand } => {
            let path = get_path(target, expand);
            debug!(path, "evidence-item get");
            print_value(&client.get(&path).await?, &config.output_format);
        }
        EvidenceItemAction::Create {
            workspace_id,
            name,
            fields,
            artifacts,
            data,
            example: _,
        } => {
            let workspace_id = require(workspace_id.as_deref(), "<workspace_id>", "create")?;
            let path = collection_path(workspace_id);
            let body = body_or_data(data.as_deref(), || {
                let name = require(name.as_deref(), "--name", "create")?;
                build_create_body(name, fields, artifacts)
            })?;
            debug!(path, "evidence-item create");
            if !confirm("POST", &path)? {
                bail!("aborted");
            }
            let resp = client.post(&path, body).await?;
            print_value(&resp, &config.output_format);
            check_artifacts_created(&resp)?;
        }
        EvidenceItemAction::Update {
            target,
            name,
            fields,
            artifacts,
            archive_artifacts,
            restore_artifacts,
            data,
            example: _,
        } => {
            let workspace_id = require(target.workspace_id.as_deref(), "<workspace_id>", "update")?;
            let evidence_id = *require(target.evidence_id.as_ref(), "<evidence_id>", "update")?;
            let path = item_path(workspace_id, evidence_id);
            let body = body_or_data(data.as_deref(), || {
                build_update_body(&UpdateChanges {
                    name: name.as_deref(),
                    fields,
                    artifacts,
                    archive: archive_artifacts,
                    restore: restore_artifacts,
                })
            })?;
            debug!(path, "evidence-item update");
            if !confirm("PUT", &path)? {
                bail!("aborted");
            }
            let resp = client.put(&path, body).await?;
            print_value(&resp, &config.output_format);
            check_artifacts_created(&resp)?;
        }
        EvidenceItemAction::Remove(target) => {
            let path = item_path(&target.workspace_id, target.evidence_id);
            debug!(path, "evidence-item remove");
            if !confirm("DELETE", &remove_prompt(&path))? {
                bail!("aborted");
            }
            print_value(&client.delete(&path, None).await?, &config.output_format);
        }
        EvidenceItemAction::Upload { workspace_id, file } => {
            let path = files_path(workspace_id);
            if !std::fs::metadata(file).is_ok_and(|m| m.is_file()) {
                bail!("--file `{}` is not a readable file", file.display());
            }
            debug!(path, file = %file.display(), "evidence-item upload");
            if !confirm("POST", &path)? {
                bail!("aborted");
            }
            let resp = client.post_multipart(&path, &build_upload_form(file)).await?;
            let key = uploaded_file_key(&resp)?;
            debug!(
                mime = ?resp.get("mimeType"),
                size = ?resp.get("fileSize"),
                "evidence-item upload stored"
            );
            println!("{key}");
        }
        EvidenceItemAction::Artifact { action } => handle_artifact(action, client, config, confirm).await?,
    }
    Ok(())
}

/// Resolve the optional `--example` ids into an `ArtifactRef`.
fn artifact_ref(
    workspace_id: Option<&str>,
    evidence_id: Option<u64>,
    artifact_id: Option<u64>,
    verb: &str,
) -> Result<ArtifactRef> {
    match (workspace_id, evidence_id, artifact_id) {
        (Some(w), Some(e), Some(a)) => Ok(ArtifactRef {
            workspace_id: w.to_string(),
            evidence_id: e,
            artifact_id: a,
        }),
        _ => bail!(
            "`evidence-item artifact {verb}` requires <workspace_id> <evidence_id> <artifact_id> (or use --example)"
        ),
    }
}

async fn handle_artifact(
    action: &EvidenceArtifactAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        EvidenceArtifactAction::List {
            target,
            name,
            artifact_type,
            archived,
        } => {
            let path = artifact_list_path(target, name.as_deref(), artifact_type, *archived);
            debug!(path, "evidence-item artifact list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        EvidenceArtifactAction::Update {
            target,
            name,
            clear_name,
            filed_at,
            clear_filed_at,
            data,
            example: _,
        } => {
            let target = artifact_ref(
                target.workspace_id.as_deref(),
                target.evidence_id,
                target.artifact_id,
                "update",
            )?;
            let path = artifact_path(&target);
            let body = body_or_data(data.as_deref(), || {
                build_artifact_update_body(name.as_deref(), *clear_name, filed_at.as_deref(), *clear_filed_at)
            })?;
            debug!(path, "evidence-item artifact update");
            if !confirm("PUT", &path)? {
                bail!("aborted");
            }
            print_value(&client.put(&path, body).await?, &config.output_format);
        }
        EvidenceArtifactAction::Remove(target) => {
            let path = artifact_path(target);
            debug!(path, "evidence-item artifact remove");
            if !confirm("DELETE", &artifact_remove_prompt(&path))? {
                bail!("aborted");
            }
            print_value(&client.delete(&path, None).await?, &config.output_format);
        }
        EvidenceArtifactAction::RunAction {
            target,
            action,
            data,
            example: _,
        } => {
            let target = artifact_ref(
                target.workspace_id.as_deref(),
                target.evidence_id,
                target.artifact_id,
                "run-action",
            )?;
            let path = artifact_actions_path(&target);
            let body = body_or_data(data.as_deref(), || {
                let action = action.as_ref().ok_or_else(|| {
                    eyre!("`evidence-item artifact run-action` requires --action (or --data/--example)")
                })?;
                Ok(build_artifact_action_body(action))
            })?;
            debug!(path, "evidence-item artifact run-action");
            if !confirm("POST", &path)? {
                bail!("aborted");
            }
            print_value(&client.post(&path, body).await?, &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
