//! Confirmation prompts for mutating operations (POST/PUT/PATCH/DELETE).
//!
//! Every mutating API call passes through `confirm_mutation` before sending.
//! The real implementation reads from the TTY; tests inject a
//! `ConfirmFn` to control the decision without requiring a real terminal.
//!
//! Fail-closed rules:
//! - `--yes` flag set: bypass, always proceed.
//! - stdin is not a TTY and `--yes` is absent: fail closed with an error.
//! - stdin is a TTY: prompt and require an explicit `y`/`yes` response.
//!
//! The `ConfirmFn` type alias is a boxed callable (used in tests), not a
//! generic parameter, to keep the call-site API simple.

use eyre::{Result, bail};
use std::io::{IsTerminal, Write};
use tracing::{debug, instrument};

/// A boxed confirmation function. Called with `(method, path)` and returns
/// `Ok(true)` to proceed, `Ok(false)` to abort (user said no), or `Err` on
/// I/O failure. Inject a custom one in tests to avoid real TTY reads.
pub type ConfirmFn = Box<dyn Fn(&str, &str) -> Result<bool> + Send + Sync>;

/// Build the default TTY-reading confirm function. When stdin is not a TTY
/// and `yes` is false, it returns an error immediately (fail closed). When
/// stdin is a TTY it prompts and requires `y`/`yes`.
pub fn default_confirm(yes: bool) -> ConfirmFn {
    Box::new(move |method: &str, path: &str| {
        debug!(method, path, yes, "confirm_mutation called");
        if yes {
            debug!("--yes set; bypassing confirmation");
            return Ok(true);
        }
        let stdin_is_tty = std::io::stdin().is_terminal();
        if !stdin_is_tty {
            bail!(
                "mutation {} {} requires confirmation but stdin is not a TTY. \
                 Pass --yes to bypass, or run interactively.",
                method,
                path
            );
        }
        prompt_user(method, path)
    })
}

/// Build an always-yes confirm function for tests that don't need to test
/// the prompt path.
pub fn always_yes() -> ConfirmFn {
    Box::new(|_, _| Ok(true))
}

/// Build an always-no confirm function for testing abort paths.
pub fn always_no() -> ConfirmFn {
    Box::new(|_, _| Ok(false))
}

/// Build a fail-closed confirm function that errors as if stdin is not a TTY.
/// Used to verify the non-TTY-without-`--yes` path.
pub fn fail_closed() -> ConfirmFn {
    Box::new(|method: &str, path: &str| {
        bail!(
            "mutation {} {} requires confirmation but stdin is not a TTY. \
             Pass --yes to bypass, or run interactively.",
            method,
            path
        )
    })
}

/// Prompt the user interactively on the TTY. Returns `Ok(true)` on `y`/`yes`
/// (case-insensitive), `Ok(false)` on any other input.
#[instrument]
fn prompt_user(method: &str, path: &str) -> Result<bool> {
    debug!(method, path, "prompting user for confirmation");
    print!("About to {} {}. Proceed? [y/N] ", method, path);
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout for confirmation prompt")?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read confirmation response")?;

    let trimmed = input.trim().to_lowercase();
    let confirmed = trimmed == "y" || trimmed == "yes";
    debug!(input = %trimmed, confirmed, "confirmation response");
    Ok(confirmed)
}

use eyre::Context;

#[cfg(test)]
mod tests;
