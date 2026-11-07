//! clap types for `drata upload-url`.

use clap::{Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Subcommand, Debug)]
pub enum UploadUrlAction {
    /// Request a presigned upload URL. With --file, upload the file through it
    /// and print the objectKey to pass to --file-key
    Create {
        /// What the uploaded file will be attached to
        #[arg(long, value_enum, ignore_case = true)]
        purpose: UploadPurpose,
        /// Workspace ID (required for workspace-scoped purposes: evidence)
        #[arg(long)]
        workspace: Option<u64>,
        /// File to upload; its name and MIME type go into the URL request
        #[arg(long)]
        file: Option<PathBuf>,
    },
}

/// Upload purpose (spec `UploadPurposeEnum`).
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
#[clap(rename_all = "lowercase")]
pub enum UploadPurpose {
    Evidence,
}

impl UploadPurpose {
    /// True when the spec requires `workspaceId` for this purpose.
    pub fn workspace_scoped(self) -> bool {
        match self {
            UploadPurpose::Evidence => true,
        }
    }
}
