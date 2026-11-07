//! `drata compliance-check-exclusion ...`: personnel compliance-check
//! exclusions of a workspace.
//!
//! | Verb | Method | Path under `/workspaces/{w}/compliance-check-exclusions` |
//! |------|--------|------|
//! | `list` | GET | (the collection) |
//! | `create` | POST | (the collection; per-target results) |
//! | `remove` | POST | `action-delete` (per-target results) |
//!
//! Both writes answer 2xx with one result per requested target, each with its
//! own `outcome`; `bulk` prints every result and exits 1 if any is `FAILED_*`.

pub mod exclusion;

use crate::cli::ComplianceCheckExclusionAction;
use crate::client::{DrataClient, encode_path_segment};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::print_value;
use crate::resources::bulk;
use crate::spec;
use eyre::{Result, bail};
use serde_json::{Value, json};
use tracing::{debug, instrument};

const TEMPLATE: &str = "/workspaces/{workspaceId}/compliance-check-exclusions";

pub fn example_if_requested(action: &ComplianceCheckExclusionAction) -> Option<Result<String>> {
    match action {
        ComplianceCheckExclusionAction::Create { example: true, .. } => Some(
            spec::example_for_operation("POST", TEMPLATE)
                .and_then(|s| s.ok_or_else(|| eyre::eyre!("operation `POST {TEMPLATE}` has no JSON request body"))),
        ),
        _ => None,
    }
}

/// `/workspaces/{w}/compliance-check-exclusions`. Pure.
pub fn collection_path(workspace_id: &str) -> String {
    format!(
        "/workspaces/{}/compliance-check-exclusions",
        encode_path_segment(workspace_id)
    )
}

/// One verb resolved to its request.
#[derive(Debug)]
pub enum Call {
    List {
        path: String,
    },
    /// A confirmed POST answered with per-target results.
    Bulk {
        path: String,
        body: Value,
    },
}

/// Resolve a verb to its request. Reads `--data` (file or stdin) but sends nothing.
pub fn plan(action: &ComplianceCheckExclusionAction) -> Result<Call> {
    use ComplianceCheckExclusionAction as A;
    Ok(match action {
        A::List {
            workspace_id,
            compliance_check,
            expand,
        } => Call::List {
            path: exclusion::list_path(workspace_id, compliance_check.as_ref(), expand),
        },
        A::Create {
            workspace_id,
            target_id,
            compliance_check,
            reason,
            start_date,
            end_date,
            data,
            example: _,
        } => {
            let Some(workspace_id) = workspace_id else {
                bail!(
                    "`compliance-check-exclusion create` requires <workspace_id>; use --example to print the body skeleton"
                );
            };
            Call::Bulk {
                path: collection_path(workspace_id),
                body: match data {
                    Some(raw) => read_data(raw, DataSource::Any)?,
                    None => exclusion::build_create_body(
                        target_id,
                        compliance_check,
                        reason.as_deref(),
                        start_date.as_deref(),
                        end_date.as_deref(),
                    )?,
                },
            }
        }
        A::Remove {
            workspace_id,
            target_id,
            compliance_check,
        } => Call::Bulk {
            path: format!("{}/action-delete", collection_path(workspace_id)),
            body: exclusion::build_delete_body(target_id, compliance_check)?,
        },
    })
}

#[instrument(skip_all)]
pub async fn handle(
    action: &ComplianceCheckExclusionAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match plan(action)? {
        Call::List { path } => {
            debug!(path, "compliance-check-exclusion list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        Call::Bulk { path, body } => {
            debug!(path, "compliance-check-exclusion bulk");
            if !confirm("POST", &path)? {
                bail!("aborted");
            }
            bulk::post(client, config, &path, body, bulk::OUTCOME).await?;
        }
    }
    Ok(())
}
