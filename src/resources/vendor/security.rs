//! `drata vendor security-review ...` - security review command group for a vendor.
//!
//! Implements all 10 security-review operations from the Drata spec under
//! `drata vendor security-review <verb>`. All paths are nested under
//! `/vendors/{vendorId}`.
//!
//! ## Verb map
//!
//! | Verb | Method | Path |
//! |------|--------|------|
//! | `list` | GET | `/security-reviews` |
//! | `create` | POST | `/security-reviews` |
//! | `create-with-file` | POST | `/security-reviews/with-file` |
//! | `get` | GET | `/security-reviews/{srId}` |
//! | `update` | PUT | `/security-reviews/{srId}` |
//! | `actions` | GET | `/security-reviews/{srId}/actions` |
//! | `run-action` | POST | `/security-reviews/{srId}/actions` |
//! | `questionnaires` | GET | `/security-reviews/{srId}/security-questionnaires` |
//! | `upload-questionnaire` | POST | `/security-questionnaires` |
//! | `upload-questionnaire-to-review` | POST | `/security-reviews/{srId}/security-questionnaires` |
//!
//! ## Enum serialization (confirmed against spec)
//!
//! - `SecurityReviewStatus` -> body key `securityReviewStatus`:
//!   `NOT_YET_STARTED | IN_PROGRESS | COMPLETED | NOT_REQUIRED`
//! - `SecurityReviewType` -> body key `securityReviewType`:
//!   `SECURITY | SOC_REPORT | UPLOAD_REPORT`
//! - `SecurityReviewAction` -> body key `action`:
//!   `finalize | reopen`
//!
//! ## Content-type notes
//!
//! The `upload-questionnaire` and `upload-questionnaire-to-review` endpoints are
//! confirmed `multipart/form-data` with a `files` array (Q1a). `create-with-file`
//! is sent as multipart with a single `file` part; if the endpoint actually
//! requires `application/json`, it returns `415` and an actionable error message
//! points the user at the `raw` JSON fallback (Q1b).

use crate::cli::{
    SecurityReviewAction, SecurityReviewDecision, SecurityReviewDocumentType, SecurityReviewStatus, SecurityReviewType,
    VendorSecurityReviewAction,
};
use crate::client::{ApiError, DrataClient, Multipart};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::expand::append_expand;
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use eyre::{Result, bail};
use reqwest::StatusCode;
use serde_json::{Value, json};
use std::io;
use std::path::Path;
use tracing::debug;

// ---------------------------------------------------------------------------
// Serialization helpers
// ---------------------------------------------------------------------------

/// Render a `SecurityReviewStatus` to its spec string value.
pub(crate) fn status_str(s: &SecurityReviewStatus) -> &'static str {
    match s {
        SecurityReviewStatus::NotYetStarted => "NOT_YET_STARTED",
        SecurityReviewStatus::InProgress => "IN_PROGRESS",
        SecurityReviewStatus::Completed => "COMPLETED",
        SecurityReviewStatus::NotRequired => "NOT_REQUIRED",
    }
}

/// Render a `SecurityReviewType` to its spec string value.
pub(crate) fn type_str(t: &SecurityReviewType) -> &'static str {
    match t {
        SecurityReviewType::Security => "SECURITY",
        SecurityReviewType::SocReport => "SOC_REPORT",
        SecurityReviewType::UploadReport => "UPLOAD_REPORT",
    }
}

/// Render a `SecurityReviewAction` to its spec string value.
pub(crate) fn action_str(a: &SecurityReviewAction) -> &'static str {
    match a {
        SecurityReviewAction::Finalize => "finalize",
        SecurityReviewAction::Reopen => "reopen",
    }
}

/// Render a `SecurityReviewDecision` to its spec string value.
pub(crate) fn decision_str(d: &SecurityReviewDecision) -> &'static str {
    match d {
        SecurityReviewDecision::Pending => "PENDING",
        SecurityReviewDecision::Approved => "APPROVED",
        SecurityReviewDecision::ApprovedWithConditions => "APPROVED_WITH_CONDITIONS",
        SecurityReviewDecision::Rejected => "REJECTED",
    }
}

// ---------------------------------------------------------------------------
// Example skeletons
// ---------------------------------------------------------------------------

/// JSON skeleton printed by `vendor security-review create --example`.
const SECURITY_REVIEW_CREATE_EXAMPLE: &str = r#"{
  "securityReviewStatus": "NOT_YET_STARTED",
  "securityReviewType": "SECURITY",
  "reviewDeadlineAt": "2026-12-31T00:00:00.000Z",
  "title": "Annual vendor security review",
  "note": "Free-form notes about this review",
  "requestedAt": "2026-06-28T00:00:00.000Z",
  "requesterUserId": 0
}
"#;

/// JSON skeleton printed by `vendor security-review update --example`.
/// `socForm` is a nested object (`SocReviewFormSaveRequestPublicV2Dto`); only a
/// representative subset of its sections is shown. Pass it via `--data` or
/// `--soc-form '<json object>'`.
/// `updatedAt` overrides the review's last-updated timestamp; it is the only
/// field the spec accepts on a completed review.
const SECURITY_REVIEW_UPDATE_EXAMPLE: &str = r#"{
  "title": "Updated review title",
  "socForm": {
    "reviewerInformation": {},
    "complianceScope": {},
    "reportOpinion": {}
  },
  "updatedAt": "2025-07-01T16:45:55.246Z"
}
"#;

/// Returns the example skeleton if the action is a `--example` request.
/// Called from `vendor::example_if_requested` before config/auth load.
pub fn example_if_requested(action: &VendorSecurityReviewAction) -> Option<&'static str> {
    match action {
        VendorSecurityReviewAction::Create { example: true, .. } => Some(SECURITY_REVIEW_CREATE_EXAMPLE),
        VendorSecurityReviewAction::Update { example: true, .. } => Some(SECURITY_REVIEW_UPDATE_EXAMPLE),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

/// Dispatch a `vendor security-review <verb>` action.
pub async fn handle(
    action: &VendorSecurityReviewAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        VendorSecurityReviewAction::List {
            vendor_id,
            status,
            review_type,
            decision,
            created_from,
            created_to,
            updated_from,
            updated_to,
            expand,
            all,
        } => {
            let path = build_list_path(
                vendor_id,
                status,
                review_type,
                decision,
                created_from.as_deref(),
                created_to.as_deref(),
                updated_from.as_deref(),
                updated_to.as_deref(),
                expand,
            );
            list(client, config, &path, *all).await
        }
        VendorSecurityReviewAction::Create {
            vendor_id,
            review_deadline_at,
            status,
            review_type,
            title,
            note,
            requested_at,
            requester_user_id,
            data,
            example: _,
        } => {
            create(
                client,
                config,
                confirm,
                vendor_id,
                review_deadline_at.as_deref(),
                status.as_ref(),
                review_type.as_ref(),
                title.as_deref(),
                note.as_deref(),
                requested_at.as_deref(),
                *requester_user_id,
                data.as_deref(),
            )
            .await
        }
        VendorSecurityReviewAction::Get {
            vendor_id,
            security_review_id,
            expand,
        } => get(client, config, vendor_id, *security_review_id, expand).await,
        VendorSecurityReviewAction::Update {
            vendor_id,
            security_review_id,
            title,
            soc_form,
            updated_at,
            data,
            example: _,
        } => {
            update(
                client,
                config,
                confirm,
                vendor_id,
                *security_review_id,
                title.as_deref(),
                soc_form.as_deref(),
                updated_at.as_deref(),
                data.as_deref(),
            )
            .await
        }
        VendorSecurityReviewAction::Actions {
            vendor_id,
            security_review_id,
        } => actions(client, config, vendor_id, *security_review_id).await,
        VendorSecurityReviewAction::RunAction {
            vendor_id,
            security_review_id,
            action,
            decision,
            note,
            reset_decision,
        } => {
            let body = build_run_action_body(action, decision.as_ref(), note.as_deref(), *reset_decision)?;
            run_action(client, config, confirm, vendor_id, *security_review_id, body).await
        }
        VendorSecurityReviewAction::Questionnaires {
            vendor_id,
            security_review_id,
        } => questionnaires(client, config, vendor_id, *security_review_id).await,
        VendorSecurityReviewAction::CreateWithFile {
            vendor_id,
            file,
            title,
            review_deadline_at,
            status,
            review_type,
            document_type,
            note,
            requested_at,
            requester_user_id,
        } => {
            create_with_file(
                client,
                config,
                confirm,
                vendor_id,
                file,
                title,
                review_deadline_at,
                status,
                review_type,
                document_type,
                note.as_deref(),
                requested_at.as_deref(),
                *requester_user_id,
            )
            .await
        }
        VendorSecurityReviewAction::UploadQuestionnaire { vendor_id, file } => {
            upload_questionnaire(client, config, confirm, vendor_id, file).await
        }
        VendorSecurityReviewAction::UploadQuestionnaireToReview {
            vendor_id,
            security_review_id,
            file,
        } => upload_questionnaire_to_review(client, config, confirm, vendor_id, *security_review_id, file).await,
    }
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn list(client: &DrataClient, config: &Config, path: &str, all: bool) -> Result<()> {
    debug!(path, all, "security_review list");

    if all {
        let mut stdout = io::stdout();
        let total = client.stream_all(path, &mut stdout).await?;
        debug!(total, "security_review list stream complete");
    } else {
        let items = client.get_all(path).await?;
        let count = items.len();
        let result = json!({ "data": items });
        print_value(&result, &config.output_format);
        debug!(count, "security_review list complete");
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
async fn create(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    vendor_id: &str,
    review_deadline_at: Option<&str>,
    status: Option<&SecurityReviewStatus>,
    review_type: Option<&SecurityReviewType>,
    title: Option<&str>,
    note: Option<&str>,
    requested_at: Option<&str>,
    requester_user_id: Option<u64>,
    data: Option<&str>,
) -> Result<()> {
    debug!(
        vendor_id,
        has_review_deadline_at = review_deadline_at.is_some(),
        status = status.map(status_str),
        review_type = review_type.map(type_str),
        has_title = title.is_some(),
        has_note = note.is_some(),
        has_requested_at = requested_at.is_some(),
        has_requester_user_id = requester_user_id.is_some(),
        has_data = data.is_some(),
        "security_review create"
    );

    let path = format!("/vendors/{}/security-reviews", vendor_id);

    if !confirm("POST", &path)? {
        bail!("aborted");
    }

    let body: Value = if let Some(raw) = data {
        read_data(raw, DataSource::Any)?
    } else {
        // The trio is required unless --data (or --example) supplies the body.
        let review_deadline_at = review_deadline_at.ok_or_else(|| {
            eyre::eyre!("`vendor security-review create` requires --review-deadline-at (or use --data/--example)")
        })?;
        let status = status.ok_or_else(|| {
            eyre::eyre!("`vendor security-review create` requires --status (or use --data/--example)")
        })?;
        let review_type = review_type
            .ok_or_else(|| eyre::eyre!("`vendor security-review create` requires --type (or use --data/--example)"))?;
        // Required fields with translated body keys (securityReviewStatus / securityReviewType).
        let mut b = json!({
            "securityReviewStatus": status_str(status),
            "securityReviewType": type_str(review_type),
            "reviewDeadlineAt": review_deadline_at,
        });
        set_opt_str(&mut b, "title", title);
        set_opt_str(&mut b, "note", note);
        set_opt_str(&mut b, "requestedAt", requested_at);
        if let Some(id) = requester_user_id {
            b["requesterUserId"] = json!(id);
        }
        b
    };

    let result = client.post(&path, body).await?;
    print_value(&result, &config.output_format);
    debug!("security_review create complete");
    Ok(())
}

async fn get(
    client: &DrataClient,
    config: &Config,
    vendor_id: &str,
    security_review_id: u64,
    expand: &[String],
) -> Result<()> {
    debug!(
        vendor_id,
        security_review_id,
        expand_len = expand.len(),
        "security_review get"
    );
    let base = format!("/vendors/{}/security-reviews/{}", vendor_id, security_review_id);
    let path = append_expand(&base, expand);
    let resp = client.get(&path).await?;
    print_value(&resp, &config.output_format);
    debug!(vendor_id, security_review_id, "security_review get complete");
    Ok(())
}

async fn update(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    vendor_id: &str,
    security_review_id: u64,
    title: Option<&str>,
    soc_form: Option<&str>,
    updated_at: Option<&str>,
    data: Option<&str>,
) -> Result<()> {
    debug!(
        vendor_id,
        security_review_id,
        has_title = title.is_some(),
        has_soc_form = soc_form.is_some(),
        has_updated_at = updated_at.is_some(),
        has_data = data.is_some(),
        "security_review update"
    );

    let path = format!("/vendors/{}/security-reviews/{}", vendor_id, security_review_id);

    if !confirm("PUT", &path)? {
        bail!("aborted");
    }

    let body: Value = if let Some(raw) = data {
        read_data(raw, DataSource::Any)?
    } else {
        build_update_body(title, soc_form, updated_at)?
    };

    let result = client.put(&path, body).await?;
    print_value(&result, &config.output_format);
    debug!(vendor_id, security_review_id, "security_review update complete");
    Ok(())
}

async fn actions(client: &DrataClient, config: &Config, vendor_id: &str, security_review_id: u64) -> Result<()> {
    debug!(vendor_id, security_review_id, "security_review actions");
    let path = format!("/vendors/{}/security-reviews/{}/actions", vendor_id, security_review_id);
    let resp = client.get(&path).await?;
    print_value(&resp, &config.output_format);
    debug!(vendor_id, security_review_id, "security_review actions complete");
    Ok(())
}

/// Build the `UpdateDTO` body: only `title`, `socForm` and `updatedAt`, never
/// create's fields. `socForm` is a nested object
/// (`SocReviewFormSaveRequestPublicV2Dto`), so `--soc-form` is parsed as JSON
/// rather than sent as a bare string. Pure.
fn build_update_body(title: Option<&str>, soc_form: Option<&str>, updated_at: Option<&str>) -> Result<Value> {
    let mut b = json!({});
    set_opt_str(&mut b, "title", title);
    if let Some(sf) = soc_form {
        let parsed: Value =
            serde_json::from_str(sf).map_err(|e| eyre::eyre!("--soc-form must be a JSON object: {}", e))?;
        b["socForm"] = parsed;
    }
    set_opt_str(&mut b, "updatedAt", updated_at);
    Ok(b)
}

/// Build the `run-action` body. The spec accepts `decision` and `note` only
/// with `finalize` and `resetDecision` only with `reopen`; a flag paired with
/// the other action is rejected here, before any confirm or request. Pure.
fn build_run_action_body(
    action: &SecurityReviewAction,
    decision: Option<&SecurityReviewDecision>,
    note: Option<&str>,
    reset_decision: bool,
) -> Result<Value> {
    let mut body = json!({ "action": action_str(action) });
    match action {
        SecurityReviewAction::Finalize => {
            if reset_decision {
                bail!("--reset-decision is only valid with --action reopen");
            }
            if let Some(d) = decision {
                body["decision"] = json!(decision_str(d));
            }
            set_opt_str(&mut body, "note", note);
        }
        SecurityReviewAction::Reopen => {
            if decision.is_some() || note.is_some() {
                bail!("--decision and --note are only valid with --action finalize");
            }
            if reset_decision {
                body["resetDecision"] = json!(true);
            }
        }
    }
    Ok(body)
}

async fn run_action(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    vendor_id: &str,
    security_review_id: u64,
    body: Value,
) -> Result<()> {
    debug!(vendor_id, security_review_id, action = %body["action"], "security_review run-action");

    let path = format!("/vendors/{}/security-reviews/{}/actions", vendor_id, security_review_id);

    if !confirm("POST", &path)? {
        bail!("aborted");
    }

    let result = client.post(&path, body).await?;
    print_value(&result, &config.output_format);
    debug!(vendor_id, security_review_id, "security_review run-action complete");
    Ok(())
}

async fn questionnaires(client: &DrataClient, config: &Config, vendor_id: &str, security_review_id: u64) -> Result<()> {
    debug!(vendor_id, security_review_id, "security_review questionnaires");
    let path = format!(
        "/vendors/{}/security-reviews/{}/security-questionnaires",
        vendor_id, security_review_id
    );
    let items = client.get_all(&path).await?;
    let count = items.len();
    let result = json!({ "data": items });
    print_value(&result, &config.output_format);
    debug!(
        vendor_id,
        security_review_id, count, "security_review questionnaires complete"
    );
    Ok(())
}

/// Build the `list` request path with bracketed array filter params, the
/// created/updated date bounds, and expand. Filters are `status[]` / `type[]` /
/// `decision[]` per the spec; the brackets are percent-encoded in the key like
/// `expand[]` (`src/expand.rs`). Pure - no I/O - so the query encoding can be
/// unit-tested.
#[allow(clippy::too_many_arguments)]
fn build_list_path(
    vendor_id: &str,
    status: &[SecurityReviewStatus],
    review_type: &[SecurityReviewType],
    decision: &[SecurityReviewDecision],
    created_from: Option<&str>,
    created_to: Option<&str>,
    updated_from: Option<&str>,
    updated_to: Option<&str>,
    expand: &[String],
) -> String {
    let mut q = QueryPath::new(format!("/vendors/{}/security-reviews", vendor_id));
    q.push_each("status[]", status.iter().map(status_str))
        .push_each("type[]", review_type.iter().map(type_str))
        .push_each("decision[]", decision.iter().map(decision_str))
        .push_opt("createdAtFrom", created_from)
        .push_opt("createdAtTo", created_to)
        .push_opt("updatedAtFrom", updated_from)
        .push_opt("updatedAtTo", updated_to);
    append_expand(&q.finish(), expand)
}

// ---------------------------------------------------------------------------
// Multipart handlers (Phase 3)
// ---------------------------------------------------------------------------

/// Build the multipart body for `create-with-file`: a single `file` part plus
/// the create scalar fields (translated body keys, sparse optionals). Pure -
/// no I/O, so the part shape can be unit-tested.
#[allow(clippy::too_many_arguments)]
fn build_create_with_file_form(
    file: &Path,
    title: &str,
    review_deadline_at: &str,
    status: &SecurityReviewStatus,
    review_type: &SecurityReviewType,
    document_type: &SecurityReviewDocumentType,
    note: Option<&str>,
    requested_at: Option<&str>,
    requester_user_id: Option<u64>,
) -> Multipart {
    // Single `file` part (Q1b: treated as multipart per design; 415 path below).
    let mut form = Multipart::single("file", file);
    form.add_field("title", title)
        .add_field("reviewDeadlineAt", review_deadline_at)
        .add_field("securityReviewStatus", status_str(status))
        .add_field("securityReviewType", type_str(review_type))
        .add_field("documentType", wire_name(document_type))
        .add_opt_field("note", note)
        .add_opt_field("requestedAt", requested_at)
        .add_opt_field("requesterUserId", requester_user_id.map(|id| id.to_string()));
    form
}

/// Build the multipart body for the questionnaire-upload endpoints: a multi-file
/// `files` array (each file appended under the same `files` field). Pure - no I/O.
fn build_files_form(files: &[std::path::PathBuf]) -> Multipart {
    let mut form = Multipart::new();
    for f in files {
        form.add_file("files", f);
    }
    form
}

/// Map a multipart `415 Unsupported Media Type` into an actionable error that
/// points the user at the JSON `raw` fallback. Q1b: `create-with-file`'s
/// content-type is the one unverified unknown (read-only key can't probe it);
/// if the endpoint actually wants JSON, this is how the operator finds out.
fn map_with_file_415(err: eyre::Report, path: &str) -> eyre::Report {
    if let Some(api) = err.downcast_ref::<ApiError>()
        && api.status == StatusCode::UNSUPPORTED_MEDIA_TYPE
    {
        return eyre::eyre!(
            "create-with-file was rejected with 415 Unsupported Media Type.\n\
             This endpoint was sent as multipart/form-data; it may instead require \
             application/json with a base64 `file` field.\n\
             Fall back to the raw passthrough, e.g.:\n  \
             drata raw POST {} --data '<json body with file field>'\n\
             Original error: {}",
            path,
            api.formatted
        );
    }
    err
}

#[allow(clippy::too_many_arguments)]
async fn create_with_file(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    vendor_id: &str,
    file: &Path,
    title: &str,
    review_deadline_at: &str,
    status: &SecurityReviewStatus,
    review_type: &SecurityReviewType,
    document_type: &SecurityReviewDocumentType,
    note: Option<&str>,
    requested_at: Option<&str>,
    requester_user_id: Option<u64>,
) -> Result<()> {
    debug!(
        vendor_id,
        file = %file.display(),
        title,
        review_deadline_at,
        status = status_str(status),
        review_type = type_str(review_type),
        document_type = %wire_name(document_type),
        has_note = note.is_some(),
        has_requested_at = requested_at.is_some(),
        has_requester_user_id = requester_user_id.is_some(),
        "security_review create-with-file"
    );

    let path = format!("/vendors/{}/security-reviews/with-file", vendor_id);

    if !confirm("POST", &path)? {
        bail!("aborted");
    }

    let form = build_create_with_file_form(
        file,
        title,
        review_deadline_at,
        status,
        review_type,
        document_type,
        note,
        requested_at,
        requester_user_id,
    );

    let result = client
        .post_multipart(&path, &form)
        .await
        .map_err(|e| map_with_file_415(e, &path))?;
    print_value(&result, &config.output_format);
    debug!(vendor_id, "security_review create-with-file complete");
    Ok(())
}

async fn upload_questionnaire(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    vendor_id: &str,
    files: &[std::path::PathBuf],
) -> Result<()> {
    debug!(
        vendor_id,
        file_count = files.len(),
        "security_review upload-questionnaire"
    );

    let path = format!("/vendors/{}/security-questionnaires", vendor_id);

    if !confirm("POST", &path)? {
        bail!("aborted");
    }

    let form = build_files_form(files);
    let result = client.post_multipart(&path, &form).await?;
    print_value(&result, &config.output_format);
    debug!(
        vendor_id,
        file_count = files.len(),
        "security_review upload-questionnaire complete"
    );
    Ok(())
}

async fn upload_questionnaire_to_review(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    vendor_id: &str,
    security_review_id: u64,
    files: &[std::path::PathBuf],
) -> Result<()> {
    debug!(
        vendor_id,
        security_review_id,
        file_count = files.len(),
        "security_review upload-questionnaire-to-review"
    );

    let path = format!(
        "/vendors/{}/security-reviews/{}/security-questionnaires",
        vendor_id, security_review_id
    );

    if !confirm("POST", &path)? {
        bail!("aborted");
    }

    let form = build_files_form(files);
    let result = client.post_multipart(&path, &form).await?;
    print_value(&result, &config.output_format);
    debug!(
        vendor_id,
        security_review_id,
        file_count = files.len(),
        "security_review upload-questionnaire-to-review complete"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Insert `key: value` into `body` only when `value` is `Some`. Keeps update
/// bodies sparse so unset fields are not overwritten.
pub(crate) fn set_opt_str(body: &mut Value, key: &str, value: Option<&str>) {
    if let Some(v) = value {
        body[key] = json!(v);
    }
}

#[cfg(test)]
mod tests;
