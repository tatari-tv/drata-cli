#![allow(clippy::unwrap_used)]
#[cfg(test)]
use super::*;

#[test]
fn monitor_actions_are_constructible() {
    let list = MonitorAction::List {
        workspace_id: "ws1".to_string(),
        all: false,
        draft: None,
        name: None,
    };
    assert!(matches!(list, MonitorAction::List { .. }));

    let get = MonitorAction::Get {
        workspace_id: "ws1".to_string(),
        test_id: "t1".to_string(),
    };
    assert!(matches!(get, MonitorAction::Get { .. }));

    let update = MonitorAction::Update {
        workspace_id: "ws1".to_string(),
        test_id: "t1".to_string(),
        name: None,
        enabled: Some(true),
        description: None,
    };
    assert!(matches!(update, MonitorAction::Update { .. }));

    let excl = MonitorAction::Exclusions {
        workspace_id: "ws1".to_string(),
        test_id: "t1".to_string(),
    };
    assert!(matches!(excl, MonitorAction::Exclusions { .. }));

    let fail = MonitorAction::Failures {
        workspace_id: "ws1".to_string(),
        test_id: "t1".to_string(),
    };
    assert!(matches!(fail, MonitorAction::Failures { .. }));

    let pass = MonitorAction::Passes {
        workspace_id: "ws1".to_string(),
        test_id: "t1".to_string(),
    };
    assert!(matches!(pass, MonitorAction::Passes { .. }));
}

#[test]
fn monitor_list_path_encodes_draft_and_name() {
    assert_eq!(
        build_list_path("1", Some(false), Some("MFA check")),
        "/workspaces/1/monitoring-tests?draft=false&name=MFA%20check"
    );
    assert_eq!(build_list_path("1", None, None), "/workspaces/1/monitoring-tests");
}

use crate::cli::Cli;
use crate::client::ApiError;
use clap::Parser;
use reqwest::StatusCode;

fn parsed(argv: &[&str]) -> MonitorAction {
    let cli = Cli::try_parse_from(std::iter::once("drata").chain(["monitor"]).chain(argv.iter().copied()))
        .expect("argv parses");
    match cli.command {
        crate::cli::Commands::Monitor { action } => action,
        _ => unreachable!("monitor argv"),
    }
}

fn planned(argv: &[&str]) -> Call {
    plan(&parsed(argv)).expect("plans").expect("a planned verb")
}

const RECIPE: &str = r#"{"assertion":"nofail","allConnections":true,"providers":[{"provider":"HR","resources":[]}]}"#;

#[test]
fn test_path_encodes_a_named_draft_as_one_segment() {
    assert_eq!(
        test_path("1", "name:MFA check/v2", "recipe"),
        "/workspaces/1/monitoring-tests/name:MFA%20check%2Fv2/recipe"
    );
    assert_eq!(tests_path("1"), "/workspaces/1/monitoring-tests");
}

#[test]
fn create_body_has_name_recipe_and_optionals_only_when_given() {
    let recipe = json!({"assertion": "nofail"});
    assert_eq!(
        custom::build_create_body(Some("n"), None, &[], Some(recipe.clone())).unwrap(),
        json!({"name": "n", "recipe": recipe})
    );
    assert_eq!(
        custom::build_create_body(Some("n"), Some("d"), &[3, 4], Some(recipe.clone())).unwrap(),
        json!({"name": "n", "description": "d", "connectionIds": [3, 4], "recipe": recipe})
    );
}

#[test]
fn create_body_requires_name_and_recipe() {
    let err = custom::build_create_body(None, None, &[], Some(json!({}))).unwrap_err();
    assert!(err.to_string().contains("--name"), "{err}");
    let err = custom::build_create_body(Some("n"), None, &[], None).unwrap_err();
    assert!(err.to_string().contains("--recipe"), "{err}");
}

#[test]
fn connection_ids_are_capped_at_the_spec_maximum() {
    let ids: Vec<u64> = (0..=custom::MAX_CONNECTION_IDS as u64).collect();
    let err = custom::build_recipe_body(&ids, Some(json!({}))).unwrap_err();
    assert!(err.to_string().contains("300"), "{err}");
}

#[test]
fn recipe_must_be_an_object() {
    assert!(custom::check_recipe(json!({"assertion": "nofail"})).is_ok());
    let err = custom::check_recipe(json!([1])).unwrap_err();
    assert!(err.to_string().contains("JSON object"), "{err}");
}

#[test]
fn validate_and_recipe_bodies_wrap_the_recipe() {
    let recipe = json!({"assertion": "nofail"});
    assert_eq!(
        custom::build_validate_body(Some(recipe.clone())).unwrap(),
        json!({"recipe": recipe})
    );
    assert_eq!(
        custom::build_recipe_body(&[9], Some(recipe.clone())).unwrap(),
        json!({"recipe": recipe, "connectionIds": [9]})
    );
    assert!(custom::build_validate_body(None).is_err());
}

#[test]
fn an_invalid_recipe_fails_after_printing() {
    assert!(custom::check_validation(&json!({"isValid": true, "errors": []})).is_ok());
    let err = custom::check_validation(&json!({"isValid": false, "errors": [{}, {}]})).unwrap_err();
    assert_eq!(err.to_string(), "the recipe is invalid: 2 error(s)");
    let err = custom::check_validation(&json!({"id": 1})).unwrap_err();
    assert!(err.to_string().contains("isValid"), "{err}");
}

fn api_error(status: StatusCode, message: &str) -> eyre::Report {
    ApiError {
        status,
        formatted: format!("API error {status} on POST /x: {message}"),
        body: json!({"message": message}).to_string(),
    }
    .into()
}

#[test]
fn retest_names_409_and_429_apart_and_keeps_the_servers_message() {
    let conflict = format!(
        "{:#}",
        custom::retest_error(api_error(StatusCode::CONFLICT, "run in flight"), "7")
    );
    assert!(conflict.contains("already has a run in flight"), "{conflict}");
    assert!(conflict.contains("run in flight"), "{conflict}");
    let cooldown = format!(
        "{:#}",
        custom::retest_error(api_error(StatusCode::TOO_MANY_REQUESTS, "cooling down"), "7")
    );
    assert!(cooldown.contains("cooldown"), "{cooldown}");
    assert!(cooldown.contains("cooling down"), "{cooldown}");
    assert!(!cooldown.contains("in flight"), "{cooldown}");
    let other = format!(
        "{:#}",
        custom::retest_error(api_error(StatusCode::NOT_FOUND, "no test"), "7")
    );
    assert_eq!(other, "API error 404 Not Found on POST /x: no test");
}

#[test]
fn exclusion_create_body_pairs_ids_with_connection_ids() {
    let failures: Vec<String> = ["sg-1", "4", "sg-2", "null"].iter().map(|s| s.to_string()).collect();
    assert_eq!(
        exclusion::build_create_body(Some("public"), &failures).unwrap(),
        json!({"reason": "public", "exclusions": [
            {"id": "sg-1", "connectionId": 4},
            {"id": "sg-2", "connectionId": null}
        ]})
    );
}

#[test]
fn exclusion_create_rejects_a_bad_connection_id_and_a_missing_reason() {
    let failures: Vec<String> = ["sg-1", "four"].iter().map(|s| s.to_string()).collect();
    let err = exclusion::build_create_body(Some("r"), &failures).unwrap_err();
    assert!(err.to_string().contains("number or `null`"), "{err}");
    let failures: Vec<String> = ["sg-1", "4"].iter().map(|s| s.to_string()).collect();
    assert!(exclusion::build_create_body(None, &failures).is_err());
    assert!(exclusion::build_create_body(Some("r"), &[]).is_err());
}

#[test]
fn codebase_create_and_delete_bodies() {
    let ids = vec!["1|cfg|repo|main.tf".to_string()];
    assert_eq!(
        exclusion::build_codebase_create_body(Some("r"), &ids).unwrap(),
        json!({"reason": "r", "findingIds": ids})
    );
    assert_eq!(
        exclusion::build_delete_body(&[1, 2]).unwrap(),
        json!({"exclusionIds": [1, 2]})
    );
    let too_many: Vec<u64> = (0..=exclusion::MAX_ITEMS as u64).collect();
    assert!(exclusion::build_delete_body(&too_many).is_err());
}

#[test]
fn codebase_list_path_encodes_filters() {
    assert_eq!(
        exclusion::build_codebase_list_path(
            "/workspaces/1/monitoring-tests/2/codebase-exclusions",
            Some("org/repo"),
            &[crate::cli::CodebaseExclusionExpand::ExclusionDesignator],
        ),
        "/workspaces/1/monitoring-tests/2/codebase-exclusions?repositoryName=org/repo&expand%5B%5D=exclusionDesignator"
    );
}

#[test]
fn plan_resolves_every_phase_six_verb() {
    let cases: Vec<(Vec<&str>, &str, &str)> = vec![
        (
            vec!["create", "1", "--name", "n", "--recipe", RECIPE],
            "POST",
            "/workspaces/1/monitoring-tests",
        ),
        (
            vec!["validate-recipe", "1", "--recipe", RECIPE],
            "POST",
            "/workspaces/1/monitoring-tests/action-validate-recipe",
        ),
        (
            vec!["recipe", "1", "2", "--recipe", RECIPE],
            "PUT",
            "/workspaces/1/monitoring-tests/2/recipe",
        ),
        (
            vec!["retest", "1", "2"],
            "POST",
            "/workspaces/1/monitoring-tests/2/action-retest",
        ),
        (
            vec!["exclusion", "create", "1", "2", "--reason", "r", "--failure", "sg", "3"],
            "POST",
            "/workspaces/1/monitoring-tests/2/exclusions",
        ),
        (
            vec!["exclusion", "remove", "1", "2", "--exclusion-id", "5"],
            "POST",
            "/workspaces/1/monitoring-tests/2/exclusions/action-delete",
        ),
        (
            vec!["codebase-exclusion", "list", "1", "2"],
            "GET",
            "/workspaces/1/monitoring-tests/2/codebase-exclusions",
        ),
        (
            vec![
                "codebase-exclusion",
                "create",
                "1",
                "2",
                "--reason",
                "r",
                "--finding-id",
                "f",
            ],
            "POST",
            "/workspaces/1/monitoring-tests/2/codebase-exclusions",
        ),
        (
            vec!["codebase-exclusion", "remove", "1", "2", "--exclusion-id", "5"],
            "POST",
            "/workspaces/1/monitoring-tests/2/codebase-exclusions/action-delete",
        ),
    ];
    for (argv, method, path) in cases {
        let call = planned(&argv);
        assert_eq!((call.method(), call.path()), (method, path), "{argv:?}");
    }
}

#[test]
fn plan_kinds_carry_the_exit_rules() {
    assert!(matches!(planned(&["retest", "1", "2"]), Call::Retest { .. }));
    assert!(matches!(
        planned(&["validate-recipe", "1", "--recipe", RECIPE]),
        Call::Validate { .. }
    ));
    assert!(matches!(
        planned(&["exclusion", "remove", "1", "2", "--exclusion-id", "5"]),
        Call::Bulk { .. }
    ));
    assert!(plan(&parsed(&["get", "1", "2"])).unwrap().is_none());
}

#[test]
fn data_is_the_whole_body() {
    let call = planned(&["recipe", "1", "2", "--data", r#"{"recipe":{"x":1}}"#]);
    match call {
        Call::Put { body, .. } => assert_eq!(body, json!({"recipe": {"x": 1}})),
        other => panic!("{other:?}"),
    }
}

#[test]
fn example_runs_without_ids_and_carries_a_full_recipe() {
    for argv in [
        vec!["create", "--example"],
        vec!["validate-recipe", "--example"],
        vec!["recipe", "--example"],
    ] {
        let out = example_if_requested(&parsed(&argv)).expect("example").expect("renders");
        let body: Value = serde_json::from_str(&out).unwrap();
        assert_eq!(
            body["recipe"]["providers"][0]["resources"][0]["resource"],
            json!("Employees"),
            "{argv:?}: {out}"
        );
    }
    for argv in [
        vec!["exclusion", "create", "--example"],
        vec!["codebase-exclusion", "create", "--example"],
    ] {
        let out = example_if_requested(&parsed(&argv)).expect("example").expect("renders");
        assert!(out.contains("reason"), "{argv:?}: {out}");
    }
}
