#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn enumerates_every_operation_in_the_committed_spec() {
    let ops = operations(Backend::Drata).unwrap();
    assert_eq!(ops.len(), 308, "spec should expose exactly 308 operations");
}

#[test]
fn every_operation_has_method_path_and_id() {
    let ops = operations(Backend::Drata).unwrap();
    for op in &ops {
        assert!(!op.method.is_empty(), "operation missing method: {op:?}");
        assert!(op.path.starts_with('/'), "path should be absolute: {op:?}");
        assert!(!op.operation_id.is_empty(), "operation missing operationId: {op:?}");
    }
}

#[test]
fn methods_are_upper_cased_and_known() {
    let ops = operations(Backend::Drata).unwrap();
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
    let tags = operation_tags(Backend::Drata).unwrap();
    assert_eq!(tags.len(), 51, "expected 51 operation-level tags, got {}", tags.len());
    assert!(tags.contains(&"Audit Requests".to_string()));
    assert!(tags.contains(&"Procurement Connection Mappings".to_string()));
}

#[test]
fn fourteen_operations_are_multipart_uploads() {
    let multipart = operations(Backend::Drata)
        .unwrap()
        .into_iter()
        .filter(|op| op.multipart)
        .count();
    assert_eq!(multipart, 14, "expected 14 multipart upload operations");
}

#[test]
fn find_by_method_path_resolves_a_known_op() {
    let op = find_by_method_path(Backend::Drata, "post", "/vendors").unwrap();
    assert!(op.is_some());
    let op = op.unwrap();
    assert_eq!(op.method, "POST");
    assert!(op.tags.contains(&"Vendors".to_string()));
}

#[test]
fn find_by_method_path_returns_none_for_unknown() {
    assert!(
        find_by_method_path(Backend::Drata, "get", "/no/such/path")
            .unwrap()
            .is_none()
    );
}

#[test]
fn example_for_create_vendor_includes_required_name() {
    let example = example_for_operation(Backend::Drata, "post", "/vendors")
        .unwrap()
        .unwrap();
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
    let example = example_for_operation(Backend::Drata, "post", "/vendors")
        .unwrap()
        .unwrap();
    assert!(
        !example.contains("$ref"),
        "skeleton must not leak a raw $ref: {example}"
    );
}

#[test]
fn example_for_get_operation_is_none() {
    // A GET has no request body, so no skeleton.
    let example = example_for_operation(Backend::Drata, "get", "/vendors").unwrap();
    assert!(example.is_none());
}

#[test]
fn example_for_unknown_operation_is_none() {
    let example = example_for_operation(Backend::Drata, "post", "/no/such/path").unwrap();
    assert!(example.is_none());
}

#[test]
fn example_for_schema_builds_a_named_component() {
    let resource = example_for_schema(Backend::Drata, "CustomTestRecipeResourcePublicV2Dto").unwrap();
    assert_eq!(resource["resource"], json!("Employees"));
    assert!(resource["evaluator"]["all"].is_array(), "{resource}");
}

#[test]
fn example_for_unknown_schema_is_an_error() {
    let err = example_for_schema(Backend::Drata, "NoSuchSchemaDto").unwrap_err();
    assert!(err.to_string().contains("NoSuchSchemaDto"), "{err}");
}

#[test]
fn enumerates_every_operation_in_the_safebase_spec() {
    let ops = operations(Backend::Safebase).unwrap();
    assert_eq!(ops.len(), 47, "SafeBase spec should expose exactly 47 operations");
    let count = |m: &str| ops.iter().filter(|op| op.method == m).count();
    assert_eq!(
        (
            count("GET"),
            count("POST"),
            count("PATCH"),
            count("DELETE"),
            count("PUT")
        ),
        (22, 12, 6, 6, 1)
    );
    // No operationId assertion: the SafeBase spec omits one (`PATCH /accounts/{id}`).
    for op in &ops {
        assert!(op.path.starts_with('/'), "path should be absolute: {op:?}");
    }
}

#[test]
fn safebase_has_one_multipart_operation() {
    let multipart: Vec<_> = operations(Backend::Safebase)
        .unwrap()
        .into_iter()
        .filter(|op| op.multipart)
        .collect();
    assert_eq!(multipart.len(), 1);
    assert_eq!(
        (multipart[0].method.as_str(), multipart[0].path.as_str()),
        ("POST", "/questionnaires")
    );
}

#[test]
fn find_by_method_path_is_scoped_to_its_backend() {
    let op = find_by_method_path(Backend::Safebase, "patch", "/document/{id}")
        .unwrap()
        .unwrap();
    assert_eq!(op.method, "PATCH");
    assert!(
        find_by_method_path(Backend::Drata, "patch", "/document/{id}")
            .unwrap()
            .is_none()
    );
    assert!(
        find_by_method_path(Backend::Safebase, "post", "/vendors")
            .unwrap()
            .is_none()
    );
}

#[test]
fn skeleton_reads_an_openapi_31_type_list_as_its_non_null_type() {
    let spec = Backend::Safebase.spec().unwrap();
    assert_eq!(
        build_skeleton(spec, &json!({"type": ["string", "null"]}), 0),
        json!("string")
    );
    assert_eq!(build_skeleton(spec, &json!({"type": ["null", "integer"]}), 0), json!(0));
    assert_eq!(
        build_skeleton(spec, &json!({"type": ["boolean", "null"]}), 0),
        json!(false)
    );
    assert_eq!(build_skeleton(spec, &json!({"type": ["null"]}), 0), Value::Null);
}

#[test]
fn skeleton_reads_a_type_list_from_the_safebase_spec() {
    let spec = Backend::Safebase.spec().unwrap();
    let expires = spec
        .pointer("/paths/~1documents~1upload/put/requestBody/content/application~1json/schema/properties/expiresAt")
        .unwrap();
    assert_eq!(expires["type"], json!(["string", "null"]));
    assert_eq!(build_skeleton(spec, expires, 0), json!("string"));
}

#[test]
fn example_for_safebase_patch_reads_the_safebase_spec() {
    // A skeleton carries required fields only, so pick an op that has some.
    let example = example_for_operation(Backend::Safebase, "post", "/documents/upload")
        .unwrap()
        .unwrap();
    let value: Value = serde_json::from_str(&example).unwrap();
    assert!(value.get("fileName").is_some(), "{value}");
    // The same method+path is not in the Drata spec.
    assert!(
        example_for_operation(Backend::Drata, "post", "/documents/upload")
            .unwrap()
            .is_none()
    );
}

#[test]
fn example_for_safebase_schema_and_enum_values_take_the_backend() {
    assert!(example_for_schema(Backend::Safebase, "NoSuchSchemaDto").is_err());
    assert!(enum_values(Backend::Safebase, "ClientTypeEnum").is_err());
    assert!(enum_values(Backend::Drata, "ClientTypeEnum").is_ok());
}
