//! clap types for `drata safebase`, the SafeBase Trust Center API namespace.

use super::RawArgs;
use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum SafebaseAction {
    /// Save the profile's SafeBase credential (api key + write flag); its
    /// Drata credential is left as it is
    Login {
        /// SafeBase API key to store
        #[arg(long)]
        api_key: String,
        /// Mark this credential write-enabled (allows non-GET requests)
        #[arg(long = "allow-writes")]
        allow_writes: bool,
    },
    /// Remove the profile's SafeBase credential; its Drata credential stays
    Logout,
    /// Generic passthrough to any SafeBase operation: `raw <METHOD> <path> ...`
    /// (same flags as `drata raw`; GET, POST, PUT, PATCH, DELETE)
    Raw(RawArgs),
}
