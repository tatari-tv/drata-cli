//! `drata vendor ...` - the curated vendors vertical, end-to-end.
//!
//! This module is the template the later phases copy: a typed `VendorAction`
//! clap enum, typed create/update args, cursor-paginated list filtered through
//! the 3-tier matcher, a `--example` skeleton, and questionnaire sub-commands.
//! Writes (create/update/remove/send) go through the client, which enforces the
//! write guardrail, then through the confirm gate.
//!
//! Phase 4 adds:
//! - `--all` NDJSON streaming on list
//! - `--expand` on list/get (spec: `expand[]` query param)
//! - `--file` multipart upload on vendor documents
//! - confirm-on-mutation gate (POST/PUT/DELETE ask before sending)
//!
//! Drata vendor JSON is camelCase (confirmed against
//! `spec/drata-openapi-v2.json`): `renewalDate`, `recipientEmail`, etc. Bodies
//! are built incrementally with `json!`.

use crate::cli::{
    ReviewDeadlineStatus, VendorAction, VendorCategory, VendorQuestionnaireAction, VendorSearchArgs, VendorStatus,
    VendorType,
};
use crate::client::{DrataClient, Multipart};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::expand::append_expand;
use crate::filter;
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use crate::resources::search;
use eyre::{Result, bail};
use serde_json::{Value, json};
use std::io;
use tracing::{debug, instrument};

/// JSON skeleton printed by `drata vendor create --example`. Generated from the
/// spec's create-vendor request schema (only `name` is required).
const VENDOR_CREATE_EXAMPLE: &str = r#"{
  "name": "Example Vendor",
  "category": "SECURITY",
  "risk": "LOW",
  "status": "ACTIVE",
  "url": "https://vendor.example.com",
  "notes": "Free-form notes about this vendor"
}
"#;

/// Returns the skeleton to print if this is a `--example` request, else `None`.
/// Checked before config/auth load so `--example` needs no key.
pub fn example_if_requested(action: &VendorAction) -> Option<Result<String>> {
    match action {
        VendorAction::Create { example: true, .. } => Some(Ok(VENDOR_CREATE_EXAMPLE.to_string())),
        VendorAction::SecurityReview { action: sub } => security::example_if_requested(sub),
        _ => None,
    }
}

pub async fn handle(action: &VendorAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        VendorAction::List {
            patterns,
            all,
            expand,
            status,
            category,
            vendor_type,
            vendor_type_id,
            vendor_user_id,
            sub_processor,
            action_required,
            review_deadline_status,
        } => {
            let path = build_list_path(
                expand,
                status,
                category,
                vendor_type,
                vendor_type_id,
                vendor_user_id,
                *sub_processor,
                *action_required,
                review_deadline_status,
            );
            list(client, config, patterns, *all, &path).await
        }
        VendorAction::Get { id, expand } => get(client, config, id, expand).await,
        VendorAction::Create {
            name,
            category,
            risk,
            status,
            url,
            notes,
            example: _,
        } => {
            create(
                client,
                config,
                confirm,
                name.as_deref(),
                category.as_deref(),
                risk.as_deref(),
                status.as_deref(),
                url.as_deref(),
                notes.as_deref(),
            )
            .await
        }
        VendorAction::Update {
            id,
            name,
            category,
            risk,
            status,
            url,
            notes,
            vendor_type_id,
        } => {
            update(
                client,
                config,
                confirm,
                id,
                name.as_deref(),
                category.as_deref(),
                risk.as_deref(),
                status.as_deref(),
                url.as_deref(),
                notes.as_deref(),
                *vendor_type_id,
            )
            .await
        }
        VendorAction::Remove { id } => remove(client, config, confirm, id).await,
        VendorAction::Upload {
            vendor_id,
            file,
            doc_type,
            security_review_id,
        } => {
            upload(
                client,
                config,
                confirm,
                vendor_id,
                file,
                doc_type.as_deref(),
                *security_review_id,
            )
            .await
        }
        VendorAction::Search(args) => {
            let (query, facets) = build_search(args);
            search::run(client, config, query, &facets).await
        }
        VendorAction::TrustOverview { vendor_id } => {
            let resp = client.get(&build_trust_overview_path(vendor_id)).await?;
            print_value(&resp, &config.output_format);
            Ok(())
        }
        VendorAction::Questionnaire { action } => questionnaire(client, config, confirm, action).await,
        VendorAction::SecurityReview { action } => security::handle(action, client, config, confirm).await,
    }
}

// ---------------------------------------------------------------------------
// CRUD
// ---------------------------------------------------------------------------

/// Build the `GET /vendors` path from the list filters: one bracketed key per
/// value for the array filters, scalars for the two booleans, then `expand[]`.
/// Pure, so the encoding is unit-tested.
#[allow(clippy::too_many_arguments)]
fn build_list_path(
    expand: &[String],
    status: &[VendorStatus],
    category: &[VendorCategory],
    vendor_type: &[VendorType],
    vendor_type_id: &[u64],
    vendor_user_id: &[u64],
    sub_processor: Option<bool>,
    action_required: Option<bool>,
    review_deadline_status: &[ReviewDeadlineStatus],
) -> String {
    let mut q = QueryPath::new("/vendors");
    q.push_each("statuses[]", status.iter().map(wire_name))
        .push_each("categories[]", category.iter().map(wire_name))
        .push_each("types[]", vendor_type.iter().map(wire_name))
        .push_each("vendorTypeIds[]", vendor_type_id)
        .push_each("vendorUserIds[]", vendor_user_id)
        .push_opt("isSubProcessor", sub_processor)
        .push_opt("actionRequired", action_required)
        .push_each(
            "nextReviewDeadlineStatus[]",
            review_deadline_status.iter().map(wire_name),
        );
    append_expand(&q.finish(), expand)
}

/// The `GET /vendors-search` query and the facet wire names. Array filters
/// repeat their bracketed key once per value; scalars are plain keys. Pure.
fn build_search(args: &VendorSearchArgs) -> (QueryPath, Vec<String>) {
    let mut q = QueryPath::new("/vendors-search");
    q.push_opt("q", args.q.as_deref())
        .push_each("status[]", args.status.iter().map(wire_name))
        .push_each("category[]", args.category.iter().map(wire_name))
        .push_each("vendorTypeIds[]", &args.vendor_type_id)
        .push_each("vendorUserIds[]", &args.vendor_user_id)
        .push_opt("risk", args.risk.as_ref().map(wire_name))
        .push_opt("impactLevel", args.impact_level.as_ref().map(wire_name))
        .push_opt("isSubProcessor", args.sub_processor)
        .push_opt("actionRequired", args.action_required)
        .push_opt("renewalDateGte", args.renewal_from.as_deref())
        .push_opt("renewalDateLte", args.renewal_to.as_deref())
        .push_each(
            "nextReviewDeadlineStatus[]",
            args.review_deadline_status.iter().map(wire_name),
        )
        .push_each(
            "securityReviewStatus[]",
            args.security_review_status.iter().map(wire_name),
        )
        .push_each(
            "securityReviewDecision[]",
            args.security_review_decision.iter().map(wire_name),
        )
        .push_each("expand[]", args.expand.iter().map(wire_name));
    let facets = args.facet.iter().map(wire_name).collect();
    (q, facets)
}

/// `GET /vendors/{vendorId}/trust-overview` path. Pure.
fn build_trust_overview_path(vendor_id: &str) -> String {
    format!("/vendors/{}/trust-overview", vendor_id)
}

/// `GET /vendors/{vendorId}/questionnaires/{questionnaireId}/questions` path. Pure.
fn build_questions_path(vendor_id: &str, questionnaire_id: &str) -> String {
    format!("/vendors/{}/questionnaires/{}/questions", vendor_id, questionnaire_id)
}

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, patterns: &[String], all: bool, path: &str) -> Result<()> {
    debug!(patterns_len = patterns.len(), all, path, "vendor list");
    // --all streams the full tenant export as NDJSON; name-pattern filtering is
    // buffered and does not apply to streaming mode. Reject the combination so
    // the caller cannot silently get a wrong result.
    if all && !patterns.is_empty() {
        bail!(
            "--all and name patterns are mutually exclusive. \
             --all streams the full unfiltered export; \
             omit --all to filter by name patterns."
        );
    }
    if all {
        let mut stdout = io::stdout();
        client.stream_all(path, &mut stdout).await?;
    } else {
        let raw = client.get_all(path).await?;
        let filtered = filter::filter_into(raw, patterns, vendor_name);
        let result = json!({ "data": filtered });
        print_value(&result, &config.output_format);
    }
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, id: &str, expand: &[String]) -> Result<()> {
    debug!(id, expand_len = expand.len(), "vendor get");
    let path = append_expand(&format!("/vendors/{}", id), expand);
    let resp = client.get(&path).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(client, config, confirm))]
async fn create(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    name: Option<&str>,
    category: Option<&str>,
    risk: Option<&str>,
    status: Option<&str>,
    url: Option<&str>,
    notes: Option<&str>,
) -> Result<()> {
    let name = name.ok_or_else(|| eyre::eyre!("`drata vendor create` requires --name (or use --example)"))?;
    debug!(name, "vendor create");
    if !confirm("POST", "/vendors")? {
        bail!("aborted");
    }
    let mut body = json!({ "name": name });
    set_opt(&mut body, "category", category);
    set_opt(&mut body, "risk", risk);
    set_opt(&mut body, "status", status);
    set_opt(&mut body, "url", url);
    set_opt(&mut body, "notes", notes);

    let result = client.post("/vendors", body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(client, config, confirm))]
async fn update(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    id: &str,
    name: Option<&str>,
    category: Option<&str>,
    risk: Option<&str>,
    status: Option<&str>,
    url: Option<&str>,
    notes: Option<&str>,
    vendor_type_id: Option<u64>,
) -> Result<()> {
    debug!(id, ?vendor_type_id, "vendor update");
    if !confirm("PUT", &format!("/vendors/{}", id))? {
        bail!("aborted");
    }
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "category", category);
    set_opt(&mut body, "risk", risk);
    set_opt(&mut body, "status", status);
    set_opt(&mut body, "url", url);
    set_opt(&mut body, "notes", notes);
    if let Some(v) = vendor_type_id {
        body["vendorTypeId"] = json!(v);
    }

    let result = client.put(&format!("/vendors/{}", id), body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config, confirm))]
async fn remove(client: &DrataClient, config: &Config, confirm: &ConfirmFn, id: &str) -> Result<()> {
    debug!(id, "vendor remove");
    if !confirm("DELETE", &format!("/vendors/{}", id))? {
        bail!("aborted");
    }
    let result = client.delete(&format!("/vendors/{}", id), None).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config, confirm))]
async fn upload(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    vendor_id: &str,
    file: &std::path::Path,
    doc_type: Option<&str>,
    security_review_id: Option<u64>,
) -> Result<()> {
    debug!(vendor_id, file = %file.display(), "vendor upload document");
    let path = format!("/vendors/{}/documents", vendor_id);
    if !confirm("POST", &path)? {
        bail!("aborted");
    }
    // Spec requires the `file` part; `type` and `securityReviewId` are optional.
    let mut form = Multipart::single("file", file);
    form.add_opt_field("type", doc_type)
        .add_opt_field("securityReviewId", security_review_id.map(|id| id.to_string()));
    let result = client.post_multipart(&path, &form).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

// ---------------------------------------------------------------------------
// Questionnaire subresource
// ---------------------------------------------------------------------------

#[instrument(skip_all)]
async fn questionnaire(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    action: &VendorQuestionnaireAction,
) -> Result<()> {
    match action {
        VendorQuestionnaireAction::List { vendor_id } => {
            debug!(vendor_id, "questionnaire list");
            let all = client
                .get_all(&format!("/vendors/{}/questionnaires", vendor_id))
                .await?;
            let result = json!({ "data": all });
            print_value(&result, &config.output_format);
            Ok(())
        }
        VendorQuestionnaireAction::Get {
            vendor_id,
            questionnaire_id,
        } => {
            debug!(vendor_id, questionnaire_id, "questionnaire get");
            let resp = client
                .get(&format!("/vendors/{}/questionnaires/{}", vendor_id, questionnaire_id))
                .await?;
            print_value(&resp, &config.output_format);
            Ok(())
        }
        VendorQuestionnaireAction::Questions {
            vendor_id,
            questionnaire_id,
        } => {
            debug!(vendor_id, questionnaire_id, "questionnaire questions");
            let items = client
                .get_all(&build_questions_path(vendor_id, questionnaire_id))
                .await?;
            print_value(&json!({ "data": items }), &config.output_format);
            Ok(())
        }
        VendorQuestionnaireAction::Send {
            vendor_id,
            email,
            questionnaire_id,
            security_review_id,
            email_content,
            email_subject,
        } => {
            debug!(vendor_id, questionnaire_id, "questionnaire send");
            if !confirm("POST", &format!("/vendors/{}/questionnaires", vendor_id))? {
                bail!("aborted");
            }
            let mut body = json!({
                "email": email,
                "questionnaireId": questionnaire_id,
                "securityReviewId": security_review_id,
                "emailContent": email_content,
            });
            if let Some(subject) = email_subject {
                body["emailSubject"] = json!(subject);
            }
            let result = client
                .post(&format!("/vendors/{}/questionnaires", vendor_id), body)
                .await?;
            print_value(&result, &config.output_format);
            Ok(())
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Insert `key: value` into `body` only when `value` is `Some`. Keeps update
/// bodies sparse so unset fields are not overwritten.
fn set_opt(body: &mut Value, key: &str, value: Option<&str>) {
    if let Some(v) = value {
        body[key] = json!(v);
    }
}

fn vendor_name(value: &Value) -> &str {
    value.get("name").and_then(|v| v.as_str()).unwrap_or("")
}

#[cfg(test)]
mod tests;

pub mod reference;
pub mod security;
