//! `drata vendor security-review ...` - security reviews for a vendor.
//!
//! All paths are nested under `/vendors/{vendorId}/security-reviews`.
//! Enum serialization confirmed against the Drata spec:
//!   - status: `NOT_YET_STARTED | IN_PROGRESS | COMPLETED | NOT_REQUIRED`
//!     (body key: `securityReviewStatus`)
//!   - type: `SECURITY | SOC_REPORT | UPLOAD_REPORT`
//!     (body key: `securityReviewType`)
//!   - action: `finalize | reopen`
//!
//! Phase 2 implements all GET handlers (list, get, actions, questionnaires)
//! and JSON mutating handlers (create, update, run-action).
//! Phase 3 adds the multipart handlers: `create-with-file` (single `file` part)
//! and `upload-questionnaire` / `upload-questionnaire-to-review` (multi-file
//! `files` array). The questionnaire endpoints are confirmed multipart (Q1a);
//! `create-with-file`'s content-type is unverified (Q1b), so a `415` rejection
//! is mapped to an actionable error pointing at the `raw` JSON fallback.

use crate::cli::{SecurityReviewAction, SecurityReviewStatus, SecurityReviewType, VendorSecurityReviewAction};
use crate::client::{ApiError, DrataClient, Multipart};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::expand::append_expand;
use crate::output::print_value;
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
const SECURITY_REVIEW_UPDATE_EXAMPLE: &str = r#"{
  "title": "Updated review title",
  "socForm": "SOC2_TYPE_II"
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
            expand,
            all,
        } => {
            list(
                client,
                config,
                vendor_id,
                status.as_ref(),
                review_type.as_ref(),
                expand,
                *all,
            )
            .await
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
                review_deadline_at,
                status,
                review_type,
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
        } => run_action(client, config, confirm, vendor_id, *security_review_id, action).await,
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
                document_type.as_deref(),
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

async fn list(
    client: &DrataClient,
    config: &Config,
    vendor_id: &str,
    status: Option<&SecurityReviewStatus>,
    review_type: Option<&SecurityReviewType>,
    expand: &[String],
    all: bool,
) -> Result<()> {
    debug!(
        vendor_id,
        status = status.map(status_str),
        review_type = review_type.map(type_str),
        expand_len = expand.len(),
        all,
        "security_review list"
    );

    let mut base = format!("/vendors/{}/security-reviews", vendor_id);

    // Append filter query params before expand params.
    let mut sep = '?';
    if let Some(s) = status {
        base.push(sep);
        base.push_str(&format!("status={}", status_str(s)));
        sep = '&';
    }
    if let Some(t) = review_type {
        base.push(sep);
        base.push_str(&format!("type={}", type_str(t)));
    }

    let path = append_expand(&base, expand);

    if all {
        let mut stdout = io::stdout();
        let total = client.stream_all(&path, &mut stdout).await?;
        debug!(total, "security_review list stream complete");
    } else {
        let items = client.get_all(&path).await?;
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
    review_deadline_at: &str,
    status: &SecurityReviewStatus,
    review_type: &SecurityReviewType,
    title: Option<&str>,
    note: Option<&str>,
    requested_at: Option<&str>,
    requester_user_id: Option<u64>,
    data: Option<&str>,
) -> Result<()> {
    debug!(
        vendor_id,
        review_deadline_at,
        status = status_str(status),
        review_type = type_str(review_type),
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
        serde_json::from_str(raw).map_err(|e| eyre::eyre!("--data is not valid JSON: {}", e))?
    } else {
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
) -> Result<()> {
    debug!(
        vendor_id,
        security_review_id,
        has_title = title.is_some(),
        has_soc_form = soc_form.is_some(),
        "security_review update"
    );

    let path = format!("/vendors/{}/security-reviews/{}", vendor_id, security_review_id);

    if !confirm("PUT", &path)? {
        bail!("aborted");
    }

    // UpdateDTO has ONLY title and socForm - not create's fields.
    let mut body = json!({});
    set_opt_str(&mut body, "title", title);
    set_opt_str(&mut body, "socForm", soc_form);

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

async fn run_action(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    vendor_id: &str,
    security_review_id: u64,
    action: &SecurityReviewAction,
) -> Result<()> {
    debug!(
        vendor_id,
        security_review_id,
        action = action_str(action),
        "security_review run-action"
    );

    let path = format!("/vendors/{}/security-reviews/{}/actions", vendor_id, security_review_id);

    if !confirm("POST", &path)? {
        bail!("aborted");
    }

    let body = json!({ "action": action_str(action) });
    let result = client.post(&path, body).await?;
    print_value(&result, &config.output_format);
    debug!(
        vendor_id,
        security_review_id,
        action = action_str(action),
        "security_review run-action complete"
    );
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
    document_type: Option<&str>,
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
        .add_opt_field("documentType", document_type)
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
    document_type: Option<&str>,
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
        has_document_type = document_type.is_some(),
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
