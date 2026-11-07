//! `drata access-review ...`: the spec's Access Reviews tag (💎, the Access
//! Review plan feature). Applications live in `access/app.rs`, review periods
//! in `access/period.rs`; this file routes and holds what both share.
//!
//! | Command | Method | Path |
//! |---------|--------|------|
//! | `app list` | GET | `/access-review-applications` |
//! | `app get` | GET | `/access-review-applications/{a}` |
//! | `app create` | POST | `/access-review-applications` |
//! | `app update` | PUT | `/access-review-applications/{a}` |
//! | `app remove` | DELETE | `/access-review-applications/{a}` |
//! | `app users list` | GET | `/access-review-applications/{a}/users` |
//! | `app users set` | PUT | `/access-review-applications/{a}/users` (replaces the full set) |
//! | `period list` | GET | `/access-review-periods` |
//! | `period get` | GET | `/access-review-periods/{p}` |
//! | `period create` | POST | `/access-review-periods` |
//! | `period run-action` | POST | `/access-review-periods/{p}/actions` |
//! | `period app get` | GET | `/access-review-periods/{p}/applications/{r}` |
//! | `period app update` | PUT | `/access-review-periods/{p}/applications/{r}` |
//! | `period app run-action` | POST | `/access-review-periods/{p}/applications/{r}/actions` |
//! | `period app users` | GET | `/access-review-periods/{p}/applications/{r}/users` |
//! | `period evidence-package` | GET | `/access-review-periods/{p}/evidence-package` (a signed URL, printed) |

pub mod app;
pub mod period;

use crate::cli::AccessReviewAction;
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::data::{DataSource, read_data};
use crate::spec;
use eyre::{Result, eyre};
use serde_json::Value;
use tracing::instrument;

/// The spec's `--example` skeleton for an op's JSON body.
fn example(method: &str, path: &str) -> Result<String> {
    spec::example_for_operation(method, path)?
        .ok_or_else(|| eyre!("operation `{method} {path}` has no JSON request body"))
}

/// `--data` when given, otherwise the body the flags build.
fn body_or_data(data: Option<&str>, build: impl FnOnce() -> Result<Value>) -> Result<Value> {
    match data {
        Some(raw) => read_data(raw, DataSource::Any),
        None => build(),
    }
}

/// An id clap leaves optional only for `--example`.
fn require<T: Copy>(value: Option<T>, what: &str, verb: &str) -> Result<T> {
    value.ok_or_else(|| eyre!("`access-review {verb}` requires {what} (or use --example)"))
}

pub fn example_if_requested(action: &AccessReviewAction) -> Option<Result<String>> {
    match action {
        AccessReviewAction::App { action } => app::example_if_requested(action),
        AccessReviewAction::Period { action } => period::example_if_requested(action),
    }
}

#[instrument(skip_all)]
pub async fn handle(
    action: &AccessReviewAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        AccessReviewAction::App { action } => app::handle(action, client, config, confirm).await,
        AccessReviewAction::Period { action } => period::handle(action, client, config, confirm).await,
    }
}
