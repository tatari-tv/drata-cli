//! `drata access-review period ...`: review periods, the applications scoped
//! into one, and the period's evidence package. Only one period may be active
//! on the account at a time; `create`'s confirm prompt says so. The evidence
//! package is a short-lived signed URL in a JSON body, printed and never
//! downloaded.

use super::{body_or_data, example, require};
use crate::cli::{
    AccessReviewActionKind, AccessReviewPeriodAction, AccessReviewPeriodAppAction, AccessReviewPeriodStatus,
    PeriodAppRef, ReviewedApplication,
};
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::output::print_value;
use crate::query::{QueryPath, wire_name};
use eyre::{Result, bail, eyre};
use serde_json::{Map, Value, json};
use std::collections::HashSet;
use tracing::{debug, instrument};

const COLLECTION: &str = "/access-review-periods";
const ITEM_TEMPLATE: &str = "/access-review-periods/{periodId}";
const APP_TEMPLATE: &str = "/access-review-periods/{periodId}/applications/{reviewAppId}";

/// `/access-review-periods/{p}`. Pure.
pub fn item_path(period_id: u64) -> String {
    format!("{COLLECTION}/{period_id}")
}

/// `/access-review-periods/{p}/actions`. Pure.
pub fn actions_path(period_id: u64) -> String {
    format!("{}/actions", item_path(period_id))
}

/// `/access-review-periods/{p}/evidence-package`. Pure.
pub fn evidence_package_path(period_id: u64) -> String {
    format!("{}/evidence-package", item_path(period_id))
}

/// `/access-review-periods/{p}/applications/{r}`. Pure.
pub fn app_path(target: &PeriodAppRef) -> String {
    format!("{}/applications/{}", item_path(target.period_id), target.review_app_id)
}

/// `/access-review-periods/{p}/applications/{r}/actions`. Pure.
pub fn app_actions_path(target: &PeriodAppRef) -> String {
    format!("{}/actions", app_path(target))
}

/// `/access-review-periods/{p}/applications/{r}/users`. Pure.
pub fn app_users_path(target: &PeriodAppRef) -> String {
    format!("{}/users", app_path(target))
}

/// `GET` the period list, filtered by status. Pure.
pub fn list_path(status: Option<&AccessReviewPeriodStatus>) -> String {
    let mut q = QueryPath::new(COLLECTION);
    q.push_opt("status", status.map(wire_name));
    q.finish()
}

/// The `create` body: the dates and one entry per `--application`, each with
/// its reviewers. An application given twice is rejected: it would be scoped
/// in twice with competing reviewers. Pure.
pub fn build_create_body(
    starting_date: &str,
    ending_date: &str,
    applications: &[ReviewedApplication],
) -> Result<Value> {
    let mut seen = HashSet::new();
    if let Some(dup) = applications.iter().find(|a| !seen.insert(a.application_id)) {
        bail!("application {} is given twice in --application", dup.application_id);
    }
    let applications: Vec<Value> = applications
        .iter()
        .map(|a| json!({ "applicationId": a.application_id, "reviewersIds": a.reviewer_ids }))
        .collect();
    Ok(json!({
        "startingDate": starting_date,
        "endingDate": ending_date,
        "applications": applications,
    }))
}

/// The period `run-action` body (`POST .../{p}/actions`). Pure.
pub fn build_action_body(action: &AccessReviewActionKind) -> Value {
    json!({ "action": wire_name(action) })
}

/// The application `run-action` body (`POST .../applications/{r}/actions`):
/// `markUnreviewedUsersOutOfScope` is sent only when the flag is set; leaving
/// it out is the spec's "reject with 409 while unreviewed users remain". Pure.
pub fn build_app_action_body(action: &AccessReviewActionKind, mark_unreviewed_out_of_scope: bool) -> Value {
    let mut body = Map::new();
    body.insert("action".to_string(), json!(wire_name(action)));
    if mark_unreviewed_out_of_scope {
        body.insert("markUnreviewedUsersOutOfScope".to_string(), json!(true));
    }
    Value::Object(body)
}

/// The application `update` body: the reviewers that replace the current ones. Pure.
pub fn build_app_update_body(reviewer_ids: &[u64]) -> Value {
    json!({ "reviewersIds": reviewer_ids })
}

/// What the confirm prompt says for `create`. Pure.
pub fn create_prompt() -> String {
    format!("{COLLECTION} (starts a review period; only one may be active on the account at a time)")
}

/// What the confirm prompt says for the application `run-action`: with
/// `markUnreviewedUsersOutOfScope` in the body, every unreviewed user is swept
/// to OUT_OF_SCOPE as the calling key's user. Read from the body about to be
/// sent, so `--data` gets it too. Pure.
pub fn app_action_prompt(path: &str, body: &Value) -> String {
    if body.get("markUnreviewedUsersOutOfScope") == Some(&Value::Bool(true)) {
        format!("{path} (marks every NOT_REVIEWED user OUT_OF_SCOPE, recorded as this API key's user)")
    } else {
        path.to_string()
    }
}

pub fn example_if_requested(action: &AccessReviewPeriodAction) -> Option<Result<String>> {
    match action {
        AccessReviewPeriodAction::Create { example: true, .. } => Some(example("POST", COLLECTION)),
        AccessReviewPeriodAction::RunAction { example: true, .. } => {
            Some(example("POST", &format!("{ITEM_TEMPLATE}/actions")))
        }
        AccessReviewPeriodAction::App {
            action: AccessReviewPeriodAppAction::Update { example: true, .. },
        } => Some(example("PUT", APP_TEMPLATE)),
        AccessReviewPeriodAction::App {
            action: AccessReviewPeriodAppAction::RunAction { example: true, .. },
        } => Some(example("POST", &format!("{APP_TEMPLATE}/actions"))),
        _ => None,
    }
}

/// Resolve the optional `--example` ids into a `PeriodAppRef`.
fn period_app_ref(period_id: Option<u64>, review_app_id: Option<u64>, verb: &str) -> Result<PeriodAppRef> {
    Ok(PeriodAppRef {
        period_id: require(period_id, "<period_id>", verb)?,
        review_app_id: require(review_app_id, "<review_app_id>", verb)?,
    })
}

fn required_action<'a>(action: Option<&'a AccessReviewActionKind>, verb: &str) -> Result<&'a AccessReviewActionKind> {
    action.ok_or_else(|| eyre!("`access-review {verb}` requires --action (or --data/--example)"))
}

#[instrument(skip_all)]
pub async fn handle(
    action: &AccessReviewPeriodAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        AccessReviewPeriodAction::List { status } => {
            let path = list_path(status.as_ref());
            debug!(path, "access-review period list");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
        AccessReviewPeriodAction::Get { period_id } => {
            let path = item_path(*period_id);
            debug!(path, "access-review period get");
            print_value(&client.get(&path).await?, &config.output_format);
        }
        AccessReviewPeriodAction::Create {
            starting_date,
            ending_date,
            applications,
            data,
            example: _,
        } => {
            let body = body_or_data(data.as_deref(), || {
                let (Some(start), Some(end)) = (starting_date.as_deref(), ending_date.as_deref()) else {
                    bail!("`access-review period create` requires --starting-date and --ending-date (or --data)");
                };
                build_create_body(start, end, applications)
            })?;
            debug!(path = COLLECTION, "access-review period create");
            if !confirm("POST", &create_prompt())? {
                bail!("aborted");
            }
            print_value(&client.post(COLLECTION, body).await?, &config.output_format);
        }
        AccessReviewPeriodAction::RunAction {
            target,
            action,
            data,
            example: _,
        } => {
            let path = actions_path(require(target.period_id, "<period_id>", "period run-action")?);
            let body = body_or_data(data.as_deref(), || {
                Ok(build_action_body(required_action(
                    action.as_ref(),
                    "period run-action",
                )?))
            })?;
            debug!(path, "access-review period run-action");
            if !confirm("POST", &path)? {
                bail!("aborted");
            }
            print_value(&client.post(&path, body).await?, &config.output_format);
        }
        AccessReviewPeriodAction::App { action } => handle_app(action, client, config, confirm).await?,
        AccessReviewPeriodAction::EvidencePackage { period_id } => {
            let path = evidence_package_path(*period_id);
            debug!(path, "access-review period evidence-package");
            print_value(&client.get(&path).await?, &config.output_format);
        }
    }
    Ok(())
}

async fn handle_app(
    action: &AccessReviewPeriodAppAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        AccessReviewPeriodAppAction::Get(target) => {
            let path = app_path(target);
            debug!(path, "access-review period app get");
            print_value(&client.get(&path).await?, &config.output_format);
        }
        AccessReviewPeriodAppAction::Update {
            target,
            reviewer_ids,
            data,
            example: _,
        } => {
            let target = period_app_ref(target.period_id, target.review_app_id, "period app update")?;
            let path = app_path(&target);
            let body = body_or_data(data.as_deref(), || Ok(build_app_update_body(reviewer_ids)))?;
            debug!(path, "access-review period app update");
            if !confirm("PUT", &path)? {
                bail!("aborted");
            }
            print_value(&client.put(&path, body).await?, &config.output_format);
        }
        AccessReviewPeriodAppAction::RunAction {
            target,
            action,
            mark_unreviewed_out_of_scope,
            data,
            example: _,
        } => {
            let target = period_app_ref(target.period_id, target.review_app_id, "period app run-action")?;
            let path = app_actions_path(&target);
            let body = body_or_data(data.as_deref(), || {
                let action = required_action(action.as_ref(), "period app run-action")?;
                Ok(build_app_action_body(action, *mark_unreviewed_out_of_scope))
            })?;
            debug!(path, "access-review period app run-action");
            if !confirm("POST", &app_action_prompt(&path, &body))? {
                bail!("aborted");
            }
            print_value(&client.post(&path, body).await?, &config.output_format);
        }
        AccessReviewPeriodAppAction::Users(target) => {
            let path = app_users_path(target);
            debug!(path, "access-review period app users");
            let items = client.get_all(&path).await?;
            print_value(&json!({ "data": items }), &config.output_format);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
