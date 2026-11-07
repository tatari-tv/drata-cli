//! `drata safebase document upload`: three steps.
//!
//! 1. `POST /documents/upload` `{fileName, mimeType, md5b64, fileSize}` ->
//!    `{fileId, signedUrl, headers}`
//! 2. PUT the bytes to `signedUrl` with exactly the returned `headers`, through
//!    `client::presigned` (its own `reqwest::Client`: no SafeBase key, no
//!    redirects, no retry). Phase 0 observed PUT; the spec text's POST is wrong
//! 3. `PUT /documents/upload` `{name, fileId, mimeType, externalId, type?}`,
//!    plus `replacementDocumentId` under `--replace`. Sent once, never retried
//!
//! Size and MIME type are checked against the embedded spec before step 1, and
//! `--external-id` must be a UUID, so a bad input sends nothing.
//!
//! Outcomes (`UploadError`): a step 1 or 2 failure, or an HTTP error on step 3,
//! is definite: no document. A step 3 that never answered (timeout, dropped
//! connection, unreadable 2xx body) is looked up once by `externalId`; only a
//! document whose `size` and `checksum` (SafeBase's md5b64) match the local
//! file counts as found, because SafeBase does not enforce `externalId`
//! uniqueness. Anything else is `outcome unknown`.

use super::{Reply, get_path};
use crate::backend::Backend;
use crate::cli::UploadArgs;
use crate::client::presigned::put_presigned;
use crate::client::{DrataClient, EnvelopeError};
use crate::confirm::ConfirmFn;
use crate::output::Table;
use crate::resources::safebase::confirmed;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use eyre::{Context, Result, bail, eyre};
use md5::{Digest, Md5};
use reqwest::Method;
use serde_json::{Value, json};
use std::path::Path;
use thiserror::Error;
use tracing::{debug, instrument, warn};
use uuid::Uuid;

/// Both upload steps on the SafeBase API share this path (POST then PUT).
pub const UPLOAD_PATH: &str = "/documents/upload";

/// Where the `POST /documents/upload` body schema lives in the SafeBase spec.
const UPLOAD_SCHEMA_PROPERTIES: &str =
    "/paths/~1documents~1upload/post/requestBody/content/application~1json/schema/properties";

/// A failed upload, by step. Each message names the ids the caller needs to
/// follow up: SafeBase has no op that deletes an upload `fileId`.
#[derive(Debug, Error)]
pub enum UploadError {
    #[error("upload step 1 (POST {UPLOAD_PATH}) failed, no document was created{}: {cause}", file_id_note(.file_id))]
    Request { file_id: Option<String>, cause: String },
    #[error("upload step 2 (file to the signed URL) failed, no document was created; fileId {file_id}: {cause}")]
    Send { file_id: String, cause: String },
    #[error("upload step 3 (PUT {UPLOAD_PATH}) failed, no document was created; fileId {file_id}: {cause}")]
    Create { file_id: String, cause: String },
    #[error(
        "outcome unknown: PUT {UPLOAD_PATH} did not answer ({cause}) and the lookup by externalId {lookup}. \
         fileId {file_id}, externalId {external_id}. Check `drata safebase document get --external {external_id}` \
         before uploading again"
    )]
    Unknown {
        file_id: String,
        external_id: String,
        cause: String,
        lookup: String,
    },
}

fn file_id_note(file_id: &Option<String>) -> String {
    file_id.as_ref().map(|f| format!("; fileId {f}")).unwrap_or_default()
}

/// A local file checked against the spec's upload limits, ready to send.
#[derive(Debug)]
pub struct LocalFile {
    pub file_name: String,
    pub mime_type: String,
    pub md5b64: String,
    pub bytes: Vec<u8>,
}

/// The base64 of the file's MD5 digest, SafeBase's `md5b64` and `checksum`.
pub fn md5b64(bytes: &[u8]) -> String {
    STANDARD.encode(Md5::digest(bytes))
}

/// The spec's upload bounds: `fileSize.maximum` and the `mimeType` enum. Read
/// from the embedded spec so the CLI checks what the API declares.
pub fn upload_limits() -> Result<(u64, Vec<String>)> {
    let props = Backend::Safebase
        .spec()?
        .pointer(UPLOAD_SCHEMA_PROPERTIES)
        .ok_or_else(|| eyre!("SafeBase spec has no POST {UPLOAD_PATH} body schema"))?;
    let max = props
        .pointer("/fileSize/maximum")
        .and_then(Value::as_u64)
        .ok_or_else(|| eyre!("SafeBase spec has no POST {UPLOAD_PATH} fileSize.maximum"))?;
    let mimes = props
        .pointer("/mimeType/enum")
        .and_then(Value::as_array)
        .ok_or_else(|| eyre!("SafeBase spec has no POST {UPLOAD_PATH} mimeType enum"))?
        .iter()
        .filter_map(|m| m.as_str().map(str::to_string))
        .collect();
    Ok((max, mimes))
}

/// The first MIME type guessed from `path`'s extension that SafeBase accepts.
/// Pure.
pub fn pick_mime(path: &Path, allowed: &[String]) -> Result<String> {
    let guesses: Vec<String> = mime_guess::from_path(path).iter().map(|m| m.to_string()).collect();
    guesses
        .iter()
        .find(|g| allowed.iter().any(|a| a == *g))
        .cloned()
        .ok_or_else(|| {
            let seen = if guesses.is_empty() {
                "no type from its extension".to_string()
            } else {
                guesses.join(" ")
            };
            eyre!(
                "`{}` is not a type SafeBase accepts ({seen}); allowed: {}",
                path.display(),
                allowed.join(" ")
            )
        })
}

/// Read `path` and check it against the spec: size at most the spec maximum
/// and a MIME type in the spec enum. The file is opened once and the read is
/// capped at `max + 1` bytes, so a swapped path or a FIFO/device reporting
/// length 0 can never be read past the limit.
#[instrument]
pub fn load(path: &Path) -> Result<LocalFile> {
    let (max, allowed) = upload_limits()?;
    let read_err = || format!("Failed to read upload file `{}`", path.display());
    let file = std::fs::File::open(path).with_context(read_err)?;
    let len = file.metadata().with_context(read_err)?.len();
    check_size(path, len, max)?;
    let mime_type = pick_mime(path, &allowed)?;
    let mut bytes = Vec::with_capacity(len as usize);
    std::io::Read::read_to_end(&mut std::io::Read::take(file, max + 1), &mut bytes).with_context(read_err)?;
    check_size(path, bytes.len() as u64, max)?;
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| eyre!("upload file `{}` has no UTF-8 file name", path.display()))?
        .to_string();
    let md5b64 = md5b64(&bytes);
    debug!(%file_name, %mime_type, bytes = bytes.len(), "loaded SafeBase upload file");
    Ok(LocalFile {
        file_name,
        mime_type,
        md5b64,
        bytes,
    })
}

fn check_size(path: &Path, len: u64, max: u64) -> Result<()> {
    if len > max {
        bail!(
            "`{}` is {len} bytes; SafeBase accepts at most {max} bytes per upload",
            path.display()
        );
    }
    Ok(())
}

/// The `externalId` to store: `--external-id` when given (must parse as a
/// UUID, the lookup path's format; sent lowercase hyphenated), else a fresh v4.
pub fn external_id(arg: Option<&str>) -> Result<String> {
    match arg {
        Some(given) => Uuid::parse_str(given)
            .map(|u| u.hyphenated().to_string())
            .map_err(|e| eyre!("--external-id `{given}` is not a UUID: {e}")),
        None => Ok(Uuid::new_v4().hyphenated().to_string()),
    }
}

/// The step 1 body. Pure.
pub fn request_body(file: &LocalFile) -> Value {
    json!({
        "fileName": file.file_name,
        "mimeType": file.mime_type,
        "md5b64": file.md5b64,
        "fileSize": file.bytes.len(),
    })
}

/// The step 3 body. Pure.
pub fn create_body(args: &UploadArgs, file_id: &str, mime_type: &str, external_id: &str) -> Value {
    let mut body = json!({
        "name": args.name,
        "fileId": file_id,
        "mimeType": mime_type,
        "externalId": external_id,
    });
    if let Some(t) = &args.doc_type {
        body["type"] = json!(t);
    }
    if let Some(id) = &args.replace {
        body["replacementDocumentId"] = json!(id);
    }
    body
}

/// What step 1 handed back.
struct Ticket {
    file_id: String,
    signed_url: String,
    headers: Vec<(String, String)>,
}

/// Parse the step 1 `result`. Any missing piece is an error; the `fileId`
/// rides along when it was present.
fn parse_ticket(resp: &Value) -> std::result::Result<Ticket, UploadError> {
    let result = resp.get("result");
    let file_id = result.and_then(|r| r.get("fileId")).and_then(Value::as_str);
    // Never echo `resp`: it may carry the signed URL, whose query is a credential.
    let file_id = file_id.ok_or_else(|| UploadError::Request {
        file_id: None,
        cause: "response has no `result.fileId` string".to_string(),
    })?;
    let signed_url = result
        .and_then(|r| r.get("signedUrl"))
        .and_then(Value::as_str)
        .ok_or_else(|| UploadError::Request {
            file_id: Some(file_id.to_string()),
            cause: "response has no `result.signedUrl` string".to_string(),
        })?;
    let headers = result
        .and_then(|r| r.get("headers"))
        .and_then(Value::as_object)
        .ok_or_else(|| UploadError::Request {
            file_id: Some(file_id.to_string()),
            cause: "response has no `result.headers` object".to_string(),
        })?
        .iter()
        .map(|(k, v)| {
            v.as_str()
                .map(|s| (k.clone(), s.to_string()))
                .ok_or_else(|| UploadError::Request {
                    file_id: Some(file_id.to_string()),
                    cause: format!("signed-URL header `{k}` is not a string"),
                })
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;
    Ok(Ticket {
        file_id: file_id.to_string(),
        signed_url: signed_url.to_string(),
        headers,
    })
}

/// A step 1 failure as error text. A 2xx that broke the envelope may still
/// carry `result.signedUrl`, whose query is a credential, so its body is
/// withheld: only the status is named. Any other failure (an HTTP error
/// status, a transport error) is shown whole; a non-2xx carries no ticket.
fn step1_cause(err: &eyre::Report) -> String {
    match err.downcast_ref::<EnvelopeError>() {
        Some(env) => format!(
            "{} answered {} but broke the {{ok, result}} envelope; body withheld, it can carry the signed URL",
            Backend::Safebase,
            env.status
        ),
        None => format!("{err:#}"),
    }
}

/// True when a step 3 failure leaves the outcome open: the request may have
/// reached SafeBase but no answer came back (timeout, dropped connection, an
/// unreadable body). A connect failure never sent the request, and an HTTP
/// status or envelope error is SafeBase's answer: both are definite.
pub fn outcome_is_open(err: &eyre::Report) -> bool {
    err.chain()
        .filter_map(|c| c.downcast_ref::<reqwest::Error>())
        .any(|e| !e.is_connect() && !e.is_builder())
}

/// True when the looked-up document is the file this run uploaded: same byte
/// count and same `checksum` (md5b64). Pure.
pub fn matches_local(doc: &Value, size: usize, md5b64: &str) -> bool {
    doc.get("size").and_then(Value::as_u64) == Some(size as u64)
        && doc.get("checksum").and_then(Value::as_str) == Some(md5b64)
}

/// Run the three steps. See the module docs for the outcome rules.
#[instrument(skip(client, confirm))]
pub async fn run(args: &UploadArgs, client: &DrataClient, confirm: &ConfirmFn) -> Result<Reply> {
    let external_id = external_id(args.external_id.as_deref())?;
    let file = load(&args.file)?;
    debug!(%external_id, replace = ?args.replace, "safebase document upload");
    confirmed(
        confirm,
        "POST",
        &format!("{UPLOAD_PATH} (then the signed URL, then PUT {UPLOAD_PATH})"),
    )?;

    let resp = client
        .post(UPLOAD_PATH, request_body(&file))
        .await
        .map_err(|e| UploadError::Request {
            file_id: None,
            cause: step1_cause(&e),
        })?;
    let ticket = parse_ticket(&resp)?;
    debug!(file_id = %ticket.file_id, "upload step 1 done");

    let size = file.bytes.len();
    put_presigned(Method::PUT, &ticket.signed_url, &ticket.headers, file.bytes)
        .await
        .map_err(|e| UploadError::Send {
            file_id: ticket.file_id.clone(),
            cause: format!("{e:#}"),
        })?;
    debug!(file_id = %ticket.file_id, "upload step 2 done");

    let body = create_body(args, &ticket.file_id, &file.mime_type, &external_id);
    match client.put_once(UPLOAD_PATH, body).await {
        Ok(resp) => match resp.get("result") {
            Some(doc) => Ok(Reply::One {
                table: Table::SafebaseDocuments,
                row: doc.clone(),
            }),
            None => {
                let cause = "answered 2xx with no `result`".to_string();
                warn!(file_id = %ticket.file_id, %external_id, %cause, "upload step 3 returned no result; looking up by externalId");
                settle(client, &ticket.file_id, &external_id, size, &file.md5b64, cause).await
            }
        },
        Err(e) if outcome_is_open(&e) => {
            let cause = format!("{e:#}");
            warn!(file_id = %ticket.file_id, %external_id, %cause, "upload step 3 unanswered; looking up by externalId");
            settle(client, &ticket.file_id, &external_id, size, &file.md5b64, cause).await
        }
        Err(e) => Err(UploadError::Create {
            file_id: ticket.file_id,
            cause: format!("{e:#}"),
        }
        .into()),
    }
}

/// The one lookup after an unanswered step 3: `GET /document/<externalId>
/// ?type=external`, sent once (not even a 429 repeats it). Never resends the
/// PUT.
async fn settle(
    client: &DrataClient,
    file_id: &str,
    external_id: &str,
    size: usize,
    md5b64: &str,
    cause: String,
) -> Result<Reply> {
    let unknown = |lookup: String| -> eyre::Report {
        UploadError::Unknown {
            file_id: file_id.to_string(),
            external_id: external_id.to_string(),
            cause: cause.clone(),
            lookup,
        }
        .into()
    };
    let found = match client.try_get_once(&get_path(external_id, true)).await {
        Ok(found) => found,
        Err(e) => return Err(unknown(format!("failed: {e:#}"))),
    };
    let Some(doc) = found.as_ref().and_then(|r| r.get("result")) else {
        return Err(unknown("found no document".to_string()));
    };
    if !matches_local(doc, size, md5b64) {
        return Err(unknown(format!(
            "found a document whose size/checksum differ from the local file (size {}, checksum {})",
            doc.get("size").unwrap_or(&Value::Null),
            doc.get("checksum").unwrap_or(&Value::Null)
        )));
    }
    Ok(Reply::Warned {
        warning: format!(
            "PUT {UPLOAD_PATH} did not answer ({cause}); the document was then found by externalId \
             {external_id} with matching size and checksum"
        ),
        table: Table::SafebaseDocuments,
        row: doc.clone(),
    })
}

#[cfg(test)]
mod tests;
