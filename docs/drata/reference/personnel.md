Title: Personnel

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Personnel/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Personnel are people who work for your organization. The [help docs](https://help.drata.com/en/collections/2653981) have more information.

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_listPersonnel)List Personnel

Get a paginated list of Personnel records.

🔒 Requires **Personnel & Users: List Personnel** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_listPersonnel!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_listPersonnel!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_listPersonnel!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_listPersonnel!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_listPersonnel!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_listPersonnel!in=query&path=expand[]&t=request)expand[]Array of strings (PersonnelExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"customFields""complianceChecks""reasonProvider""user"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_listPersonnel!in=query&path=employmentStatus[]&t=request)employmentStatus[]Array of strings (EmploymentStatusEnum)

Personnel by employment statuses

| Items Enum Value | **Description** |
| --- | --- |
| CURRENT_EMPLOYEE | Current Employee |
| FORMER_EMPLOYEE | Former Employee |
| CURRENT_CONTRACTOR | Current Contractor |
| FORMER_CONTRACTOR | Former Contractor |
| FUTURE_HIRE | Future Hire – Based on the HRIS data |
| UNKNOWN | Unknown – The personnel did not match an HRIS record |
| OUT_OF_SCOPE | Out of Scope – Manually marked as out of scope |
| SERVICE_ACCOUNT | Service Account – Automatically marked as out of scope |
| SPECIAL_FORMER_EMPLOYEE | Special Former Employee – Deprecated status for manually created personnel |
| SPECIAL_FORMER_CONTRACTOR | Special Former Contractor – Deprecated status for manually created personnel |
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_listPersonnel!in=query&path=complianceStatus[]&t=request)complianceStatus[]Array of strings (ComplianceCheckStatusEnum)

Filter Personnel by overall compliance status

Items Enum:"MISCONFIGURED""PASS""FAIL""EXCLUDED""NOT_ASSESSED"

Responses

200
Successful

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

get/personnel

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
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/personnel`,
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
*   412
*   500

1 more

application/json

`{"data": [{"id": 1,"userId": 1,"user": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"employmentStatus": "CURRENT_EMPLOYEE","notHumanReason": "This is not a real personnel, but a placeholder for anyone in charge of X","reasonProvider": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"complianceChecks": [{"id": 1,"type": "FULL_COMPLIANCE","status": "MISCONFIGURED","checkFrequency": "ONCE","expiresAt": "2019-08-24T14:15:22Z","lastCheckedAt": "2019-08-24T14:15:22Z","completionDate": "2019-08-24T14:15:22Z","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z","exclusion": {"id": 1,"reason": "Employee is on extended leave","createdById": 1,"createdByEmail": "user@company.com","createdAt": "2025-07-01T16:45:55.246Z","startDate": "2025-07-01T16:45:55.246Z","endDate": "2025-07-01T16:45:55.246Z","targetId": "1","targetType": "PERSONNEL","complianceCheckTypes": ["OFFBOARDING"]}}],"startedAt": "2023-01-01T00:00:00.000Z","separatedAt": "2023-12-31T00:00:00.000Z","statusUpdatedAt": "2019-08-24T14:15:22Z","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z","customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces)Search Personnel across all workspaces

Personnel listing across all accessible workspaces with full-text search via `q`, prefix matching on `firstName`/`lastName`, and a richer filter / sort surface than `GET /personnel`. Results are eventually consistent: use `GET /personnel` if read-your-writes freshness is required.

🔒 Requires **Personnel & Users: List Personnel** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=sort&t=request)sort string (PersonnelSortEnum)

Which field to sort by

Enum:"createdAt""updatedAt""name""startDate"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=expand[]&t=request)expand[]Array of strings (PersonnelSearchExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"customFields""complianceChecks""reasonProvider""user""groups""idpConnection""hrisConnection"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=facets[]&t=request)facets[]Array of strings (PersonnelSearchFacetEnum)

Filter dimensions for which to return value counts in the response. Only honored on the first page (when `cursor` is null). Each requested facet adds one terms aggregation to the OpenSearch query. Bounded enum facets are enriched with zero-count entries; unbounded facets (like `groupIds`) return only values present in the result set.

Items Enum:"employmentStatus""complianceStatus""syncStatus""groupIds""acceptedPoliciesStatus""bgCheckStatus""identityMfaStatus""securityTrainingStatus""hipaaTrainingStatus""aiAwarenessTrainingStatus""offboardingStatus""agentInstalledStatus""passwordManagerStatus""diskEncryptionStatus""antivirusStatus""autoUpdatesStatus""lockScreenStatus"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=q&t=request)q string<= 191 characters

Free-text search across `firstName`, `lastName`, `fullName`, `email`, and `jobTitle`. Fuzzy, prefix-aware, and relevance-ranked.

Example:q=Smith
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=firstName&t=request)firstName string<= 191 characters

Filter Personnel whose first name starts with the provided value (prefix match, case-insensitive). For example, `firstName=John` matches Personnel named John, Johnathan, Johnny, etc.

Example:firstName=John
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=lastName&t=request)lastName string<= 191 characters

Filter Personnel whose last name starts with the provided value (prefix match, case-insensitive). For example, `lastName=Sm` matches Personnel named Smith, Smithson, etc.

Example:lastName=Sm
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=employmentStatus[]&t=request)employmentStatus[]Array of strings (EmploymentStatusEnum)

Filter Personnel by one or more employment statuses.

| Items Enum Value | **Description** |
| --- | --- |
| CURRENT_EMPLOYEE | Current Employee |
| FORMER_EMPLOYEE | Former Employee |
| CURRENT_CONTRACTOR | Current Contractor |
| FORMER_CONTRACTOR | Former Contractor |
| FUTURE_HIRE | Future Hire – Based on the HRIS data |
| UNKNOWN | Unknown – The personnel did not match an HRIS record |
| OUT_OF_SCOPE | Out of Scope – Manually marked as out of scope |
| SERVICE_ACCOUNT | Service Account – Automatically marked as out of scope |
| SPECIAL_FORMER_EMPLOYEE | Special Former Employee – Deprecated status for manually created personnel |
| SPECIAL_FORMER_CONTRACTOR | Special Former Contractor – Deprecated status for manually created personnel |
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=complianceStatus[]&t=request)complianceStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by overall compliance status. `OUT_OF_SCOPE` matches personnel whose compliance status is suppressed (out-of-scope contractors, future hires, etc.).

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=acceptedPoliciesStatus[]&t=request)acceptedPoliciesStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by accepted-policies compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=bgCheckStatus[]&t=request)bgCheckStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by background-check compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=identityMfaStatus[]&t=request)identityMfaStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by identity MFA compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=securityTrainingStatus[]&t=request)securityTrainingStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by security training compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=hipaaTrainingStatus[]&t=request)hipaaTrainingStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by HIPAA training compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=aiAwarenessTrainingStatus[]&t=request)aiAwarenessTrainingStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by AI awareness training compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=offboardingStatus[]&t=request)offboardingStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by offboarding compliance status. Only matches personnel who have started or completed offboarding (i.e. have a separation date); currently-active personnel are not in scope for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=agentInstalledStatus[]&t=request)agentInstalledStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by whether the Drata Agent compliance check passes on their devices.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=passwordManagerStatus[]&t=request)passwordManagerStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by the device-rollup password-manager compliance status (worst-of-N across their devices). ⚠️ Coverage limitation: this filter excludes personnel without applicable device compliance data. This includes out-of-scope employment statuses, personnel with no devices, and personnel whose only device has an unknown management source. Filtering by `OUT_OF_SCOPE` returns no results for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=diskEncryptionStatus[]&t=request)diskEncryptionStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by the device-rollup disk-encryption compliance status (worst-of-N across their devices). ⚠️ Coverage limitation: this filter excludes personnel without applicable device compliance data. This includes out-of-scope employment statuses, personnel with no devices, and personnel whose only device has an unknown management source. Filtering by `OUT_OF_SCOPE` returns no results for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=antivirusStatus[]&t=request)antivirusStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by the device-rollup antivirus compliance status (worst-of-N across their devices). ⚠️ Coverage limitation: this filter excludes personnel without applicable device compliance data. This includes out-of-scope employment statuses, personnel with no devices, and personnel whose only device has an unknown management source. Filtering by `OUT_OF_SCOPE` returns no results for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=autoUpdatesStatus[]&t=request)autoUpdatesStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by the device-rollup auto-updates compliance status (worst-of-N across their devices). ⚠️ Coverage limitation: this filter excludes personnel without applicable device compliance data. This includes out-of-scope employment statuses, personnel with no devices, and personnel whose only device has an unknown management source. Filtering by `OUT_OF_SCOPE` returns no results for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=lockScreenStatus[]&t=request)lockScreenStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by the device-rollup lock-screen compliance status (worst-of-N across their devices). ⚠️ Coverage limitation: this filter excludes personnel without applicable device compliance data. This includes out-of-scope employment statuses, personnel with no devices, and personnel whose only device has an unknown management source. Filtering by `OUT_OF_SCOPE` returns no results for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=groupIds[]&t=request)groupIds[]Array of numbers

Filter Personnel by membership in any of the supplied group IDs.
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=syncStatus&t=request)syncStatus string (PersonnelSyncStatusPublicV2Enum)

Filter Personnel by whether they have any identity (HRIS/IDP) connection. `SYNCED` returns Personnel with at least one identity connection; `NOT_SYNCED` returns Personnel with none.

Enum:"SYNCED""NOT_SYNCED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=idpConnectionIds[]&t=request)idpConnectionIds[]Array of numbers

Filter Personnel by their identity provider connection IDs.
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnelAcrossWorkspaces!in=query&path=hrisConnectionIds[]&t=request)hrisConnectionIds[]Array of numbers

Filter Personnel by their HRIS provider connection IDs.
Array of objects (CustomFieldFilterPublicV2Dto)

Filter Personnel by Custom Field equality. Repeat the parameter with bracketed indices, e.g. `customFields[0][customFieldId]=42&customFields[0][value]=Engineering` (or `customFields[0][name]=Department` in place of `customFieldId`). Multiple entries are AND-ed.
Array of objects (CustomRangeFieldFilterPublicV2Dto)

Filter Personnel by Custom Field range. Repeat the parameter with bracketed indices, e.g. `customRangeFields[0][customFieldId]=99&customRangeFields[0][gte]=5&customRangeFields[0][lte]=10` (or `customRangeFields[0][name]=Score` in place of `customFieldId`). At least one of `gte` / `lte` must be supplied per entry. Multiple entries are AND-ed.

Responses

200
Successful

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

get/personnel-search

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
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/personnel-search`,
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
*   412
*   500

1 more

application/json

`{"data": [{"id": 1,"userId": 1,"user": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"employmentStatus": "CURRENT_EMPLOYEE","notHumanReason": "This is not a real personnel, but a placeholder for anyone in charge of X","reasonProvider": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"syncStatus": "SYNCED","idpConnectionId": 1,"hrisConnectionId": 2,"startedAt": "2023-01-01T00:00:00.000Z","separatedAt": "2023-12-31T00:00:00.000Z","statusUpdatedAt": "2019-08-24T14:15:22Z","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z","complianceChecks": {"data": [{"id": 1,"type": "FULL_COMPLIANCE","status": "MISCONFIGURED","checkFrequency": "ONCE","expiresAt": "2019-08-24T14:15:22Z","lastCheckedAt": "2019-08-24T14:15:22Z","completionDate": "2019-08-24T14:15:22Z","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z","exclusion": {"id": 1,"reason": "Employee is on extended leave","createdById": 1,"createdByEmail": "user@company.com","createdAt": "2025-07-01T16:45:55.246Z","startDate": "2025-07-01T16:45:55.246Z","endDate": "2025-07-01T16:45:55.246Z","targetId": "1","targetType": "PERSONNEL","complianceCheckTypes": ["OFFBOARDING"]}}],"totalCount": 142},"customFields": {"data": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}],"totalCount": 6},"groups": {"data": [{"id": 1,"name": "Engineering Team","externalId": "external-group-123","source": "GOOGLE","connectionId": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 12},"idpConnection": {"id": "1","clientType": "GOOGLE","clientId": "drata.com","clientAlias": "My-connection-alias-1","state": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectedAt": "2025-07-01T16:45:55.246Z","failedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"},"hrisConnection": {"id": "1","clientType": "GOOGLE","clientId": "drata.com","clientAlias": "My-connection-alias-1","state": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectedAt": "2025-07-01T16:45:55.246Z","failedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"}}],"pagination": {"cursor": "string","totalCount": 0},"facets": {"employmentStatus": [{"value": "CURRENT_EMPLOYEE","count": 312},{"value": "FORMER_EMPLOYEE","count": 24}]}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel)Search Personnel within a workspace

Workspace-scoped Personnel listing with full-text search via `q`, prefix matching on `firstName`/`lastName`, and a richer filter / sort surface than `GET /personnel`. Results are eventually consistent; use `GET /personnel` if read-your-writes freshness is required.

🔒 Requires **Personnel & Users: List Personnel** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=sort&t=request)sort string (PersonnelSortEnum)

Which field to sort by

Enum:"createdAt""updatedAt""name""startDate"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=expand[]&t=request)expand[]Array of strings (PersonnelSearchExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"customFields""complianceChecks""reasonProvider""user""groups""idpConnection""hrisConnection"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=facets[]&t=request)facets[]Array of strings (PersonnelSearchFacetEnum)

Filter dimensions for which to return value counts in the response. Only honored on the first page (when `cursor` is null). Each requested facet adds one terms aggregation to the OpenSearch query. Bounded enum facets are enriched with zero-count entries; unbounded facets (like `groupIds`) return only values present in the result set.

Items Enum:"employmentStatus""complianceStatus""syncStatus""groupIds""acceptedPoliciesStatus""bgCheckStatus""identityMfaStatus""securityTrainingStatus""hipaaTrainingStatus""aiAwarenessTrainingStatus""offboardingStatus""agentInstalledStatus""passwordManagerStatus""diskEncryptionStatus""antivirusStatus""autoUpdatesStatus""lockScreenStatus"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=q&t=request)q string<= 191 characters

Free-text search across `firstName`, `lastName`, `fullName`, `email`, and `jobTitle`. Fuzzy, prefix-aware, and relevance-ranked.

Example:q=Smith
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=firstName&t=request)firstName string<= 191 characters

Filter Personnel whose first name starts with the provided value (prefix match, case-insensitive). For example, `firstName=John` matches Personnel named John, Johnathan, Johnny, etc.

Example:firstName=John
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=lastName&t=request)lastName string<= 191 characters

Filter Personnel whose last name starts with the provided value (prefix match, case-insensitive). For example, `lastName=Sm` matches Personnel named Smith, Smithson, etc.

Example:lastName=Sm
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=employmentStatus[]&t=request)employmentStatus[]Array of strings (EmploymentStatusEnum)

Filter Personnel by one or more employment statuses.

| Items Enum Value | **Description** |
| --- | --- |
| CURRENT_EMPLOYEE | Current Employee |
| FORMER_EMPLOYEE | Former Employee |
| CURRENT_CONTRACTOR | Current Contractor |
| FORMER_CONTRACTOR | Former Contractor |
| FUTURE_HIRE | Future Hire – Based on the HRIS data |
| UNKNOWN | Unknown – The personnel did not match an HRIS record |
| OUT_OF_SCOPE | Out of Scope – Manually marked as out of scope |
| SERVICE_ACCOUNT | Service Account – Automatically marked as out of scope |
| SPECIAL_FORMER_EMPLOYEE | Special Former Employee – Deprecated status for manually created personnel |
| SPECIAL_FORMER_CONTRACTOR | Special Former Contractor – Deprecated status for manually created personnel |
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=complianceStatus[]&t=request)complianceStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by overall compliance status. `OUT_OF_SCOPE` matches personnel whose compliance status is suppressed (out-of-scope contractors, future hires, etc.).

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=acceptedPoliciesStatus[]&t=request)acceptedPoliciesStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by accepted-policies compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=bgCheckStatus[]&t=request)bgCheckStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by background-check compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=identityMfaStatus[]&t=request)identityMfaStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by identity MFA compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=securityTrainingStatus[]&t=request)securityTrainingStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by security training compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=hipaaTrainingStatus[]&t=request)hipaaTrainingStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by HIPAA training compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=aiAwarenessTrainingStatus[]&t=request)aiAwarenessTrainingStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by AI awareness training compliance status.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=offboardingStatus[]&t=request)offboardingStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by offboarding compliance status. Only matches personnel who have started or completed offboarding (i.e. have a separation date); currently-active personnel are not in scope for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=agentInstalledStatus[]&t=request)agentInstalledStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by whether the Drata Agent compliance check passes on their devices.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=passwordManagerStatus[]&t=request)passwordManagerStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by the device-rollup password-manager compliance status (worst-of-N across their devices). ⚠️ Coverage limitation: this filter excludes personnel without applicable device compliance data. This includes out-of-scope employment statuses, personnel with no devices, and personnel whose only device has an unknown management source. Filtering by `OUT_OF_SCOPE` returns no results for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=diskEncryptionStatus[]&t=request)diskEncryptionStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by the device-rollup disk-encryption compliance status (worst-of-N across their devices). ⚠️ Coverage limitation: this filter excludes personnel without applicable device compliance data. This includes out-of-scope employment statuses, personnel with no devices, and personnel whose only device has an unknown management source. Filtering by `OUT_OF_SCOPE` returns no results for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=antivirusStatus[]&t=request)antivirusStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by the device-rollup antivirus compliance status (worst-of-N across their devices). ⚠️ Coverage limitation: this filter excludes personnel without applicable device compliance data. This includes out-of-scope employment statuses, personnel with no devices, and personnel whose only device has an unknown management source. Filtering by `OUT_OF_SCOPE` returns no results for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=autoUpdatesStatus[]&t=request)autoUpdatesStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by the device-rollup auto-updates compliance status (worst-of-N across their devices). ⚠️ Coverage limitation: this filter excludes personnel without applicable device compliance data. This includes out-of-scope employment statuses, personnel with no devices, and personnel whose only device has an unknown management source. Filtering by `OUT_OF_SCOPE` returns no results for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=lockScreenStatus[]&t=request)lockScreenStatus[]Array of strings (ComplianceCheckStatusPublicV2Enum)

Filter Personnel by the device-rollup lock-screen compliance status (worst-of-N across their devices). ⚠️ Coverage limitation: this filter excludes personnel without applicable device compliance data. This includes out-of-scope employment statuses, personnel with no devices, and personnel whose only device has an unknown management source. Filtering by `OUT_OF_SCOPE` returns no results for this filter.

Items Enum:"PASS""FAIL""MISCONFIGURED""EXCLUDED""OUT_OF_SCOPE""NOT_ASSESSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=groupIds[]&t=request)groupIds[]Array of numbers

Filter Personnel by membership in any of the supplied group IDs.
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=syncStatus&t=request)syncStatus string (PersonnelSyncStatusPublicV2Enum)

Filter Personnel by whether they have any identity (HRIS/IDP) connection. `SYNCED` returns Personnel with at least one identity connection; `NOT_SYNCED` returns Personnel with none.

Enum:"SYNCED""NOT_SYNCED"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=idpConnectionIds[]&t=request)idpConnectionIds[]Array of numbers

Filter Personnel by their identity provider connection IDs.
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_searchPersonnel!in=query&path=hrisConnectionIds[]&t=request)hrisConnectionIds[]Array of numbers

Filter Personnel by their HRIS provider connection IDs.
Array of objects (CustomFieldFilterPublicV2Dto)

Filter Personnel by Custom Field equality. Repeat the parameter with bracketed indices, e.g. `customFields[0][customFieldId]=42&customFields[0][value]=Engineering` (or `customFields[0][name]=Department` in place of `customFieldId`). Multiple entries are AND-ed.
Array of objects (CustomRangeFieldFilterPublicV2Dto)

Filter Personnel by Custom Field range. Repeat the parameter with bracketed indices, e.g. `customRangeFields[0][customFieldId]=99&customRangeFields[0][gte]=5&customRangeFields[0][lte]=10` (or `customRangeFields[0][name]=Score` in place of `customFieldId`). At least one of `gte` / `lte` must be supplied per entry. Multiple entries are AND-ed.

Responses

200
Successful

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

get/workspaces/{workspaceId}/personnel-search

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/personnel-search`,
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
*   412
*   500

1 more

application/json

`{"data": [{"id": 1,"userId": 1,"user": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"employmentStatus": "CURRENT_EMPLOYEE","notHumanReason": "This is not a real personnel, but a placeholder for anyone in charge of X","reasonProvider": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"syncStatus": "SYNCED","idpConnectionId": 1,"hrisConnectionId": 2,"startedAt": "2023-01-01T00:00:00.000Z","separatedAt": "2023-12-31T00:00:00.000Z","statusUpdatedAt": "2019-08-24T14:15:22Z","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z","complianceChecks": {"data": [{"id": 1,"type": "FULL_COMPLIANCE","status": "MISCONFIGURED","checkFrequency": "ONCE","expiresAt": "2019-08-24T14:15:22Z","lastCheckedAt": "2019-08-24T14:15:22Z","completionDate": "2019-08-24T14:15:22Z","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z","exclusion": {"id": 1,"reason": "Employee is on extended leave","createdById": 1,"createdByEmail": "user@company.com","createdAt": "2025-07-01T16:45:55.246Z","startDate": "2025-07-01T16:45:55.246Z","endDate": "2025-07-01T16:45:55.246Z","targetId": "1","targetType": "PERSONNEL","complianceCheckTypes": ["OFFBOARDING"]}}],"totalCount": 142},"customFields": {"data": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}],"totalCount": 6},"groups": {"data": [{"id": 1,"name": "Engineering Team","externalId": "external-group-123","source": "GOOGLE","connectionId": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 12},"idpConnection": {"id": "1","clientType": "GOOGLE","clientId": "drata.com","clientAlias": "My-connection-alias-1","state": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectedAt": "2025-07-01T16:45:55.246Z","failedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"},"hrisConnection": {"id": "1","clientType": "GOOGLE","clientId": "drata.com","clientAlias": "My-connection-alias-1","state": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectedAt": "2025-07-01T16:45:55.246Z","failedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"}}],"pagination": {"cursor": "string","totalCount": 0},"facets": {"employmentStatus": [{"value": "CURRENT_EMPLOYEE","count": 312},{"value": "FORMER_EMPLOYEE","count": 24}]}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_getPerson)Get Personnel

Get a single Personnel record.

🔒 Requires **Personnel & Users: Get Personnel Details** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

An integer Personnel ID or User's email address prefixed with `email:`

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_getPerson!in=query&path=expand[]&t=request)expand[]Array of strings (PersonnelExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"customFields""complianceChecks""reasonProvider""user"

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

get/personnel/{personnelId}

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
  const personnelId = 'YOUR_personnelId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/personnel/${personnelId}`,
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

`{"id": 1,"userId": 1,"user": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"employmentStatus": "CURRENT_EMPLOYEE","notHumanReason": "This is not a real personnel, but a placeholder for anyone in charge of X","reasonProvider": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"complianceChecks": [{"id": 1,"type": "FULL_COMPLIANCE","status": "MISCONFIGURED","checkFrequency": "ONCE","expiresAt": "2019-08-24T14:15:22Z","lastCheckedAt": "2019-08-24T14:15:22Z","completionDate": "2019-08-24T14:15:22Z","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z","exclusion": {"id": 1,"reason": "Employee is on extended leave","createdById": 1,"createdByEmail": "user@company.com","createdAt": "2025-07-01T16:45:55.246Z","startDate": "2025-07-01T16:45:55.246Z","endDate": "2025-07-01T16:45:55.246Z","targetId": "1","targetType": "PERSONNEL","complianceCheckTypes": ["OFFBOARDING"]}}],"startedAt": "2023-01-01T00:00:00.000Z","separatedAt": "2023-12-31T00:00:00.000Z","statusUpdatedAt": "2019-08-24T14:15:22Z","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z","customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_modifyPerson)Update Personnel

Update a single Personnel record. Note: Once fields are manually updated, automatic updates from identity providers (IDP) and HRIS systems will be ignored for those fields. Use the resync endpoint to restore automatic updates.

🔒 Requires **Personnel & Users: Update Personnel (API v2)** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

An integer Personnel ID or User's email address prefixed with `email:`

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_modifyPerson!path=startedAt&t=request)startedAt string

The date when this person started working at the company. Note: Once manually set, automatic updates from identity providers (IDP) and HRIS systems will be ignored. Use the resync endpoint to restore automatic updates.
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_modifyPerson!path=separatedAt&t=request)separatedAt string

The date when this person was separated from the company system.
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_modifyPerson!path=employmentStatus&t=request)employmentStatus string

The desired employment status to be updated

| Enum Value | **Description** |
| --- | --- |
| CURRENT_EMPLOYEE | Current Employee |
| FORMER_EMPLOYEE | Former Employee |
| CURRENT_CONTRACTOR | Current Contractor |
| FORMER_CONTRACTOR | Former Contractor |
| FUTURE_HIRE | Future Hire – Based on the HRIS data |
| UNKNOWN | Unknown – The personnel did not match an HRIS record |
| OUT_OF_SCOPE | Out of Scope – Manually marked as out of scope |
| SERVICE_ACCOUNT | Service Account – Automatically marked as out of scope |
| SPECIAL_FORMER_EMPLOYEE | Special Former Employee – Deprecated status for manually created personnel |
| SPECIAL_FORMER_CONTRACTOR | Special Former Contractor – Deprecated status for manually created personnel |
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_modifyPerson!path=notHumanReason&t=request)notHumanReason string<= 30000 characters

Explains why the employment status of this personnel is marked as `OUT_OF_SCOPE`. This field is required if the `employmentStatus` is set to `OUT_OF_SCOPE`.
Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Person.

💎 Requires your account have the Custom Fields and Formulas feature. Contact your Account Manager for help upgrading.

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

put/personnel/{personnelId}

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

`{"startedAt": "2020-07-06","separatedAt": "2020-07-06","employmentStatus": "CURRENT_CONTRACTOR","notHumanReason": "This is not a real personnel, but a placeholder for anyone in charge of X","customFields": [{"id": 1,"name": "Compliance Status","value": "Security & IT"}]}`

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

`{"id": 1,"userId": 1,"user": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"employmentStatus": "CURRENT_EMPLOYEE","notHumanReason": "This is not a real personnel, but a placeholder for anyone in charge of X","reasonProvider": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"complianceChecks": [{"id": 1,"type": "FULL_COMPLIANCE","status": "MISCONFIGURED","checkFrequency": "ONCE","expiresAt": "2019-08-24T14:15:22Z","lastCheckedAt": "2019-08-24T14:15:22Z","completionDate": "2019-08-24T14:15:22Z","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z","exclusion": {"id": 1,"reason": "Employee is on extended leave","createdById": 1,"createdByEmail": "user@company.com","createdAt": "2025-07-01T16:45:55.246Z","startDate": "2025-07-01T16:45:55.246Z","endDate": "2025-07-01T16:45:55.246Z","targetId": "1","targetType": "PERSONNEL","complianceCheckTypes": ["OFFBOARDING"]}}],"startedAt": "2023-01-01T00:00:00.000Z","separatedAt": "2023-12-31T00:00:00.000Z","statusUpdatedAt": "2019-08-24T14:15:22Z","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z","customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_performPersonnelAction)Reset Personnel Sync

Perform actions on Personnel records. Supports `reset-sync` for specific Personnel IDs and `reset-sync-all` for all Personnel.

🔒 Requires **Personnel & Users: Reset IdP/HRIS Sync Status** permission.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_performPersonnelAction!path=action&t=request)action
required string

The action to perform on Personnel

Enum:"reset-sync""reset-sync-all"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/PersonnelPublicV2Controller_performPersonnelAction!path=personnelIds&t=request)personnelIds Array of numbers

Array of Personnel IDs to perform the action on. Required for `reset-sync` action. Must not be provided for `reset-sync-all` action.

Responses

201
Action performed successfully

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

409
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/personnel/actions

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

`{"action": "reset-sync","personnelIds": [1,2,3]}`

Response samples

*   201
*   400
*   401
*   403
*   409
*   412
*   500

2 more

application/json

`{"count": 42}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_listScopedPersonnelGroups)List Workspace Scoped Personnel Groups 🧪

Returns a cursor-paginated list of personnel groups currently assigned to the workspace's scope. Re-pass all original query parameters (other than the cursor) on subsequent pages — the cursor only tracks position, not query context.

🧪 **Note:** This endpoint is in beta and may change.

🔒 Requires **Workspaces: View Workspace Personnel Scope** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_listScopedPersonnelGroups!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_listScopedPersonnelGroups!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_listScopedPersonnelGroups!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_listScopedPersonnelGroups!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_listScopedPersonnelGroups!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"

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

get/workspaces/{workspaceId}/scoped-personnel-groups

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/scoped-personnel-groups`,
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

`{"data": [{"id": 42,"name": "IT Ops","connectionProvider": "GOOGLE"}],"pagination": {"cursor": "string"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_addScopedPersonnelGroup)Add Personnel Group To Workspace Scope 🧪

Adds a single personnel group to the workspace's scope. Returns 409 when the group is already assigned (active), and 400 when the group does not exist or is not synced.

🧪 **Note:** This endpoint is in beta and may change.

🔒 Requires **Workspaces: Replace Workspace Personnel Scope** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_addScopedPersonnelGroup!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_addScopedPersonnelGroup!path=groupId&t=request)groupId
required number

Personnel group ID to add to the workspace scope.

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

409
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/workspaces/{workspaceId}/scoped-personnel-groups

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

`{"groupId": 42}`

Response samples

*   201
*   400
*   401
*   403
*   404
*   409
*   412
*   500

3 more

application/json

`{"id": 42,"name": "IT Ops","connectionProvider": "GOOGLE"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_putScopedPersonnelGroups)Replace Workspace Scoped Personnel Groups 🧪

Full replace of the personnel groups in scope for the workspace. Idempotent: groups in the payload not currently in scope are added, groups currently in scope but absent from the payload are removed. Omit a group to remove it from the scope.

🧪 **Note:** This endpoint is in beta and may change.

🔒 Requires **Workspaces: Replace Workspace Personnel Scope** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_putScopedPersonnelGroups!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_putScopedPersonnelGroups!path=groupIds&t=request)groupIds
required Array of numbers

Full set of Personnel Group IDs that should be in scope after the call returns. Omitting a group currently in scope removes it; including a new group adds it. Pass an empty array to clear the scope entirely. Idempotent.

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

put/workspaces/{workspaceId}/scoped-personnel-groups

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

`{"groupIds": [1,2,3]}`

Response samples

*   400
*   401
*   403
*   404
*   412
*   500

1 more

application/json

`{"statusCode": 0,"message": "string","code": 0,"debugInfo": {"name": "string","message": "string","stack": "string"},"providerError": {"provider": "string","status": 0,"code": "string","message": "string","details": { }}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_removeScopedPersonnelGroup)Remove Personnel Group From Workspace Scope 🧪

🧪 **Note:** This endpoint is in beta and may change.

🔒 Requires **Workspaces: Remove Group From Workspace Personnel Scope** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_removeScopedPersonnelGroup!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_removeScopedPersonnelGroup!in=path&path=groupId&t=request)groupId
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

delete/workspaces/{workspaceId}/scoped-personnel-groups/{groupId}

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
  const groupId = 'YOUR_groupId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/scoped-personnel-groups/${groupId}`,
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

## [](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_getScopedPersonnelCounts)Get Workspace Scoped Personnel Counts 🧪

Returns the deduplicated count of personnel reachable through the workspace's scoped groups and the tenant-wide personnel total.

🧪 **Note:** This endpoint is in beta and may change.

🔒 Requires **Workspaces: View Workspace Personnel Scope** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Personnel/#tag/Personnel/operation/ScopedPersonnelGroupsPublicV2Controller_getScopedPersonnelCounts!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

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

get/workspaces/{workspaceId}/scoped-personnel-counts

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/scoped-personnel-counts`,
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

`{"scopedPersonnelCount": 42,"totalPersonnelCount": 120}`
