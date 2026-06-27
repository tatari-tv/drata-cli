#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn enumerates_all_167_operations() {
    let ops = operations().unwrap();
    assert_eq!(ops.len(), 167, "spec should expose exactly 167 operations");
}

#[test]
fn every_operation_has_method_path_and_id() {
    let ops = operations().unwrap();
    for op in &ops {
        assert!(!op.method.is_empty(), "operation missing method: {op:?}");
        assert!(op.path.starts_with('/'), "path should be absolute: {op:?}");
        assert!(!op.operation_id.is_empty(), "operation missing operationId: {op:?}");
    }
}

#[test]
fn methods_are_upper_cased_and_known() {
    let ops = operations().unwrap();
    for op in &ops {
        assert!(
            matches!(op.method.as_str(), "GET" | "POST" | "PUT" | "DELETE"),
            "unexpected method {} for {}",
            op.method,
            op.path
        );
    }
}

#[test]
fn operation_tags_include_the_two_missing_from_top_level() {
    // The whole point of sourcing tags from operations: these two are NOT in the
    // 33-entry top-level `tags` list.
    let tags = operation_tags().unwrap();
    assert_eq!(tags.len(), 35, "expected 35 operation-level tags, got {}", tags.len());
    assert!(tags.contains(&"Audit Requests".to_string()));
    assert!(tags.contains(&"Procurement Connection Mappings".to_string()));
}

#[test]
fn ten_operations_are_multipart_uploads() {
    let multipart = operations().unwrap().into_iter().filter(|op| op.multipart).count();
    assert_eq!(multipart, 10, "expected 10 multipart upload operations");
}

#[test]
fn find_by_method_path_resolves_a_known_op() {
    let op = find_by_method_path("post", "/vendors").unwrap();
    assert!(op.is_some());
    let op = op.unwrap();
    assert_eq!(op.method, "POST");
    assert!(op.tags.contains(&"Vendors".to_string()));
}

#[test]
fn find_by_method_path_returns_none_for_unknown() {
    assert!(find_by_method_path("get", "/no/such/path").unwrap().is_none());
}

#[test]
fn example_for_create_vendor_includes_required_name() {
    let example = example_for_operation("post", "/vendors").unwrap().unwrap();
    let parsed: Value = serde_json::from_str(&example).unwrap();
    // `name` is a required property of the create-vendor schema.
    assert!(
        parsed.get("name").is_some(),
        "create-vendor example should include name: {example}"
    );
    // The skeleton must be valid JSON object output.
    assert!(parsed.is_object());
}

#[test]
fn example_resolves_enum_refs_to_a_real_value() {
    // `risk` resolves through allOf -> $ref VendorRiskEnum; the skeleton should
    // carry one of the enum's allowed values (or its declared example), never a
    // dangling $ref.
    let example = example_for_operation("post", "/vendors").unwrap().unwrap();
    assert!(
        !example.contains("$ref"),
        "skeleton must not leak a raw $ref: {example}"
    );
}

#[test]
fn example_for_get_operation_is_none() {
    // A GET has no request body, so no skeleton.
    let example = example_for_operation("get", "/vendors").unwrap();
    assert!(example.is_none());
}

#[test]
fn example_for_unknown_operation_is_none() {
    let example = example_for_operation("post", "/no/such/path").unwrap();
    assert!(example.is_none());
}
