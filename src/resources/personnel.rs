//! `drata personnel ...` - personnel records.
//!
//! Paths: `/personnel` (list), `/personnel/{personnelId}` (get/update),
//! `/personnel/actions` (bulk action). Confirmed camelCase from spec:
//! `employmentStatus`, `startedAt`, `separatedAt`, `notHumanReason`.
//! Enum field promoted to `clap::ValueEnum`: `employmentStatus`.
//!
//! Phase 4 adds: `--all` NDJSON streaming, `--expand`, confirm-on-mutation.

use crate::cli::{EmploymentStatus, PersonnelAction};
use crate::client::DrataClient;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::expand::append_expand;
use crate::output::print_value;
use eyre::{Result, bail};
use serde_json::{Value, json};
use std::io;
use tracing::{debug, instrument};

pub async fn handle(
    action: &PersonnelAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        PersonnelAction::List { all, expand } => list(client, config, *all, expand).await,
        PersonnelAction::Get { personnel_id, expand } => get(client, config, personnel_id, expand).await,
        PersonnelAction::Update {
            personnel_id,
            employment_status,
            started_at,
            separated_at,
            not_human_reason,
        } => {
            update(
                client,
                config,
                confirm,
                personnel_id,
                employment_status.as_ref(),
                started_at.as_deref(),
                separated_at.as_deref(),
                not_human_reason.as_deref(),
            )
            .await
        }
    }
}

// ---------------------------------------------------------------------------
// Verbs
// ---------------------------------------------------------------------------

#[instrument(skip(client, config))]
async fn list(client: &DrataClient, config: &Config, all: bool, expand: &[String]) -> Result<()> {
    debug!(all, expand_len = expand.len(), "personnel list");
    let base = append_expand("/personnel", expand);
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
async fn get(client: &DrataClient, config: &Config, personnel_id: &str, expand: &[String]) -> Result<()> {
    debug!(personnel_id, expand_len = expand.len(), "personnel get");
    let path = append_expand(&format!("/personnel/{}", personnel_id), expand);
    let resp = client.get(&path).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config, confirm))]
async fn update(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    personnel_id: &str,
    employment_status: Option<&EmploymentStatus>,
    started_at: Option<&str>,
    separated_at: Option<&str>,
    not_human_reason: Option<&str>,
) -> Result<()> {
    debug!(personnel_id, "personnel update");
    if !confirm("PUT", &format!("/personnel/{}", personnel_id))? {
        bail!("aborted");
    }
    let mut body = json!({});
    if let Some(es) = employment_status {
        body["employmentStatus"] = json!(employment_status_str(es));
    }
    set_opt(&mut body, "startedAt", started_at);
    set_opt(&mut body, "separatedAt", separated_at);
    set_opt(&mut body, "notHumanReason", not_human_reason);
    let result = client.put(&format!("/personnel/{}", personnel_id), body).await?;
    print_value(&result, &config.output_format);
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

fn employment_status_str(es: &EmploymentStatus) -> &'static str {
    match es {
        EmploymentStatus::CurrentEmployee => "CURRENT_EMPLOYEE",
        EmploymentStatus::FormerEmployee => "FORMER_EMPLOYEE",
        EmploymentStatus::CurrentContractor => "CURRENT_CONTRACTOR",
        EmploymentStatus::FormerContractor => "FORMER_CONTRACTOR",
        EmploymentStatus::OutOfScope => "OUT_OF_SCOPE",
        EmploymentStatus::Unknown => "UNKNOWN",
        EmploymentStatus::SpecialFormerEmployee => "SPECIAL_FORMER_EMPLOYEE",
        EmploymentStatus::SpecialFormerContractor => "SPECIAL_FORMER_CONTRACTOR",
        EmploymentStatus::FutureHire => "FUTURE_HIRE",
        EmploymentStatus::ServiceAccount => "SERVICE_ACCOUNT",
    }
}

#[cfg(test)]
mod tests;
