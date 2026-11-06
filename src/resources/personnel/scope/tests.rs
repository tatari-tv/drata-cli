#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn paths_encode_the_workspace_and_name_the_group() {
    assert_eq!(groups_path("1"), "/workspaces/1/scoped-personnel-groups");
    assert_eq!(counts_path("1"), "/workspaces/1/scoped-personnel-counts");
    assert_eq!(group_path("1", 42), "/workspaces/1/scoped-personnel-groups/42");
    assert_eq!(groups_path("a/b"), "/workspaces/a%2Fb/scoped-personnel-groups");
}

#[test]
fn add_body_is_one_group_id() {
    assert_eq!(add_body(42), json!({"groupId": 42}));
}

#[test]
fn replace_body_carries_every_group_and_nothing_else() {
    assert_eq!(replace_body(&[1, 2, 3]), json!({"groupIds": [1, 2, 3]}));
    assert_eq!(replace_body(&[]), json!({"groupIds": []}));
}

#[test]
fn replace_prompt_says_unlisted_groups_are_removed() {
    let p = replace_prompt("/workspaces/1/scoped-personnel-groups", &[1, 2]);
    assert!(p.contains("exactly groups 1, 2"), "{p}");
    assert!(p.contains("groups not listed are removed"), "{p}");
    assert!(replace_prompt("/p", &[]).contains("the scope is cleared"));
}
