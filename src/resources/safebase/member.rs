//! `drata safebase member list | add | remove`.

use super::{Reply, confirmed, notice_message, reply_from};
use crate::cli::{EmailNotice, SafebaseMemberAction};
use crate::client::{DrataClient, encode_path_segment};
use crate::confirm::ConfirmFn;
use crate::output::Table;
use crate::query::QueryPath;
use eyre::Result;
use serde_json::{Value, json};
use tracing::{debug, instrument};

#[instrument(skip(client, confirm))]
pub async fn execute(action: &SafebaseMemberAction, client: &DrataClient, confirm: &ConfirmFn) -> Result<Reply> {
    match action {
        SafebaseMemberAction::List { email, sfdc_account_id } => {
            let path = list_path(email.as_deref(), sfdc_account_id.as_deref());
            debug!(path, "safebase member list");
            let resp = client.get(&path).await?;
            Ok(reply_from(&resp, Table::SafebaseMembers))
        }
        SafebaseMemberAction::Add {
            account_id,
            email,
            first_name,
            last_name,
            notice,
        } => {
            let path = members_path(account_id);
            debug!(path, "safebase member add");
            confirmed(confirm, "POST", &path)?;
            let resp = client
                .post(&path, add_body(email, first_name, last_name, notice))
                .await?;
            Ok(reply_from(&resp, Table::SafebaseMembers))
        }
        SafebaseMemberAction::Remove { account_id, email } => {
            let path = members_path(account_id);
            debug!(path, count = email.len(), "safebase member remove");
            confirmed(confirm, "DELETE", &path)?;
            let resp = client.delete(&path, Some(json!({ "members": email }))).await?;
            Ok(reply_from(&resp, Table::SafebaseMembers))
        }
    }
}

/// `/accounts/{id}/members`, the add (POST) and remove (DELETE) path. Pure.
fn members_path(account_id: &str) -> String {
    format!("/accounts/{}/members", encode_path_segment(account_id))
}

/// `AccountMemberPayloadArray` for one member with names; `message` only on
/// an opt-in email. Pure.
fn add_body(email: &str, first_name: &str, last_name: &str, notice: &EmailNotice) -> Value {
    let mut body = json!({
        "members": [{ "email": email, "firstName": first_name, "lastName": last_name }]
    });
    if let Some(message) = notice_message(notice) {
        body["message"] = message;
    }
    body
}

/// `GET /accounts/members` path with the optional `email` and `sfdcAccountId`
/// filters. Pure.
fn list_path(email: Option<&str>, sfdc_account_id: Option<&str>) -> String {
    let mut q = QueryPath::new("/accounts/members");
    q.push_opt("email", email).push_opt("sfdcAccountId", sfdc_account_id);
    q.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_path_carries_only_the_given_filters() {
        assert_eq!(list_path(None, None), "/accounts/members");
        assert_eq!(
            list_path(Some("a@b.co"), Some("001")),
            "/accounts/members?email=a@b.co&sfdcAccountId=001"
        );
        assert_eq!(list_path(None, Some("001")), "/accounts/members?sfdcAccountId=001");
    }

    #[test]
    fn members_path_encodes_the_account_id() {
        assert_eq!(members_path("a-1"), "/accounts/a-1/members");
        assert_eq!(members_path("a/b"), "/accounts/a%2Fb/members");
    }

    #[test]
    fn add_body_omits_message_without_opt_in() {
        let body = add_body("a@b.co", "Ada", "Byron", &EmailNotice::default());
        assert_eq!(
            body,
            json!({"members": [{"email": "a@b.co", "firstName": "Ada", "lastName": "Byron"}]})
        );
        let notify = EmailNotice {
            message: None,
            notify: true,
        };
        assert_eq!(add_body("a@b.co", "Ada", "Byron", &notify)["message"], json!(true));
    }
}
