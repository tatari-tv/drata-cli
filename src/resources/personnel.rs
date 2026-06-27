//! `drata personnel ...` - personnel records.
//!
//! Paths: `/personnel` (list), `/personnel/{personnelId}` (get/update),
//! `/personnel/actions` (bulk action). Confirmed camelCase from spec:
//! `employmentStatus`, `startedAt`, `separatedAt`, `notHumanReason`.
//! Enum field promoted to `clap::ValueEnum`: `employmentStatus`.

use crate::cli::{EmploymentStatus, PersonnelAction};
use crate::client::DrataClient;
use crate::config::Config;
use crate::output::print_value;
use eyre::Result;
use serde_json::{Value, json};
use tracing::{debug, instrument};

pub async fn handle(action: &PersonnelAction, client: &DrataClient, config: &Config) -> Result<()> {
    match action {
        PersonnelAction::List => list(client, config).await,
        PersonnelAction::Get { personnel_id } => get(client, config, personnel_id).await,
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
async fn list(client: &DrataClient, config: &Config) -> Result<()> {
    debug!("personnel list");
    let all = client.get_all("/personnel").await?;
    let result = json!({ "data": all });
    print_value(&result, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn get(client: &DrataClient, config: &Config, personnel_id: &str) -> Result<()> {
    debug!(personnel_id, "personnel get");
    let resp = client.get(&format!("/personnel/{}", personnel_id)).await?;
    print_value(&resp, &config.output_format);
    Ok(())
}

#[instrument(skip(client, config))]
async fn update(
    client: &DrataClient,
    config: &Config,
    personnel_id: &str,
    employment_status: Option<&EmploymentStatus>,
    started_at: Option<&str>,
    separated_at: Option<&str>,
    not_human_reason: Option<&str>,
) -> Result<()> {
    debug!(personnel_id, "personnel update");
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
