//! Custom monitoring tests: create, recipe validate and replace, and retest.
//!
//! Every builder here is pure: the handler in `monitor.rs` reads `--recipe` and
//! `--data`, calls these, confirms, and sends.

use crate::client::ApiError;
use eyre::{Result, bail};
use reqwest::StatusCode;
use serde_json::{Value, json};

/// The spec's `maxItems` on `connectionIds`.
pub const MAX_CONNECTION_IDS: usize = 300;

/// A `--recipe` value must be a JSON object (`CustomTestRecipePublicV2Dto`).
pub fn check_recipe(recipe: Value) -> Result<Value> {
    if !recipe.is_object() {
        bail!("--recipe must be a JSON object (see --example), got {}", recipe);
    }
    Ok(recipe)
}

fn check_connection_ids(connection_ids: &[u64]) -> Result<()> {
    if connection_ids.len() > MAX_CONNECTION_IDS {
        bail!(
            "--connection-id takes {MAX_CONNECTION_IDS} IDs at most, got {}",
            connection_ids.len()
        );
    }
    Ok(())
}

fn require_recipe(recipe: Option<Value>) -> Result<Value> {
    recipe.ok_or_else(|| eyre::eyre!("requires --recipe (or use --data/--example)"))
}

/// `POST .../monitoring-tests` body. `name` and `recipe` are required by the
/// spec; `description` and `connectionIds` are sent only when given.
pub fn build_create_body(
    name: Option<&str>,
    description: Option<&str>,
    connection_ids: &[u64],
    recipe: Option<Value>,
) -> Result<Value> {
    let name = name.ok_or_else(|| eyre::eyre!("requires --name (or use --data/--example)"))?;
    check_connection_ids(connection_ids)?;
    let mut body = json!({ "name": name, "recipe": require_recipe(recipe)? });
    if let Some(d) = description {
        body["description"] = json!(d);
    }
    if !connection_ids.is_empty() {
        body["connectionIds"] = json!(connection_ids);
    }
    Ok(body)
}

/// `POST .../action-validate-recipe` body: `{"recipe": ...}`.
pub fn build_validate_body(recipe: Option<Value>) -> Result<Value> {
    Ok(json!({ "recipe": require_recipe(recipe)? }))
}

/// `PUT .../{testId}/recipe` body: the replacement `recipe`, plus
/// `connectionIds` when given.
pub fn build_recipe_body(connection_ids: &[u64], recipe: Option<Value>) -> Result<Value> {
    check_connection_ids(connection_ids)?;
    let mut body = json!({ "recipe": require_recipe(recipe)? });
    if !connection_ids.is_empty() {
        body["connectionIds"] = json!(connection_ids);
    }
    Ok(body)
}

/// The validate op answers 200 whether or not the recipe is valid; an invalid
/// recipe fails the command after its errors print. A body without a boolean
/// `isValid` is an error, never a pass.
pub fn check_validation(value: &Value) -> Result<()> {
    match value.get("isValid").and_then(Value::as_bool) {
        Some(true) => Ok(()),
        Some(false) => {
            let n = value.get("errors").and_then(Value::as_array).map_or(0, Vec::len);
            bail!("the recipe is invalid: {n} error(s)")
        }
        None => bail!(
            "expected a boolean `isValid` in the validation response, got: {}",
            value
        ),
    }
}

/// Name a retest's 409 (a run is already in flight) and 429 (the cooldown after
/// a completed run) apart. The wrapped `ApiError` still prints the server's
/// message; any other error passes through unchanged.
pub fn retest_error(err: eyre::Report, test_id: &str) -> eyre::Report {
    let status = err.downcast_ref::<ApiError>().map(|api| api.status);
    match status {
        Some(StatusCode::CONFLICT) => err.wrap_err(format!(
            "monitoring test {test_id} already has a run in flight; \
             wait until `monitor get` shows checkStatus leave TESTING"
        )),
        Some(StatusCode::TOO_MANY_REQUESTS) => err.wrap_err(format!(
            "monitoring test {test_id} is in the cooldown that follows a completed run; retest later"
        )),
        _ => err,
    }
}
