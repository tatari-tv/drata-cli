//! `drata monitor ...` - monitoring tests in a workspace.
//!
//! Paths under `/workspaces/{workspaceId}/monitoring-tests`. Confirmed camelCase
//! fields from spec: `checkResultStatus`, `checkStatus`, `lastPassedAt`,
//! `failedSince`, `disabledByUser`. Update body: `name`, `enabled`, `description`.
//!
//! Sub-commands: list/get/update/exclusions/failures/passes, tickets, and the
//! custom-test and exclusion verbs resolved by `plan`:
//!
//! | Verb | Method | Path under `.../monitoring-tests` |
//! |------|--------|-----------------------------------|
//! | `create` | POST | (the collection) |
//! | `validate-recipe` | POST | `action-validate-recipe` (saves nothing; no confirm) |
//! | `recipe` | PUT | `{testId}/recipe` |
//! | `retest` | POST | `{testId}/action-retest` (sent once: 409 and 429 are not retried) |
//! | `exclusion create` | POST | `{testId}/exclusions` (per-item results) |
//! | `exclusion remove` | POST | `{testId}/exclusions/action-delete` (per-item results) |
//! | `codebase-exclusion list` | GET | `{testId}/codebase-exclusions` |
//! | `codebase-exclusion create` | POST | `{testId}/codebase-exclusions` (per-item results) |
//! | `codebase-exclusion remove` | POST | `{testId}/codebase-exclusions/action-delete` (per-item results) |

pub mod custom;
pub mod exclusion;

use crate::cli::{
    ExampleMonitorTestIds, MonitorAction, MonitorCodebaseExclusionAction, MonitorExclusionAction, MonitorTestIds,
    MonitorTicketAction,
};
use crate::client::{DrataClient, encode_path_segment};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::output::print_value;
use crate::query::QueryPath;
use crate::resources::{bulk, ticket};
use crate::spec;
use eyre::{Result, bail};
use serde_json::{Value, json};
use tracing::{debug, instrument};

/// The spec path template of the monitoring-test collection.
const TESTS_TEMPLATE: &str = "/workspaces/{workspaceId}/monitoring-tests";

pub fn example_if_requested(action: &MonitorAction) -> Option<Result<String>> {
    // (method, path suffix, whether the body carries a recipe)
    let (method, suffix, has_recipe) = match action {
        MonitorAction::Ticket {
            action: MonitorTicketAction::Create { args, .. },
        } => return ticket::create_example(args, ticket::MONITOR_CREATE_SPEC_PATH),
        MonitorAction::Create { example: true, .. } => ("POST", "", true),
        MonitorAction::ValidateRecipe { example: true, .. } => ("POST", "/action-validate-recipe", true),
        MonitorAction::Recipe { example: true, .. } => ("PUT", "/{testId}/recipe", true),
        MonitorAction::Exclusion {
            action: MonitorExclusionAction::Create { example: true, .. },
        } => ("POST", "/{testId}/exclusions", false),
        MonitorAction::CodebaseExclusion {
            action: MonitorCodebaseExclusionAction::Create { example: true, .. },
        } => ("POST", "/{testId}/codebase-exclusions", false),
        _ => return None,
    };
    Some(body_example(method, &format!("{TESTS_TEMPLATE}{suffix}"), has_recipe))
}

/// The op's body skeleton from the spec, with a full recipe spliced in when
/// the body carries one.
fn body_example(method: &str, path: &str, has_recipe: bool) -> Result<String> {
    let skeleton = spec::example_for_operation(method, path)?
        .ok_or_else(|| eyre::eyre!("operation `{} {}` has no JSON request body", method, path))?;
    if !has_recipe {
        return Ok(skeleton);
    }
    let mut body: Value = serde_json::from_str(&skeleton)?;
    body["recipe"] = recipe_example()?;
    Ok(format!("{}\n", serde_json::to_string_pretty(&body)?))
}

/// A recipe skeleton assembled from its component schemas: the generic
/// skeleton stops at its depth bound before a provider's resources, which hold
/// the test's conditions.
pub fn recipe_example() -> Result<Value> {
    let resource = spec::example_for_schema("CustomTestRecipeResourcePublicV2Dto")?;
    let mut provider = spec::example_for_schema("CustomTestRecipeProviderPublicV2Dto")?;
    provider["resources"] = json!([resource]);
    let mut recipe = spec::example_for_schema("CustomTestRecipePublicV2Dto")?;
    recipe["providers"] = json!([provider]);
    Ok(recipe)
}

/// `/workspaces/{ws}/monitoring-tests`, each id one encoded segment. Pure.
pub fn tests_path(workspace_id: &str) -> String {
    format!("/workspaces/{}/monitoring-tests", encode_path_segment(workspace_id))
}

/// `/workspaces/{ws}/monitoring-tests/{test}/<segment>`. Pure.
pub fn test_path(workspace_id: &str, test_id: &str, segment: &str) -> String {
    format!(
        "{}/{}/{}",
        tests_path(workspace_id),
        encode_path_segment(test_id),
        segment
    )
}

fn ids_path(ids: &MonitorTestIds, segment: &str) -> String {
    test_path(&ids.workspace_id, &ids.test_id, segment)
}

/// Unwrap the clap-optional ids; they are optional only so `--example` runs.
fn required_ids(ids: &ExampleMonitorTestIds, verb: &str) -> Result<(String, String)> {
    match (&ids.workspace_id, &ids.test_id) {
        (Some(w), Some(t)) => Ok((w.clone(), t.clone())),
        _ => bail!("`monitor {verb}` requires <workspace_id> <test_id>; use --example to print the body skeleton"),
    }
}

fn required_workspace(workspace_id: &Option<String>, verb: &str) -> Result<String> {
    workspace_id.clone().ok_or_else(|| {
        eyre::eyre!("`monitor {verb}` requires <workspace_id>; use --example to print the body skeleton")
    })
}

/// `--data` when given, else the flag-built body.
fn body_or_data(data: Option<&str>, build: impl FnOnce() -> Result<Value>) -> Result<Value> {
    match data {
        Some(raw) => read_data(raw, DataSource::Any),
        None => build(),
    }
}

/// Read `--recipe` (inline, @file or -) when given.
fn read_recipe(recipe: Option<&str>) -> Result<Option<Value>> {
    recipe
        .map(|r| read_data(r, DataSource::Any).and_then(custom::check_recipe))
        .transpose()
}

/// One custom-test or exclusion verb resolved to its request.
#[derive(Debug)]
pub enum Call {
    Get {
        path: String,
    },
    /// A confirmed JSON POST whose response prints whole.
    Post {
        path: String,
        body: Value,
    },
    /// `validate-recipe`: a POST that saves nothing, so no confirm; exits 1 when
    /// the recipe is invalid.
    Validate {
        path: String,
        body: Value,
    },
    Put {
        path: String,
        body: Value,
    },
    /// `retest`: a bodiless POST sent once (`Retry429::Never`).
    Retest {
        path: String,
        test_id: String,
    },
    /// A confirmed POST answered with per-item results.
    Bulk {
        path: String,
        body: Value,
    },
}

impl Call {
    pub fn method(&self) -> &'static str {
        match self {
            Call::Get { .. } => "GET",
            Call::Put { .. } => "PUT",
            Call::Post { .. } | Call::Validate { .. } | Call::Retest { .. } | Call::Bulk { .. } => "POST",
        }
    }

    pub fn path(&self) -> &str {
        match self {
            Call::Get { path }
            | Call::Post { path, .. }
            | Call::Validate { path, .. }
            | Call::Put { path, .. }
            | Call::Retest { path, .. }
            | Call::Bulk { path, .. } => path,
        }
    }
}

/// Resolve a custom-test or exclusion verb to its request. Reads `--recipe`
/// and `--data` (file or stdin) but sends nothing. `None` for the verbs
/// `handle` runs directly.
pub fn plan(action: &MonitorAction) -> Result<Option<Call>> {
    use MonitorAction as M;
    Ok(Some(match action {
        M::Create {
            workspace_id,
            name,
            description,
            connection_id,
            recipe,
            data,
            example: _,
        } => Call::Post {
            path: tests_path(&required_workspace(workspace_id, "create")?),
            body: body_or_data(data.as_deref(), || {
                custom::build_create_body(
                    name.as_deref(),
                    description.as_deref(),
                    connection_id,
                    read_recipe(recipe.as_deref())?,
                )
            })?,
        },
        M::ValidateRecipe {
            workspace_id,
            recipe,
            data,
            example: _,
        } => Call::Validate {
            path: format!(
                "{}/action-validate-recipe",
                tests_path(&required_workspace(workspace_id, "validate-recipe")?)
            ),
            body: body_or_data(data.as_deref(), || {
                custom::build_validate_body(read_recipe(recipe.as_deref())?)
            })?,
        },
        M::Recipe {
            ids,
            connection_id,
            recipe,
            data,
            example: _,
        } => {
            let (w, t) = required_ids(ids, "recipe")?;
            Call::Put {
                path: test_path(&w, &t, "recipe"),
                body: body_or_data(data.as_deref(), || {
                    custom::build_recipe_body(connection_id, read_recipe(recipe.as_deref())?)
                })?,
            }
        }
        M::Retest(ids) => Call::Retest {
            path: ids_path(ids, "action-retest"),
            test_id: ids.test_id.clone(),
        },
        M::Exclusion { action } => match action {
            MonitorExclusionAction::Create {
                ids,
                reason,
                failures,
                data,
                example: _,
            } => {
                let (w, t) = required_ids(ids, "exclusion create")?;
                Call::Bulk {
                    path: test_path(&w, &t, "exclusions"),
                    body: body_or_data(data.as_deref(), || {
                        exclusion::build_create_body(reason.as_deref(), failures)
                    })?,
                }
            }
            MonitorExclusionAction::Remove { ids, exclusion_id } => Call::Bulk {
                path: ids_path(ids, "exclusions/action-delete"),
                body: exclusion::build_delete_body(exclusion_id)?,
            },
        },
        M::CodebaseExclusion { action } => match action {
            MonitorCodebaseExclusionAction::List {
                ids,
                repository_name,
                expand,
            } => Call::Get {
                path: exclusion::build_codebase_list_path(
                    &ids_path(ids, "codebase-exclusions"),
                    repository_name.as_deref(),
                    expand,
                ),
            },
            MonitorCodebaseExclusionAction::Create {
                ids,
                reason,
                finding_id,
                data,
                example: _,
            } => {
                let (w, t) = required_ids(ids, "codebase-exclusion create")?;
                Call::Bulk {
                    path: test_path(&w, &t, "codebase-exclusions"),
                    body: body_or_data(data.as_deref(), || {
                        exclusion::build_codebase_create_body(reason.as_deref(), finding_id)
                    })?,
                }
            }
            MonitorCodebaseExclusionAction::Remove { ids, exclusion_id } => Call::Bulk {
                path: ids_path(ids, "codebase-exclusions/action-delete"),
                body: exclusion::build_delete_body(exclusion_id)?,
            },
        },
        _ => return Ok(None),
    }))
}

/// Run a planned call: confirm a mutation, send it, print the response, and
/// apply the verb's exit rule (invalid recipe, failed item, retest 409/429).
#[instrument(skip_all)]
async fn run(call: Call, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    let (method, path) = (call.method(), call.path().to_string());
    debug!(method, path, "monitor");
    let needs_confirm = !matches!(call, Call::Get { .. } | Call::Validate { .. });
    if needs_confirm && !confirm(method, &path)? {
        bail!("aborted");
    }
    match call {
        Call::Get { path } => {
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        Call::Post { path, body } => print_value(&client.post(&path, body).await?, &config.output_format),
        Call::Put { path, body } => print_value(&client.put(&path, body).await?, &config.output_format),
        Call::Validate { path, body } => {
            let result = client.post(&path, body).await?;
            print_value(&result, &config.output_format);
            custom::check_validation(&result)?;
        }
        Call::Retest { path, test_id } => {
            let result = client
                .post_once(&path)
                .await
                .map_err(|e| custom::retest_error(e, &test_id))?;
            print_value(&result, &config.output_format);
        }
        Call::Bulk { path, body } => bulk::post(client, config, &path, body, bulk::STATUS_CODE).await?,
    }
    debug!(method, path, "monitor complete");
    Ok(())
}

pub async fn handle(action: &MonitorAction, client: &DrataClient, config: &Config, confirm: &ConfirmFn) -> Result<()> {
    match action {
        MonitorAction::List {
            workspace_id,
            all,
            draft,
            name,
        } => {
            let path = build_list_path(workspace_id, *draft, name.as_deref());
            list(client, config, &path, *all).await
        }
        MonitorAction::Get { workspace_id, test_id } => get(client, config, workspace_id, test_id).await,
        MonitorAction::Update {
            workspace_id,
            test_id,
            name,
            enabled,
            description,
        } => {
            update(
                client,
                config,
                confirm,
                workspace_id,
                test_id,
                name.as_deref(),
                *enabled,
                description.as_deref(),
            )
            .await
        }
        MonitorAction::Exclusions { workspace_id, test_id } => exclusions(client, config, workspace_id, test_id).await,
        MonitorAction::Failures { workspace_id, test_id } => failures(client, config, workspace_id, test_id).await,
        MonitorAction::Passes { workspace_id, test_id } => passes(client, config, workspace_id, test_id).await,
        MonitorAction::Ticket { action } => {
            let (tickets, op) = ticket::monitor_target(action)?;
            ticket::handle(client, config, confirm, &tickets, op).await
        }
        MonitorAction::Create { .. }
        | MonitorAction::ValidateRecipe { .. }
        | MonitorAction::Recipe { .. }
        | MonitorAction::Retest(_)
        | MonitorAction::Exclusion { .. }
        | MonitorAction::CodebaseExclusion { .. } => match plan(action)? {
            Some(call) => run(call, client, config, confirm).await,
            None => bail!("internal: monitor verb has no plan"),
        },
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

/// Build the `GET .../monitoring-tests` path with the `draft` and `name`
/// filters. Pure.
fn build_list_path(workspace_id: &str, draft: Option<bool>, name: Option<&str>) -> String {
    let mut q = QueryPath::new(format!("/workspaces/{}/monitoring-tests", workspace_id));
    q.push_opt("draft", draft).push_opt("name", name);
    q.finish()
}

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, path: &str, all: bool) -> Result<()> {
    debug!(path, all, "monitor list");
    if all {
        let mut stdout = std::io::stdout();
        client.stream_all(path, &mut stdout).await?;
    } else {
        let items = client.get_all(path).await?;
        let result = json!({ "data": items });
        print_value(&result, &config.output_format);
    }
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, workspace_id: &str, test_id: &str) -> Result<()> {
    debug!(workspace_id, test_id, "monitor get");
    let resp = client
        .get(&format!("/workspaces/{}/monitoring-tests/{}", workspace_id, test_id))
        .await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config, confirm))]
async fn update(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    workspace_id: &str,
    test_id: &str,
    name: Option<&str>,
    enabled: Option<bool>,
    description: Option<&str>,
) -> Result<()> {
    debug!(workspace_id, test_id, "monitor update");
    let path = format!("/workspaces/{}/monitoring-tests/{}", workspace_id, test_id);
    if !confirm("PUT", &path)? {
        bail!("aborted");
    }
    let mut body = json!({});
    if let Some(v) = name {
        body["name"] = json!(v);
    }
    if let Some(v) = enabled {
        body["enabled"] = json!(v);
    }
    if let Some(v) = description {
        body["description"] = json!(v);
    }
    let result = client.put(&path, body).await?;
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn exclusions(client: &DrataClient, config: &Config, workspace_id: &str, test_id: &str) -> Result<()> {
    debug!(workspace_id, test_id, "monitor exclusions");
    let items = client
        .get_all(&format!(
            "/workspaces/{}/monitoring-tests/{}/exclusions",
            workspace_id, test_id
        ))
        .await?;
    let result = json!({ "data": items });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn failures(client: &DrataClient, config: &Config, workspace_id: &str, test_id: &str) -> Result<()> {
    debug!(workspace_id, test_id, "monitor failures");
    let items = client
        .get_all(&format!(
            "/workspaces/{}/monitoring-tests/{}/failures",
            workspace_id, test_id
        ))
        .await?;
    let result = json!({ "data": items });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn passes(client: &DrataClient, config: &Config, workspace_id: &str, test_id: &str) -> Result<()> {
    debug!(workspace_id, test_id, "monitor passes");
    let items = client
        .get_all(&format!(
            "/workspaces/{}/monitoring-tests/{}/passes",
            workspace_id, test_id
        ))
        .await?;
    let result = json!({ "data": items });
    print_value(&result, &config.output_format);
    Ok(())
}

#[cfg(test)]
mod tests;
