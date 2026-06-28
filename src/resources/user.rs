//! `drata user ...` - users and roles (read-only).
//!
//! Covers GET /users, GET /users/{userId}, GET /roles, GET /roles/{roleId},
//! GET /roles/{roleId}/users. All read-only; no create/update/delete curated here.
//! Confirmed camelCase fields from spec: `firstName`, `lastName`, `jobTitle`,
//! `createdAt`, `drataTermsAgreedAt`.

use crate::cli::UserAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use eyre::Result;
use serde_json::json;
use tracing::{debug, instrument};

pub async fn handle(action: &UserAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        UserAction::List { all } => list(client, config, *all).await,
        UserAction::Get { user_id } => get(client, config, user_id).await,
        UserAction::Roles => roles(client, config).await,
        UserAction::Role { role_id } => role(client, config, role_id).await,
        UserAction::RoleUsers { role_id, all } => role_users(client, config, role_id, *all).await,
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, all: bool) -> Result<()> {
    debug!(all, "user list");
    if all {
        let mut stdout = std::io::stdout();
        client.stream_all("/users", &mut stdout).await?;
    } else {
        let items = client.get_all("/users").await?;
        let result = json!({ "data": items });
        print_value(&result, &config.output_format);
    }
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, user_id: &str) -> Result<()> {
    debug!(user_id, "user get");
    let resp = client.get(&format!("/users/{}", user_id)).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn roles(client: &DrataClient, config: &Config) -> Result<()> {
    debug!("user roles list");
    let items = client.get_all("/roles").await?;
    let result = json!({ "data": items });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn role(client: &DrataClient, config: &Config, role_id: &str) -> Result<()> {
    debug!(role_id, "user role get");
    let resp = client.get(&format!("/roles/{}", role_id)).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn role_users(client: &DrataClient, config: &Config, role_id: &str, all: bool) -> Result<()> {
    debug!(role_id, all, "user role-users list");
    if all {
        let mut stdout = std::io::stdout();
        client
            .stream_all(&format!("/roles/{}/users", role_id), &mut stdout)
            .await?;
    } else {
        let items = client.get_all(&format!("/roles/{}/users", role_id)).await?;
        let result = json!({ "data": items });
        print_value(&result, &config.output_format);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
