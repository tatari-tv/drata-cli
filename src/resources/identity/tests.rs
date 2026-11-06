#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{IdentityExpand, client_type};

fn target() -> IdentityRef {
    IdentityRef {
        workspace_id: "1".into(),
        identity_id: "7".into(),
    }
}

fn list_args() -> IdentityListArgs {
    IdentityListArgs {
        workspace_id: "1".into(),
        identity_id: None,
        username: None,
        email: None,
        client_type: None,
        connection_id: None,
        expand: vec![],
    }
}

#[test]
fn identity_paths_for_each_kind() {
    for (kind, segment) in [
        (Kind::Infrastructure, "infrastructure-user-identities"),
        (Kind::VersionControl, "version-control-user-identities"),
    ] {
        let base = format!("/workspaces/1/{segment}");
        assert_eq!(collection_path(kind, "1"), base);
        assert_eq!(list_path(kind, &list_args()), base);
        assert_eq!(item_path(kind, &target()), format!("{base}/7"));
        assert_eq!(actions_path(kind, &target()), format!("{base}/7/actions"));
        assert_eq!(
            get_path(
                kind,
                &IdentityGetArgs {
                    target: target(),
                    expand: vec![IdentityExpand::User, IdentityExpand::ServiceAccountDesignator],
                }
            ),
            format!("{base}/7?expand%5B%5D=user&expand%5B%5D=serviceAccountDesignator")
        );
        assert_eq!(
            kind.actions_template(),
            format!("/workspaces/{{workspaceId}}/{segment}/{{identityId}}/actions")
        );
    }
}

#[test]
fn list_path_encodes_every_filter() {
    let args = IdentityListArgs {
        identity_id: Some("1a2b".into()),
        username: Some("j doe".into()),
        email: Some("j@x.test".into()),
        client_type: Some("GITHUB".into()),
        connection_id: Some(9),
        expand: vec![IdentityExpand::Connection],
        ..list_args()
    };
    assert_eq!(
        list_path(Kind::VersionControl, &args),
        "/workspaces/1/version-control-user-identities?identityId=1a2b&username=j%20doe&email=j@x.test\
         &clientType=GITHUB&connectionId=9&expand%5B%5D=connection"
    );
}

#[test]
fn path_segments_are_encoded() {
    let t = IdentityRef {
        workspace_id: "1".into(),
        identity_id: "a/b".into(),
    };
    assert_eq!(
        item_path(Kind::Infrastructure, &t),
        "/workspaces/1/infrastructure-user-identities/a%2Fb"
    );
}

#[test]
fn run_action_body_link_user_needs_user_id() {
    assert_eq!(
        build_run_action_body("linkUser", Some(5), None).unwrap(),
        json!({"action": "linkUser", "userId": 5})
    );
    let err = build_run_action_body("linkUser", None, None).unwrap_err().to_string();
    assert!(err.contains("--user-id"), "{err}");
}

#[test]
fn run_action_body_mark_service_account_needs_reason() {
    assert_eq!(
        build_run_action_body("markServiceAccount", None, Some("deploys")).unwrap(),
        json!({"action": "markServiceAccount", "reason": "deploys"})
    );
    let err = build_run_action_body("markServiceAccount", None, None)
        .unwrap_err()
        .to_string();
    assert!(err.contains("--reason"), "{err}");
    let long = "x".repeat(MAX_REASON + 1);
    assert!(build_run_action_body("markServiceAccount", None, Some(&long)).is_err());
}

#[test]
fn run_action_body_rejects_fields_for_the_wrong_action() {
    assert_eq!(
        build_run_action_body("unlinkUser", None, None).unwrap(),
        json!({"action": "unlinkUser"})
    );
    assert!(build_run_action_body("unlinkUser", Some(1), None).is_err());
    assert!(build_run_action_body("retractWriteAccess", None, Some("r")).is_err());
    assert!(build_run_action_body("markServiceAccount", Some(1), Some("r")).is_err());
}

#[test]
fn client_type_matches_the_spec_case_insensitively() {
    assert_eq!(client_type("github").unwrap(), "GITHUB");
    assert_eq!(client_type("Mergedev_Bamboo_Hr").unwrap(), "MERGEDEV_BAMBOO_HR");
    assert!(client_type("not-a-client").is_err());
}

#[test]
fn examples_come_from_each_kinds_own_op() {
    for kind in [Kind::Infrastructure, Kind::VersionControl] {
        let skeleton: Value = serde_json::from_str(&example(kind).unwrap()).unwrap();
        assert!(skeleton.get("action").is_some(), "{kind:?}: {skeleton}");
    }
}
