//! `drata policy ...` - policies.
//!
//! Paths: `/policies` (list/create), `/policies/{policyId}` (get/update),
//! plus sub-resources (actions, versions, approval-configuration).
//! Confirmed camelCase: `ownerId`, `sourceType`, `renewalDate`, `policyVersionId`.
//! Enum field promoted to `clap::ValueEnum`: `sourceType` (UPLOADED/EXTERNAL).
//!
//! Phase 4 adds: `--all` NDJSON streaming, `--expand`, confirm-on-mutation,
//! and `--file` multipart upload (for UPLOADED source type on create).

use crate::cli::{PolicyAction, PolicySourceType};
use crate::client::{DrataClient, Multipart};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::expand::append_expand;
use crate::output::print_value;
use crate::spec;
use eyre::{Result, bail};
use serde_json::{Value, json};
use std::io;
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

pub async fn handle(action: &PolicyAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        PolicyAction::List { all, expand } => list(client, config, *all, expand).await,
        PolicyAction::Get { policy_id, expand } => get(client, config, policy_id, expand).await,
        PolicyAction::Create {
            name,
            owner_id,
            source_type,
            description,
            renewal_date,
            file,
            example: _,
        } => {
            create(
                client,
                config,
                confirm,
                name.as_deref(),
                *owner_id,
                source_type.as_ref(),
                description.as_deref(),
                renewal_date.as_deref(),
                file.as_deref(),
            )
            .await
        }
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
                confirm,
                policy_id,
                name.as_deref(),
                *owner_id,
                description.as_deref(),
                renewal_date.as_deref(),
            )
            .await
        }
        PolicyAction::Actions { policy_id } => actions(client, config, policy_id).await,
        PolicyAction::Versions { policy_id, expand } => versions(client, config, policy_id, expand).await,
        PolicyAction::Version { policy_id, version_id } => version(client, config, policy_id, version_id).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, all: bool, expand: &[String]) -> Result<()> {
    debug!(all, expand_len = expand.len(), "policy list");
    let base = append_expand("/policies", expand);
    if all {
        let mut stdout = io::stdout();
        client.stream_all(&base, &mut stdout).await?;
    } else {
        let items = client.get_all(&base).await?;
        let result = json!({ "data": items });
        print_value(&result, &config.output_format);
    }
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, policy_id: &str, expand: &[String]) -> Result<()> {
    debug!(policy_id, expand_len = expand.len(), "policy get");
    let path = append_expand(&format!("/policies/{}", policy_id), expand);
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
    owner_id: Option<u64>,
    source_type: Option<&PolicySourceType>,
    description: Option<&str>,
    renewal_date: Option<&str>,
    file: Option<&std::path::Path>,
) -> Result<()> {
    debug!(has_file = file.is_some(), "policy create");
    // Spec requires name, ownerId, sourceType, renewalDate, and description for
    // both the JSON and multipart shapes. Validate before confirming.
    let name = name.ok_or_else(|| eyre::eyre!("`drata policy create` requires --name (or use --example)"))?;
    let owner_id =
        owner_id.ok_or_else(|| eyre::eyre!("`drata policy create` requires --owner-id (or use --example)"))?;
    let source_type =
        source_type.ok_or_else(|| eyre::eyre!("`drata policy create` requires --source-type (or use --example)"))?;
    let description =
        description.ok_or_else(|| eyre::eyre!("`drata policy create` requires --description (or use --example)"))?;
    let renewal_date =
        renewal_date.ok_or_else(|| eyre::eyre!("`drata policy create` requires --renewal-date (or use --example)"))?;
    let source_type_value = source_type_str(source_type);

    if !confirm("POST", "/policies")? {
        bail!("aborted");
    }
    // When a file is provided, use multipart upload. --file is only valid with
    // sourceType UPLOADED (or when sourceType is omitted, implying UPLOADED).
    // EXTERNAL policies use an externalFileId, not a file upload.
    if let Some(path) = file {
        if let PolicySourceType::External = source_type {
            bail!(
                "--file is only valid for UPLOADED policies. \
                 EXTERNAL policies reference an existing file via --source-type external \
                 and do not accept a file upload. Remove --file or change --source-type."
            );
        }
        let mut form = Multipart::single("file", path);
        form.add_field("name", name)
            .add_field("ownerId", owner_id.to_string())
            .add_field("sourceType", source_type_value)
            .add_field("description", description)
            .add_field("renewalDate", renewal_date);
        let result = client.post_multipart("/policies", &form).await?;
        print_value(&result, &config.output_format);
        return Ok(());
    }
    let body = json!({
        "name": name,
        "ownerId": owner_id,
        "sourceType": source_type_value,
        "description": description,
        "renewalDate": renewal_date,
    });
    let result = client.post("/policies", body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[allow(clippy::too_many_arguments)]
#[instrument(skip(client, config, confirm))]
async fn update(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    policy_id: &str,
    name: Option<&str>,
    owner_id: Option<u64>,
    description: Option<&str>,
    renewal_date: Option<&str>,
) -> Result<()> {
    debug!(policy_id, "policy update");
    if !confirm("PUT", &format!("/policies/{}", policy_id))? {
        bail!("aborted");
    }
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
async fn versions(client: &DrataClient, config: &Config, policy_id: &str, expand: &[String]) -> Result<()> {
    debug!(policy_id, expand_len = expand.len(), "policy versions");
    let base = append_expand(&format!("/policies/{}/policy-versions", policy_id), expand);
    let all = client.get_all(&base).await?;
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
