# Drata API v2 - Endpoint Index

Generated from `spec/drata-openapi-v2.json` (OpenAPI 3.0.0, spec version V2).

Servers:
- `https://public-api.drata.com/public/v2`
- `https://public-api.eu.drata.com/public/v2`
- `https://public-api.apac.drata.com/public/v2`

## Access Reviews

| Method | Path | Summary |
|---|---|---|
| `GET` | `/access-review-applications` | List Access Review Applications |
| `POST` | `/access-review-applications` | Create Access Review Application |
| `GET` | `/access-review-applications/{applicationId}` | Get Access Review Application |
| `PUT` | `/access-review-applications/{applicationId}` | Update Access Review Application |
| `DELETE` | `/access-review-applications/{applicationId}` | Delete Access Review Application |
| `GET` | `/access-review-applications/{applicationId}/users` | List Access Review Application Users |
| `PUT` | `/access-review-applications/{applicationId}/users` | Create or Update Access Review Application Users |
| `GET` | `/access-review-periods` | List Access Reviews |
| `POST` | `/access-review-periods` | Conduct Access Review |
| `GET` | `/access-review-periods/{periodId}` | Get Access Review |
| `GET` | `/access-review-periods/{periodId}/applications/{reviewAppId}` | Get Application in Access Review |
| `PUT` | `/access-review-periods/{periodId}/applications/{reviewAppId}` | Update Application Reviewers in Access Review |
| `GET` | `/access-review-periods/{periodId}/applications/{reviewAppId}/users` | List Application Users in Access Review |
| `GET` | `/access-review-periods/{periodId}/evidence-package` | Download Access Review Evidence Package |
| `POST` | `/access-review-periods/{periodId}/applications/{reviewAppId}/actions` | Perform Access Review Application Action |
| `POST` | `/access-review-periods/{periodId}/actions` | Perform Access Review Action |

## Assets

| Method | Path | Summary |
|---|---|---|
| `GET` | `/assets` | List Assets |
| `POST` | `/assets` | Create Asset |
| `GET` | `/assets/{assetId}` | Get Asset |
| `PUT` | `/assets/{assetId}` | Update Asset |
| `DELETE` | `/assets/{assetId}` | Remove Asset |

## Audit Requests

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}/dcf-suggestions` | List Audit Request DCF Suggestions 🧪 |
| `GET` | `/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}/dcf-suggestions/{suggestionId}` | Get Audit Request DCF Suggestion 🧪 |
| `POST` | `/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}/dcf-suggestions/{suggestionId}/actions` | Decide an Audit Request DCF Suggestion 🧪 |
| `POST` | `/workspaces/{workspaceId}/audits/{auditId}/requests/action-accept-dcf-suggestions` | Accept Audit Request DCF Suggestions 🧪 |
| `POST` | `/workspaces/{workspaceId}/audits/{auditId}/requests/action-generate-dcf-suggestions` | Generate Audit Request DCF Suggestions 🧪 |
| `GET` | `/workspaces/{workspaceId}/audits/{auditId}/requests` | List Audit Requests |
| `GET` | `/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}` | Get Audit Request |

## Audits

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/audits` | List Audits |
| `GET` | `/workspaces/{workspaceId}/audits/{auditId}` | Get Audit |

## Background Checks

| Method | Path | Summary |
|---|---|---|
| `POST` | `/background-checks` | Create Manual Background Check |

## Company

| Method | Path | Summary |
|---|---|---|
| `GET` | `/company` | Get Company |

## Compliance Check Exclusions

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/compliance-check-exclusions` | List Compliance Check Exclusions |
| `POST` | `/workspaces/{workspaceId}/compliance-check-exclusions` | Exclude Personnel Compliance Checks |
| `POST` | `/workspaces/{workspaceId}/compliance-check-exclusions/action-delete` | Delete Personnel Compliance Check Exclusions |

## Control Notes

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/controls/{controlId}/notes` | List Control Notes |
| `POST` | `/workspaces/{workspaceId}/controls/{controlId}/notes` | Create Control Note |
| `GET` | `/workspaces/{workspaceId}/controls/{controlId}/notes/{noteId}` | Get Control Note |
| `PUT` | `/workspaces/{workspaceId}/controls/{controlId}/notes/{noteId}` | Update Control Note |
| `DELETE` | `/workspaces/{workspaceId}/controls/{controlId}/notes/{noteId}` | Delete Control Note |

## Control Owners

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/controls/{controlId}/owners` | List Control Owners |
| `POST` | `/workspaces/{workspaceId}/controls/{controlId}/owners` | Add Control Owner |
| `PUT` | `/workspaces/{workspaceId}/controls/{controlId}/owners` | Modify Control Owners |
| `DELETE` | `/workspaces/{workspaceId}/controls/{controlId}/owners/{ownerId}` | Delete Control Owner |

## Control Tickets

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/controls/{controlId}/tickets` | List Control Tickets |
| `POST` | `/workspaces/{workspaceId}/controls/{controlId}/tickets` | Link Control Ticket |
| `DELETE` | `/workspaces/{workspaceId}/controls/{controlId}/tickets/{ticketId}` | Unlink Control Ticket |

## Controls

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/controls` | List Controls |
| `POST` | `/workspaces/{workspaceId}/controls` | Create Control |
| `GET` | `/workspaces/{workspaceId}/controls/{controlId}` | Get Control |
| `PUT` | `/workspaces/{workspaceId}/controls/{controlId}` | Modify Control |
| `GET` | `/workspaces/{workspaceId}/controls/{controlId}/requirements` | List Requirements for Control |
| `POST` | `/workspaces/{workspaceId}/controls/action-reset-mappings` | Reset Control Requirement Mappings |
| `POST` | `/workspaces/{workspaceId}/controls/{controlId}/actions` | Perform Control Action |
| `GET` | `/workspaces/{workspaceId}/controls-requirement-comparison` | Compare Control Requirements |

## Custom Connections

| Method | Path | Summary |
|---|---|---|
| `GET` | `/custom-connections` | List Custom Connections |
| `POST` | `/custom-connections` | Create Custom Connection |
| `GET` | `/custom-connections/{connectionId}` | Get Custom Connection |
| `PUT` | `/custom-connections/{connectionId}` | Update Custom Connection |
| `DELETE` | `/custom-connections/{connectionId}` | Delete Custom Connection |
| `GET` | `/workspaces/{workspaceId}/custom-connections/{connectionId}/resources/{resourceId}/attributes` | Get Custom Connection Resource Attributes |

## Custom Data Records

| Method | Path | Summary |
|---|---|---|
| `GET` | `/custom-connections/{connectionId}/resources/{resourceId}/records` | List Custom Data Records |
| `POST` | `/custom-connections/{connectionId}/resources/{resourceId}/records` | Upsert Custom Data Records |
| `GET` | `/custom-connections/{connectionId}/resources/{resourceId}/sessions` | List Custom Data Sessions |
| `POST` | `/custom-connections/{connectionId}/resources/{resourceId}/sessions/{sessionId}` | Upsert Custom Data Records with Session Management |
| `POST` | `/custom-connections/{connectionId}/resources/{resourceId}/sessions/{sessionId}/actions` | Perform Session Action |
| `PUT` | `/custom-connections/{connectionId}/resources/{resourceId}/records/{recordId}` | Update Custom Data Record |
| `DELETE` | `/custom-connections/{connectionId}/resources/{resourceId}/records/{recordId}` | Delete Custom Connection Data Record |

## Custom Field Definitions

| Method | Path | Summary |
|---|---|---|
| `GET` | `/custom-field-definitions` | List Custom Field Definitions |
| `POST` | `/custom-field-definitions` | Create Custom Field Definition |
| `GET` | `/custom-field-definitions/{customFieldId}` | Get Custom Field Definition |

## Device Documents

| Method | Path | Summary |
|---|---|---|
| `GET` | `/devices/{deviceId}/documents` | List Device Documents |
| `POST` | `/devices/{deviceId}/documents` | Upload Device Document |
| `GET` | `/devices/{deviceId}/documents/{documentId}` | Get Device Document |
| `DELETE` | `/devices/{deviceId}/documents/{documentId}` | Delete Device Document |

## Devices

| Method | Path | Summary |
|---|---|---|
| `GET` | `/devices` | List Devices |
| `GET` | `/personnel/{personnelId}/devices` | List Devices for Personnel |
| `GET` | `/devices/{deviceId}` | Get Device |
| `GET` | `/connections/{connectionId}/devices` | List Devices for Connection 🧪 |
| `GET` | `/connections/{connectionId}/devices` | List Devices for Connection 🧪 |
| `GET` | `/devices/{deviceId}/apps` | List Apps for Device |
| `POST` | `/custom-connections/{connectionId}/devices` | Create or update Device for Custom Connection 🧪 |
| `DELETE` | `/custom-connections/{connectionId}/devices/{deviceId}` | Delete Device |

## Events

| Method | Path | Summary |
|---|---|---|
| `GET` | `/events` | List Events |
| `GET` | `/events/{eventId}` | Get Event |
| `POST` | `/events/{eventId}/download-jobs` | Create Event PDF Download Job |
| `GET` | `/events/{eventId}/download-jobs/{jobId}` | Get Event PDF Export |

## Evidence

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/evidence` | List Evidence Items |
| `POST` | `/workspaces/{workspaceId}/evidence` | Create Evidence |
| `POST` | `/workspaces/{workspaceId}/evidence-files` | Upload Evidence Artifact File |
| `GET` | `/workspaces/{workspaceId}/evidence/{evidenceId}` | Get Evidence Item |
| `PUT` | `/workspaces/{workspaceId}/evidence/{evidenceId}` | Update Evidence |
| `DELETE` | `/workspaces/{workspaceId}/evidence/{evidenceId}` | Delete Evidence |
| `GET` | `/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts` | List Evidence Artifacts |
| `PUT` | `/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}` | Update Evidence Artifact |
| `DELETE` | `/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}` | Delete Evidence Artifact |
| `POST` | `/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}/actions` | Perform Evidence Artifact Action |

## Evidence Library

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/evidence-library` | List Evidence Library Items |
| `POST` | `/workspaces/{workspaceId}/evidence-library` | Create Evidence Library Item |
| `GET` | `/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}` | Get Evidence Library Item |
| `PUT` | `/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}` | Update Evidence Library Item |
| `DELETE` | `/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}` | Delete Evidence Library Item |
| `GET` | `/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}/versions/{versionId}` | Get Evidence Library Version |

## Frameworks

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/frameworks` | List Frameworks |
| `POST` | `/workspaces/{workspaceId}/frameworks` | Create Custom Framework |
| `GET` | `/workspaces/{workspaceId}/framework-requirements` | List Framework Requirements |
| `PUT` | `/workspaces/{workspaceId}/framework-requirements/{frameworkRequirementId}` | Update Framework Requirement |
| `GET` | `/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements` | List Framework Requirements |
| `POST` | `/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements` | Create Framework Requirements |
| `GET` | `/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements/{requirementId}/controls` | List Controls for a Framework Requirement |
| `PUT` | `/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements/{requirementId}` | Update Framework Requirement |
| `PUT` | `/workspaces/{workspaceId}/frameworks/{frameworkId}` | Update Custom Framework |

## Groups

| Method | Path | Summary |
|---|---|---|
| `GET` | `/groups` | List Groups |

## HRIS User Identities

| Method | Path | Summary |
|---|---|---|
| `GET` | `/custom-connections/{connectionId}/hris-user-identities` | List HRIS User Identities |
| `POST` | `/custom-connections/{connectionId}/hris-user-identities` | Create or Update HRIS User Identities |
| `GET` | `/custom-connections/{connectionId}/hris-user-identities/{userIdentityId}` | Get HRIS User Identity |
| `DELETE` | `/custom-connections/{connectionId}/hris-user-identities/{userIdentityId}` | Delete HRIS User Identity |

## Infrastructure User Identities

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/infrastructure-user-identities` | List Infrastructure User Identities |
| `GET` | `/workspaces/{workspaceId}/infrastructure-user-identities/{identityId}` | Get Infrastructure User Identity |
| `GET` | `/workspaces/{workspaceId}/infrastructure-user-identities/{identityId}/actions` | List Infrastructure User Identity Actions |
| `POST` | `/workspaces/{workspaceId}/infrastructure-user-identities/{identityId}/actions` | Perform Infrastructure User Identity Action |

## Monitoring Test Tickets

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets` | List Monitoring Test Tickets |
| `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets` | Link Monitoring Test Ticket |
| `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets/action-create` | Create Monitoring Test Ticket |
| `DELETE` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets/{ticketId}` | Unlink Monitoring Test Ticket |

## Monitoring Tests

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/monitoring-tests` | List Monitoring Tests |
| `POST` | `/workspaces/{workspaceId}/monitoring-tests` | Create Custom Monitoring Test |
| `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}` | Get Monitoring Test |
| `PUT` | `/workspaces/{workspaceId}/monitoring-tests/{testId}` | Update Monitoring Test |
| `POST` | `/workspaces/{workspaceId}/monitoring-tests/action-validate-recipe` | Validate Custom Monitoring Test Recipe |
| `PUT` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/recipe` | Update Custom Monitoring Test Recipe |
| `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions` | List Monitoring Test Exclusions |
| `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions` | Create Monitoring Test Exclusions |
| `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions/action-delete` | Delete Monitoring Test Exclusions |
| `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions` | List Monitoring Test Codebase Exclusions |
| `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions` | Create Monitoring Test Codebase Exclusions |
| `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions/action-delete` | Delete Monitoring Test Codebase Exclusions |
| `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/failures` | List Monitoring Test Failures |
| `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/passes` | List Monitoring Test Passes |
| `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/action-retest` | Run Monitoring Test |

## Personnel

| Method | Path | Summary |
|---|---|---|
| `GET` | `/personnel` | List Personnel |
| `GET` | `/personnel-search` | Search Personnel across all workspaces |
| `GET` | `/workspaces/{workspaceId}/personnel-search` | Search Personnel within a workspace |
| `GET` | `/personnel/{personnelId}` | Get Personnel |
| `PUT` | `/personnel/{personnelId}` | Update Personnel |
| `POST` | `/personnel/actions` | Reset Personnel Sync |
| `GET` | `/workspaces/{workspaceId}/scoped-personnel-groups` | List Workspace Scoped Personnel Groups 🧪 |
| `POST` | `/workspaces/{workspaceId}/scoped-personnel-groups` | Add Personnel Group To Workspace Scope 🧪 |
| `PUT` | `/workspaces/{workspaceId}/scoped-personnel-groups` | Replace Workspace Scoped Personnel Groups 🧪 |
| `DELETE` | `/workspaces/{workspaceId}/scoped-personnel-groups/{groupId}` | Remove Personnel Group From Workspace Scope 🧪 |
| `GET` | `/workspaces/{workspaceId}/scoped-personnel-counts` | Get Workspace Scoped Personnel Counts 🧪 |

## Policies

| Method | Path | Summary |
|---|---|---|
| `GET` | `/policies` | List Policies |
| `POST` | `/policies` | Create Policy |
| `GET` | `/policies/{policyId}/policy-versions` | List Policy Versions |
| `POST` | `/policies/{policyId}/policy-versions` | Create Policy Version |
| `PUT` | `/policies/{policyId}/policy-versions/{policyVersionId}/file` | Replace Policy Version File |
| `GET` | `/policies/{policyId}` | Get Policy |
| `PUT` | `/policies/{policyId}` | Modify Policy |
| `PUT` | `/policies/{policyId}/owner` | Assign Policy Owner |
| `GET` | `/policies/{policyId}/approval-configuration` | Get Policy Approval Configuration |
| `POST` | `/policies/{policyId}/approval-configuration` | Add Review Group Configuration |
| `PUT` | `/policies/{policyId}/approval-configuration/{approvalConfigurationTier}` | Update Review Group Configuration |
| `DELETE` | `/policies/{policyId}/approval-configuration/{approvalConfigurationTier}` | Remove Review Group Configuration |
| `GET` | `/policies/{policyId}/actions` | List Policy Actions |
| `POST` | `/policies/{policyId}/actions` | Perform Policy Action |
| `GET` | `/policies/{policyId}/policy-versions/{policyVersionId}` | Get Policy Version |

## Policy Languages

| Method | Path | Summary |
|---|---|---|
| `GET` | `/policy-language-settings` | List Policy Language Settings |
| `GET` | `/policy-language-versions` | List Policy Language Versions |
| `GET` | `/policies/{policyId}/policy-versions/{policyVersionId}/languages` | List Policy Version Languages |
| `GET` | `/policy-language-versions/{policyLanguageVersionId}` | Get Policy Language Version |

## Procurement Connection Mappings

| Method | Path | Summary |
|---|---|---|
| `GET` | `/connections/{connectionId}/vendor-mapping` | Get Vendor Mapping |
| `PUT` | `/connections/{connectionId}/vendor-mapping` | Update Vendor Mapping |

## Risk Documents

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}/documents` | List Risk Documents |
| `POST` | `/risk-registers/{riskRegisterId}/risks/{riskId}/documents` | Upload Risk Documents |
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}/documents/{documentId}` | Get Risk Document |
| `DELETE` | `/risk-registers/{riskRegisterId}/risks/{riskId}/documents/{documentId}` | Delete Risk Document |

## Risk Library

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers/{riskRegisterId}/risk-library` | List Risk Libraries |
| `GET` | `/risk-registers/{riskRegisterId}/risk-library/{riskLibraryId}` | Get Risk Library Item by ID. |
| `POST` | `/risk-library/action-copy` | Copies Risk Library Items to a Risk Register. |

## Risk Notes

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}/notes` | List Risk Notes |
| `POST` | `/risk-registers/{riskRegisterId}/risks/{riskId}/notes` | Create Risk Note |
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}/notes/{noteId}` | Get Risk Note |
| `PUT` | `/risk-registers/{riskRegisterId}/risks/{riskId}/notes/{noteId}` | Update Risk Note |
| `DELETE` | `/risk-registers/{riskRegisterId}/risks/{riskId}/notes/{noteId}` | Delete Risk Note |

## Risk Registers

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers` | List Risk Registers |
| `POST` | `/risk-registers` | Create Risk Register |
| `GET` | `/risk-registers/{riskRegisterId}` | Get Risk Register |
| `PUT` | `/risk-registers/{riskRegisterId}` | Update Risk Register |
| `DELETE` | `/risk-registers/{riskRegisterId}` | Delete Risk Register |

## Risk Tickets

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}/tickets` | List Risk Tickets |
| `POST` | `/risk-registers/{riskRegisterId}/risks/{riskId}/tickets` | Link Risk Ticket |
| `POST` | `/risk-registers/{riskRegisterId}/risks/{riskId}/tickets/action-create` | Create Risk Ticket |
| `DELETE` | `/risk-registers/{riskRegisterId}/risks/{riskId}/tickets/{ticketId}` | Unlink Risk Ticket |

## Risks

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers/{riskRegisterId}/risks` | List Risks |
| `POST` | `/risk-registers/{riskRegisterId}/risks` | Create Risk |
| `GET` | `/risk-registers/{riskRegisterId}/risks-search` | Search Risks by Risk Register |
| `GET` | `/risks-search` | Search Risks |
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}` | Get Risk |
| `PUT` | `/risk-registers/{riskRegisterId}/risks/{riskId}` | Update Risk |
| `DELETE` | `/risk-registers/{riskRegisterId}/risks/{riskId}` | Delete Risk |
| `GET` | `/risk-registers/{riskRegisterId}/insights` | Get Risk Insights |

## Standard Connections

| Method | Path | Summary |
|---|---|---|
| `GET` | `/aws-connections` | List AWS Connections |
| `POST` | `/aws-connections` | Create AWS Connection |
| `GET` | `/aws-connections/{connectionId}` | Get AWS Connection |
| `PUT` | `/aws-connections/{connectionId}` | Update AWS Connection |
| `DELETE` | `/aws-connections/{connectionId}` | Delete AWS Connection |
| `GET` | `/azure-connections` | List Azure Connections |
| `POST` | `/azure-connections` | Create Azure Connection |
| `GET` | `/azure-connections/{connectionId}` | Get Azure Connection |
| `PUT` | `/azure-connections/{connectionId}` | Update Azure Connection |
| `DELETE` | `/azure-connections/{connectionId}` | Delete Azure Connection |
| `GET` | `/gcp-connections` | List GCP Connections |
| `POST` | `/gcp-connections` | Create GCP Connection |
| `GET` | `/gcp-connections/{connectionId}` | Get GCP Connection |
| `PUT` | `/gcp-connections/{connectionId}` | Update GCP Connection |
| `DELETE` | `/gcp-connections/{connectionId}` | Delete GCP Connection |
| `GET` | `/gitlab-connections` | List GitLab Connections |
| `POST` | `/gitlab-connections` | Create GitLab Connection |
| `GET` | `/gitlab-connections/{connectionId}` | Get GitLab Connection |
| `PUT` | `/gitlab-connections/{connectionId}` | Update GitLab Connection |
| `DELETE` | `/gitlab-connections/{connectionId}` | Delete GitLab Connection |

## Tasks

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/tasks` | List Tasks |
| `POST` | `/workspaces/{workspaceId}/tasks` | Create Task |
| `GET` | `/workspaces/{workspaceId}/tasks/{taskId}` | Get Task |
| `PUT` | `/workspaces/{workspaceId}/tasks/{taskId}` | Update Task |
| `POST` | `/workspaces/{workspaceId}/tasks/{taskId}/actions` | Perform Task Action |
| `GET` | `/workspaces/{workspaceId}/upcoming-tasks` | List Upcoming Tasks |

## Uploads

| Method | Path | Summary |
|---|---|---|
| `POST` | `/upload-urls` | Request Upload URL |

## User Documents

| Method | Path | Summary |
|---|---|---|
| `GET` | `/users/{userId}/documents` | List User Documents |
| `POST` | `/users/{userId}/documents` | Upload User Document |
| `GET` | `/users/{userId}/documents/{documentId}` | Get User Document |
| `DELETE` | `/users/{userId}/documents/{documentId}` | Delete User Document |

## User's Assigned Policies

| Method | Path | Summary |
|---|---|---|
| `GET` | `/users/{userId}/assigned-policies` | Get User's Assigned Policy details |
| `POST` | `/users/{userId}/assigned-policies/{policyId}/action-acknowledge` | Acknowledge User's Assigned Policy |

## Users and Roles

| Method | Path | Summary |
|---|---|---|
| `GET` | `/roles` | List Roles |
| `GET` | `/roles/{roleId}` | Get Role |
| `GET` | `/roles/{roleId}/users` | List Users with Role |
| `GET` | `/users` | List Users |
| `GET` | `/users/{userId}` | Get User |

## Vendor Criteria

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendor-criteria` | List Vendor Criteria |

## Vendor Criteria Creations

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/vendor-criteria-creations` | List Vendor Criteria Creations |

## Vendor Documents

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendors/{vendorId}/documents` | List Vendor Documents |
| `POST` | `/vendors/{vendorId}/documents` | Upload Vendor Document |
| `GET` | `/vendors/{vendorId}/documents/{documentId}` | Get Vendor Document |

## Vendor Questionnaire Templates

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendor-questionnaire-templates` | List Vendor Questionnaire Templates |
| `GET` | `/vendor-questionnaire-templates/{questionnaireId}/questions` | List Vendor Questionnaire Template Questions |

## Vendor Risk Levels

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendor-risk-levels` | List Vendor Risk Levels |

## Vendor Security Reviews

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments` | List Assessments 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments/{assessmentId}` | Get Assessment 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments-stats` | Get Assessments Stats 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/observations` | List Observations 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/observations` | Create Observation 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}` | Get Observation 🧪 |
| `PUT` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}` | Update Observation 🧪 |
| `DELETE` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}` | Delete Observation 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/residual-risk` | Get Security Review Residual Risk 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/notes` | List Notes 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/notes` | Create Note 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}` | Get Note 🧪 |
| `PUT` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}` | Update Note 🧪 |
| `DELETE` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}` | Delete Note 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/workflow` | Get Vendor Security Review Workflow 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews` | List Vendor Security Reviews |
| `POST` | `/vendors/{vendorId}/security-reviews` | Create Vendor Security Review |
| `POST` | `/vendors/{vendorId}/security-reviews/with-file` | Create Vendor Security Review with File |
| `GET` | `/vendor-security-reviews` | List Vendor Security Reviews Across Vendors |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}` | Get Vendor Security Review 🧪 |
| `PUT` | `/vendors/{vendorId}/security-reviews/{securityReviewId}` | Update Vendor Security Review 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/summary/status` | Get Vendor Security Review Summary 🧪 |
| `POST` | `/vendors/{vendorId}/security-questionnaires` | Upload Security Questionnaire |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires` | List Security Review Questionnaires 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires` | Upload Security Questionnaire |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/actions` | List Security Review Actions 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/actions` | Perform Security Review Action 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/agent` | Start Agent Review on Security Review 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/criteria-confirmations` | Confirm Agent Review Criteria 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/assessment-runs` | Start Agent Review Assessment 🧪 |
| `DELETE` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/assessment-runs/current` | Stop Agent Review Assessment 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/follow-up-questionnaires` | Generate Agent Review Follow-up Questionnaire 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/follow-up-questionnaire` | Get Agent Review Follow-up Questionnaire 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/follow-up-questionnaire-sends` | Send Agent Review Follow-up Questionnaire 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/finalizations` | Finalize Agent Review 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/documents` | Add Agent Review Documents 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/public-document-collections` | Collect Agent Review Public Trust Center Documents 🧪 |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/access-request-requirements` | Get Agent Review Trust Center Access Request Requirements 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/access-requests` | Request Agent Review Trust Center Access 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/access-links` | Provide Agent Review Trust Center Access Link 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/access-request-skips` | Skip Agent Review Trust Center Access Request 🧪 |

## Vendor Types

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendor-types` | List Vendor Types |
| `POST` | `/vendor-types` | Create Vendor Type |
| `PUT` | `/vendor-types/{vendorTypeId}` | Update Vendor Type |
| `DELETE` | `/vendor-types/{vendorTypeId}` | Delete Vendor Type |

## Vendors

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendors` | List Vendors |
| `POST` | `/vendors` | Create Vendor |
| `GET` | `/vendors-search` | Search Vendors |
| `GET` | `/vendors-stats` | Get Vendor Statistics |
| `GET` | `/vendors/{vendorId}` | Get Vendor |
| `PUT` | `/vendors/{vendorId}` | Update Vendor |
| `DELETE` | `/vendors/{vendorId}` | Remove Vendor |
| `GET` | `/vendors/{vendorId}/questionnaires` | List Vendor Questionnaires |
| `POST` | `/vendors/{vendorId}/questionnaires` | Send Questionnaire to Vendor |
| `GET` | `/vendors/{vendorId}/questionnaires/{questionnaireId}/questions` | List Vendor Questionnaire Questions |
| `GET` | `/vendors/{vendorId}/questionnaires/{questionnaireId}` | Get Vendor Questionnaire |
| `GET` | `/vendors/{vendorId}/trust-overview` | Get Vendor Trust Center Overview |

## Version Control User Identities

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/version-control-user-identities` | List Version Control User Identities |
| `GET` | `/workspaces/{workspaceId}/version-control-user-identities/{identityId}` | Get Version Control User Identity |
| `GET` | `/workspaces/{workspaceId}/version-control-user-identities/{identityId}/actions` | List Version Control User Identity Actions |
| `POST` | `/workspaces/{workspaceId}/version-control-user-identities/{identityId}/actions` | Perform Version Control User Identity Action |

## Workspaces

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces` | List Workspaces |
