#![allow(clippy::unwrap_used)]
use super::*;

const PROVIDERS: [(Provider, &str); 4] = [
    (Provider::Aws, "/aws-connections"),
    (Provider::Azure, "/azure-connections"),
    (Provider::Gcp, "/gcp-connections"),
    (Provider::Gitlab, "/gitlab-connections"),
];

#[test]
fn paths_for_each_provider() {
    for (provider, base) in PROVIDERS {
        assert_eq!(provider.collection(), base);
        assert_eq!(list_path(provider, &[]), base);
        assert_eq!(
            list_path(provider, &[ConnectionExpand::Workspaces]),
            format!("{base}?expand%5B%5D=workspaces")
        );
        assert_eq!(item_path(provider, "7").unwrap(), format!("{base}/7"));
        let args = ConnectionGetArgs {
            connection_id: "7".to_string(),
            expand: vec![ConnectionExpand::Workspaces],
        };
        assert_eq!(
            get_path(provider, &args).unwrap(),
            format!("{base}/7?expand%5B%5D=workspaces")
        );
    }
}

#[test]
fn prefixed_ids_are_one_encoded_segment_for_their_own_provider() {
    let cases = [
        (
            Provider::Aws,
            "roleArn:arn:aws:iam::123456789012:role/path/DrataAuditRole",
            "/aws-connections/roleArn:arn:aws:iam::123456789012:role%2Fpath%2FDrataAuditRole",
        ),
        (
            Provider::Azure,
            "subscriptionId:28ffcc9b-be6d-4bbf-b53a-057049debc38",
            "/azure-connections/subscriptionId:28ffcc9b-be6d-4bbf-b53a-057049debc38",
        ),
        (
            Provider::Gcp,
            "projectId:my-project",
            "/gcp-connections/projectId:my-project",
        ),
    ];
    for (provider, id, want) in cases {
        assert_eq!(item_path(provider, id).unwrap(), want, "{provider:?}");
    }
}

#[test]
fn ids_outside_the_providers_spec_forms_are_refused() {
    let refused = [
        (Provider::Gitlab, "projectId:my-project", "numeric connection id"),
        (Provider::Gitlab, "abc", "numeric connection id"),
        (Provider::Aws, "subscriptionId:x", "roleArn:<value>"),
        (
            Provider::Azure,
            "roleArn:arn:aws:iam::1:role/X",
            "subscriptionId:<value>",
        ),
        (Provider::Gcp, "projectId:", "projectId:<value>"),
        (Provider::Gcp, "-1", "projectId:<value>"),
        (Provider::Aws, "roleArn:arn:aws:iam::1:role%2FX", "unencoded"),
    ];
    for (provider, id, want) in refused {
        let msg = item_path(provider, id).unwrap_err().to_string();
        assert!(msg.contains(want), "{provider:?} {id}: {msg}");
    }
}

#[test]
fn read_body_rejects_inline_json_without_echoing_it() {
    let err = read_body(r#"{"applicationSecret":"s3cret-value"}"#).unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("@file") && msg.contains("stdin"), "{msg}");
    assert!(!msg.contains("s3cret-value"), "{msg}");
}

#[test]
fn read_body_reads_a_file_and_requires_an_object() {
    let dir = std::env::temp_dir().join(format!("drata-connection-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let object = dir.join("object.json");
    std::fs::write(&object, r#"{"name":"n"}"#).unwrap();
    assert_eq!(
        read_body(&format!("@{}", object.display())).unwrap(),
        json!({"name": "n"})
    );
    let array = dir.join("array.json");
    std::fs::write(&array, r#"["s3cret-value"]"#).unwrap();
    let msg = read_body(&format!("@{}", array.display())).unwrap_err().to_string();
    assert!(msg.contains("an array") && !msg.contains("s3cret-value"), "{msg}");
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn examples_come_from_each_providers_own_ops() {
    let credential = [
        (Provider::Aws, "roleArn"),
        (Provider::Azure, "applicationSecret"),
        (Provider::Gcp, "serviceAccountKey"),
        (Provider::Gitlab, "personalAccessToken"),
    ];
    for (provider, field) in credential {
        let create: Value = serde_json::from_str(&example("POST", provider.collection()).unwrap()).unwrap();
        assert!(create.get(field).is_some(), "{provider:?}: {create}");
        let update: Value = serde_json::from_str(&example("PUT", &provider.item_template()).unwrap()).unwrap();
        assert!(update.get("name").is_some(), "{provider:?}: {update}");
    }
}
