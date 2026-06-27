//! `drata vendor ...` - the curated vendors vertical, end-to-end.
//!
//! This module is the template the later phases copy: a typed `VendorAction`
//! clap enum, typed create/update args, cursor-paginated list filtered through
//! the 3-tier matcher, a `--example` skeleton, and questionnaire sub-commands.
//! Writes (create/update/remove/send) go through the client, which enforces the
//! write guardrail.
//!
//! Drata vendor JSON is camelCase (confirmed against
//! `spec/drata-openapi-v2.json`): `renewalDate`, `recipientEmail`, etc. Bodies
//! are built incrementally with `json!`.

use crate::cli::{VendorAction, VendorQuestionnaireAction};
use crate::client::DrataClient;
use crate::config::Config;
use crate::filter;
use crate::output::print_value;
use eyre::Result;
use serde_json::{Value, json};
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
pub fn example_if_requested(action: &VendorAction) -> Option<&'static str> {
    match action {
        VendorAction::Create { example: true, .. } => Some(VENDOR_CREATE_EXAMPLE),
        _ => None,
    }
}

pub async fn handle(action: &VendorAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        VendorAction::List { patterns } => list(client, config, patterns).await,
        VendorAction::Get { id } => get(client, config, id).await,
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
        } => {
            update(
                client,
                config,
                id,
                name.as_deref(),
                category.as_deref(),
                risk.as_deref(),
                status.as_deref(),
                url.as_deref(),
                notes.as_deref(),
            )
            .await
        }
        VendorAction::Remove { id } => remove(client, config, id).await,
        VendorAction::Questionnaire { action } => questionnaire(client, config, action).await,
    }
}

// ---------------------------------------------------------------------------
// CRUD
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, patterns: &[String]) -> Result<()> {
    debug!(patterns_len = patterns.len(), "vendor list");
    let all = client.get_all("/vendors").await?;
    let filtered = filter::filter_into(all, patterns, vendor_name);
    let result = json!({ "data": filtered });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, id: &str) -> Result<()> {
    debug!(id, "vendor get");
    let resp = client.get(&format!("/vendors/{}", id)).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(client, config))]
async fn create(
    client: &DrataClient,
    config: &Config,
    name: Option<&str>,
    category: Option<&str>,
    risk: Option<&str>,
    status: Option<&str>,
    url: Option<&str>,
    notes: Option<&str>,
) -> Result<()> {
    let name = name.ok_or_else(|| eyre::eyre!("`drata vendor create` requires --name (or use --example)"))?;
    debug!(name, "vendor create");
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
#[instrument(skip(client, config))]
async fn update(
    client: &DrataClient,
    config: &Config,
    id: &str,
    name: Option<&str>,
    category: Option<&str>,
    risk: Option<&str>,
    status: Option<&str>,
    url: Option<&str>,
    notes: Option<&str>,
) -> Result<()> {
    debug!(id, "vendor update");
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "category", category);
    set_opt(&mut body, "risk", risk);
    set_opt(&mut body, "status", status);
    set_opt(&mut body, "url", url);
    set_opt(&mut body, "notes", notes);

    let result = client.put(&format!("/vendors/{}", id), body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn remove(client: &DrataClient, config: &Config, id: &str) -> Result<()> {
    debug!(id, "vendor remove");
    let result = client.delete(&format!("/vendors/{}", id)).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

// ---------------------------------------------------------------------------
// Questionnaire subresource
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn questionnaire(client: &DrataClient, config: &Config, action: &VendorQuestionnaireAction) -> Result<()> {
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
        VendorQuestionnaireAction::Send {
            vendor_id,
            email,
            questionnaire_id,
            security_review_id,
            email_content,
            email_subject,
        } => {
            debug!(vendor_id, questionnaire_id, "questionnaire send");
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
