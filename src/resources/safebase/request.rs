//! `drata safebase request list | approve | decline`.

use super::{Reply, confirmed, notice_message, reply_from};
use crate::cli::{EmailNotice, SafebaseRequestAction, SafebaseRequestStatus};
use crate::client::{DrataClient, encode_path_segment};
use crate::confirm::ConfirmFn;
use crate::output::Table;
use crate::query::{QueryPath, wire_name};
use eyre::Result;
use serde_json::{Value, json};
use tracing::{debug, instrument};

#[instrument(skip(client, confirm))]
pub async fn execute(action: &SafebaseRequestAction, client: &DrataClient, confirm: &ConfirmFn) -> Result<Reply> {
    match action {
        SafebaseRequestAction::List { status } => {
            let path = list_path(status.as_ref());
            debug!(path, "safebase request list");
            let resp = client.get(&path).await?;
            Ok(reply_from(&resp, Table::SafebaseRequests))
        }
        SafebaseRequestAction::Approve { request_id, account_id } => {
            let path = decision_path(request_id, "approve");
            debug!(path, "safebase request approve");
            confirmed(confirm, "POST", &path)?;
            let resp = client.post(&path, json!({ "accountId": account_id })).await?;
            Ok(reply_from(&resp, Table::SafebaseAccounts))
        }
        SafebaseRequestAction::Decline { request_id, notice } => {
            let path = decision_path(request_id, "decline");
            debug!(path, "safebase request decline");
            confirmed(confirm, "POST", &path)?;
            let resp = client.post(&path, decline_body(notice)).await?;
            Ok(reply_from(&resp, Table::SafebaseAccounts))
        }
    }
}

/// `/requests/{requestId}/<verb>`. Pure.
fn decision_path(request_id: &str, verb: &str) -> String {
    format!("/requests/{}/{verb}", encode_path_segment(request_id))
}

/// The decline body: `{}`, or `{message}` on an opt-in email. Pure.
fn decline_body(notice: &EmailNotice) -> Value {
    match notice_message(notice) {
        Some(message) => json!({ "message": message }),
        None => json!({}),
    }
}

/// `GET /requests` path with the optional `status` filter. Pure.
fn list_path(status: Option<&SafebaseRequestStatus>) -> String {
    let mut q = QueryPath::new("/requests");
    q.push_opt("status", status.map(wire_name));
    q.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_path_uses_the_spec_status_names() {
        assert_eq!(list_path(None), "/requests");
        assert_eq!(
            list_path(Some(&SafebaseRequestStatus::Pending)),
            "/requests?status=pending"
        );
        assert_eq!(
            list_path(Some(&SafebaseRequestStatus::Approved)),
            "/requests?status=approved"
        );
        assert_eq!(
            list_path(Some(&SafebaseRequestStatus::Declined)),
            "/requests?status=declined"
        );
    }

    #[test]
    fn decision_path_encodes_the_request_id() {
        assert_eq!(decision_path("r-1", "approve"), "/requests/r-1/approve");
        assert_eq!(decision_path("r/1", "decline"), "/requests/r%2F1/decline");
    }

    #[test]
    fn decline_body_carries_message_only_on_opt_in() {
        assert_eq!(decline_body(&EmailNotice::default()), json!({}));
        let text = EmailNotice {
            message: Some("not now".to_string()),
            notify: false,
        };
        assert_eq!(decline_body(&text), json!({"message": "not now"}));
    }
}
