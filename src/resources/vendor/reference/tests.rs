#![allow(clippy::unwrap_used)]
use super::*;

#[test]
fn criteria_path_encodes_every_filter() {
    let path = build_criteria_path(
        Some("data"),
        &[ImpactLevel::Major, ImpactLevel::Critical],
        &[3],
        &[4, 5],
        Some(false),
        Some(9),
        &[CriterionType::AiGenerated],
        &[Severity::Low],
        &[CriteriaExpand::VendorTypes, CriteriaExpand::QuestionnaireFile],
    );
    assert_eq!(
        path,
        "/vendor-criteria?name=data\
         &impactLevels%5B%5D=MAJOR&impactLevels%5B%5D=CRITICAL\
         &vendorRiskLevelIds%5B%5D=3\
         &vendorTypeIds%5B%5D=4&vendorTypeIds%5B%5D=5\
         &isDraft=false&securityReviewId=9\
         &criterionTypes%5B%5D=AI_GENERATED\
         &severities%5B%5D=LOW\
         &expand%5B%5D=vendorTypes&expand%5B%5D=questionnaireFile"
    );
}

#[test]
fn criteria_path_without_filters_is_bare() {
    assert_eq!(
        build_criteria_path(None, &[], &[], &[], None, None, &[], &[], &[]),
        "/vendor-criteria"
    );
}

#[test]
fn criteria_creation_path_is_workspace_scoped() {
    assert_eq!(
        build_criteria_creation_path("7", &[CriteriaCreationStatus::InProgress]),
        "/workspaces/7/vendor-criteria-creations?statuses%5B%5D=IN_PROGRESS"
    );
}

#[test]
fn template_paths() {
    assert_eq!(
        build_template_path(
            Some("sec"),
            Some(&TemplateStatus::Active),
            &[VendorRisk::High],
            &[VendorCategory::Security]
        ),
        "/vendor-questionnaire-templates?title=sec&status=ACTIVE\
         &riskLevels%5B%5D=HIGH&categories%5B%5D=SECURITY"
    );
    assert_eq!(
        build_template_questions_path("12"),
        "/vendor-questionnaire-templates/12/questions"
    );
}

#[test]
fn security_reviews_path_is_cross_vendor() {
    let path = build_security_reviews_path(
        &[SecurityReviewStatus::InProgress],
        &[SecurityReviewType::SocReport],
        &[SecurityReviewDecision::ApprovedWithConditions],
        Some("2026-01-01"),
        None,
        None,
        Some("2026-12-31"),
        &[8, 9],
        &["vendor".to_string()],
    );
    assert_eq!(
        path,
        "/vendor-security-reviews?status%5B%5D=IN_PROGRESS&type%5B%5D=SOC_REPORT\
         &decision%5B%5D=APPROVED_WITH_CONDITIONS\
         &createdAtFrom=2026-01-01&updatedAtTo=2026-12-31\
         &vendorIds%5B%5D=8&vendorIds%5B%5D=9&expand%5B%5D=vendor"
    );
}
