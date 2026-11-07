#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::{ExamplePeriodRef, reviewed_application};

fn target() -> PeriodAppRef {
    PeriodAppRef {
        period_id: 3,
        review_app_id: 9,
    }
}

#[test]
fn paths_address_the_period_and_its_applications() {
    assert_eq!(item_path(3), "/access-review-periods/3");
    assert_eq!(actions_path(3), "/access-review-periods/3/actions");
    assert_eq!(evidence_package_path(3), "/access-review-periods/3/evidence-package");
    assert_eq!(app_path(&target()), "/access-review-periods/3/applications/9");
    assert_eq!(
        app_actions_path(&target()),
        "/access-review-periods/3/applications/9/actions"
    );
    assert_eq!(
        app_users_path(&target()),
        "/access-review-periods/3/applications/9/users"
    );
}

#[test]
fn list_path_carries_the_status_filter() {
    assert_eq!(list_path(None), "/access-review-periods");
    assert_eq!(
        list_path(Some(&AccessReviewPeriodStatus::Completed)),
        "/access-review-periods?status=COMPLETED"
    );
}

#[test]
fn create_body_scopes_each_application_with_its_reviewers() {
    let apps = [
        reviewed_application("1:10:11").unwrap(),
        reviewed_application("2:12").unwrap(),
    ];
    assert_eq!(
        build_create_body("2026-11-01T00:00:00Z", "2026-12-01T00:00:00Z", &apps).unwrap(),
        json!({
            "startingDate": "2026-11-01T00:00:00Z",
            "endingDate": "2026-12-01T00:00:00Z",
            "applications": [
                {"applicationId": 1, "reviewersIds": [10, 11]},
                {"applicationId": 2, "reviewersIds": [12]},
            ],
        })
    );
}

#[test]
fn create_body_rejects_an_application_given_twice() {
    let apps = [
        reviewed_application("1:10").unwrap(),
        reviewed_application("1:11").unwrap(),
    ];
    let err = build_create_body("a", "b", &apps).unwrap_err();
    assert!(err.to_string().contains("application 1 is given twice"), "{err}");
}

#[test]
fn period_action_body_is_the_action_alone() {
    assert_eq!(
        build_action_body(&AccessReviewActionKind::Complete),
        json!({"action": "complete"})
    );
}

#[test]
fn app_action_body_sends_the_sweep_flag_only_when_set() {
    assert_eq!(
        build_app_action_body(&AccessReviewActionKind::Complete, false),
        json!({"action": "complete"})
    );
    assert_eq!(
        build_app_action_body(&AccessReviewActionKind::Complete, true),
        json!({"action": "complete", "markUnreviewedUsersOutOfScope": true})
    );
}

#[test]
fn app_update_body_replaces_the_reviewers() {
    assert_eq!(build_app_update_body(&[4, 5]), json!({"reviewersIds": [4, 5]}));
}

#[test]
fn prompts_name_the_one_active_period_and_the_out_of_scope_sweep() {
    let create = create_prompt();
    assert!(
        create.starts_with("/access-review-periods (") && create.contains("only one may be active"),
        "{create}"
    );
    let path = "/access-review-periods/3/applications/9/actions";
    let sweep = app_action_prompt(
        path,
        &json!({"action": "complete", "markUnreviewedUsersOutOfScope": true}),
    );
    assert!(sweep.starts_with(path) && sweep.contains("OUT_OF_SCOPE"), "{sweep}");
    assert_eq!(app_action_prompt(path, &json!({"action": "complete"})), path);
}

#[test]
fn example_only_for_body_verbs_with_the_flag() {
    let run = AccessReviewPeriodAction::RunAction {
        target: ExamplePeriodRef { period_id: None },
        action: None,
        data: None,
        example: true,
    };
    let skeleton = example_if_requested(&run).unwrap().unwrap();
    assert!(skeleton.contains("complete"), "{skeleton}");
    let create = AccessReviewPeriodAction::Create {
        starting_date: None,
        ending_date: None,
        applications: vec![],
        data: None,
        example: true,
    };
    let skeleton = example_if_requested(&create).unwrap().unwrap();
    assert!(skeleton.contains("startingDate"), "{skeleton}");
    assert!(example_if_requested(&AccessReviewPeriodAction::Get { period_id: 1 }).is_none());
}

#[test]
fn application_flag_parses_an_id_and_its_reviewers() {
    assert_eq!(
        reviewed_application("7:1: 2:3").unwrap(),
        ReviewedApplication {
            application_id: 7,
            reviewer_ids: vec![1, 2, 3],
        }
    );
}

#[test]
fn application_flag_rejects_no_reviewers_and_non_numeric_ids() {
    for raw in ["7", "7:", "x:1", "7:1:y", "7:1:"] {
        let err = reviewed_application(raw).unwrap_err();
        assert!(
            err.contains("APPLICATION_ID:REVIEWER_ID[:REVIEWER_ID...]"),
            "{raw}: {err}"
        );
    }
}

#[test]
fn application_flag_never_splits_on_a_comma() {
    let err = reviewed_application("7:10,11").unwrap_err();
    assert!(
        err.starts_with("`10,11` is not a numeric reviewer id") && err.contains("REVIEWER_ID[:REVIEWER_ID...]"),
        "{err}"
    );
}
