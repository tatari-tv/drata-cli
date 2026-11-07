#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{AccessReviewAppAction, review_user, role_grant};

fn user(raw: &str) -> ReviewUser {
    review_user(raw).unwrap()
}

fn role(raw: &str) -> RoleGrant {
    role_grant(raw).unwrap()
}

#[test]
fn paths_address_the_application_and_its_users() {
    assert_eq!(item_path(7), "/access-review-applications/7");
    assert_eq!(users_path(7), "/access-review-applications/7/users");
}

#[test]
fn create_body_carries_name_reviewers_and_optional_url() {
    assert_eq!(
        build_create_body("AWS", Some("https://aws.amazon.com"), &[1, 2]),
        json!({"name": "AWS", "websiteUrl": "https://aws.amazon.com", "reviewersIds": [1, 2]})
    );
    assert_eq!(
        build_create_body("AWS", None, &[3]),
        json!({"name": "AWS", "reviewersIds": [3]})
    );
}

#[test]
fn update_body_sends_only_named_fields_and_clears_the_url_with_null() {
    assert_eq!(
        build_update_body(Some("GCP"), None, false, &[]).unwrap(),
        json!({"name": "GCP"})
    );
    assert_eq!(
        build_update_body(None, None, true, &[4, 5]).unwrap(),
        json!({"websiteUrl": null, "reviewersIds": [4, 5]})
    );
    assert_eq!(
        build_update_body(None, Some("https://x.example"), false, &[]).unwrap(),
        json!({"websiteUrl": "https://x.example"})
    );
}

#[test]
fn empty_update_is_rejected() {
    let err = build_update_body(None, None, false, &[]).unwrap_err();
    assert!(err.to_string().contains("needs --name"), "{err}");
}

#[test]
fn users_body_carries_every_user_passed_and_nothing_else() {
    let users = [
        user("1a2b:jane.doe:jane@acme.com"),
        user("3c4d:bob:bob@acme.com"),
        user("5e6f:eve:eve@acme.com"),
    ];
    let roles = [role("1a2b:Admin"), role("5e6f:Viewer"), role("1a2b:ReadOnly")];
    assert_eq!(
        build_users_body(&users, &roles).unwrap(),
        json!({"identities": [
            {"identityId": "1a2b", "username": "jane.doe", "email": "jane@acme.com", "roles": ["Admin", "ReadOnly"]},
            {"identityId": "3c4d", "username": "bob", "email": "bob@acme.com"},
            {"identityId": "5e6f", "username": "eve", "email": "eve@acme.com", "roles": ["Viewer"]},
        ]})
    );
}

#[test]
fn users_body_rejects_a_repeated_identity_id() {
    let users = [user("1a2b:jane:jane@acme.com"), user("1a2b:janet:janet@acme.com")];
    let err = build_users_body(&users, &[]).unwrap_err();
    assert!(err.to_string().contains("`1a2b` is given twice"), "{err}");
}

#[test]
fn users_body_rejects_a_role_for_an_unknown_user_or_given_twice() {
    let users = [user("1a2b:jane:jane@acme.com")];
    let err = build_users_body(&users, &[role("9z:Admin")]).unwrap_err();
    assert!(err.to_string().contains("which no --user gives"), "{err}");
    let err = build_users_body(&users, &[role("1a2b:Admin"), role("1a2b:Admin")]).unwrap_err();
    assert!(err.to_string().contains("`1a2b:Admin` is given twice"), "{err}");
}

#[test]
fn users_set_prompt_says_omitted_users_are_removed_and_counts_them() {
    let body = build_users_body(&[user("a:b:c@d.e"), user("f:g:h@i.j")], &[]).unwrap();
    let prompt = users_set_prompt("/access-review-applications/7/users", &body);
    assert!(
        prompt.starts_with("/access-review-applications/7/users (") && prompt.contains("full user list with 2 users"),
        "{prompt}"
    );
    assert!(prompt.contains("users omitted from it are removed"), "{prompt}");
    let data = users_set_prompt("/p", &json!({"other": 1}));
    assert!(
        data.contains("the --data body") && data.contains("omitted from it are removed"),
        "{data}"
    );
}

#[test]
fn remove_prompt_names_the_period_side_effect() {
    let prompt = remove_prompt("/access-review-applications/7");
    assert!(
        prompt.starts_with("/access-review-applications/7 (") && prompt.contains("review period"),
        "{prompt}"
    );
}

#[test]
fn example_only_for_body_verbs_with_the_flag() {
    let create = AccessReviewAppAction::Create {
        name: None,
        website_url: None,
        reviewer_ids: vec![],
        data: None,
        example: true,
    };
    let skeleton = example_if_requested(&create).unwrap().unwrap();
    assert!(skeleton.contains("reviewersIds"), "{skeleton}");
    let set = AccessReviewAppAction::Users {
        action: AccessReviewAppUsersAction::Set {
            application_id: None,
            users: vec![],
            roles: vec![],
            data: None,
            example: true,
        },
    };
    let skeleton = example_if_requested(&set).unwrap().unwrap();
    assert!(skeleton.contains("identities"), "{skeleton}");
    assert!(example_if_requested(&AccessReviewAppAction::List).is_none());
    let no_flag = AccessReviewAppAction::Update {
        application_id: Some(1),
        name: Some("x".to_string()),
        website_url: None,
        clear_website_url: false,
        reviewer_ids: vec![],
        data: None,
        example: false,
    };
    assert!(example_if_requested(&no_flag).is_none());
}

#[test]
fn user_flag_parses_exactly_three_parts() {
    assert_eq!(
        user("1a2b: jane.doe :jane@acme.com"),
        ReviewUser {
            identity_id: "1a2b".to_string(),
            username: "jane.doe".to_string(),
            email: "jane@acme.com".to_string(),
        }
    );
}

#[test]
fn user_flag_rejects_a_missing_empty_or_fourth_part() {
    for raw in [
        "1a2b",
        "1a2b:jane",
        "1a2b::jane@acme.com",
        ":jane:jane@acme.com",
        "1a2b:jane: ",
        "1a2b:jane:jane@acme.com:Admin",
    ] {
        let err = review_user(raw).unwrap_err();
        assert!(err.contains("IDENTITY_ID:USERNAME:EMAIL"), "{raw}: {err}");
    }
    assert!(
        review_user("1a2b:jane:jane@acme.com:Admin")
            .unwrap_err()
            .contains("--role")
    );
}

#[test]
fn role_flag_takes_everything_after_the_first_colon_literally() {
    assert_eq!(
        role("1a2b:Admin,ReadOnly"),
        RoleGrant {
            identity_id: "1a2b".to_string(),
            role: "Admin,ReadOnly".to_string(),
        }
    );
    assert_eq!(role("1a2b:org:admin").role, "org:admin");
    for raw in ["1a2b", "1a2b:", ":Admin"] {
        let err = role_grant(raw).unwrap_err();
        assert!(err.contains("IDENTITY_ID:ROLE"), "{raw}: {err}");
    }
}
