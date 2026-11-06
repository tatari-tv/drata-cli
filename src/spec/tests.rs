#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn enumerates_every_operation_in_the_committed_spec() {
    let ops = operations().unwrap();
    assert_eq!(ops.len(), 308, "spec should expose exactly 308 operations");
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
fn operation_tags_include_tags_once_missing_from_top_level() {
    // Tags are sourced from operations because the top-level `tags` list once
    // omitted these two; keep asserting them so a regression in either list shows.
    let tags = operation_tags().unwrap();
    assert_eq!(tags.len(), 51, "expected 51 operation-level tags, got {}", tags.len());
    assert!(tags.contains(&"Audit Requests".to_string()));
    assert!(tags.contains(&"Procurement Connection Mappings".to_string()));
}

#[test]
fn fourteen_operations_are_multipart_uploads() {
    let multipart = operations().unwrap().into_iter().filter(|op| op.multipart).count();
    assert_eq!(multipart, 14, "expected 14 multipart upload operations");
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

#[test]
fn example_for_schema_builds_a_named_component() {
    let resource = example_for_schema("CustomTestRecipeResourcePublicV2Dto").unwrap();
    assert_eq!(resource["resource"], json!("Employees"));
    assert!(resource["evaluator"]["all"].is_array(), "{resource}");
}

#[test]
fn example_for_unknown_schema_is_an_error() {
    let err = example_for_schema("NoSuchSchemaDto").unwrap_err();
    assert!(err.to_string().contains("NoSuchSchemaDto"), "{err}");
}
