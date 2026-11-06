#![allow(clippy::unwrap_used)]
use super::*;

fn opts(values: &[&str]) -> Vec<String> {
    values.iter().map(|s| s.to_string()).collect()
}

#[test]
fn list_path_encodes_entity_type_and_hidden() {
    assert_eq!(list_path(None, None), "/custom-field-definitions");
    assert_eq!(
        list_path(Some(&CustomFieldEntityType::FrameworkRequirements), Some(false)),
        "/custom-field-definitions?entityType=FRAMEWORK_REQUIREMENTS&isHidden=false"
    );
}

#[test]
fn get_path_keeps_name_prefix_and_encodes_the_name() {
    assert_eq!(get_path("42"), "/custom-field-definitions/42");
    assert_eq!(
        get_path("name:Regulatory Owner"),
        "/custom-field-definitions/name:Regulatory%20Owner"
    );
    assert_eq!(get_path("name:a/b"), "/custom-field-definitions/name:a%2Fb");
}

#[test]
fn create_body_always_sends_hidden_and_required() {
    let body = build_create_body(Some("Owner"), None, Some(&CustomFieldType::Text), false, false, &[]).unwrap();
    assert_eq!(
        body,
        json!({"name": "Owner", "fieldType": "TEXT", "isHidden": false, "isRequired": false})
    );
    let body = build_create_body(
        Some("Owner"),
        Some("who owns it"),
        Some(&CustomFieldType::LongText),
        true,
        true,
        &[],
    )
    .unwrap();
    assert_eq!(body["fieldType"], json!("LONG_TEXT"));
    assert_eq!(body["description"], json!("who owns it"));
    assert_eq!(body["isHidden"], json!(true));
    assert_eq!(body["isRequired"], json!(true));
}

#[test]
fn options_field_builds_ordered_options_from_position_one() {
    let body = build_create_body(
        Some("Color"),
        None,
        Some(&CustomFieldType::Options),
        false,
        false,
        &opts(&["RED", "BLUE"]),
    )
    .unwrap();
    assert_eq!(
        body["options"],
        json!([
            {"value": "RED", "position": 1, "isHidden": false},
            {"value": "BLUE", "position": 2, "isHidden": false}
        ])
    );
}

#[test]
fn options_rules_follow_the_spec() {
    let make = |t: CustomFieldType, o: &[&str]| {
        build_create_body(Some("n"), None, Some(&t), false, false, &opts(o)).map_err(|e| e.to_string())
    };
    assert!(
        make(CustomFieldType::Options, &[])
            .unwrap_err()
            .contains("requires --option")
    );
    assert!(
        make(CustomFieldType::Options, &["only"])
            .unwrap_err()
            .contains("2 to 100")
    );
    let many: Vec<String> = (0..101).map(|i| i.to_string()).collect();
    assert!(
        build_create_body(Some("n"), None, Some(&CustomFieldType::Options), false, false, &many)
            .unwrap_err()
            .to_string()
            .contains("2 to 100")
    );
    assert!(
        make(CustomFieldType::Text, &["a", "b"])
            .unwrap_err()
            .contains("only for an OPTIONS")
    );
}

#[test]
fn create_requires_name_and_type_and_bounds_lengths() {
    let e = |r: Result<Value>| r.unwrap_err().to_string();
    assert!(
        e(build_create_body(
            None,
            None,
            Some(&CustomFieldType::Text),
            false,
            false,
            &[]
        ))
        .contains("--name")
    );
    assert!(e(build_create_body(Some("n"), None, None, false, false, &[])).contains("--field-type"));
    assert!(
        e(build_create_body(
            Some(&"x".repeat(192)),
            None,
            Some(&CustomFieldType::Text),
            false,
            false,
            &[]
        ))
        .contains("191")
    );
}
