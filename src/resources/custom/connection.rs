//! Custom Connection resource attribute path. Pure.

use crate::client::encode_path_segment;

/// `/workspaces/{w}/custom-connections/{c}/resources/{r}/attributes`, each id
/// one encoded segment: `connection_id` and `resource_id` may be
/// `name:<clientAlias or resource name>`, and a name can hold a space or slash.
pub fn attributes_path(workspace_id: &str, connection_id: &str, resource_id: &str) -> String {
    format!(
        "/workspaces/{}/custom-connections/{}/resources/{}/attributes",
        encode_path_segment(workspace_id),
        encode_path_segment(connection_id),
        encode_path_segment(resource_id)
    )
}

#[cfg(test)]
mod tests;
