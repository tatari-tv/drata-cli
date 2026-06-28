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
//! Phase 1 provides the clap surface + dispatch wiring only.
//! Phases 2-3 add the HTTP handlers.

use crate::cli::{SecurityReviewAction, SecurityReviewStatus, SecurityReviewType, VendorSecurityReviewAction};
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use eyre::Result;
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
// Example skeleton
// ---------------------------------------------------------------------------

/// JSON skeleton for `vendor security-review create --example`.
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

/// Returns the example skeleton if the action is a `--example` request.
/// Called from `vendor::example_if_requested` before config/auth load.
pub fn example_if_requested(action: &VendorSecurityReviewAction) -> Option<&'static str> {
    match action {
        VendorSecurityReviewAction::Create { example: true, .. } => Some(SECURITY_REVIEW_CREATE_EXAMPLE),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Dispatch
// ---------------------------------------------------------------------------

/// Dispatch a `vendor security-review <verb>` action.
///
/// Phase 1: stubs only - each arm logs entry and returns an informative
/// `bail!`. Phases 2-3 replace the stubs with real HTTP calls.
pub async fn handle(
    action: &VendorSecurityReviewAction,
    _client: &DrataClient,
    _config: &Config,
    _confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        VendorSecurityReviewAction::List {
            vendor_id,
            status,
            review_type,
            expand,
            all,
        } => {
            debug!(
                vendor_id,
                status = status.as_ref().map(status_str),
                review_type = review_type.as_ref().map(type_str),
                expand_len = expand.len(),
                all,
                "security_review list (stub)"
            );
            eyre::bail!("vendor security-review list: not yet implemented (Phase 2)")
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
                "security_review create (stub)"
            );
            eyre::bail!("vendor security-review create: not yet implemented (Phase 2)")
        }
        VendorSecurityReviewAction::Get {
            vendor_id,
            security_review_id,
            expand,
        } => {
            debug!(
                vendor_id,
                security_review_id,
                expand_len = expand.len(),
                "security_review get (stub)"
            );
            eyre::bail!("vendor security-review get: not yet implemented (Phase 2)")
        }
        VendorSecurityReviewAction::Update {
            vendor_id,
            security_review_id,
            title,
            soc_form,
        } => {
            debug!(
                vendor_id,
                security_review_id,
                has_title = title.is_some(),
                has_soc_form = soc_form.is_some(),
                "security_review update (stub)"
            );
            eyre::bail!("vendor security-review update: not yet implemented (Phase 2)")
        }
        VendorSecurityReviewAction::Actions {
            vendor_id,
            security_review_id,
        } => {
            debug!(vendor_id, security_review_id, "security_review actions (stub)");
            eyre::bail!("vendor security-review actions: not yet implemented (Phase 2)")
        }
        VendorSecurityReviewAction::RunAction {
            vendor_id,
            security_review_id,
            action,
        } => {
            debug!(
                vendor_id,
                security_review_id,
                action = action_str(action),
                "security_review run-action (stub)"
            );
            eyre::bail!("vendor security-review run-action: not yet implemented (Phase 2)")
        }
        VendorSecurityReviewAction::Questionnaires {
            vendor_id,
            security_review_id,
        } => {
            debug!(vendor_id, security_review_id, "security_review questionnaires (stub)");
            eyre::bail!("vendor security-review questionnaires: not yet implemented (Phase 2)")
        }
    }
}

#[cfg(test)]
mod tests;
