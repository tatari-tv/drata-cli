#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::ComputedEvidenceStatus;

fn no_artifacts() -> NewArtifactArgs {
    NewArtifactArgs {
        urls: vec![],
        file_keys: vec![],
        ticket_urls: vec![],
        filed_at: None,
    }
}

fn no_fields() -> EvidenceItemFields {
    EvidenceItemFields {
        description: None,
        implementation_guidance: None,
        steps_to_reproduce: None,
        owner_id: None,
        control_ids: vec![],
        renewal_date: None,
        renewal_schedule_type: None,
    }
}

fn item(workspace_id: &str, evidence_id: u64) -> EvidenceItemRef {
    EvidenceItemRef {
        workspace_id: workspace_id.to_string(),
        evidence_id,
    }
}

fn artifact() -> ArtifactRef {
    ArtifactRef {
        workspace_id: "1".to_string(),
        evidence_id: 2,
        artifact_id: 3,
    }
}

#[test]
fn paths_address_the_evidence_tag_not_the_library() {
    assert_eq!(collection_path("1"), "/workspaces/1/evidence");
    assert_eq!(item_path("1", 2), "/workspaces/1/evidence/2");
    assert_eq!(files_path("1"), "/workspaces/1/evidence-files");
    assert_eq!(artifacts_path(&item("1", 2)), "/workspaces/1/evidence/2/artifacts");
    assert_eq!(artifact_path(&artifact()), "/workspaces/1/evidence/2/artifacts/3");
    assert_eq!(
        artifact_actions_path(&artifact()),
        "/workspaces/1/evidence/2/artifacts/3/actions"
    );
}

#[test]
fn workspace_id_stays_one_segment() {
    assert_eq!(collection_path("a/b"), "/workspaces/a%2Fb/evidence");
    assert_eq!(files_path("a b"), "/workspaces/a%20b/evidence-files");
}

#[test]
fn list_path_encodes_every_filter() {
    let args = EvidenceItemListArgs {
        workspace_id: "1".to_string(),
        name: Some("SOC 2".to_string()),
        owner_ids: vec![4, 5],
        framework_ids: vec![6],
        status: vec![ComputedEvidenceStatus::NeedsArtifact],
        artifact_type: vec![ArtifactType::S3File],
        expand: vec![EvidenceItemExpand::Owners, EvidenceItemExpand::Artifacts],
    };
    assert_eq!(
        list_path(&args),
        "/workspaces/1/evidence?name=SOC%202&ownerIds%5B%5D=4&ownerIds%5B%5D=5&frameworkIds%5B%5D=6\
         &evidenceStatuses%5B%5D=NEEDS_ARTIFACT&artifactTypes%5B%5D=S3_FILE\
         &expand%5B%5D=owners&expand%5B%5D=artifacts"
    );
}

#[test]
fn get_and_artifact_list_paths_carry_their_query() {
    assert_eq!(
        get_path(&item("1", 2), &[EvidenceItemExpand::Controls]),
        "/workspaces/1/evidence/2?expand%5B%5D=controls"
    );
    assert_eq!(get_path(&item("1", 2), &[]), "/workspaces/1/evidence/2");
    assert_eq!(
        artifact_list_path(&item("1", 2), Some("x"), &[ArtifactType::Url], Some(false)),
        "/workspaces/1/evidence/2/artifacts?name=x&artifactTypes%5B%5D=URL&isArchived=false"
    );
}

#[test]
fn create_body_types_each_artifact_and_files_it() {
    let artifacts = NewArtifactArgs {
        urls: vec!["https://example.com/a".to_string()],
        file_keys: vec!["acct/evidence-library/1/f.pdf".to_string()],
        ticket_urls: vec!["https://jira.example.com/browse/SEC-1".to_string()],
        filed_at: Some("2026-11-01".to_string()),
    };
    let mut fields = no_fields();
    fields.owner_id = Some(9);
    fields.control_ids = vec![1, 2];
    fields.renewal_schedule_type = Some(RenewalScheduleType::OneYear);
    let body = build_create_body("Training", &fields, &artifacts).unwrap();
    assert_eq!(
        body,
        json!({
            "name": "Training",
            "ownerId": 9,
            "controlIds": [1, 2],
            "renewalScheduleType": "ONE_YEAR",
            "artifacts": [
                {"type": "URL", "url": "https://example.com/a", "filedAt": "2026-11-01"},
                {"type": "S3_FILE", "fileKey": "acct/evidence-library/1/f.pdf", "filedAt": "2026-11-01"},
                {"type": "TICKET_PROVIDER", "ticketUrl": "https://jira.example.com/browse/SEC-1", "filedAt": "2026-11-01"},
            ],
        })
    );
}

#[test]
fn create_body_always_sends_the_required_artifacts_array() {
    let body = build_create_body("x", &no_fields(), &no_artifacts()).unwrap();
    assert_eq!(body, json!({"name": "x", "artifacts": []}));
}

#[test]
fn filed_at_is_required_with_an_artifact_and_rejected_without_one() {
    let mut artifacts = no_artifacts();
    artifacts.urls = vec!["https://example.com".to_string()];
    let err = build_new_artifacts(&artifacts, "type").unwrap_err();
    assert!(err.to_string().contains("--filed-at is required"), "{err}");
    let mut stray = no_artifacts();
    stray.filed_at = Some("2026-11-01".to_string());
    let err = build_new_artifacts(&stray, "type").unwrap_err();
    assert!(err.to_string().contains("applies only to new artifacts"), "{err}");
}

#[test]
fn update_body_names_new_artifacts_artifact_type_and_maps_archive_and_restore() {
    let artifacts = NewArtifactArgs {
        urls: vec!["https://example.com/b".to_string()],
        file_keys: vec![],
        ticket_urls: vec![],
        filed_at: Some("2026-11-02".to_string()),
    };
    let fields = no_fields();
    let body = build_update_body(&UpdateChanges {
        name: Some("Renamed"),
        fields: &fields,
        artifacts: &artifacts,
        archive: &[1, 2],
        restore: &[3],
    })
    .unwrap();
    assert_eq!(
        body,
        json!({
            "name": "Renamed",
            "newArtifacts": [{"artifactType": "URL", "url": "https://example.com/b", "filedAt": "2026-11-02"}],
            "pastArtifacts": [1, 2],
            "currentArtifacts": [3],
        })
    );
}

#[test]
fn update_rejects_nothing_to_change_and_contradictions() {
    let fields = no_fields();
    let artifacts = no_artifacts();
    let empty = UpdateChanges {
        name: None,
        fields: &fields,
        artifacts: &artifacts,
        archive: &[],
        restore: &[],
    };
    assert!(
        build_update_body(&empty)
            .unwrap_err()
            .to_string()
            .contains("at least one field")
    );
    let both = UpdateChanges {
        archive: &[7],
        restore: &[7],
        ..empty
    };
    assert!(build_update_body(&both).unwrap_err().to_string().contains("artifact 7"));
}

#[test]
fn update_schedule_other_than_none_needs_a_renewal_date() {
    let artifacts = no_artifacts();
    let mut fields = no_fields();
    fields.renewal_schedule_type = Some(RenewalScheduleType::SixMonths);
    let changes = UpdateChanges {
        name: None,
        fields: &fields,
        artifacts: &artifacts,
        archive: &[],
        restore: &[],
    };
    assert!(
        build_update_body(&changes)
            .unwrap_err()
            .to_string()
            .contains("--renewal-date")
    );
    let mut none = no_fields();
    none.renewal_schedule_type = Some(RenewalScheduleType::None);
    let body = build_update_body(&UpdateChanges {
        fields: &none,
        ..changes
    })
    .unwrap();
    assert_eq!(body, json!({"renewalScheduleType": "NONE"}));
}

#[test]
fn artifact_update_body_sets_clears_or_omits_each_field() {
    assert_eq!(
        build_artifact_update_body(Some("n"), false, None, true).unwrap(),
        json!({"artifactName": "n", "filedAt": null})
    );
    assert_eq!(
        build_artifact_update_body(None, true, Some("2026-01-01"), false).unwrap(),
        json!({"artifactName": null, "filedAt": "2026-01-01"})
    );
    assert!(build_artifact_update_body(None, false, None, false).is_err());
}

#[test]
fn artifact_action_body_uses_the_spec_value() {
    assert_eq!(
        build_artifact_action_body(&ArtifactActionKind::Archive),
        json!({"action": "archive"})
    );
    assert_eq!(
        build_artifact_action_body(&ArtifactActionKind::Restore),
        json!({"action": "restore"})
    );
}

#[test]
fn upload_form_is_one_file_part_named_file() {
    let form = build_upload_form(Path::new("/tmp/x.pdf"));
    assert_eq!(form.files.len(), 1);
    assert_eq!(form.files[0].field, "file");
    assert!(form.fields.is_empty());
}

#[test]
fn remove_prompts_say_what_is_deleted() {
    let p = remove_prompt("/workspaces/1/evidence/2");
    assert!(p.starts_with("/workspaces/1/evidence/2 "), "{p}");
    assert!(p.contains("all of its artifacts"), "{p}");
    let p = artifact_remove_prompt("/workspaces/1/evidence/2/artifacts/3");
    assert!(p.contains("version history"), "{p}");
}

#[test]
fn fewer_artifacts_created_than_requested_fails() {
    let partial = json!({"id": 5, "artifactsRequested": 3, "artifactsCreated": 2});
    let err = check_artifacts_created(&partial).unwrap_err().to_string();
    assert!(err.contains("evidence item 5") && err.contains("2 of 3"), "{err}");
    assert!(check_artifacts_created(&json!({"artifactsRequested": 2, "artifactsCreated": 2})).is_ok());
    assert!(check_artifacts_created(&json!({"id": 1})).is_ok());
}

#[test]
fn upload_response_must_carry_a_file_key() {
    assert_eq!(uploaded_file_key(&json!({"fileKey": "k"})).unwrap(), "k");
    assert!(uploaded_file_key(&json!({"id": 1})).is_err());
}

#[test]
fn examples_come_from_the_spec() {
    use clap::Parser;
    for argv in [
        vec!["drata", "evidence-item", "create", "--example"],
        vec!["drata", "evidence-item", "update", "--example"],
        vec!["drata", "evidence-item", "artifact", "update", "--example"],
        vec!["drata", "evidence-item", "artifact", "run-action", "--example"],
    ] {
        let cli = crate::cli::Cli::try_parse_from(&argv).unwrap();
        let crate::cli::Commands::EvidenceItem { action } = &cli.command else {
            panic!("not evidence-item");
        };
        let out = example_if_requested(action)
            .unwrap_or_else(|| panic!("no example for {argv:?}"))
            .unwrap();
        assert!(out.trim_start().starts_with('{'), "{out}");
    }
}
