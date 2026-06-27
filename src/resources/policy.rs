//! `drata policy ...` - policies.
//!
//! Paths: `/policies` (list/create), `/policies/{policyId}` (get/update),
//! plus sub-resources (actions, versions, approval-configuration).
//! Confirmed camelCase: `ownerId`, `sourceType`, `renewalDate`, `policyVersionId`.
//! Enum field promoted to `clap::ValueEnum`: `sourceType` (UPLOADED/EXTERNAL).

use crate::cli::{PolicyAction, PolicySourceType};
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use crate::spec;
use eyre::Result;
use serde_json::{Value, json};
use tracing::{debug, instrument};

pub fn example_if_requested(action: &PolicyAction) -> Option<Result<String>> {
    match action {
        PolicyAction::Create { example: true, .. } => Some(example_skeleton("POST", "/policies")),
        _ => None,
    }
}

fn example_skeleton(method: &str, path: &str) -> Result<String> {
    spec::example_for_operation(method, path)?
        .ok_or_else(|| eyre::eyre!("operation `{} {}` has no JSON request body", method, path))
}

pub async fn handle(action: &PolicyAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        PolicyAction::List => list(client, config).await,
        PolicyAction::Get { policy_id } => get(client, config, policy_id).await,
        PolicyAction::Create {
            name,
            owner_id,
            source_type,
            example: _,
        } => create(client, config, name.as_deref(), *owner_id, source_type.as_ref()).await,
        PolicyAction::Update {
            policy_id,
            name,
            owner_id,
            description,
            renewal_date,
        } => {
            update(
                client,
                config,
                policy_id,
                name.as_deref(),
                *owner_id,
                description.as_deref(),
                renewal_date.as_deref(),
            )
            .await
        }
        PolicyAction::Actions { policy_id } => actions(client, config, policy_id).await,
        PolicyAction::Versions { policy_id } => versions(client, config, policy_id).await,
        PolicyAction::Version { policy_id, version_id } => version(client, config, policy_id, version_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config) -> Result<()> {
    debug!("policy list");
    let all = client.get_all("/policies").await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, policy_id: &str) -> Result<()> {
    debug!(policy_id, "policy get");
    let resp = client.get(&format!("/policies/{}", policy_id)).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn create(
    client: &DrataClient,
    config: &Config,
    name: Option<&str>,
    owner_id: Option<u64>,
    source_type: Option<&PolicySourceType>,
) -> Result<()> {
    debug!("policy create");
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    if let Some(id) = owner_id {
        body["ownerId"] = json!(id);
    }
    if let Some(st) = source_type {
        body["sourceType"] = json!(source_type_str(st));
    }
    let result = client.post("/policies", body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(client, config))]
async fn update(
    client: &DrataClient,
    config: &Config,
    policy_id: &str,
    name: Option<&str>,
    owner_id: Option<u64>,
    description: Option<&str>,
    renewal_date: Option<&str>,
) -> Result<()> {
    debug!(policy_id, "policy update");
    let mut body = json!({});
    set_opt(&mut body, "name", name);
    set_opt(&mut body, "description", description);
    set_opt(&mut body, "renewalDate", renewal_date);
    if let Some(id) = owner_id {
        body["ownerId"] = json!(id);
    }
    let result = client.put(&format!("/policies/{}", policy_id), body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn actions(client: &DrataClient, config: &Config, policy_id: &str) -> Result<()> {
    debug!(policy_id, "policy actions");
    let all = client.get_all(&format!("/policies/{}/actions", policy_id)).await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn versions(client: &DrataClient, config: &Config, policy_id: &str) -> Result<()> {
    debug!(policy_id, "policy versions");
    let all = client
        .get_all(&format!("/policies/{}/policy-versions", policy_id))
        .await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn version(client: &DrataClient, config: &Config, policy_id: &str, version_id: &str) -> Result<()> {
    debug!(policy_id, version_id, "policy version get");
    let resp = client
        .get(&format!("/policies/{}/policy-versions/{}", policy_id, version_id))
        .await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn set_opt(body: &mut Value, key: &str, value: Option<&str>) {
    if let Some(v) = value {
        body[key] = json!(v);
    }
}

fn source_type_str(st: &PolicySourceType) -> &'static str {
    match st {
        PolicySourceType::Uploaded => "UPLOADED",
        PolicySourceType::External => "EXTERNAL",
    }
}

#[cfg(test)]
mod tests;
