//! `drata safebase member list`.

use super::{Reply, reply_from};
use crate::cli::SafebaseMemberAction;
use crate::client::DrataClient;
use crate::output::Table;
use crate::query::QueryPath;
use eyre::Result;
use tracing::{debug, instrument};

#[instrument(skip(client))]
pub async fn fetch(action: &SafebaseMemberAction, client: &DrataClient) -> Result<Reply> {
    match action {
        SafebaseMemberAction::List { email, sfdc_account_id } => {
            let path = list_path(email.as_deref(), sfdc_account_id.as_deref());
            debug!(path, "safebase member list");
            let resp = client.get(&path).await?;
            Ok(reply_from(&resp, Table::SafebaseMembers))
        }
    }
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
}
