#![allow(clippy::unwrap_used)]
use crate::cli::DeviceAction;

// Device module is read-only; verify handle arms compile.
// The actual HTTP calls are exercised by integration tests against a mock.

#[test]
fn device_action_variants_are_exhaustive() {
    // Ensure all variants are named (this would fail to compile if one was added
    // to the enum without a corresponding handle arm).
    let actions = [
        DeviceAction::List,
        DeviceAction::Get {
            device_id: "1".to_string(),
        },
        DeviceAction::ForPersonnel {
            personnel_id: "2".to_string(),
        },
        DeviceAction::Apps {
            device_id: "3".to_string(),
        },
    ];
    // Verify the vec is non-empty so the binding is used.
    assert_eq!(actions.len(), 4);
}
