#![allow(clippy::unwrap_used)]
use super::*;

const COLLECTION: &str = "/vendors/1/security-reviews/2/notes";

#[test]
fn collection_path_names_the_kind() {
    assert_eq!(collection_path("1", 2, Kind::Note), COLLECTION);
    assert_eq!(
        collection_path("1", 2, Kind::Observation),
        "/vendors/1/security-reviews/2/observations"
    );
}

#[test]
fn list_path_adds_assessment_filter_and_expand() {
    assert_eq!(build_list_path(COLLECTION, None, &[]), COLLECTION);
    assert_eq!(
        build_list_path(COLLECTION, Some(9), &["user".to_string()]),
        format!("{COLLECTION}?vendorAssessmentId=9&expand%5B%5D=user")
    );
}

#[test]
fn item_path_appends_the_id() {
    assert_eq!(build_item_path(COLLECTION, 7), format!("{COLLECTION}/7"));
}

#[test]
fn create_body_uses_the_kinds_text_field() {
    assert_eq!(
        build_create_body(Kind::Note, Some("n"), Some("s"), Some(4)).unwrap(),
        json!({"note": "n", "source": "s", "vendorAssessmentId": 4})
    );
    assert_eq!(
        build_create_body(Kind::Observation, Some("o"), None, None).unwrap(),
        json!({"observation": "o"})
    );
}

#[test]
fn create_body_requires_the_text() {
    let err = build_create_body(Kind::Observation, None, Some("s"), None).unwrap_err();
    assert!(err.to_string().contains("--observation"), "{err}");
}

#[test]
fn update_body_is_sparse_and_can_clear_source() {
    assert_eq!(
        build_update_body(Kind::Note, Some("n"), None, false).unwrap(),
        json!({"note": "n"})
    );
    assert_eq!(
        build_update_body(Kind::Note, None, None, true).unwrap(),
        json!({"source": null})
    );
    assert_eq!(
        build_update_body(Kind::Observation, None, Some("s"), false).unwrap(),
        json!({"source": "s"})
    );
}

#[test]
fn empty_update_is_an_error() {
    let err = build_update_body(Kind::Note, None, None, false).unwrap_err();
    assert!(err.to_string().contains("--clear-source"), "{err}");
}

#[test]
fn note_create_without_ids_is_an_error() {
    let action = SecurityReviewNoteAction::Create {
        vendor_id: None,
        security_review_id: None,
        note: None,
        source: None,
        assessment_id: None,
        data: None,
        example: true,
    };
    assert!(note_target(&action).is_err());
}

#[test]
fn observation_remove_targets_the_item() {
    let action = SecurityReviewObservationAction::Remove {
        vendor_id: "1".into(),
        security_review_id: 2,
        observation_id: 3,
    };
    let target = observation_target(&action).unwrap();
    assert_eq!(target.collection, "/vendors/1/security-reviews/2/observations");
    assert!(matches!(target.op, Op::Remove { id: 3 }));
}

#[test]
fn examples_come_from_the_spec_for_each_kind() {
    let note = SecurityReviewNoteAction::Create {
        vendor_id: None,
        security_review_id: None,
        note: None,
        source: None,
        assessment_id: None,
        data: None,
        example: true,
    };
    let text = example_if_requested(Some(&note), None).unwrap().unwrap();
    assert!(text.contains("\"note\""), "{text}");
    let obs = SecurityReviewObservationAction::Update {
        vendor_id: None,
        security_review_id: None,
        observation_id: None,
        observation: None,
        source: None,
        clear_source: false,
        data: None,
        example: true,
    };
    let text = example_if_requested(None, Some(&obs)).unwrap().unwrap();
    assert!(text.contains("\"observation\""), "{text}");
}

#[test]
fn no_example_without_the_flag() {
    let note = SecurityReviewNoteAction::Remove {
        vendor_id: "1".into(),
        security_review_id: 2,
        note_id: 3,
    };
    assert!(example_if_requested(Some(&note), None).is_none());
}

#[test]
fn verb_names_the_op_without_its_text() {
    let create = Op::Create {
        text: Some("private-text"),
        source: None,
        assessment_id: None,
        data: None,
    };
    assert_eq!(create.verb(), "create");
    assert!(
        format!("{create:?}").contains("private-text"),
        "the op itself carries the text"
    );
    let update = Op::Update {
        id: 1,
        text: None,
        source: None,
        clear_source: false,
        data: Some("{}"),
    };
    assert_eq!(update.verb(), "update");
    assert_eq!(
        Op::List {
            assessment_id: None,
            expand: vec![]
        }
        .verb(),
        "list"
    );
    assert_eq!(Op::Get { id: 1, expand: vec![] }.verb(), "get");
    assert_eq!(Op::Remove { id: 1 }.verb(), "remove");
}
