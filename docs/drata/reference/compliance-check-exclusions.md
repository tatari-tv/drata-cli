Title: Compliance Check Exclusions

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Compliance Check Exclusions remove a curated list of personnel from specific personnel compliance checks (e.g. Offboarding) without setting them out of scope — their status in other tests and in the audit population is unchanged. An Exclusion covers one subject (`targetId`/`targetType`) for one or more compliance check types, and only counts while it is in force: from its `startDate` until its `endDate`, or indefinitely when `endDate` is null. Excluding a subject that is already excluded from some of the requested types adds only the types it was missing. Deleting an Exclusion for some of the types it covers narrows it to the remaining types rather than removing it, so removing one check never silently removes another. Both bulk endpoints report a per-target outcome, so a single invalid, already-handled or out-of-scope personnel ID does not fail the batch.

## [](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_listComplianceCheckExclusions)List Compliance Check Exclusions

Returns a cursor-paginated list of personnel compliance-check exclusions that are currently in force, most-recently-created first. When the workspace has a configured personnel scope, results are limited to personnel inside it; when it has none, the whole tenant is returned. Optionally filter to a single compliance check type.

🔒 Requires **Monitoring: List Compliance Check Exclusions** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_listComplianceCheckExclusions!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_listComplianceCheckExclusions!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_listComplianceCheckExclusions!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_listComplianceCheckExclusions!in=query&path=sort&t=request)sort string (ComplianceCheckExclusionSortTypeLimitedEnum)

Which field to sort by. Only `createdAt` is supported for this collection. Use `sortDir` for ascending or descending.

Value:"createdAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_listComplianceCheckExclusions!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_listComplianceCheckExclusions!in=query&path=complianceCheck&t=request)complianceCheck string (ComplianceCheckTypeEnum)

Only return exclusions covering this compliance check type.

Enum:"FULL_COMPLIANCE""ACCEPTED_POLICIES""IDENTITY_MFA""BG_CHECK""AGENT_INSTALLED""PASSWORD_MANAGER""HDD_ENCRYPTION""ANTIVIRUS""AUTO_UPDATES""LOCK_SCREEN""SECURITY_TRAINING""LOCATION_SERVICES""HIPAA_TRAINING""OFFBOARDING""NIST_AI_TRAINING"
[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_listComplianceCheckExclusions!in=query&path=expand[]&t=request)expand[]Array of strings (ComplianceCheckExclusionExpandEnum)

List of subcollections and sub-objects to expand. Anything not listed is omitted from each Exclusion and its join is skipped: `complianceCheckTypes` for the checks the Exclusion covers, `createdBy` for `createdById` and `createdByEmail`.

Items Enum:"complianceCheckTypes""createdBy"

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

get/workspaces/{workspaceId}/compliance-check-exclusions

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/compliance-check-exclusions`,
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
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"data": [{"id": 1,"reason": "Employee is on extended leave","createdById": 1,"createdByEmail": "user@company.com","createdAt": "2025-07-01T16:45:55.246Z","startDate": "2025-07-01T16:45:55.246Z","endDate": "2025-07-01T16:45:55.246Z","targetId": "1","targetType": "PERSONNEL","complianceCheckTypes": ["OFFBOARDING"]}],"pagination": {"cursor": "string"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkExcludePersonnel)Exclude Personnel Compliance Checks

Excludes a curated list of personnel from the given compliance check types (e.g. Offboarding) without setting anyone out of scope — their status in other tests and in the audit population is unchanged. A target already excluded from every requested type is left untouched; one excluded from only some of them gets a new exclusion covering the rest. Each requested target id is reported with a per-target outcome, so an id that is invalid, already excluded, or outside this workspace's personnel scope does not fail the batch. Time-boxed exclusions are supported via `startDate` / `endDate`. When the workspace has a configured personnel scope, `workspaceId` limits which personnel the caller may target; when it has none, the whole tenant is targetable.

🔒 Requires **Monitoring: Exclude Personnel From Compliance Checks** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkExcludePersonnel!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkExcludePersonnel!path=targetIds&t=request)targetIds
required Array of strings[ 1 .. 500 ] items

Personnel IDs to exclude. Each is reported with a per-target outcome, so an ID that does not resolve to a person, or is outside this workspace, does not fail the batch.
[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkExcludePersonnel!path=complianceCheckTypes&t=request)complianceCheckTypes
required Array of strings (ComplianceCheckTypeEnum)

Compliance check types the personnel are excluded from. `OFFBOARDING` is the primary use case; any type returned by the allowable list is accepted.

Items Enum:"FULL_COMPLIANCE""ACCEPTED_POLICIES""IDENTITY_MFA""BG_CHECK""AGENT_INSTALLED""PASSWORD_MANAGER""HDD_ENCRYPTION""ANTIVIRUS""AUTO_UPDATES""LOCK_SCREEN""SECURITY_TRAINING""LOCATION_SERVICES""HIPAA_TRAINING""OFFBOARDING""NIST_AI_TRAINING"
[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkExcludePersonnel!path=reason&t=request)reason
required string<= 30000 characters

Reason for the exclusion.
[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkExcludePersonnel!path=startDate&t=request)startDate
required string<date-time>

When the exclusion takes effect (ISO-8601).
[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkExcludePersonnel!path=endDate&t=request)endDate string or null<date-time>

When the exclusion expires (ISO-8601), or null for an open-ended exclusion. Time-boxed exclusions are supported here because they are personnel-backed. Must be after `startDate`.

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

post/workspaces/{workspaceId}/compliance-check-exclusions

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

`{"targetIds": ["1","2"],"complianceCheckTypes": ["OFFBOARDING"],"reason": "Offboarded before the audit window.","startDate": "2025-07-01T16:45:55.246Z","endDate": "2025-07-01T16:45:55.246Z"}`

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

`{"results": [{"targetId": "1","outcome": "EXCLUDED","exclusionIds": [10]}],"successCount": 2,"skippedCount": 1}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkDeleteExclusions)Delete Personnel Compliance Check Exclusions

Deletes a curated list of personnel's exclusions for the given compliance check types, putting those personnel back in scope for those checks. Symmetric with the exclude call (same personnel `targetIds` + `complianceCheckTypes`). An exclusion covering only the requested types is archived; one that also covers other types is narrowed to just those other types, so deleting an exclusion for one check never silently removes it for another. Each requested target id is reported with a per-target outcome; a target with no matching in-force exclusion, or one outside this workspace, does not fail the batch. When the workspace has a configured personnel scope, `workspaceId` limits which personnel the caller may target; when it has none, the whole tenant is targetable.

🔒 Requires **Monitoring: Re-include Personnel Into Compliance Checks** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkDeleteExclusions!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkDeleteExclusions!path=targetIds&t=request)targetIds
required Array of strings[ 1 .. 500 ] items

Personnel IDs whose exclusions should be deleted. Each is reported with a per-target outcome, so a target with no matching in-force exclusion does not fail the batch.
[](https://developers.drata.com/openapi/reference/v2/tag/Compliance-Check-Exclusions/#tag/Compliance-Check-Exclusions/operation/ComplianceCheckExclusionsPublicV2Controller_bulkDeleteExclusions!path=complianceCheckTypes&t=request)complianceCheckTypes
required Array of strings (ComplianceCheckTypeEnum)

Compliance check types to delete the exclusions for. An exclusion covering only these types is archived; one that also covers other types is narrowed to just those other types, so deleting for one check never silently deletes another.

Items Enum:"FULL_COMPLIANCE""ACCEPTED_POLICIES""IDENTITY_MFA""BG_CHECK""AGENT_INSTALLED""PASSWORD_MANAGER""HDD_ENCRYPTION""ANTIVIRUS""AUTO_UPDATES""LOCK_SCREEN""SECURITY_TRAINING""LOCATION_SERVICES""HIPAA_TRAINING""OFFBOARDING""NIST_AI_TRAINING"

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

post/workspaces/{workspaceId}/compliance-check-exclusions/action-delete

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

`{"targetIds": ["1","2"],"complianceCheckTypes": ["OFFBOARDING"]}`

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

`{"results": [{"targetId": "1","outcome": "DELETED","exclusionIds": [10]}],"successCount": 2,"skippedCount": 1}`
