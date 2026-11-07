Title: Evidence

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Evidence/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Evidence items hold one or more artifacts, the files, URLs, or ticket references that demonstrate a control is operating.
Use the evidence-files endpoint to pre-upload a file, then reference the returned fileKey when creating or updating an Evidence item.

## [](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence)List Evidence Items

List evidence items with pagination, filtering, and optional expand.

🔒 Requires **Evidence Library: List Evidence** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=query&path=expand[]&t=request)expand[]Array of strings (EvidenceExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"owners""artifacts""controls"
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=query&path=name&t=request)name string<= 191 characters

Filter evidence items by name (prefix match)

Example:name=SOC2 Evidence
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=query&path=ownerIds[]&t=request)ownerIds[]Array of numbers

IDs of the users that own the evidence current version.

Example:ownerIds[]=1&ownerIds[]=2&ownerIds[]=3
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=query&path=frameworkIds[]&t=request)frameworkIds[]Array of numbers

Array of Ids of frameworks to be used as filters

Example:frameworkIds[]=1&frameworkIds[]=2&frameworkIds[]=3
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=query&path=evidenceStatuses[]&t=request)evidenceStatuses[]Array of strings (ComputedEvidenceStatusEnum)

Filter evidence items by statuses

Items Enum:"NEEDS_ARTIFACT""EXPIRED""EXPIRING_SOON""TEST_ERROR""TEST_FAILED""READY""TEST_PASSED""TEST_UNUSED""TEST_DISABLED""TEST_DELETED"
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidence!in=query&path=artifactTypes[]&t=request)artifactTypes[]Array of strings (ArtifactTypeEnum)

Filter evidence items by artifact types

Items Enum:"URL""S3_FILE""TICKET_PROVIDER""NONE""GOOGLE_DRIVE""ONE_DRIVE""BOX""DROPBOX""SHARE_POINT""TEST_RESULT"

Responses

200
Successful

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/workspaces/{workspaceId}/evidence

Request samples

*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

2 more

import fetch from 'node-fetch';

async function run() {
  const workspaceId = 'YOUR_workspaceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/evidence`,
    {
      method: 'GET',
      headers: {
        Authorization: 'Bearer <YOUR_API_KEY_HERE>'
      }
    }
  );

  const data = await resp.text();
  console.log(data);
}

run();

Response samples

*   200
*   401
*   403
*   404
*   412
*   500

1 more

application/json

`{"data": [{"id": 1,"name": "Security Policy Document","description": "This document outlines our security policies and procedures.","implementationGuidance": "Follow the company security training guidelines and ensure all employees complete the training within 30 days of onboarding.","evidenceTemplateCode": "REQ-12","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","renewalDate": "2020-07-06","renewalScheduleType": "ONE_YEAR","status": "READY","safeBaseSyncStatus": "COMPLETE","owners": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"artifacts": {"data": [{"id": 1,"artifactName": "Q4-2024-Security-Audit.pdf","extension": "pdf","artifactVersion": 1,"artifactSafeBasesyncStatus": "COMPLETE","artifactType": "S3_FILE","filedAt": "2025-07-01T16:45:55.246Z","archivedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"controls": {"data": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"_links": {"self": {}}}],"pagination": {"cursor": "string"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidence)Create Evidence

Create a new Evidence item with one or more artifacts. File artifacts must be pre-uploaded via the evidence-files endpoint and referenced by `fileKey`.

🔒 Requires **Evidence Library: Create Evidence** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidence!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidence!path=name&t=request)name
required string<= 191 characters

Evidence name
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidence!path=description&t=request)description string or null<= 30000 characters

Evidence description
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidence!path=implementationGuidance&t=request)implementationGuidance string or null<= 30000 characters

Guidance for implementing evidence
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidence!path=stepsToReproduce&t=request)stepsToReproduce string or null<= 30000 characters

Steps to reproduce or collect this evidence
required Array of objects (EvidenceArtifactRequestPublicV2Dto)

Array of artifacts to create for this evidence. Each artifact should reference a pre-uploaded file (via fileKey) or a URL.
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidence!path=ownerId&t=request)ownerId number

Owner user ID
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidence!path=controlIds&t=request)controlIds Array of numbers or null

List of control IDs to link to this evidence
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidence!path=renewalDate&t=request)renewalDate string or null

Evidence renewal date. Optional - if not provided and renewalScheduleType is set, it may be auto-calculated by the backend based on the schedule type.
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidence!path=renewalScheduleType&t=request)renewalScheduleType string or null

Evidence renewal schedule type

Enum:"ONE_MONTH""TWO_MONTHS""THREE_MONTHS""SIX_MONTHS""ONE_YEAR""CUSTOM""NONE"

Responses

201
Created

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/workspaces/{workspaceId}/evidence

Request samples

*   Payload
*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

3 more

application/json

`{"name": "Security Training","description": "Security Training completed evidence","implementationGuidance": "Example of architectural diagram www.drata/arch-diagram-example.com","stepsToReproduce": "Steps to reproduce the issue or evidence collection process","artifacts": [{"type": "S3_FILE","artifactName": "Security Policy v2.pdf","fileKey": "123/evidence-library/62545abd-5469-4a19-abc2-55df8809b225/policy.pdf","filedAt": "2020-07-06"},{"type": "URL","artifactName": "External Documentation","filedAt": "2020-07-06"}],"ownerId": 1,"controlIds": [1,2,3],"renewalDate": "2020-07-06","renewalScheduleType": "ONE_YEAR"}`

Response samples

*   201
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"id": 1,"name": "Security Policy Document","description": "This document outlines our security policies and procedures.","implementationGuidance": "Follow the company security training guidelines and ensure all employees complete the training within 30 days of onboarding.","evidenceTemplateCode": "REQ-12","renewalDate": "2020-07-06","renewalScheduleType": "ONE_YEAR","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","artifactsRequested": 3,"artifactsCreated": 2,"controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"linkedWorkspaces": [{"id": 1,"name": "Drata"}],"artifacts": [{"id": 1,"source": "document.pdf","artifactName": "document.pdf","type": "S3_FILE","artifactVersion": 1,"filedAt": "2020-07-06","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","metadata": {"originalFileName": "security-policy.pdf","mimeType": "application/pdf","extension": "pdf"},"ticketUrl": "string"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_uploadArtifactFile)Upload Evidence Artifact File

Pre-upload a file to be used as an evidence artifact. Provide the binary as either a multipart `file` or a Base64-encoded `base64File`; exactly one is required. Returns a `fileKey` that can be referenced in the create/update Evidence endpoints.

🔒 Requires **Evidence Library: Create Evidence** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_uploadArtifactFile!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: multipart/form-data
required

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_uploadArtifactFile!path=file&t=request)file string<binary>

Artifact source as a file. Provide exactly one of `file` or `base64File`. Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .gif, .jpg, .jpeg, .png, .json, .csv, .md, .markdown, .txt, .log, .zip, .msg, .mp4
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_uploadArtifactFile!path=base64File&t=request)base64File string

Artifact source as a Base64-encoded file in data URL format (alternative to `file`). Provide exactly one of `file` or `base64File`.

Responses

201
Created

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/workspaces/{workspaceId}/evidence-files

Request samples

*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

2 more

import fetch from 'node-fetch';

async function run() {
  const workspaceId = 'YOUR_workspaceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/evidence-files`,
    {
      method: 'POST',
      headers: {
        'Content-Type': 'multipart/form-data',
        Authorization: 'Bearer <YOUR_API_KEY_HERE>'
      }
    }
  );

  const data = await resp.text();
  console.log(data);
}

run();

Response samples

*   201
*   400
*   401
*   403
*   412
*   500

1 more

application/json

`{"fileKey": "account-id/evidence-library/uuid/file.pdf","originalFilename": "My Security Training (v2).pdf","mimeType": "application/pdf","fileSize": 1024}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_getEvidence)Get Evidence Item

Get a specific evidence item by ID with optional expand.

🔒 Requires **Evidence Library: List Evidence** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_getEvidence!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_getEvidence!in=path&path=evidenceId&t=request)evidenceId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_getEvidence!in=query&path=expand[]&t=request)expand[]Array of strings (EvidenceExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"owners""artifacts""controls"

Responses

200
Successful

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/workspaces/{workspaceId}/evidence/{evidenceId}

Request samples

*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

2 more

import fetch from 'node-fetch';

async function run() {
  const workspaceId = 'YOUR_workspaceId_PARAMETER';
  const evidenceId = 'YOUR_evidenceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/evidence/${evidenceId}`,
    {
      method: 'GET',
      headers: {
        Authorization: 'Bearer <YOUR_API_KEY_HERE>'
      }
    }
  );

  const data = await resp.text();
  console.log(data);
}

run();

Response samples

*   200
*   401
*   403
*   404
*   412
*   500

1 more

application/json

`{"id": 1,"name": "Security Policy Document","description": "This document outlines our security policies and procedures.","implementationGuidance": "Follow the company security training guidelines and ensure all employees complete the training within 30 days of onboarding.","evidenceTemplateCode": "REQ-12","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","renewalDate": "2020-07-06","renewalScheduleType": "ONE_YEAR","status": "READY","safeBaseSyncStatus": "COMPLETE","owners": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"artifacts": {"data": [{"id": 1,"artifactName": "Q4-2024-Security-Audit.pdf","extension": "pdf","artifactVersion": 1,"artifactSafeBasesyncStatus": "COMPLETE","artifactType": "S3_FILE","filedAt": "2025-07-01T16:45:55.246Z","archivedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"controls": {"data": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"_links": {"self": {}}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence)Update Evidence

Update an existing Evidence item. Supports multi-artifact operations: add new artifacts, archive artifacts (past), restore artifacts (current), and replace artifacts.

🔒 Requires **Evidence Library: Update Evidence** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!in=path&path=evidenceId&t=request)evidenceId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!path=name&t=request)name string<= 191 characters

Evidence name
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!path=description&t=request)description string or null<= 30000 characters

Evidence description
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!path=implementationGuidance&t=request)implementationGuidance string or null<= 30000 characters

Guidance for implementing evidence. Partial-update: omit to leave the current value unchanged; send null to clear it.
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!path=stepsToReproduce&t=request)stepsToReproduce string or null<= 30000 characters

Steps to reproduce or collect this evidence. Partial-update: omit to leave the current value unchanged; send null to clear it.
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!path=renewalDate&t=request)renewalDate string or null

Renewal date (ISO 8601 format, required if renewalScheduleType is not NONE)
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!path=renewalScheduleType&t=request)renewalScheduleType string or null

Renewal schedule type for the evidence

Enum:"ONE_MONTH""TWO_MONTHS""THREE_MONTHS""SIX_MONTHS""ONE_YEAR""CUSTOM""NONE"
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!path=ownerId&t=request)ownerId number

Evidence owner user ID
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!path=controlIds&t=request)controlIds Array of numbers or null

List of control IDs to link (additions only, cannot remove existing links)
Array of objects (EvidenceArtifactAddRequestPublicV2Dto)

New artifacts to add to the evidence
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!path=pastArtifacts&t=request)pastArtifacts Array of numbers or null

List of artifact IDs to archive (move to past)
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidence!path=currentArtifacts&t=request)currentArtifacts Array of numbers or null

List of artifact IDs to restore (move to current)
Array of objects (EvidenceArtifactReplaceRequestPublicV2Dto)

Artifacts to replace by adding a new version to an existing artifact chain. Preserves the artifact ID across versions.

Responses

200
Successful

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

put/workspaces/{workspaceId}/evidence/{evidenceId}

Request samples

*   Payload
*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

3 more

application/json

`{"name": "Updated Security Training","description": "Security Training completed evidence","implementationGuidance": "Example of architectural diagram www.drata/arch-diagram-example.com","stepsToReproduce": "Steps to reproduce the issue or evidence collection process","renewalDate": "2020-07-06","renewalScheduleType": "ONE_YEAR","ownerId": 789,"controlIds": [101,102],"newArtifacts": [{"artifactType": "URL","filedAt": "2020-07-06","artifactName": "Policy Document"}],"pastArtifacts": [1,2],"currentArtifacts": [3],"replaceArtifacts": [{"existingArtifactId": 1,"artifactType": "URL","filedAt": "2020-07-06","artifactName": "Updated Policy Document"}]}`

Response samples

*   200
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"id": 1,"name": "Security Policy Document","description": "This document outlines our security policies and procedures.","implementationGuidance": "Follow the company security training guidelines and ensure all employees complete the training within 30 days of onboarding.","evidenceTemplateCode": "REQ-12","renewalDate": "2020-07-06","renewalScheduleType": "ONE_YEAR","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","artifactsRequested": 3,"artifactsCreated": 2,"controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"linkedWorkspaces": [{"id": 1,"name": "Drata"}],"artifacts": [{"id": 1,"source": "document.pdf","artifactName": "document.pdf","type": "S3_FILE","artifactVersion": 1,"filedAt": "2020-07-06","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","metadata": {"originalFileName": "security-policy.pdf","mimeType": "application/pdf","extension": "pdf"},"ticketUrl": "string"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_deleteEvidence)Delete Evidence

Permanently delete an Evidence item and all of its artifacts. To delete a single artifact and leave the Evidence item in place, use the delete artifact endpoint.

🔒 Requires **Evidence Library: Delete Evidence** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_deleteEvidence!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_deleteEvidence!in=path&path=evidenceId&t=request)evidenceId
required number

Responses

204
No Content

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

delete/workspaces/{workspaceId}/evidence/{evidenceId}

Request samples

*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

2 more

import fetch from 'node-fetch';

async function run() {
  const workspaceId = 'YOUR_workspaceId_PARAMETER';
  const evidenceId = 'YOUR_evidenceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/evidence/${evidenceId}`,
    {
      method: 'DELETE',
      headers: {
        Authorization: 'Bearer <YOUR_API_KEY_HERE>'
      }
    }
  );

  if (resp.status === 204) {
    console.log('success');
  } else {
    const data = await resp.text();
    console.log(data);
  }
}

run();

Response samples

*   401
*   403
*   404
*   412
*   500

application/json

`{"statusCode": 0,"message": "string","code": 0,"debugInfo": {"name": "string","message": "string","stack": "string"},"providerError": {"provider": "string","status": 0,"code": "string","message": "string","details": { }}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidenceArtifacts)List Evidence Artifacts

List the artifacts of a single evidence item. Returns one entry per artifact, not one per artifact version, so each `id` is stable across replacements and is the ID accepted by the other artifact endpoints.

🔒 Requires **Evidence Library: List Evidence** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidenceArtifacts!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidenceArtifacts!in=path&path=evidenceId&t=request)evidenceId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidenceArtifacts!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidenceArtifacts!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidenceArtifacts!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidenceArtifacts!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidenceArtifacts!in=query&path=name&t=request)name string<= 191 characters

Filter artifacts by name (prefix match)

Example:name=Q4-2024-Security-Audit
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidenceArtifacts!in=query&path=artifactTypes[]&t=request)artifactTypes[]Array of strings (ArtifactType)

Filter artifacts by type

Items Enum:"URL""S3_FILE""TICKET_PROVIDER""NONE""GOOGLE_DRIVE""ONE_DRIVE""BOX""DROPBOX""SHARE_POINT""TEST_RESULT"
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_listEvidenceArtifacts!in=query&path=isArchived&t=request)isArchived boolean

True returns only archived (past) artifacts, false only active ones. Omit to return both.

Example:isArchived=false

Responses

200
Successful

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts

Request samples

*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

2 more

import fetch from 'node-fetch';

async function run() {
  const workspaceId = 'YOUR_workspaceId_PARAMETER';
  const evidenceId = 'YOUR_evidenceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/evidence/${evidenceId}/artifacts`,
    {
      method: 'GET',
      headers: {
        Authorization: 'Bearer <YOUR_API_KEY_HERE>'
      }
    }
  );

  const data = await resp.text();
  console.log(data);
}

run();

Response samples

*   200
*   401
*   403
*   404
*   412
*   500

1 more

application/json

`{"data": [{"id": 1,"artifactName": "Q4-2024-Security-Audit.pdf","extension": "pdf","artifactVersion": 1,"artifactSafeBasesyncStatus": "COMPLETE","artifactType": "S3_FILE","filedAt": "2025-07-01T16:45:55.246Z","archivedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidenceArtifact)Update Evidence Artifact

Partially update an existing artifact's `artifactName` and/or `filedAt` in place on its current version. To replace the artifact's actual source (file/URL/ticket), use `replaceArtifacts` on the update Evidence endpoint instead.

🔒 Requires **Evidence Library: Update Evidence** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidenceArtifact!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidenceArtifact!in=path&path=evidenceId&t=request)evidenceId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidenceArtifact!in=path&path=artifactId&t=request)artifactId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidenceArtifact!path=artifactName&t=request)artifactName string or null<= 191 characters

Custom name for the artifact. Omit to leave unchanged; send null to clear it.
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_updateEvidenceArtifact!path=filedAt&t=request)filedAt string or null

The date when the artifact was filed/effective. Omit to leave unchanged; send null to clear it.

Responses

200
Successful

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

put/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}

Request samples

*   Payload
*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

3 more

application/json

`{"artifactName": "Q4 Security Review","filedAt": "2020-07-06"}`

Response samples

*   200
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"id": 1,"artifactName": "Q4-2024-Security-Audit.pdf","extension": "pdf","artifactVersion": 1,"source": "https://example.com/evidence/document.pdf","artifactSafeBasesyncStatus": "COMPLETE","artifactType": "S3_FILE","filedAt": "2025-07-01T16:45:55.246Z","archivedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_deleteEvidenceArtifact)Delete Evidence Artifact

Permanently delete a single artifact from an evidence item, including its full version history. To move an artifact to past artifacts instead, pass its ID in `pastArtifacts` on the update Evidence endpoint.

🔒 Requires **Evidence Library: Delete Evidence** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_deleteEvidenceArtifact!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_deleteEvidenceArtifact!in=path&path=evidenceId&t=request)evidenceId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_deleteEvidenceArtifact!in=path&path=artifactId&t=request)artifactId
required number

Responses

204
No Content

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

delete/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}

Request samples

*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

2 more

import fetch from 'node-fetch';

async function run() {
  const workspaceId = 'YOUR_workspaceId_PARAMETER';
  const evidenceId = 'YOUR_evidenceId_PARAMETER';
  const artifactId = 'YOUR_artifactId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/evidence/${evidenceId}/artifacts/${artifactId}`,
    {
      method: 'DELETE',
      headers: {
        Authorization: 'Bearer <YOUR_API_KEY_HERE>'
      }
    }
  );

  if (resp.status === 204) {
    console.log('success');
  } else {
    const data = await resp.text();
    console.log(data);
  }
}

run();

Response samples

*   400
*   401
*   403
*   404
*   412
*   500

1 more

application/json

`{"name": "string","statusCode": 0,"message": "string","code": 0}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidenceArtifactAction)Perform Evidence Artifact Action

Archive an artifact (move it to past artifacts) or restore it (move it back to current artifacts). Both are reversible and leave the version history intact.

🔒 Requires **Evidence Library: Update Evidence** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidenceArtifactAction!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidenceArtifactAction!in=path&path=evidenceId&t=request)evidenceId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidenceArtifactAction!in=path&path=artifactId&t=request)artifactId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Evidence/#tag/Evidence/operation/EvidencePublicV2Controller_createEvidenceArtifactAction!path=action&t=request)action
required string

Action to execute: `archive` moves the artifact to past artifacts, `restore` moves it back to current artifacts. Both are reversible; to remove an artifact and its version history permanently, use the delete artifact endpoint instead.

Enum:"archive""restore"

Responses

200
Successful

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}/actions

Request samples

*   Payload
*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

3 more

application/json

`{"action": "archive"}`

Response samples

*   200
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"id": 1,"artifactName": "Q4-2024-Security-Audit.pdf","extension": "pdf","artifactVersion": 1,"source": "https://example.com/evidence/document.pdf","artifactSafeBasesyncStatus": "COMPLETE","artifactType": "S3_FILE","filedAt": "2025-07-01T16:45:55.246Z","archivedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z"}`
