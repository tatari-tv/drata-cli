//! `drata personnel search`: the query for `/personnel-search` and
//! `/workspaces/{workspaceId}/personnel-search`. Pure.

use crate::cli::{ComplianceCheckStatus, PersonnelSearchArgs};
use crate::client::encode_path_segment;
use crate::query::{QueryPath, wire_name};
use eyre::{Result, bail};

/// The spec's `maxLength` on `q`, `firstName` and `lastName`.
const MAX_TEXT: usize = 191;

/// The search query and the facet wire names. Pure.
pub fn build_search(args: &PersonnelSearchArgs) -> Result<(QueryPath, Vec<String>)> {
    for (flag, value) in [
        ("--q", &args.q),
        ("--first-name", &args.first_name),
        ("--last-name", &args.last_name),
    ] {
        if value.as_ref().is_some_and(|v| v.chars().count() > MAX_TEXT) {
            bail!("{flag} takes at most {MAX_TEXT} characters");
        }
    }
    let base = match &args.workspace_id {
        Some(id) => format!("/workspaces/{}/personnel-search", encode_path_segment(id)),
        None => "/personnel-search".to_string(),
    };
    let mut q = QueryPath::new(base);
    q.push_opt("q", args.q.as_deref())
        .push_opt("firstName", args.first_name.as_deref())
        .push_opt("lastName", args.last_name.as_deref())
        .push_each("employmentStatus[]", args.employment_status.iter().map(wire_name));
    for (key, statuses) in status_filters(args) {
        q.push_each(key, statuses.iter().map(wire_name));
    }
    q.push_each("groupIds[]", &args.group_id)
        .push_opt("syncStatus", args.sync_status.as_ref().map(wire_name))
        .push_each("idpConnectionIds[]", &args.idp_connection_id)
        .push_each("hrisConnectionIds[]", &args.hris_connection_id);
    for (i, filter) in args.custom_field.iter().enumerate() {
        let (field, value) = split_filter("--custom-field", filter)?;
        q.push(&format!("customFields[{i}][{}]", field_key(field)), field)
            .push(&format!("customFields[{i}][value]"), value);
    }
    for (i, filter) in args.custom_range_field.iter().enumerate() {
        let (field, range) = split_filter("--custom-range-field", filter)?;
        let (gte, lte) = range
            .split_once("..")
            .ok_or_else(|| eyre::eyre!("--custom-range-field `{filter}`: expected <id|name>=<gte>..<lte>"))?;
        if gte.is_empty() && lte.is_empty() {
            bail!("--custom-range-field `{filter}`: give at least one of gte or lte");
        }
        q.push(&format!("customRangeFields[{i}][{}]", field_key(field)), field);
        if !gte.is_empty() {
            q.push(&format!("customRangeFields[{i}][gte]"), gte);
        }
        if !lte.is_empty() {
            q.push(&format!("customRangeFields[{i}][lte]"), lte);
        }
    }
    q.push_each("expand[]", args.expand.iter().map(wire_name));
    let facets = args.facet.iter().map(wire_name).collect();
    Ok((q, facets))
}

/// Each status filter's query key with its values, in the spec's order.
fn status_filters(args: &PersonnelSearchArgs) -> [(&'static str, &Vec<ComplianceCheckStatus>); 14] {
    [
        ("complianceStatus[]", &args.compliance_status),
        ("acceptedPoliciesStatus[]", &args.accepted_policies_status),
        ("bgCheckStatus[]", &args.bg_check_status),
        ("identityMfaStatus[]", &args.identity_mfa_status),
        ("securityTrainingStatus[]", &args.security_training_status),
        ("hipaaTrainingStatus[]", &args.hipaa_training_status),
        ("aiAwarenessTrainingStatus[]", &args.ai_awareness_training_status),
        ("offboardingStatus[]", &args.offboarding_status),
        ("agentInstalledStatus[]", &args.agent_installed_status),
        ("passwordManagerStatus[]", &args.password_manager_status),
        ("diskEncryptionStatus[]", &args.disk_encryption_status),
        ("antivirusStatus[]", &args.antivirus_status),
        ("autoUpdatesStatus[]", &args.auto_updates_status),
        ("lockScreenStatus[]", &args.lock_screen_status),
    ]
}

/// `<id|name>=<rest>` split at the first `=`. Pure.
fn split_filter<'a>(flag: &str, filter: &'a str) -> Result<(&'a str, &'a str)> {
    match filter.split_once('=') {
        Some((field, rest)) if !field.is_empty() => Ok((field, rest)),
        _ => bail!("{flag} `{filter}`: expected <id|name>=<value>"),
    }
}

/// A numeric field reference is a `customFieldId`, anything else a `name`.
fn field_key(field: &str) -> &'static str {
    if field.parse::<u64>().is_ok() { "customFieldId" } else { "name" }
}

#[cfg(test)]
mod tests;
