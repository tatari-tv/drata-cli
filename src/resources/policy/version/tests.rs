#![allow(clippy::unwrap_used)]
use super::*;
use crate::cli::PolicyVersionCreateArgs;

fn args() -> PolicyVersionCreateArgs {
    PolicyVersionCreateArgs {
        policy_id: Some("7".into()),
        source_type: Some(PolicySourceType::Uploaded),
        owner_id: Some(42),
        material_change: Some(true),
        renewal_date: Some("2027-01-01".into()),
        requires_approval: None,
        external_file_id: None,
        file: Some("policy.pdf".into()),
        file_name: None,
        description: None,
        change_summary: None,
        changes_explanation: None,
        requires_acknowledgement: None,
        data: None,
        example: false,
    }
}

fn external() -> PolicyVersionCreateArgs {
    PolicyVersionCreateArgs {
        source_type: Some(PolicySourceType::External),
        external_file_id: Some(2055),
        file: None,
        ..args()
    }
}

fn field<'a>(form: &'a Multipart, name: &str) -> Option<&'a str> {
    form.fields.iter().find(|(k, _)| k == name).map(|(_, v)| v.as_str())
}

#[test]
fn paths_name_the_policy_and_version() {
    assert_eq!(create_path("7"), "/policies/7/policy-versions");
    assert_eq!(replace_file_path("7", "9"), "/policies/7/policy-versions/9/file");
    assert_eq!(create_path("a/b"), "/policies/a%2Fb/policy-versions");
}

#[test]
fn uploaded_create_is_multipart_with_the_specs_field_names() {
    let mut a = args();
    a.requires_approval = Some(false);
    a.change_summary = Some("typo".into());
    let Payload::Multipart(form) = build_create(&a).unwrap() else {
        panic!("a file part must be multipart");
    };
    assert_eq!(form.files.len(), 1);
    assert_eq!(form.files[0].field, "file");
    assert_eq!(field(&form, "sourceType"), Some("UPLOADED"));
    assert_eq!(field(&form, "ownerId"), Some("42"));
    assert_eq!(field(&form, "isMaterialChange"), Some("true"));
    assert_eq!(field(&form, "renewalDate"), Some("2027-01-01"));
    assert_eq!(field(&form, "requiresApproval"), Some("false"));
    assert_eq!(field(&form, "changeSummary"), Some("typo"));
    assert_eq!(field(&form, "description"), None, "unset optionals are not sent");
}

#[test]
fn external_create_is_typed_json() {
    let Payload::Json(body) = build_create(&external()).unwrap() else {
        panic!("no file part must be JSON");
    };
    assert_eq!(
        body,
        serde_json::json!({
            "sourceType": "EXTERNAL",
            "ownerId": 42,
            "isMaterialChange": true,
            "renewalDate": "2027-01-01",
            "externalFileId": 2055,
        })
    );
}

#[test]
fn create_rejects_flags_of_the_other_source_type() {
    let mut no_file = args();
    no_file.file = None;
    assert!(
        build_create(&no_file)
            .unwrap_err()
            .to_string()
            .contains("requires --file")
    );

    let mut both = args();
    both.external_file_id = Some(1);
    assert!(build_create(&both).unwrap_err().to_string().contains("only valid with"));

    let mut ext_file = external();
    ext_file.file = Some("x.pdf".into());
    assert!(
        build_create(&ext_file)
            .unwrap_err()
            .to_string()
            .contains("only valid with")
    );

    let mut ext_none = external();
    ext_none.external_file_id = None;
    assert!(
        build_create(&ext_none)
            .unwrap_err()
            .to_string()
            .contains("requires --external-file-id")
    );
}

#[test]
fn create_requires_the_specs_required_fields_without_data() {
    let mut a = args();
    a.material_change = None;
    assert!(build_create(&a).unwrap_err().to_string().contains("--material-change"));
    let mut a = args();
    a.renewal_date = None;
    assert!(build_create(&a).unwrap_err().to_string().contains("--renewal-date"));
}

#[test]
fn replace_file_form_has_file_and_optional_language() {
    let form = build_replace_file(Path::new("p.pdf"), Some("fr-FR"));
    assert_eq!(form.files.len(), 1);
    assert_eq!(form.files[0].field, "file");
    assert_eq!(field(&form, "language"), Some("fr-FR"));
    let bare = build_replace_file(Path::new("p.pdf"), None);
    assert!(bare.fields.is_empty());
}

#[test]
fn example_only_for_create_with_flag() {
    let mut a = args();
    a.example = true;
    let skeleton = example_if_requested(&PolicyVersionAction::Create(a)).unwrap().unwrap();
    assert!(skeleton.contains("isMaterialChange"), "{skeleton}");
    assert!(example_if_requested(&PolicyVersionAction::Create(args())).is_none());
}
