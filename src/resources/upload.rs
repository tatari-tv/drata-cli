//! `drata upload-url create` - request a presigned upload URL (`POST
//! /upload-urls`) and, with `--file`, upload the file through it.
//!
//! Without `--file` the response (`presignedUrl`, `objectKey`, `expiresAt`,
//! `webUploadUrl`) is printed for the caller to use. With `--file` the file's
//! name and MIME type go into the request, the bytes are PUT to the presigned
//! URL with a matching `Content-Type` (the only header S3 checks), and only the
//! `objectKey` is printed: the value `--file-key` takes. The presigned URL is
//! never printed or logged in that mode; its query string is the signature.

use crate::cli::{UploadPurpose, UploadUrlAction};
use crate::client::DrataClient;
use crate::client::presigned::put_presigned;
use crate::config::Config;
use crate::confirm::ConfirmFn;
use crate::output::print_value;
use crate::query::wire_name;
use eyre::{Context, Result, bail, eyre};
use reqwest::Method;
use serde_json::{Value, json};
use std::path::Path;
use tracing::{debug, instrument};

const UPLOAD_URLS_PATH: &str = "/upload-urls";

pub async fn handle(
    action: &UploadUrlAction,
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
) -> Result<()> {
    match action {
        UploadUrlAction::Create {
            purpose,
            workspace,
            file,
        } => create(client, config, confirm, *purpose, *workspace, file.as_deref()).await,
    }
}

/// A local file read into memory, with the name and MIME type the URL
/// request declares for it.
#[derive(Debug)]
pub struct UploadFile {
    pub name: String,
    pub content_type: String,
    pub bytes: Vec<u8>,
}

/// Read `path` and derive its upload name and MIME type (from the extension,
/// `application/octet-stream` when unknown).
pub fn load_file(path: &Path) -> Result<UploadFile> {
    let bytes = std::fs::read(path).with_context(|| format!("Failed to read upload file `{}`", path.display()))?;
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| eyre!("upload file `{}` has no UTF-8 file name", path.display()))?
        .to_string();
    let content_type = mime_guess::from_path(path).first_or_octet_stream().to_string();
    debug!(%name, %content_type, bytes = bytes.len(), "loaded upload file");
    Ok(UploadFile {
        name,
        content_type,
        bytes,
    })
}

/// The `POST /upload-urls` body. Fails when a workspace-scoped purpose has no
/// workspace, before any request.
pub fn build_request_body(purpose: UploadPurpose, workspace: Option<u64>, file: Option<&UploadFile>) -> Result<Value> {
    if purpose.workspace_scoped() && workspace.is_none() {
        bail!("--purpose {} requires --workspace", wire_name(&purpose));
    }
    let mut body = json!({ "purpose": wire_name(&purpose) });
    if let Some(id) = workspace {
        body["workspaceId"] = json!(id);
    }
    if let Some(f) = file {
        body["fileName"] = json!(f.name);
        body["contentType"] = json!(f.content_type);
    }
    Ok(body)
}

/// A string field of the `/upload-urls` response; missing is a loud error.
fn response_str<'a>(resp: &'a Value, field: &str) -> Result<&'a str> {
    resp.get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| eyre!("POST {UPLOAD_URLS_PATH} response has no `{field}` string"))
}

#[instrument(skip(client, config, confirm))]
async fn create(
    client: &DrataClient,
    config: &Config,
    confirm: &ConfirmFn,
    purpose: UploadPurpose,
    workspace: Option<u64>,
    file: Option<&Path>,
) -> Result<()> {
    debug!(?purpose, ?workspace, has_file = file.is_some(), "upload-url create");

    // Read the file before any request: an unreadable file fails before the mutation.
    let upload = file.map(load_file).transpose()?;
    let body = build_request_body(purpose, workspace, upload.as_ref())?;

    if !confirm("POST", UPLOAD_URLS_PATH)? {
        bail!("aborted");
    }
    let resp = client.post(UPLOAD_URLS_PATH, body).await?;

    let Some(upload) = upload else {
        print_value(&resp, &config.output_format);
        return Ok(());
    };

    let presigned_url = response_str(&resp, "presignedUrl")?;
    let object_key = response_str(&resp, "objectKey")?.to_string();
    let headers = vec![("Content-Type".to_string(), upload.content_type)];
    put_presigned(Method::PUT, presigned_url, &headers, upload.bytes).await?;

    debug!(%object_key, "upload-url create uploaded file");
    println!("{object_key}");
    Ok(())
}

#[cfg(test)]
mod tests;
