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
        assert_eq!(item_path(provider, 7), format!("{base}/7"));
        let args = ConnectionGetArgs {
            connection_id: 7,
            expand: vec![ConnectionExpand::Workspaces],
        };
        assert_eq!(get_path(provider, &args), format!("{base}/7?expand%5B%5D=workspaces"));
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
