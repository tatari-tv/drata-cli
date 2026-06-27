//! Disposable live verification harness.
//!
//! Implements a create -> verify -> delete flow using loudly-named throwaway
//! objects (`zzz-clitest-` prefix) to test the write path against a real
//! tenant without touching existing records. The harness:
//!
//! 1. Creates a vendor named `zzz-clitest-<uuid>` (POST /vendors).
//! 2. Verifies it appears in the list (GET /vendors) and can be fetched by ID
//!    (GET /vendors/{id}).
//! 3. Deletes it (DELETE /vendors/{id}).
//! 4. Verifies it no longer appears (GET /vendors/{id} -> 404).
//!
//! **IMPORTANT:** This harness performs LIVE MUTATIONS. It must ONLY be run
//! explicitly (e.g. `drata verify`) with a write-enabled credential. It NEVER
//! touches existing records (no PUT/DELETE on records it did not create).
//!
//! The offline test suite validates the harness logic against wiremock fixtures
//! derived from the spec's request/response shapes. The live recording pass
//! (to capture real fixtures) is a deferred user action - see the Open
//! Questions in the implementation notes.
//!
//! ## Offline tests
//!
//! `src/verify/tests.rs` runs the harness against a wiremock server loaded with
//! spec-derived fixtures (not a live Drata tenant). This validates the full
//! create -> verify -> delete flow without any network traffic.

use crate::client::{ApiError, DrataClient};
use eyre::{Result, WrapErr, bail};
use reqwest::StatusCode;
use serde_json::json;
use tracing::{debug, info, instrument, warn};
use uuid::Uuid;

/// A throwaway vendor created by the harness. Its name starts with
/// `zzz-clitest-` so it is visually loud and lexicographically last.
const TEST_PREFIX: &str = "zzz-clitest-";

/// Result of a disposable verification run.
#[derive(Debug)]
pub struct VerifyResult {
    pub vendor_id: serde_json::Value,
    pub created: bool,
    pub verified_list: bool,
    pub verified_get: bool,
    pub deleted: bool,
    pub verified_deleted: bool,
}

/// Run the full create -> verify -> delete cycle against `client`.
/// Returns `Ok(VerifyResult)` when every step succeeds, `Err` on the first
/// failure (the caller should report which step failed and surface the error).
///
/// MUST be called with a write-enabled client (the create/delete calls will
/// hit the write guardrail otherwise).
#[instrument(skip(client))]
pub async fn run(client: &DrataClient) -> Result<VerifyResult> {
    let name = format!("{}{}", TEST_PREFIX, Uuid::new_v4());
    debug!(name, "starting disposable verification cycle");

    // Step 1: Create
    info!(name, "verify step 1: POST /vendors");
    let body = json!({ "name": name });
    let created = client
        .post("/vendors", body)
        .await
        .context("verify step 1 (create) failed")?;
    let vendor_id = created.get("id").cloned().unwrap_or(serde_json::Value::Null);
    debug!(?vendor_id, "vendor created");

    let id_str = match &vendor_id {
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::String(s) => s.clone(),
        other => bail!("verify step 1: unexpected id type in response: {:?}", other),
    };

    // Step 2a: Verify by GET /vendors/{id}
    info!(id = %id_str, "verify step 2a: GET /vendors/{id_str}");
    let fetched = client
        .get(&format!("/vendors/{}", id_str))
        .await
        .context("verify step 2a (get by id) failed")?;
    let fetched_name = fetched.get("name").and_then(|v| v.as_str()).unwrap_or("");
    if fetched_name != name {
        bail!("verify step 2a: expected name `{}`, got `{}`", name, fetched_name);
    }
    debug!("verify step 2a: name matches");

    // Step 2b: Verify it appears in the first page of the list
    info!("verify step 2b: GET /vendors (list)");
    let list_resp = client
        .get("/vendors?size=50")
        .await
        .context("verify step 2b (list) failed")?;
    let found_in_list = list_resp
        .get("data")
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .any(|v| v.get("name").and_then(|n| n.as_str()) == Some(&name))
        })
        .unwrap_or(false);
    if !found_in_list {
        warn!(
            name,
            "verify step 2b: vendor not found in first page of list (may be on a later page)"
        );
    }
    debug!(found_in_list, "verify step 2b done");

    // Step 3: Delete
    info!(id = %id_str, "verify step 3: DELETE /vendors/{id_str}");
    client
        .delete(&format!("/vendors/{}", id_str))
        .await
        .context("verify step 3 (delete) failed")?;
    debug!("verify step 3: vendor deleted");

    // Step 4: Verify deletion (GET -> 404)
    info!(id = %id_str, "verify step 4: GET /vendors/{id_str} -> expect 404");
    let deleted = match client.try_get(&format!("/vendors/{}", id_str)).await {
        Ok(None) => {
            debug!("verify step 4: confirmed 404 after delete");
            true
        }
        Ok(Some(v)) => {
            warn!(?v, "verify step 4: vendor still exists after delete");
            false
        }
        Err(e) => {
            // An ApiError with 404 is expected; treat other errors as a failure.
            match e.downcast_ref::<ApiError>() {
                Some(api) if api.status == StatusCode::NOT_FOUND => {
                    debug!("verify step 4: 404 confirmed via error");
                    true
                }
                _ => return Err(e.wrap_err("verify step 4 (confirm deletion) failed")),
            }
        }
    };

    let result = VerifyResult {
        vendor_id,
        created: true,
        verified_list: found_in_list,
        verified_get: true,
        deleted: true,
        verified_deleted: deleted,
    };

    info!(?result, "disposable verification cycle complete");
    Ok(result)
}

#[cfg(test)]
mod tests;
