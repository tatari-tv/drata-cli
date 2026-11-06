//! `drata access-review app ...`: Access Review Applications and the user set
//! loaded into each. `users set` replaces the whole set; its confirm prompt
//! says users left out are removed.

use super::{body_or_data, example, require};
use crate::cli::{AccessReviewAppAction, AccessReviewAppUsersAction, ReviewUser};
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::output::print_value;
use eyre::{Result, bail};
use serde_json::{Map, Value, json};
use std::collections::HashSet;
use tracing::{debug, instrument};

const COLLECTION: &str = "/access-review-applications";
const ITEM_TEMPLATE: &str = "/access-review-applications/{applicationId}";
const USERS_TEMPLATE: &str = "/access-review-applications/{applicationId}/users";

/// `/access-review-applications/{a}`. Pure.
pub fn item_path(application_id: u64) -> String {
    format!("{COLLECTION}/{application_id}")
}

/// `/access-review-applications/{a}/users`. Pure.
pub fn users_path(application_id: u64) -> String {
    format!("{}/users", item_path(application_id))
}

/// The `create` body: `name` and `reviewersIds` (both spec-required), plus
/// `websiteUrl` when given. Pure.
pub fn build_create_body(name: &str, website_url: Option<&str>, reviewer_ids: &[u64]) -> Value {
    let mut body = Map::new();
    body.insert("name".to_string(), json!(name));
    if let Some(url) = website_url {
        body.insert("websiteUrl".to_string(), json!(url));
    }
    body.insert("reviewersIds".to_string(), json!(reviewer_ids));
    Value::Object(body)
}

/// The `update` body: a partial update, so only what a flag names is sent.
/// `--clear-website-url` sends `null` (the spec's documented clear). An update
/// that changes nothing is rejected before the confirm. Pure.
pub fn build_update_body(
    name: Option<&str>,
    website_url: Option<&str>,
    clear_website_url: bool,
    reviewer_ids: &[u64],
) -> Result<Value> {
    let mut body = Map::new();
    if let Some(name) = name {
        body.insert("name".to_string(), json!(name));
    }
    match (website_url, clear_website_url) {
        (Some(url), _) => {
            body.insert("websiteUrl".to_string(), json!(url));
        }
        (None, true) => {
            body.insert("websiteUrl".to_string(), Value::Null);
        }
        (None, false) => {}
    }
    if !reviewer_ids.is_empty() {
        body.insert("reviewersIds".to_string(), json!(reviewer_ids));
    }
    if body.is_empty() {
        bail!(
            "`access-review app update` needs --name, --website-url, --clear-website-url or --reviewer-id (or --data)"
        );
    }
    Ok(Value::Object(body))
}

/// The `users set` body: `identities`, one entry per `--user` and nothing
/// else. `roles` is sent only when given (the spec: "Omit for an empty
/// list"). Two users with one identity id are rejected: the spec makes the id
/// unique within the application. Pure.
pub fn build_users_body(users: &[ReviewUser]) -> Result<Value> {
    let mut seen = HashSet::new();
    let mut identities = Vec::with_capacity(users.len());
    for user in users {
        if !seen.insert(user.identity_id.as_str()) {
            bail!("identity id `{}` is given twice in --user", user.identity_id);
        }
        let mut identity = Map::new();
        identity.insert("identityId".to_string(), json!(user.identity_id));
        identity.insert("username".to_string(), json!(user.username));
        identity.insert("email".to_string(), json!(user.email));
        if !user.roles.is_empty() {
            identity.insert("roles".to_string(), json!(user.roles));
        }
        identities.push(Value::Object(identity));
    }
    Ok(json!({ "identities": identities }))
}

/// What the confirm prompt says for `users set`: the list is the complete set,
/// and everyone not in it is removed. The count comes from the body about to
/// be sent, so `--data` gets it too. Pure.
pub fn users_set_prompt(path: &str, body: &Value) -> String {
    let count = match body.get("identities").and_then(Value::as_array) {
        Some(identities) => format!("{} users", identities.len()),
        None => "the --data body".to_string(),
    };
    format!(
        "{path} (replaces the application's full user list with {count}; \
         users omitted from it are removed from the application and its open review)"
    )
}

/// What the confirm prompt says for `remove`. Pure.
pub fn remove_prompt(path: &str) -> String {
    format!("{path} (deletes the application and removes it from every review period it has not completed in)")
}

pub fn example_if_requested(action: &AccessReviewAppAction) -> Option<Result<String>> {
    match action {
        AccessReviewAppAction::Create { example: true, .. } => Some(example("POST", COLLECTION)),
        AccessReviewAppAction::Update { example: true, .. } => Some(example("PUT", ITEM_TEMPLATE)),
        AccessReviewAppAction::Users {
            action: AccessReviewAppUsersAction::Set { example: true, .. },
        } => Some(example("PUT", USERS_TEMPLATE)),
        _ => None,
    }
}

#[instrument(skip_all)]
pub async fn handle(
    action: &AccessReviewAppAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        AccessReviewAppAction::List => {
            debug!(path = COLLECTION, "access-review app list");
            let items = client.get_all(COLLECTION).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        AccessReviewAppAction::Get { application_id } => {
            let path = item_path(*application_id);
            debug!(path, "access-review app get");
            print_value(&client.get(&path).await?, &config.output_format);
        }
        AccessReviewAppAction::Create {
            name,
            website_url,
            reviewer_ids,
            data,
            example: _,
        } => {
            let body = body_or_data(data.as_deref(), || {
                let name = name
                    .as_deref()
                    .ok_or_else(|| eyre::eyre!("`access-review app create` requires --name (or --data)"))?;
                Ok(build_create_body(name, website_url.as_deref(), reviewer_ids))
            })?;
            debug!(path = COLLECTION, "access-review app create");
            if !confirm("POST", COLLECTION)? {
                bail!("aborted");
            }
            print_value(&client.post(COLLECTION, body).await?, &config.output_format);
        }
        AccessReviewAppAction::Update {
            application_id,
            name,
            website_url,
            clear_website_url,
            reviewer_ids,
            data,
            example: _,
        } => {
            let path = item_path(require(*application_id, "<application_id>", "app update")?);
            let body = body_or_data(data.as_deref(), || {
                build_update_body(
                    name.as_deref(),
                    website_url.as_deref(),
                    *clear_website_url,
                    reviewer_ids,
                )
            })?;
            debug!(path, "access-review app update");
            if !confirm("PUT", &path)? {
                bail!("aborted");
            }
            print_value(&client.put(&path, body).await?, &config.output_format);
        }
        AccessReviewAppAction::Remove { application_id } => {
            let path = item_path(*application_id);
            debug!(path, "access-review app remove");
            if !confirm("DELETE", &remove_prompt(&path))? {
                bail!("aborted");
            }
            print_value(&client.delete(&path).await?, &config.output_format);
        }
        AccessReviewAppAction::Users { action } => handle_users(action, client, config, confirm).await?,
    }
    Ok(())
}

async fn handle_users(
    action: &AccessReviewAppUsersAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        AccessReviewAppUsersAction::List { application_id } => {
            let path = users_path(*application_id);
            debug!(path, "access-review app users list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        AccessReviewAppUsersAction::Set {
            application_id,
            users,
            data,
            example: _,
        } => {
            let path = users_path(require(*application_id, "<application_id>", "app users set")?);
            let body = body_or_data(data.as_deref(), || build_users_body(users))?;
            debug!(path, users = users.len(), "access-review app users set");
            if !confirm("PUT", &users_set_prompt(&path, &body))? {
                bail!("aborted");
            }
            print_value(&client.put(&path, body).await?, &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
