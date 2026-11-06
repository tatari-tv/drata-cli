//! `drata group list`: `GET /groups`, the account's personnel groups. The
//! resolved `id` feeds `personnel search --group-id` and `personnel group add`.

use crate::cli::GroupAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use crate::query::QueryPath;
use eyre::{Result, bail};
use serde_json::json;
use tracing::instrument;

/// The spec's `maxLength` on `q`.
const MAX_NAME: usize = 191;

/// `GET /groups` with the name-substring filter `q`. Pure.
pub fn list_path(name: Option<&str>) -> Result<String> {
    if name.is_some_and(|n| n.chars().count() > MAX_NAME) {
        bail!("--name takes at most {MAX_NAME} characters");
    }
    let mut q = QueryPath::new("/groups");
    q.push_opt("q", name);
    Ok(q.finish())
}

#[instrument(skip_all)]
pub async fn handle(action: &GroupAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        GroupAction::List { name } => {
            let items = client.get_all(&list_path(name.as_deref())?).await?;
            print_value(&json!({ "data": items }), &config.output_format);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests;
