# Drata API v2 - Endpoint Index

Generated from `drata-openapi-v2.json` (OpenAPI 3.0.0). 110 paths, 167 operations, 35 tags.

Servers:
- `https://public-api.drata.com/public/v2`
- `https://public-api.eu.drata.com/public/v2`
- `https://public-api.apac.drata.com/public/v2`

## Assets (5)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/assets` | List Assets |
| `POST` | `/assets` | Create Asset |
| `GET` | `/assets/{assetId}` | Get Asset |
| `PUT` | `/assets/{assetId}` | Update Asset |
| `DELETE` | `/assets/{assetId}` | Remove Asset |

## Audit Requests (2)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/audits/{auditId}/requests` | List Audit Requests |
| `GET` | `/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}` | Get Audit Request |

## Audits (2)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/audits` | List Audits |
| `GET` | `/workspaces/{workspaceId}/audits/{auditId}` | Get Audit |

## Background Checks (1)

| Method | Path | Summary |
|---|---|---|
| `POST` | `/background-checks` | Create Manual Background Check |

## Company (1)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/company` | Get Company |

## Control Library (3)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/control-library` | List Control Library |
| `GET` | `/control-library/{templateId}` | Get Control Library Item |
| `POST` | `/control-library/action-import` | Import Control Library Controls |

## Control Notes (5)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/controls/{controlId}/notes` | List Control Notes |
| `POST` | `/workspaces/{workspaceId}/controls/{controlId}/notes` | Create Control Note |
| `GET` | `/workspaces/{workspaceId}/controls/{controlId}/notes/{noteId}` | Get Control Note |
| `PUT` | `/workspaces/{workspaceId}/controls/{controlId}/notes/{noteId}` | Update Control Note |
| `DELETE` | `/workspaces/{workspaceId}/controls/{controlId}/notes/{noteId}` | Delete Control Note |

## Control Owners (4)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/controls/{controlId}/owners` | List Control Owners |
| `POST` | `/workspaces/{workspaceId}/controls/{controlId}/owners` | Add Control Owner |
| `PUT` | `/workspaces/{workspaceId}/controls/{controlId}/owners` | Modify Control Owners |
| `DELETE` | `/workspaces/{workspaceId}/controls/{controlId}/owners/{ownerId}` | Delete Control Owner |

## Controls (8)

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

## Custom Connections (5)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/custom-connections` | List Custom Connections |
| `POST` | `/custom-connections` | Create Custom Connection |
| `GET` | `/custom-connections/{connectionId}` | Get Custom Connection |
| `PUT` | `/custom-connections/{connectionId}` | Update Custom Connection |
| `DELETE` | `/custom-connections/{connectionId}` | Delete Custom Connection |

## Custom Data Records (7)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/custom-connections/{connectionId}/resources/{resourceId}/records` | List Custom Data Records |
| `POST` | `/custom-connections/{connectionId}/resources/{resourceId}/records` | Upsert Custom Data Records |
| `GET` | `/custom-connections/{connectionId}/resources/{resourceId}/sessions` | List Custom Data Sessions |
| `POST` | `/custom-connections/{connectionId}/resources/{resourceId}/sessions/{sessionId}` | Upsert Custom Data Records with Session Management |
| `POST` | `/custom-connections/{connectionId}/resources/{resourceId}/sessions/{sessionId}/actions` | Perform Session Action |
| `PUT` | `/custom-connections/{connectionId}/resources/{resourceId}/records/{recordId}` | Update Custom Data Record |
| `DELETE` | `/custom-connections/{connectionId}/resources/{resourceId}/records/{recordId}` | Delete Custom Connection Data Record |

## Device Documents (4)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/devices/{deviceId}/documents` | List Device Documents |
| `POST` | `/devices/{deviceId}/documents` | Upload Device Document |
| `GET` | `/devices/{deviceId}/documents/{documentId}` | Get Device Document |
| `DELETE` | `/devices/{deviceId}/documents/{documentId}` | Delete Device Document |

## Devices (7)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/devices` | List Devices |
| `GET` | `/personnel/{personnelId}/devices` | List Devices for Personnel |
| `GET` | `/devices/{deviceId}` | Get Device |
| `GET` | `/connections/{connectionId}/devices` | List Devices for Connection 🧪 |
| `GET` | `/devices/{deviceId}/apps` | List Apps for Device |
| `POST` | `/custom-connections/{connectionId}/devices` | Create or update Device for Custom Connection 🧪 |
| `DELETE` | `/custom-connections/{connectionId}/devices/{deviceId}` | Delete Device |

## Events (4)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/events` | List Events |
| `GET` | `/events/{eventId}` | Get Event |
| `POST` | `/events/{eventId}/download-jobs` | Create Event PDF Download Job |
| `GET` | `/events/{eventId}/download-jobs/{jobId}` | Get Event PDF Export |

## Evidence Library (6)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/evidence-library` | List Evidence Library Items |
| `POST` | `/workspaces/{workspaceId}/evidence-library` | Create Evidence Library Item |
| `GET` | `/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}` | Get Evidence Library Item |
| `PUT` | `/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}` | Update Evidence Library Item |
| `DELETE` | `/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}` | Delete Evidence Library Item |
| `GET` | `/workspaces/{workspaceId}/evidence-library/{evidenceLibraryId}/versions/{versionId}` | Get Evidence Library Version |

## Frameworks (8)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/frameworks` | List Frameworks |
| `POST` | `/workspaces/{workspaceId}/frameworks` | Create Custom Framework |
| `GET` | `/workspaces/{workspaceId}/framework-requirements` | List Framework Requirements |
| `PUT` | `/workspaces/{workspaceId}/framework-requirements/{frameworkRequirementId}` | Update Framework Requirement |
| `GET` | `/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements` | List Framework Requirements |
| `POST` | `/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements` | Create Framework Requirements |
| `PUT` | `/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements/{requirementId}` | Update Framework Requirement |
| `PUT` | `/workspaces/{workspaceId}/frameworks/{frameworkId}` | Update Custom Framework |

## HRIS User Identities (4)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/custom-connections/{connectionId}/hris-user-identities` | List HRIS User Identities 🧪 |
| `POST` | `/custom-connections/{connectionId}/hris-user-identities` | Create or Update HRIS User Identities 🧪 |
| `GET` | `/custom-connections/{connectionId}/hris-user-identities/{userIdentityId}` | Get HRIS User Identity 🧪 |
| `DELETE` | `/custom-connections/{connectionId}/hris-user-identities/{userIdentityId}` | Delete HRIS User Identity 🧪 |

## Monitoring Tests (6)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/monitoring-tests` | List Monitoring Tests |
| `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}` | Get Monitoring Test |
| `PUT` | `/workspaces/{workspaceId}/monitoring-tests/{testId}` | Update Monitoring Test |
| `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions` | List Monitoring Test Exclusions |
| `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/failures` | List Monitoring Test Failures |
| `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/passes` | List Monitoring Test Passes |

## Personnel (4)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/personnel` | List Personnel |
| `GET` | `/personnel/{personnelId}` | Get Personnel |
| `PUT` | `/personnel/{personnelId}` | Update Personnel |
| `POST` | `/personnel/actions` | Reset Personnel Sync |

## Policies (13)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/policies` | List Policies |
| `POST` | `/policies` | Create Policy |
| `GET` | `/policies/{policyId}` | Get Policy |
| `PUT` | `/policies/{policyId}` | Modify Policy |
| `PUT` | `/policies/{policyId}/owner` | Assign Policy Owner |
| `GET` | `/policies/{policyId}/approval-configuration` | Get Policy Approval Configuration |
| `POST` | `/policies/{policyId}/approval-configuration` | Add Review Group Configuration |
| `PUT` | `/policies/{policyId}/approval-configuration/{approvalConfigurationTier}` | Update Review Group Configuration |
| `DELETE` | `/policies/{policyId}/approval-configuration/{approvalConfigurationTier}` | Remove Review Group Configuration |
| `GET` | `/policies/{policyId}/actions` | List Policy Actions |
| `POST` | `/policies/{policyId}/actions` | Perform Policy Action |
| `GET` | `/policies/{policyId}/policy-versions` | List Policy Versions |
| `GET` | `/policies/{policyId}/policy-versions/{policyVersionId}` | Get Policy Version |

## Procurement Connection Mappings (2)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/connections/{connectionId}/vendor-mapping` | Get Vendor Mapping |
| `PUT` | `/connections/{connectionId}/vendor-mapping` | Update Vendor Mapping |

## Risk Documents (4)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}/documents` | List Risk Documents |
| `POST` | `/risk-registers/{riskRegisterId}/risks/{riskId}/documents` | Upload Risk Documents |
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}/documents/{documentId}` | Get Risk Document |
| `DELETE` | `/risk-registers/{riskRegisterId}/risks/{riskId}/documents/{documentId}` | Delete Risk Document |

## Risk Library (3)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers/{riskRegisterId}/risk-library` | List Risk Libraries |
| `GET` | `/risk-registers/{riskRegisterId}/risk-library/{riskLibraryId}` | Get Risk Library Item by ID. |
| `POST` | `/risk-library/action-copy` | Copies Risk Library Items to a Risk Register. |

## Risk Notes (5)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}/notes` | List Risk Notes |
| `POST` | `/risk-registers/{riskRegisterId}/risks/{riskId}/notes` | Create Risk Note |
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}/notes/{noteId}` | Get Risk Note |
| `PUT` | `/risk-registers/{riskRegisterId}/risks/{riskId}/notes/{noteId}` | Update Risk Note |
| `DELETE` | `/risk-registers/{riskRegisterId}/risks/{riskId}/notes/{noteId}` | Delete Risk Note |

## Risk Registers (5)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers` | List Risk Registers |
| `POST` | `/risk-registers` | Create Risk Register |
| `GET` | `/risk-registers/{riskRegisterId}` | Get Risk Register |
| `PUT` | `/risk-registers/{riskRegisterId}` | Update Risk Register |
| `DELETE` | `/risk-registers/{riskRegisterId}` | Delete Risk Register |

## Risks (5)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/risk-registers/{riskRegisterId}/risks` | List Risks |
| `POST` | `/risk-registers/{riskRegisterId}/risks` | Create Risk |
| `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}` | Get Risk |
| `PUT` | `/risk-registers/{riskRegisterId}/risks/{riskId}` | Update Risk |
| `GET` | `/risk-registers/{riskRegisterId}/insights` | Get Risk Insights |

## Tasks (6)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces/{workspaceId}/tasks` | List Tasks |
| `POST` | `/workspaces/{workspaceId}/tasks` | Create Task |
| `GET` | `/workspaces/{workspaceId}/tasks/{taskId}` | Get Task |
| `PUT` | `/workspaces/{workspaceId}/tasks/{taskId}` | Update Task |
| `POST` | `/workspaces/{workspaceId}/tasks/{taskId}/actions` | Perform Task Action |
| `GET` | `/workspaces/{workspaceId}/upcoming-tasks` | List Upcoming Tasks |

## User Documents (4)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/users/{userId}/documents` | List User Documents |
| `POST` | `/users/{userId}/documents` | Upload User Document |
| `GET` | `/users/{userId}/documents/{documentId}` | Get User Document |
| `DELETE` | `/users/{userId}/documents/{documentId}` | Delete User Document |

## User's Assigned Policies (2)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/users/{userId}/assigned-policies` | Get User's Assigned Policy details |
| `POST` | `/users/{userId}/assigned-policies/{policyId}/action-acknowledge` | Acknowledge User's Assigned Policy |

## Users and Roles (5)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/roles` | List Roles |
| `GET` | `/roles/{roleId}` | Get Role |
| `GET` | `/roles/{roleId}/users` | List Users with Role |
| `GET` | `/users` | List Users |
| `GET` | `/users/{userId}` | Get User |

## Vendor Documents (3)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendors/{vendorId}/documents` | List Vendor Documents |
| `POST` | `/vendors/{vendorId}/documents` | Upload Vendor Document |
| `GET` | `/vendors/{vendorId}/documents/{documentId}` | Get Vendor Document |

## Vendor Security Reviews (10)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendors/{vendorId}/security-reviews` | List Vendor Security Reviews |
| `POST` | `/vendors/{vendorId}/security-reviews` | Create Vendor Security Review |
| `POST` | `/vendors/{vendorId}/security-reviews/with-file` | Create Vendor Security Review with File |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}` | Get Vendor Security Review 🧪 |
| `PUT` | `/vendors/{vendorId}/security-reviews/{securityReviewId}` | Update Vendor Security Review 🧪 |
| `POST` | `/vendors/{vendorId}/security-questionnaires` | Upload Security Questionnaire |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires` | List Security Review Questionnaires 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires` | Upload Security Questionnaire |
| `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/actions` | List Security Review Actions 🧪 |
| `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/actions` | Perform Security Review Action 🧪 |

## Vendor Types (4)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendor-types` | List Vendor Types |
| `POST` | `/vendor-types` | Create Vendor Type |
| `PUT` | `/vendor-types/{vendorTypeId}` | Update Vendor Type |
| `DELETE` | `/vendor-types/{vendorTypeId}` | Delete Vendor Type |

## Vendors (9)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/vendors` | List Vendors |
| `POST` | `/vendors` | Create Vendor |
| `GET` | `/vendors-stats` | Get Vendor Statistics |
| `GET` | `/vendors/{vendorId}` | Get Vendor |
| `PUT` | `/vendors/{vendorId}` | Update Vendor |
| `DELETE` | `/vendors/{vendorId}` | Remove Vendor |
| `GET` | `/vendors/{vendorId}/questionnaires` | List Vendor Questionnaires |
| `POST` | `/vendors/{vendorId}/questionnaires` | Send Questionnaire to Vendor |
| `GET` | `/vendors/{vendorId}/questionnaires/{questionnaireId}` | Get Vendor Questionnaire |

## Workspaces (1)

| Method | Path | Summary |
|---|---|---|
| `GET` | `/workspaces` | List Workspaces |

