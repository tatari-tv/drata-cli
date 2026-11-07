//! `drata audit request dcf-suggestion ...`: the spec's beta DCF suggestion ops
//! under Audit Requests. `generate` answers 202 with one row per request and
//! `accept` answers one row per processed request; both go through
//! `resources::bulk`, so every row prints and a failed row exits 1. The 202 is
//! not polled: status is read with `audit get --expand dcfSuggestionsSummary`.
//!
//! | Command | Method | Path |
//! |---------|--------|------|
//! | `generate` | POST | `/workspaces/{w}/audits/{a}/requests/action-generate-dcf-suggestions` |
//! | `accept` | POST | `/workspaces/{w}/audits/{a}/requests/action-accept-dcf-suggestions` |
//! | `list` | GET | `/workspaces/{w}/audits/{a}/requests/{r}/dcf-suggestions` |
//! | `get` | GET | `/workspaces/{w}/audits/{a}/requests/{r}/dcf-suggestions/{s}` |
//! | `run-action` | POST | `/workspaces/{w}/audits/{a}/requests/{r}/dcf-suggestions/{s}/actions` |

use crate::cli::{
    AuditRequestRef, DcfConfidenceLevel, DcfSortDirection, DcfSuggestionAction, DcfSuggestionDecision,
    DcfSuggestionRef, DcfSuggestionSort, DcfSuggestionStatus, ExampleAuditRef, ExampleDcfSuggestionRef,
};
use crate::client::{DrataClient, encode_path_segment};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use crate::resources::bulk;
use crate::spec;
use eyre::{Result, bail, eyre};
use serde_json::{Map, Value, json};
use std::collections::HashSet;
use tracing::{debug, instrument};

const REQUESTS_TEMPLATE: &str = "/workspaces/{workspaceId}/audits/{auditId}/requests";
const SUGGESTION_TEMPLATE: &str =
    "/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}/dcf-suggestions/{suggestionId}";

/// The most requests one `generate` call takes (spec `maxItems`).
const GENERATE_MAX: usize = 100;
/// The most requests one `accept` call takes (spec `maxItems`).
const ACCEPT_MAX: usize = 500;

fn requests_path(workspace_id: &str, audit_id: &str) -> String {
    format!(
        "/workspaces/{}/audits/{}/requests",
        encode_path_segment(workspace_id),
        encode_path_segment(audit_id)
    )
}

/// `POST .../requests/action-generate-dcf-suggestions`. Pure.
pub fn generate_path(workspace_id: &str, audit_id: &str) -> String {
    format!(
        "{}/action-generate-dcf-suggestions",
        requests_path(workspace_id, audit_id)
    )
}

/// `POST .../requests/action-accept-dcf-suggestions`. Pure.
pub fn accept_path(workspace_id: &str, audit_id: &str) -> String {
    format!(
        "{}/action-accept-dcf-suggestions",
        requests_path(workspace_id, audit_id)
    )
}

/// `.../requests/{r}/dcf-suggestions`, with the list filters as query pairs. Pure.
pub fn list_path(
    target: &AuditRequestRef,
    status: &[DcfSuggestionStatus],
    confidence: &[DcfConfidenceLevel],
    sort: Option<&DcfSuggestionSort>,
    sort_dir: Option<&DcfSortDirection>,
) -> String {
    let mut q = QueryPath::new(format!(
        "{}/{}/dcf-suggestions",
        requests_path(&target.workspace_id, &target.audit_id),
        target.request_id
    ));
    q.push_each("status[]", status.iter().map(wire_name));
    q.push_each("confidenceLevel[]", confidence.iter().map(wire_name));
    q.push_opt("sort", sort.map(wire_name));
    q.push_opt("sortDir", sort_dir.map(wire_name));
    q.finish()
}

/// `.../requests/{r}/dcf-suggestions/{s}`. Pure.
pub fn item_path(target: &DcfSuggestionRef) -> String {
    format!(
        "{}/{}/dcf-suggestions/{}",
        requests_path(&target.workspace_id, &target.audit_id),
        target.request_id,
        target.suggestion_id
    )
}

/// `.../requests/{r}/dcf-suggestions/{s}/actions`. Pure.
pub fn actions_path(target: &DcfSuggestionRef) -> String {
    format!("{}/actions", item_path(target))
}

fn request_ids_checked(ids: &[u64], max: usize, verb: &str) -> Result<()> {
    if ids.is_empty() {
        bail!("`audit request dcf-suggestion {verb}` needs at least one --request-id");
    }
    if ids.len() > max {
        bail!(
            "{} request ids exceed the {max} one `{verb}` call takes; send them in batches",
            ids.len()
        );
    }
    let mut seen = HashSet::new();
    if let Some(dup) = ids.iter().find(|id| !seen.insert(**id)) {
        bail!("request id {dup} is given twice in --request-id");
    }
    Ok(())
}

/// The `generate` body: one entry per request id, 1 to 100, no repeats. Pure.
pub fn build_generate_body(request_ids: &[u64]) -> Result<Value> {
    request_ids_checked(request_ids, GENERATE_MAX, "generate")?;
    Ok(json!({ "requestIds": request_ids }))
}

/// The `accept` body: `requestIds` or `allRequests`, never both and never
/// neither, plus `confidenceLevels` only when given (leaving it out accepts
/// every level). Pure.
pub fn build_accept_body(
    request_ids: &[u64],
    all_requests: bool,
    confidence_levels: &[DcfConfidenceLevel],
) -> Result<Value> {
    let mut body = Map::new();
    match (request_ids.is_empty(), all_requests) {
        (false, true) => bail!("--request-id and --all-requests cannot be sent together"),
        (true, false) => {
            bail!("`audit request dcf-suggestion accept` needs --request-id or --all-requests (or --data)")
        }
        (false, false) => {
            request_ids_checked(request_ids, ACCEPT_MAX, "accept")?;
            body.insert("requestIds".to_string(), json!(request_ids));
        }
        (true, true) => {
            body.insert("allRequests".to_string(), json!(true));
        }
    }
    if !confidence_levels.is_empty() {
        let mut seen = HashSet::new();
        let levels: Vec<String> = confidence_levels
            .iter()
            .map(wire_name)
            .filter(|l| seen.insert(l.clone()))
            .collect();
        body.insert("confidenceLevels".to_string(), json!(levels));
    }
    Ok(Value::Object(body))
}

/// The `run-action` body. Pure.
pub fn build_action_body(decision: &DcfSuggestionDecision) -> Value {
    json!({ "action": wire_name(decision) })
}

/// What the confirm prompt says for `accept`: `allRequests` maps controls onto
/// every request in the audit with a matching pending suggestion, so the prompt
/// says so. Read from the body about to be sent, so `--data` gets it too. Pure.
pub fn accept_prompt(path: &str, body: &Value) -> String {
    if body.get("allRequests") == Some(&Value::Bool(true)) {
        format!("{path} (accepts pending suggestions on EVERY matching request in the audit, up to 500)")
    } else {
        path.to_string()
    }
}

fn example(method: &str, path: &str) -> Result<String> {
    spec::example_for_operation(crate::backend::Backend::Drata, method, path)?
        .ok_or_else(|| eyre!("operation `{method} {path}` has no JSON request body"))
}

pub fn example_if_requested(action: &DcfSuggestionAction) -> Option<Result<String>> {
    let requests = REQUESTS_TEMPLATE;
    match action {
        DcfSuggestionAction::Generate { example: true, .. } => {
            Some(example("POST", &format!("{requests}/action-generate-dcf-suggestions")))
        }
        DcfSuggestionAction::Accept { example: true, .. } => {
            Some(example("POST", &format!("{requests}/action-accept-dcf-suggestions")))
        }
        DcfSuggestionAction::RunAction { example: true, .. } => {
            Some(example("POST", &format!("{SUGGESTION_TEMPLATE}/actions")))
        }
        _ => None,
    }
}

fn require<'a>(value: &'a Option<String>, what: &str, verb: &str) -> Result<&'a str> {
    value
        .as_deref()
        .ok_or_else(|| eyre!("`audit request dcf-suggestion {verb}` requires {what} (or use --example)"))
}

fn audit_ids<'a>(audit: &'a ExampleAuditRef, verb: &str) -> Result<(&'a str, &'a str)> {
    Ok((
        require(&audit.workspace_id, "<workspace_id>", verb)?,
        require(&audit.audit_id, "<audit_id>", verb)?,
    ))
}

fn suggestion_ref(target: &ExampleDcfSuggestionRef) -> Result<DcfSuggestionRef> {
    let verb = "run-action";
    let number = |value: Option<u64>, what: &str| {
        value.ok_or_else(|| eyre!("`audit request dcf-suggestion {verb}` requires {what} (or use --example)"))
    };
    Ok(DcfSuggestionRef {
        workspace_id: require(&target.workspace_id, "<workspace_id>", verb)?.to_string(),
        audit_id: require(&target.audit_id, "<audit_id>", verb)?.to_string(),
        request_id: number(target.request_id, "<request_id>")?,
        suggestion_id: number(target.suggestion_id, "<suggestion_id>")?,
    })
}

/// `--data` when given, otherwise the body the flags build.
fn body_or_data(data: Option<&str>, build: impl FnOnce() -> Result<Value>) -> Result<Value> {
    match data {
        Some(raw) => read_data(raw, DataSource::Any),
        None => build(),
    }
}

#[instrument(skip_all)]
pub async fn handle(
    action: &DcfSuggestionAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        DcfSuggestionAction::Generate {
            audit,
            request_ids,
            data,
            example: _,
        } => {
            let (workspace_id, audit_id) = audit_ids(audit, "generate")?;
            let path = generate_path(workspace_id, audit_id);
            let body = body_or_data(data.as_deref(), || build_generate_body(request_ids))?;
            debug!(path, "audit request dcf-suggestion generate");
            if !confirm("POST", &path)? {
                bail!("aborted");
            }
            bulk::post(client, config, &path, body, bulk::GENERATE_STATUS).await?;
        }
        DcfSuggestionAction::Accept {
            audit,
            request_ids,
            all_requests,
            confidence_level,
            data,
            example: _,
        } => {
            let (workspace_id, audit_id) = audit_ids(audit, "accept")?;
            let path = accept_path(workspace_id, audit_id);
            let body = body_or_data(data.as_deref(), || {
                build_accept_body(request_ids, *all_requests, confidence_level)
            })?;
            debug!(path, "audit request dcf-suggestion accept");
            if !confirm("POST", &accept_prompt(&path, &body))? {
                bail!("aborted");
            }
            bulk::post(client, config, &path, body, bulk::ACCEPT_ERROR).await?;
        }
        DcfSuggestionAction::List {
            request,
            status,
            confidence_level,
            sort,
            sort_dir,
        } => {
            let path = list_path(request, status, confidence_level, sort.as_ref(), sort_dir.as_ref());
            debug!(path, "audit request dcf-suggestion list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        DcfSuggestionAction::Get(target) => {
            let path = item_path(target);
            debug!(path, "audit request dcf-suggestion get");
            print_value(&client.get(&path).await?, &config.output_format);
        }
        DcfSuggestionAction::RunAction {
            target,
            action,
            data,
            example: _,
        } => {
            let target = suggestion_ref(target)?;
            let path = actions_path(&target);
            let body = body_or_data(data.as_deref(), || {
                let decision = action.as_ref().ok_or_else(|| {
                    eyre!("`audit request dcf-suggestion run-action` requires --action (or --data/--example)")
                })?;
                Ok(build_action_body(decision))
            })?;
            debug!(path, "audit request dcf-suggestion run-action");
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
