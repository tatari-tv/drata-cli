//! The security-review agent workflow (all 17 ops beta).
//!
//! Every verb addresses `/vendors/{v}/security-reviews/{r}/<segment>`:
//!
//! | Verb | Method | Segment | Success |
//! |------|--------|---------|---------|
//! | `start-agent` | POST | `agent` | 202 |
//! | `confirm-criteria` | POST | `criteria-confirmations` | 200 |
//! | `start-assessment` | POST | `assessment-runs` | 202 |
//! | `stop-assessment` | DELETE | `assessment-runs/current` | 202 |
//! | `collect-public-documents` | POST | `public-document-collections` | 202 |
//! | `access-request-requirements` | GET | `access-request-requirements` | 200 |
//! | `request-trust-center-access` | POST | `access-requests` | 202 |
//! | `provide-trust-center-access-link` | POST | `access-links` (body from @file or stdin only) | 202 |
//! | `skip-trust-center-access-request` | POST | `access-request-skips` | 202 |
//! | `add-documents` | POST (multipart) | `documents` | 201 |
//! | `generate-follow-up-questionnaire` | POST | `follow-up-questionnaires` | 202 |
//! | `follow-up-questionnaire` | GET | `follow-up-questionnaire` | 200 |
//! | `send-follow-up-questionnaire` | POST | `follow-up-questionnaire-sends` | 202 |
//! | `residual-risk` | GET | `residual-risk` | 200 |
//! | `finalize` | POST | `finalizations` | 200 |
//! | `summary` | GET | `summary/status` | 200 |
//! | `workflow` | GET | `workflow` | 200 |
//!
//! A 202 means the server accepted background work; the body is printed and
//! nothing polls (read progress with `workflow`, `summary` or the assessment
//! verbs). A step called out of order is a 422 whose `blockers` and
//! `availableActions` print through the client's API error formatting.

use crate::cli::{
    ExampleReviewIds, ReviewIds, SecurityReviewAgentAction, SecurityReviewDecision, SecurityReviewDocumentType,
};
use crate::client::{DrataClient, Multipart};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::print_value;
use crate::query::wire_name;
use crate::spec;
use eyre::{Result, bail};
use serde_json::{Value, json};
use std::path::PathBuf;
use tracing::{debug, instrument};

/// The spec's file limit on `POST .../documents`.
const MAX_DOCUMENTS: usize = 10;
/// The spec's `maxItems` on each follow-up questionnaire address list.
const MAX_RECIPIENTS: usize = 5;

/// The spec path template every agent op hangs off.
const REVIEW_TEMPLATE: &str = "/vendors/{vendorId}/security-reviews/{securityReviewId}";

/// One agent verb resolved to its request. Each variant carries the only body
/// shape its method takes, so no method/body mismatch can be built.
#[derive(Debug)]
pub enum Call {
    Get {
        path: String,
    },
    /// A JSON POST. The empty-bodied action ops send `{}`: the spec marks their
    /// `AgentReviewActionRequestPublicV2Dto` body required.
    Post {
        path: String,
        body: Value,
    },
    /// A multipart/form-data POST.
    PostForm {
        path: String,
        form: Multipart,
    },
    /// A DELETE with a JSON body (`{}`, required by the spec).
    Delete {
        path: String,
        body: Value,
    },
}

impl Call {
    pub fn method(&self) -> &'static str {
        match self {
            Call::Get { .. } => "GET",
            Call::Post { .. } | Call::PostForm { .. } => "POST",
            Call::Delete { .. } => "DELETE",
        }
    }

    pub fn path(&self) -> &str {
        match self {
            Call::Get { path } | Call::Post { path, .. } | Call::PostForm { path, .. } | Call::Delete { path, .. } => {
                path
            }
        }
    }
}

/// `/vendors/{v}/security-reviews/{r}/<segment>`. Pure.
pub fn review_path(vendor_id: &str, security_review_id: u64, segment: &str) -> String {
    format!(
        "/vendors/{}/security-reviews/{}/{}",
        vendor_id, security_review_id, segment
    )
}

fn ids_path(ids: &ReviewIds, segment: &str) -> String {
    review_path(&ids.vendor_id, ids.security_review_id, segment)
}

/// Unwrap the clap-optional ids; they are optional only so `--example` runs.
fn required_ids(ids: &ExampleReviewIds, verb: &str) -> Result<(String, u64)> {
    match (&ids.vendor_id, ids.security_review_id) {
        (Some(v), Some(r)) => Ok((v.clone(), r)),
        _ => bail!(
            "`vendor security-review {verb}` requires <vendor_id> <security_review_id>; \
             use --example to print the body skeleton"
        ),
    }
}

fn get(path: String) -> Call {
    Call::Get { path }
}

fn empty_post(path: String) -> Call {
    Call::Post { path, body: json!({}) }
}

/// `--data` when given, else the flag-built body.
fn body_or_data(data: Option<&str>, build: impl FnOnce() -> Result<Value>) -> Result<Value> {
    match data {
        Some(raw) => read_data(raw, DataSource::Any),
        None => build(),
    }
}

/// `confirm-criteria` body: `{}` (the server picks the default criteria) or
/// `{"criterionIds": [...]}`. Pure.
pub fn build_confirm_criteria_body(criterion_ids: &[u64]) -> Value {
    if criterion_ids.is_empty() {
        json!({})
    } else {
        json!({ "criterionIds": criterion_ids })
    }
}

/// `request-trust-center-access` body from the `--fields` object: `{}` when
/// absent (the requirements list no fields), else `{"fields": {...}}`. Pure.
pub fn build_access_request_body(fields: Option<Value>) -> Result<Value> {
    match fields {
        None => Ok(json!({})),
        Some(Value::Object(map)) => Ok(json!({ "fields": map })),
        Some(other) => bail!(
            "--fields must be a JSON object keyed by form field name, got {}",
            json_kind(&other)
        ),
    }
}

fn json_kind(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "a boolean",
        Value::Number(_) => "a number",
        Value::String(_) => "a string",
        Value::Array(_) => "an array",
        Value::Object(_) => "an object",
    }
}

/// Check a `provide-trust-center-access-link` body: an object whose
/// `accessLink` is a non-empty string. The link is a secret, so no error here
/// quotes it. Pure.
pub fn check_access_link_body(body: Value) -> Result<Value> {
    let ok = body
        .get("accessLink")
        .and_then(Value::as_str)
        .is_some_and(|link| !link.trim().is_empty());
    if !ok {
        bail!(
            "--data must be a JSON object with a non-empty string `accessLink` \
             (see --example), got {} without one",
            json_kind(&body)
        );
    }
    Ok(body)
}

/// `add-documents` multipart body: every file under `files`, plus `type` when
/// given. 1 to 10 files, per the spec. Pure.
pub fn build_documents_form(files: &[PathBuf], doc_type: Option<&SecurityReviewDocumentType>) -> Result<Multipart> {
    if files.is_empty() || files.len() > MAX_DOCUMENTS {
        bail!("add-documents takes 1 to {MAX_DOCUMENTS} files, got {}", files.len());
    }
    let mut form = Multipart::new();
    for f in files {
        form.add_file("files", f);
    }
    form.add_opt_field("type", doc_type.map(wire_name));
    Ok(form)
}

/// `send-follow-up-questionnaire` body. `email` is required (1 to 5), `cc` and
/// `bcc` take 5 at most; the optional strings are sent only when given. Pure.
pub fn build_follow_up_send_body(
    email: &[String],
    cc: &[String],
    bcc: &[String],
    email_subject: Option<&str>,
    email_content: Option<&str>,
) -> Result<Value> {
    if email.is_empty() {
        bail!("requires --email (or use --data/--example)");
    }
    for (flag, list) in [("--email", email), ("--cc", cc), ("--bcc", bcc)] {
        if list.len() > MAX_RECIPIENTS {
            bail!("{flag} takes {MAX_RECIPIENTS} addresses at most, got {}", list.len());
        }
    }
    let mut body = json!({ "email": email });
    if !cc.is_empty() {
        body["cc"] = json!(cc);
    }
    if !bcc.is_empty() {
        body["bcc"] = json!(bcc);
    }
    if let Some(s) = email_subject {
        body["emailSubject"] = json!(s);
    }
    if let Some(c) = email_content {
        body["emailContent"] = json!(c);
    }
    Ok(body)
}

/// `finalize` body. The decision is required: unlike `run-action`, the server
/// has no default. Pure.
pub fn build_finalize_body(decision: Option<&SecurityReviewDecision>, note: Option<&str>) -> Result<Value> {
    let decision = decision.ok_or_else(|| eyre::eyre!("requires --decision (or use --data/--example)"))?;
    let mut body = json!({ "decision": wire_name(decision) });
    if let Some(n) = note {
        body["note"] = json!(n);
    }
    Ok(body)
}

/// Resolve a verb to its request: path, method and body. Reads `--data` and
/// `--fields` (file or stdin) but sends nothing.
pub fn plan(action: &SecurityReviewAgentAction) -> Result<Call> {
    use SecurityReviewAgentAction as A;
    Ok(match action {
        A::StartAgent(ids) => empty_post(ids_path(ids, "agent")),
        A::ConfirmCriteria {
            ids,
            criterion_id,
            data,
            example: _,
        } => {
            let (v, r) = required_ids(ids, "confirm-criteria")?;
            Call::Post {
                path: review_path(&v, r, "criteria-confirmations"),
                body: body_or_data(data.as_deref(), || Ok(build_confirm_criteria_body(criterion_id)))?,
            }
        }
        A::StartAssessment(ids) => empty_post(ids_path(ids, "assessment-runs")),
        A::StopAssessment(ids) => Call::Delete {
            path: ids_path(ids, "assessment-runs/current"),
            body: json!({}),
        },
        A::CollectPublicDocuments(ids) => empty_post(ids_path(ids, "public-document-collections")),
        A::AccessRequestRequirements(ids) => get(ids_path(ids, "access-request-requirements")),
        A::RequestTrustCenterAccess {
            ids,
            fields,
            data,
            example: _,
        } => {
            let (v, r) = required_ids(ids, "request-trust-center-access")?;
            Call::Post {
                path: review_path(&v, r, "access-requests"),
                body: body_or_data(data.as_deref(), || {
                    let fields = fields.as_deref().map(|f| read_data(f, DataSource::Any)).transpose()?;
                    build_access_request_body(fields)
                })?,
            }
        }
        A::ProvideTrustCenterAccessLink { ids, data, example: _ } => {
            let (v, r) = required_ids(ids, "provide-trust-center-access-link")?;
            // The link is a secret: file or stdin only, so it never lands in
            // shell history or the process table.
            let data = data
                .as_deref()
                .ok_or_else(|| eyre::eyre!("`provide-trust-center-access-link` requires --data <@file|->"))?;
            Call::Post {
                path: review_path(&v, r, "access-links"),
                body: check_access_link_body(read_data(data, DataSource::FileOrStdinOnly)?)?,
            }
        }
        A::SkipTrustCenterAccessRequest(ids) => empty_post(ids_path(ids, "access-request-skips")),
        A::AddDocuments { ids, file, doc_type } => Call::PostForm {
            path: ids_path(ids, "documents"),
            form: build_documents_form(file, doc_type.as_ref())?,
        },
        A::GenerateFollowUpQuestionnaire(ids) => empty_post(ids_path(ids, "follow-up-questionnaires")),
        A::FollowUpQuestionnaire(ids) => get(ids_path(ids, "follow-up-questionnaire")),
        A::SendFollowUpQuestionnaire {
            ids,
            email,
            cc,
            bcc,
            email_subject,
            email_content,
            data,
            example: _,
        } => {
            let (v, r) = required_ids(ids, "send-follow-up-questionnaire")?;
            Call::Post {
                path: review_path(&v, r, "follow-up-questionnaire-sends"),
                body: body_or_data(data.as_deref(), || {
                    build_follow_up_send_body(email, cc, bcc, email_subject.as_deref(), email_content.as_deref())
                })?,
            }
        }
        A::ResidualRisk(ids) => get(ids_path(ids, "residual-risk")),
        A::Finalize {
            ids,
            decision,
            note,
            data,
            example: _,
        } => {
            let (v, r) = required_ids(ids, "finalize")?;
            Call::Post {
                path: review_path(&v, r, "finalizations"),
                body: body_or_data(data.as_deref(), || {
                    build_finalize_body(decision.as_ref(), note.as_deref())
                })?,
            }
        }
        A::Summary(ids) => get(ids_path(ids, "summary/status")),
        A::Workflow(ids) => get(ids_path(ids, "workflow")),
    })
}

/// The `--example` skeleton when the action is a body verb's `--example`.
pub fn example_if_requested(action: &SecurityReviewAgentAction) -> Option<Result<String>> {
    use SecurityReviewAgentAction as A;
    let segment = match action {
        A::ConfirmCriteria { example: true, .. } => "criteria-confirmations",
        A::RequestTrustCenterAccess { example: true, .. } => "access-requests",
        A::ProvideTrustCenterAccessLink { example: true, .. } => "access-links",
        A::SendFollowUpQuestionnaire { example: true, .. } => "follow-up-questionnaire-sends",
        A::Finalize { example: true, .. } => "finalizations",
        _ => return None,
    };
    let path = format!("{REVIEW_TEMPLATE}/{segment}");
    Some(
        spec::example_for_operation(crate::backend::Backend::Drata, "POST", &path)
            .and_then(|s| s.ok_or_else(|| eyre::eyre!("operation `POST {}` has no JSON request body", path))),
    )
}

/// Run one agent verb: plan it, confirm a mutation, send it, print the body
/// (a 202's acceptance body included).
#[instrument(skip_all)]
pub async fn handle(
    action: &SecurityReviewAgentAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    let call = plan(action)?;
    let (method, path) = (call.method(), call.path().to_string());
    // Bodies are not logged: an access link or access-request form is a secret.
    debug!(method, path, "security-review agent");
    if method != "GET" && !confirm(method, &path)? {
        bail!("aborted");
    }
    let result = match call {
        Call::Get { path } => client.get(&path).await?,
        Call::Post { path, body } => client.post(&path, body).await?,
        Call::PostForm { path, form } => client.post_multipart(&path, &form).await?,
        Call::Delete { path, body } => client.delete(&path, Some(body)).await?,
    };
    print_value(&result, &config.output_format);
    debug!(method, path, "security-review agent complete");
    Ok(())
}

#[cfg(test)]
mod tests;
