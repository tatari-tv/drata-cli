//! `drata policy-language ...`.
//!
//! | Command | Method | Path |
//! |---------|--------|------|
//! | `list` | GET | `/policies/{policyId}/policy-versions/{policyVersionId}/languages` |
//! | `settings` | GET | `/policy-language-settings` |
//! | `versions` | GET | `/policy-language-versions` |
//! | `get` | GET | `/policy-language-versions/{policyLanguageVersionId}` |
//!
//! `get` sends the client's `Accept: application/json`, so the op's `text/html`
//! representation is never requested.

use crate::cli::PolicyLanguageAction;
use crate::client::{DrataClient, encode_path_segment};
use crate::config::Config;
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use eyre::Result;
use serde_json::json;
use tracing::{debug, instrument};

/// `GET .../languages` with `statuses[]`. Pure.
pub fn list_path(policy_id: &str, version_id: &str, statuses: &[crate::cli::PolicyVersionStatus]) -> String {
    let mut q = QueryPath::new(format!(
        "/policies/{}/policy-versions/{}/languages",
        encode_path_segment(policy_id),
        encode_path_segment(version_id)
    ));
    q.push_each("statuses[]", statuses.iter().map(wire_name));
    q.finish()
}

/// `GET /policy-language-versions` with every filter the op takes. Pure.
pub fn versions_path(
    policy_version_id: Option<u64>,
    policy_id: Option<u64>,
    statuses: &[crate::cli::PolicyVersionStatus],
    expand: &[crate::cli::PolicyLanguageVersionListExpand],
) -> String {
    let mut q = QueryPath::new("/policy-language-versions");
    q.push_opt("policyVersionId", policy_version_id)
        .push_opt("policyId", policy_id)
        .push_each("statuses[]", statuses.iter().map(wire_name))
        .push_each("expand[]", expand.iter().map(wire_name));
    q.finish()
}

/// `GET /policy-language-versions/{id}` with `expand[]`. Pure.
pub fn get_path(id: &str, expand: &[crate::cli::PolicyLanguageVersionExpand]) -> String {
    let mut q = QueryPath::new(format!("/policy-language-versions/{}", encode_path_segment(id)));
    q.push_each("expand[]", expand.iter().map(wire_name));
    q.finish()
}

#[instrument(skip_all)]
pub async fn handle(action: &PolicyLanguageAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        PolicyLanguageAction::List {
            policy_id,
            version_id,
            status,
        } => {
            let path = list_path(policy_id, version_id, status);
            debug!(path, "policy-language list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        PolicyLanguageAction::Settings => {
            debug!("policy-language settings");
            print_value(&client.get("/policy-language-settings").await?, &config.output_format);
        }
        PolicyLanguageAction::Versions {
            policy_version_id,
            policy_id,
            status,
            expand,
        } => {
            let path = versions_path(*policy_version_id, *policy_id, status, expand);
            debug!(path, "policy-language versions");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        PolicyLanguageAction::Get { id, expand } => {
            let path = get_path(id, expand);
            debug!(path, "policy-language get");
            print_value(&client.get(&path).await?, &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
