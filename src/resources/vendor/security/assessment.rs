//! Security-review assessments (beta, read-only).
//!
//! | Verb | Method | Path |
//! |------|--------|------|
//! | `list` | GET | `/vendors/{v}/security-reviews/{r}/assessments` |
//! | `get` | GET | `.../assessments/{assessmentId}` |
//! | `stats` | GET | `.../assessments-stats` |

use crate::cli::{AssessmentStatus, SecurityReviewAssessmentAction};
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use eyre::Result;
use serde_json::json;
use tracing::{debug, instrument};

/// `GET .../assessments` path with its filters: one bracketed key per value,
/// `criteriaName` as a scalar. Pure.
pub fn build_list_path(
    vendor_id: &str,
    security_review_id: u64,
    criteria_name: Option<&str>,
    status: &[AssessmentStatus],
    severity: &[crate::cli::Severity],
) -> String {
    let mut q = QueryPath::new(format!(
        "/vendors/{}/security-reviews/{}/assessments",
        vendor_id, security_review_id
    ));
    q.push_opt("criteriaName", criteria_name)
        .push_each("statuses[]", status.iter().map(wire_name))
        .push_each("severities[]", severity.iter().map(wire_name));
    q.finish()
}

/// `GET .../assessments/{assessmentId}` path. Pure.
pub fn build_get_path(vendor_id: &str, security_review_id: u64, assessment_id: u64) -> String {
    format!(
        "/vendors/{}/security-reviews/{}/assessments/{}",
        vendor_id, security_review_id, assessment_id
    )
}

/// `GET .../assessments-stats` path. Pure.
pub fn build_stats_path(vendor_id: &str, security_review_id: u64) -> String {
    format!(
        "/vendors/{}/security-reviews/{}/assessments-stats",
        vendor_id, security_review_id
    )
}

#[instrument(skip_all)]
pub async fn handle(action: &SecurityReviewAssessmentAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        SecurityReviewAssessmentAction::List {
            vendor_id,
            security_review_id,
            criteria_name,
            status,
            severity,
        } => {
            let path = build_list_path(
                vendor_id,
                *security_review_id,
                criteria_name.as_deref(),
                status,
                severity,
            );
            debug!(path, "assessment list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        SecurityReviewAssessmentAction::Get {
            vendor_id,
            security_review_id,
            assessment_id,
        } => {
            let resp = client
                .get(&build_get_path(vendor_id, *security_review_id, *assessment_id))
                .await?;
            print_value(&resp, &config.output_format);
        }
        SecurityReviewAssessmentAction::Stats {
            vendor_id,
            security_review_id,
        } => {
            let resp = client.get(&build_stats_path(vendor_id, *security_review_id)).await?;
            print_value(&resp, &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
