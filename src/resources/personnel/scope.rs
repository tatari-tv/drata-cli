//! `drata personnel group ...`: a workspace's personnel scope (beta).
//!
//! | Verb | Method | Path |
//! |------|--------|------|
//! | `list` | GET | `/workspaces/{w}/scoped-personnel-groups` |
//! | `counts` | GET | `/workspaces/{w}/scoped-personnel-counts` |
//! | `add` | POST | `/workspaces/{w}/scoped-personnel-groups` |
//! | `replace` | PUT | `/workspaces/{w}/scoped-personnel-groups` |
//! | `remove` | DELETE | `/workspaces/{w}/scoped-personnel-groups/{groupId}` |

use crate::cli::PersonnelGroupAction;
use crate::client::{DrataClient, encode_path_segment};
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::output::print_value;
use eyre::{Result, bail};
use serde_json::{Value, json};
use tracing::{debug, instrument};

/// `/workspaces/{w}/scoped-personnel-groups`. Pure.
pub fn groups_path(workspace_id: &str) -> String {
    format!(
        "/workspaces/{}/scoped-personnel-groups",
        encode_path_segment(workspace_id)
    )
}

/// `/workspaces/{w}/scoped-personnel-counts`. Pure.
pub fn counts_path(workspace_id: &str) -> String {
    format!(
        "/workspaces/{}/scoped-personnel-counts",
        encode_path_segment(workspace_id)
    )
}

/// `/workspaces/{w}/scoped-personnel-groups/{groupId}`. Pure.
pub fn group_path(workspace_id: &str, group_id: u64) -> String {
    format!("{}/{group_id}", groups_path(workspace_id))
}

/// `POST` body: one group to add. Pure.
pub fn add_body(group_id: u64) -> Value {
    json!({ "groupId": group_id })
}

/// `PUT` body: the full set the scope holds afterwards. Pure.
pub fn replace_body(group_ids: &[u64]) -> Value {
    json!({ "groupIds": group_ids })
}

/// What the confirm prompt says about a `PUT`: the server removes every group
/// the body omits, so the prompt names that. Pure.
pub fn replace_prompt(path: &str, group_ids: &[u64]) -> String {
    let listed = if group_ids.is_empty() {
        "no groups, so the scope is cleared".to_string()
    } else {
        format!(
            "exactly groups {}",
            group_ids.iter().map(u64::to_string).collect::<Vec<_>>().join(", ")
        )
    };
    format!("{path} (replaces the whole scope with {listed}; groups not listed are removed)")
}

#[instrument(skip_all)]
pub async fn handle(
    action: &PersonnelGroupAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    use PersonnelGroupAction as G;
    match action {
        G::List { workspace_id } => {
            let items = client.get_all(&groups_path(workspace_id)).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        G::Counts { workspace_id } => {
            print_value(&client.get(&counts_path(workspace_id)).await?, &config.output_format);
        }
        G::Add { workspace_id, group_id } => {
            let path = groups_path(workspace_id);
            debug!(path, group_id, "personnel group add");
            if !confirm("POST", &path)? {
                bail!("aborted");
            }
            let result = client.post(&path, add_body(*group_id)).await?;
            print_value(&result, &config.output_format);
        }
        G::Replace {
            workspace_id,
            group_id,
            clear: _,
        } => {
            let path = groups_path(workspace_id);
            debug!(path, groups = group_id.len(), "personnel group replace");
            if !confirm("PUT", &replace_prompt(&path, group_id))? {
                bail!("aborted");
            }
            let result = client.put(&path, replace_body(group_id)).await?;
            print_value(&result, &config.output_format);
        }
        G::Remove { workspace_id, group_id } => {
            let path = group_path(workspace_id, *group_id);
            debug!(path, "personnel group remove");
            if !confirm("DELETE", &path)? {
                bail!("aborted");
            }
            let result = client.delete(&path).await?;
            print_value(&result, &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
