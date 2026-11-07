//! `drata safebase request list`.

use super::{Reply, reply_from};
use crate::cli::{SafebaseRequestAction, SafebaseRequestStatus};
use crate::client::DrataClient;
use crate::output::Table;
use crate::query::{QueryPath, wire_name};
use eyre::Result;
use tracing::{debug, instrument};

#[instrument(skip(client))]
pub async fn fetch(action: &SafebaseRequestAction, client: &DrataClient) -> Result<Reply> {
    match action {
        SafebaseRequestAction::List { status } => {
            let path = list_path(status.as_ref());
            debug!(path, "safebase request list");
            let resp = client.get(&path).await?;
            Ok(reply_from(&resp, Table::SafebaseRequests))
        }
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
}
