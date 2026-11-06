Title: Policies

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Policies/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
A policy is a document that outlines an organization’s commitment to following standards relevant to its operations. The [help docs](https://help.drata.com/en/articles/9202419-policy-center-overview) have more information.

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies)List Policies

List published Policies matching the provided filters.

🔒 Requires **Policies: List Policies** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies!in=query&path=expand[]&t=request)expand[]Array of strings (PolicyListExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"groups""weekTimeFrameSlas""gracePeriodSlas""p3MatrixSlas""owner"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies!in=query&path=name&t=request)name string<= 191 characters

Filter Policies by name (partial match)

Example:name=Data Protection Policy
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies!in=query&path=statuses[]&t=request)statuses[]Array of strings (PolicyStatusEnum)

Filter Policies by one or more statuses

Items Enum:"ACTIVE""ARCHIVED""REPLACED""UNACCEPTABLE""OUTDATED"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies!in=query&path=reviewerUserId&t=request)reviewerUserId string

Filter Policies by a reviewer. A Drata integer ID, an email address of the form 'email:value', or 'me' to reference the authenticated user. Note: 'me' is only available when authenticating with OAuth 2.1 Authorization Code grant, not with Public API Keys.

Example:reviewerUserId=me
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicies!in=query&path=reviewerStatus[]&t=request)reviewerStatus[]Array of strings (ReviewStatusEnum)

Filter Policies by the reviewer's review status on an active approval. Typically used with reviewerUserId.

Items Enum:"READY_FOR_REVIEW""APPROVED""CHANGES_REQUESTED""DEADLINE_EXCEEDED"

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

get/policies

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
    `https://public-api.drata.com/public/v2/policies`,
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

`{"data": [{"id": 1,"name": "Acceptable Use Policy","description": "This policy covers acceptable use of company resources.","disclaimer": "This policy is subject to change.","scope": "ALL","notifyGroups": false,"status": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","currentVersionId": 2,"version": "1","subVersion": "0","renewalDate": "2025-07-01T16:45:55.246Z","publishedAt": "2025-07-01T16:45:55.246Z","approvedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"groups": [{"id": 1,"name": "Engineering Team","externalId": "external-group-123","source": "GOOGLE","connectionId": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"weekTimeFrameSlas": [{"id": 1,"timeFrame": "1","label": "Weekly Review","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"gracePeriodSlas": [{"id": 1,"label": "Grace Period Review","gracePeriod": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"p3MatrixSlas": [{"id": 1,"label": "P3 Matrix Review","timeFrame": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy)Create Policy

Create a new Policy with an initial draft version. Supports file upload (UPLOADED) or an existing external file reference (EXTERNAL).

🔒 Requires **Policies: Create Policy** permission.

Security

**bearer** or **oauth2**

Request

##### Request Body schema:

multipart/form-data

required

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=name&t=request)name
required string<= 191 characters

Human-readable policy title shown in Drata
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=ownerId&t=request)ownerId
required number

User ID assigned as the owner of this policy
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=sourceType&t=request)sourceType
required string

Determines how policy content is sourced. UPLOADED requires a file attachment; EXTERNAL requires externalFileId.

Enum:"UPLOADED""EXTERNAL"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=renewalDate&t=request)renewalDate
required string<date>

Date when the policy is next due for review/renewal. Must be today or a future date.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=description&t=request)description
required string<= 30000 characters

Short description of the policy purpose
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=disclaimer&t=request)disclaimer string<= 1000 characters

Legal disclaimer text shown with the Policy
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=externalFileId&t=request)externalFileId number

ID of an existing external file record in Drata. Required when sourceType is EXTERNAL. Must belong to the caller's tenant.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=fileName&t=request)fileName string<= 191 characters

Friendly file name for display in the UI
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=file&t=request)file string<binary>

Policy file. Required when `sourceType` is `UPLOADED`. Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .gif, .jpg, .jpeg, .png, .json, .csv, .md, .markdown, .txt, .html, .log, .zip, .msg, .mp4
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=requiresAcknowledgement&t=request)requiresAcknowledgement boolean

Whether this policy requires user acknowledgement once published
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=assignedTo&t=request)assignedTo string

Assignment mode for policy acknowledgement

Enum:"ALL""GROUP""NONE"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicy!ct=multipart/form-data&path=groupIds&t=request)groupIds Array of numbers

IDs of groups who must acknowledge the policy. Required when assignedTo is GROUP.

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

post/policies

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

multipart/form-data

No sample

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

`{"id": 99,"name": "Data Backup Policy","description": null,"ownerId": 42,"status": "ACTIVE","requiresAcknowledgement": true,"assignedTo": "ALL","renewalDate": "2020-07-06","createdAt": "2025-07-01T16:45:55.246Z","latestVersion": {"id": 201,"version": 1,"subVersion": 0,"status": "DRAFT","sourceType": "UPLOADED","externalFileId": null,"fileName": "data-backup-policy.pdf","createdAt": "2025-07-01T16:45:55.246Z"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions)List Policy Versions

List Policy Versions for a specific Policy matching the provided filters. Language variants are not expandable here — `languages` is a one-to-many that would multiply rows inside a paginated result set. Use `GET /policy-language-versions?policyId=` for language variants across a whole Policy, or expand `languages` on the single Policy Version endpoint.

🔒 Requires **Policies: List Policies** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions!in=path&path=policyId&t=request)policyId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions!in=query&path=expand[]&t=request)expand[]Array of strings (PolicyVersionListExpandEnum)

List of subcollections and sub-objects to expand. Note `languages` is not expandable here — it is a one-to-many that would multiply rows inside a paginated result set. Use `GET /policy-language-versions?policyVersionId=` for language variants, or expand `languages` on the single Policy Version endpoint.

Items Enum:"owner""weekTimeFrameSlas""p3MatrixSlas""gracePeriodSlas""downloadUrl""downloadPdfUrl"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions!in=query&path=version&t=request)version number

Filter Policy Versions by version number

Example:version=1
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions!in=query&path=current&t=request)current boolean

Filter to only current Policy Versions
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyVersions!in=query&path=statuses[]&t=request)statuses[]Array of strings (PolicyVersionStatusEnum)

Filter Policy Versions by status

Items Enum:"NEEDS_APPROVAL""APPROVED""PUBLISHED""DRAFT""DISCARDED"

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

get/policies/{policyId}/policy-versions

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
  const policyId = 'YOUR_policyId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/policies/${policyId}/policy-versions`,
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

`{"data": [{"id": 1,"approvedAt": "2025-07-01T16:45:55.246Z","changeSummary": "string","changesExplanation": "string","createdAt": "2025-07-01T16:45:55.246Z","current": true,"description": "string","policyVersionStatus": "PUBLISHED","publishedAt": "2025-07-01T16:45:55.246Z","renewalDate": "2020-07-06","subVersion": 0,"type": "BUILDER","updatedAt": "2025-07-01T16:45:55.246Z","version": 1,"gracePeriodSlas": [{"id": 1,"label": "Grace Period Review","gracePeriod": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"p3MatrixSlas": [{"id": 1,"label": "P3 Matrix Review","timeFrame": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"policy": {"id": 1,"name": "Acceptable Use Policy","description": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assignedTo": "ALL","policyStatus": "ACTIVE","renewalDate": "2025-07-01T16:45:55.246Z"},"requiresAcknowledgment": true,"weekTimeFrameSlas": [{"id": 1,"timeFrame": "1","label": "Weekly Review","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"languages": [{"id": 1,"language": "en-US","default": false,"hasFile": true,"hasPdf": true,"hasHtml": false,"originalFileName": "information-security-policy-es.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion)Create Policy Version

Create a new version of an existing Policy. Supports file upload (UPLOADED) or an existing external file reference (EXTERNAL). A non-material change that does not require approval is published immediately; otherwise the version is created for approval. A material change always requires approval. The Policy must already have a current published version to base the new version on; if it does not (for example its only version is still a draft or awaiting approval), the request returns 404.

🔒 Requires **Policies: Create Policy Version** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!in=path&path=policyId&t=request)policyId
required number

##### Request Body schema:

multipart/form-data

required

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=sourceType&t=request)sourceType
required string

Determines how the new version content is sourced. UPLOADED requires a file attachment; EXTERNAL requires externalFileId.

Enum:"UPLOADED""EXTERNAL"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=ownerId&t=request)ownerId
required number

User ID assigned as the owner of this Policy Version
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=isMaterialChange&t=request)isMaterialChange
required boolean

Whether this is a material change: a substantive change to the policy content (as opposed to a typo or formatting fix). A material change bumps the major version (e.g. 1.x → 2.0), always requires approval, and requires every assigned user to re-accept the policy once it is published. A non-material change bumps the minor version (e.g. 1.0 → 1.1) and does not trigger re-acceptance.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=requiresApproval&t=request)requiresApproval boolean

Whether a non-material version must be approved before it is published. Defaults to `false`, which publishes a non-material change immediately. Ignored when `isMaterialChange` is `true`, since a material change always requires approval.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=renewalDate&t=request)renewalDate
required string<date>

Date when this Policy Version is next due for review/renewal. Must be today or a future date.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=externalFileId&t=request)externalFileId number

ID of an existing external file record in Drata. Required when sourceType is EXTERNAL. Must belong to the caller's tenant.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=fileName&t=request)fileName string<= 191 characters

Friendly file name for display in the UI
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=description&t=request)description string<= 30000 characters

Short description of this Policy Version
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=changeSummary&t=request)changeSummary string<= 30000 characters

A short summary of the changes made in this version
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=changesExplanation&t=request)changesExplanation string<= 768 characters

Detailed explanation of what changed in this version
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=requiresAcknowledgement&t=request)requiresAcknowledgement boolean

Whether users must acknowledge this Policy Version once published
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_createPolicyVersion!ct=multipart/form-data&path=file&t=request)file string<binary>

Policy file. Required when `sourceType` is `UPLOADED`. Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .gif, .jpg, .jpeg, .png, .json, .csv, .md, .markdown, .txt, .html, .log, .zip, .msg, .mp4

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

post/policies/{policyId}/policy-versions

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

multipart/form-data

No sample

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

`{"id": 1,"approvedAt": "2025-07-01T16:45:55.246Z","changeSummary": "string","changesExplanation": "string","createdAt": "2025-07-01T16:45:55.246Z","current": true,"description": "string","policyVersionStatus": "PUBLISHED","publishedAt": "2025-07-01T16:45:55.246Z","renewalDate": "2020-07-06","subVersion": 0,"type": "BUILDER","updatedAt": "2025-07-01T16:45:55.246Z","version": 1,"gracePeriodSlas": [{"id": 1,"label": "Grace Period Review","gracePeriod": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"p3MatrixSlas": [{"id": 1,"label": "P3 Matrix Review","timeFrame": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"policy": {"id": 1,"name": "Acceptable Use Policy","description": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assignedTo": "ALL","policyStatus": "ACTIVE","renewalDate": "2025-07-01T16:45:55.246Z"},"requiresAcknowledgment": true,"weekTimeFrameSlas": [{"id": 1,"timeFrame": "1","label": "Weekly Review","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"downloadUrl": "https://s3.amazonaws.com/bucket/file.pdf?signature=...","downloadPdfUrl": "https://s3.amazonaws.com/bucket/file.pdf?signature=...","languages": [{"id": 1,"language": "en-US","default": false,"hasFile": true,"hasPdf": true,"hasHtml": false,"originalFileName": "information-security-policy-es.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_replacePolicyVersionFile)Replace Policy Version File

Replace the file backing an existing Policy Version. Omit `language` to replace the Policy Version content itself, which is the behavior for a single-language policy. Set `language` to one of the languages configured for the organization (see `GET /policy-language-settings`) to replace only that language variant, leaving the Policy Version content itself untouched. Re-uploading a language that was previously removed replaces it in place rather than creating a second variant. Uploading for the default language of an authored (WYSIWYG) version is rejected, since that would convert the whole version to an uploaded one; use the version-level upload without a `language` to do that explicitly. The Policy Version must not be published: a published version is a record, so replacing its content requires creating a new version.

🔒 Requires **Policies: Create Policy Version** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_replacePolicyVersionFile!in=path&path=policyId&t=request)policyId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_replacePolicyVersionFile!in=path&path=policyVersionId&t=request)policyVersionId
required number

##### Request Body schema:

multipart/form-data

required

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_replacePolicyVersionFile!ct=multipart/form-data&path=language&t=request)language string

The language variant this upload targets, as a BCP-47 locale code. Must be one of the languages configured for the organization — see `GET /policy-language-settings`. Omit to replace the Policy Version content itself, which is the behavior for a single-language policy. When set to a non-default language, only that language variant's content is replaced and the Policy Version's own content is left untouched; re-uploading a language that was previously removed replaces it in place rather than creating a second variant.

Enum:"en-US""es-LA""de-DE""fr-FR""it-IT""nl""af-ZA""sq-AL""ar-SA""hy-AM""as-IN""az-AZ""eu-ES""bn-BD""bs-BA""bg-BG""my-MM""ca-ES""ceb-PH""zh-CN""zh-TW""hr-HR""cs-CZ""da-DK""et-EE""fi-FI""ff-SN""gl-ES""ka-GE""el-GR""gu-IN""ht-HT""ha-NG""he-IL""hi-IN""hu-HU""is-IS""ig-NG""id-ID""ja-JP""jv-ID""kn-IN""kk-KZ""km-KH""ko-KR""ku-TR""ky-KG""lv-LV""lt-LT""mk-MK""mai-IN""mg-MG""ms-MY""ml-IN""mi-NZ""mr-IN""mn-MN""ne-NP""no-NO""om-ET""fa-IR""pl-PL""pt-PT""pa-IN""ro-RO""ru-RU""sr-RS""sn-ZW""sd-IN""si-LK""sk-SK""sl-SI""so-SO""su-ID""sw-KE""sv-SE""tl-PH""ta-IN""te-IN""th-TH""tr-TR""uk-UA""ur-PK""uz-UZ""vi-VN""cy-GB""wo-SN""xh-ZA""yo-NG""zu-ZA""xx-LS""en-XA"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_replacePolicyVersionFile!ct=multipart/form-data&path=file&t=request)file
required string<binary>

Replacement policy file. Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .gif, .jpg, .jpeg, .png, .json, .csv, .md, .markdown, .txt, .html, .log, .zip, .msg, .mp4

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

409
The Policy Version is published. A published version is a record and its content cannot be replaced — create a new version instead.

412
The Policy Version exists but does not belong to the Policy named in the path.

500
Internal server error

put/policies/{policyId}/policy-versions/{policyVersionId}/file

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

multipart/form-data

No sample

Response samples

*   200
*   400
*   401
*   403
*   404
*   409
*   412
*   500

3 more

application/json

`{"id": 1,"approvedAt": "2025-07-01T16:45:55.246Z","changeSummary": "string","changesExplanation": "string","createdAt": "2025-07-01T16:45:55.246Z","current": true,"description": "string","policyVersionStatus": "PUBLISHED","publishedAt": "2025-07-01T16:45:55.246Z","renewalDate": "2020-07-06","subVersion": 0,"type": "BUILDER","updatedAt": "2025-07-01T16:45:55.246Z","version": 1,"gracePeriodSlas": [{"id": 1,"label": "Grace Period Review","gracePeriod": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"p3MatrixSlas": [{"id": 1,"label": "P3 Matrix Review","timeFrame": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"policy": {"id": 1,"name": "Acceptable Use Policy","description": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assignedTo": "ALL","policyStatus": "ACTIVE","renewalDate": "2025-07-01T16:45:55.246Z"},"requiresAcknowledgment": true,"weekTimeFrameSlas": [{"id": 1,"timeFrame": "1","label": "Weekly Review","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"downloadUrl": "https://s3.amazonaws.com/bucket/file.pdf?signature=...","downloadPdfUrl": "https://s3.amazonaws.com/bucket/file.pdf?signature=...","languages": [{"id": 1,"language": "en-US","default": false,"hasFile": true,"hasPdf": true,"hasHtml": false,"originalFileName": "information-security-policy-es.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_getPolicy)Get Policy

Get a specific published Policy.

🔒 Requires **Policies: List Policies** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_getPolicy!in=path&path=policyId&t=request)policyId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_getPolicy!in=query&path=expand[]&t=request)expand[]Array of strings (PolicyExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"groups""controls""weekTimeFrameSlas""gracePeriodSlas""p3MatrixSlas""owner"

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

get/policies/{policyId}

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
  const policyId = 'YOUR_policyId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/policies/${policyId}`,
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

`{"id": 1,"name": "Acceptable Use Policy","description": "This policy covers acceptable use of company resources.","disclaimer": "This policy is subject to change.","scope": "ALL","notifyGroups": false,"status": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","currentVersionId": 2,"version": "1","subVersion": "0","renewalDate": "2025-07-01T16:45:55.246Z","publishedAt": "2025-07-01T16:45:55.246Z","approvedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"groups": [{"id": 1,"name": "Engineering Team","externalId": "external-group-123","source": "GOOGLE","connectionId": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"weekTimeFrameSlas": [{"id": 1,"timeFrame": "1","label": "Weekly Review","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"gracePeriodSlas": [{"id": 1,"label": "Grace Period Review","gracePeriod": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"p3MatrixSlas": [{"id": 1,"label": "P3 Matrix Review","timeFrame": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy)Modify Policy

🔒 Requires **Policies: Modify Policy** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy!in=path&path=policyId&t=request)policyId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy!path=name&t=request)name string

Policy name. Only updatable for custom policies. Drata template policies return 400.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy!path=description&t=request)description string

Policy description.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy!path=disclaimer&t=request)disclaimer string or null

Legal disclaimer text. Pass null to clear.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy!path=renewalDate&t=request)renewalDate string

Policy renewal date. Mutually exclusive with `renewalSchedule`.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy!path=renewalSchedule&t=request)renewalSchedule string

Renewal schedule type. Auto-computes `renewalDate` as today + N months. Mutually exclusive with `renewalDate`.

Enum:"ONE_MONTH""TWO_MONTHS""THREE_MONTHS""SIX_MONTHS""ONE_YEAR"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy!path=assignedTo&t=request)assignedTo string

Personnel assignment scope.

Enum:"ALL""GROUP""NONE"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy!path=groupIds&t=request)groupIds Array of numbers

Fully replaces existing group assignments. Required when assignedTo=GROUP. Must not be set when `assignedTo` is not GROUP. Must contain at least one valid group ID.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy!path=notifyGroups&t=request)notifyGroups boolean

Notify group members of this Policy. Only valid when assignedTo=GROUP, defaults to false if not provided.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_modifyPolicy!path=controlIds&t=request)controlIds Array of numbers

Fully replaces existing control assignments. Pass an empty array to remove all.

Responses

200
Successful

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

put/policies/{policyId}

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

`{"name": "My Custom Policy","description": "This policy covers data backup procedures.","disclaimer": "This policy is subject to change.","renewalDate": "2020-07-06","renewalSchedule": "ONE_YEAR","assignedTo": "GROUP","groupIds": [1,2,3],"notifyGroups": true,"controlIds": [10,20,30]}`

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

`{"id": 1,"name": "Acceptable Use Policy","description": "This policy covers acceptable use of company resources.","disclaimer": "This policy is subject to change.","scope": "ALL","notifyGroups": false,"status": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","currentVersionId": 2,"version": "1","subVersion": "0","renewalDate": "2025-07-01T16:45:55.246Z","publishedAt": "2025-07-01T16:45:55.246Z","approvedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"groups": [{"id": 1,"name": "Engineering Team","externalId": "external-group-123","source": "GOOGLE","connectionId": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"weekTimeFrameSlas": [{"id": 1,"timeFrame": "1","label": "Weekly Review","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"gracePeriodSlas": [{"id": 1,"label": "Grace Period Review","gracePeriod": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"p3MatrixSlas": [{"id": 1,"label": "P3 Matrix Review","timeFrame": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_assignPolicyOwner)Assign Policy Owner

🔒 Requires **Policies: Assign Policy Owner** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_assignPolicyOwner!in=path&path=policyId&t=request)policyId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_assignPolicyOwner!path=userId&t=request)userId
required number

ID of the user to assign as the policy owner. The user must have one of the following roles: Admin, Tech/Gov, Workspace Administrator, Policy Manager, or Control Manager.

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

put/policies/{policyId}/owner

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

`{"userId": 42}`

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

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_getApprovalConfiguration)Get Policy Approval Configuration

🔒 Requires **Policies: Get Policy Approval Configuration** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_getApprovalConfiguration!in=path&path=policyId&t=request)policyId
required number

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

get/policies/{policyId}/approval-configuration

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
  const policyId = 'YOUR_policyId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/policies/${policyId}/approval-configuration`,
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

`{"reviewGroups": [{"name": "Legal Review","tier": 1,"consensusRule": "ALL","timeline": 7,"approvers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_addPolicyApprovalConfiguration)Add Review Group Configuration

Appends a new review group configuration (tier) to the end of the approval sequence. Other tiers are not affected. Maximum of 6 tiers per policy.

🔒 Requires **Policies: Add Policy Review Group Configuration** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_addPolicyApprovalConfiguration!in=path&path=policyId&t=request)policyId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_addPolicyApprovalConfiguration!path=name&t=request)name
required string

Name of this approval tier.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_addPolicyApprovalConfiguration!path=userIds&t=request)userIds
required Array of numbers

Approver user IDs for this tier.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_addPolicyApprovalConfiguration!path=consensusRule&t=request)consensusRule
required string

Voting rule for this tier. `ALL` requires every approver to approve; `ONE_OF` requires any one approver.

Enum:"ALL""ONE_OF"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_addPolicyApprovalConfiguration!path=timeline&t=request)timeline
required number>= 1

Number of days allowed to complete this approval tier.

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

post/policies/{policyId}/approval-configuration

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

`{"name": "Legal Review","userIds": [12,34],"consensusRule": "ALL","timeline": 7}`

Response samples

*   201
*   400
*   401
*   403
*   412
*   500

1 more

application/json

`{"name": "Legal Review","tier": 1,"consensusRule": "ALL","timeline": 7,"approvers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_updatePolicyApprovalConfiguration)Update Review Group Configuration

Updates a single review group configuration by its tier position (1-based). Other tiers are not affected. Returns 404 if the tier does not exist.

🔒 Requires **Policies: Update Policy Review Group Configuration** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_updatePolicyApprovalConfiguration!in=path&path=policyId&t=request)policyId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_updatePolicyApprovalConfiguration!in=path&path=approvalConfigurationTier&t=request)approvalConfigurationTier
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_updatePolicyApprovalConfiguration!path=name&t=request)name
required string

Name of this approval tier.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_updatePolicyApprovalConfiguration!path=userIds&t=request)userIds
required Array of numbers

Approver user IDs for this tier.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_updatePolicyApprovalConfiguration!path=consensusRule&t=request)consensusRule
required string

Voting rule for this tier. `ALL` requires every approver to approve; `ONE_OF` requires any one approver.

Enum:"ALL""ONE_OF"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_updatePolicyApprovalConfiguration!path=timeline&t=request)timeline
required number>= 1

Number of days allowed to complete this approval tier.

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

put/policies/{policyId}/approval-configuration/{approvalConfigurationTier}

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

`{"name": "Legal Review","userIds": [12,34],"consensusRule": "ALL","timeline": 7}`

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

`{"name": "Legal Review","tier": 1,"consensusRule": "ALL","timeline": 7,"approvers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_removePolicyApprovalConfiguration)Remove Review Group Configuration

Removes a single review group configuration (tier) by its tier position (1-based). Remaining tiers are renumbered to remain contiguous. Returns 404 if the tier does not exist; returns 400 if removing it would leave zero tiers.

🔒 Requires **Policies: Remove Policy Review Group Configuration** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_removePolicyApprovalConfiguration!in=path&path=policyId&t=request)policyId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_removePolicyApprovalConfiguration!in=path&path=approvalConfigurationTier&t=request)approvalConfigurationTier
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

delete/policies/{policyId}/approval-configuration/{approvalConfigurationTier}

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
  const policyId = 'YOUR_policyId_PARAMETER';
  const approvalConfigurationTier = 'YOUR_approvalConfigurationTier_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/policies/${policyId}/approval-configuration/${approvalConfigurationTier}`,
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

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyActions)List Policy Actions

List available actions the authenticated user can perform on this Policy based on its current state.

🔒 Requires **Policies: Submit Policy for Approval, Policies: Override Approve Policy, Policies: Publish Policy, Policies: Discard Policy, Policies: Reset Policy to Template** permission.

Security

**bearer** or **oauth2** or **oauth2** or **oauth2** or **oauth2** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_listPolicyActions!in=path&path=policyId&t=request)policyId
required number

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

get/policies/{policyId}/actions

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
  const policyId = 'YOUR_policyId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/policies/${policyId}/actions`,
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

`{"data": [{"action": "SubmitForApproval","label": "Submit for Approval","description": "Submit this Policy Version for approval review","payloadSchema": [{"name": "reason","type": "string","required": true,"description": "Reason for requesting changes"}]}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_performPolicyAction)Perform Policy Action

Execute an action on a Policy (e.g., submit for approval, approve, request changes, publish, discard).

🔒 Requires **Policies: Submit Policy for Approval, Policies: Override Approve Policy, Policies: Publish Policy, Policies: Discard Policy, Policies: Reset Policy to Template** permission.

Security

**bearer** or **oauth2** or **oauth2** or **oauth2** or **oauth2** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_performPolicyAction!in=path&path=policyId&t=request)policyId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_performPolicyAction!path=action&t=request)action
required string

The action to perform on the Policy

Enum:"SubmitForApproval""Approve""RequestChanges""OverrideApprove""Publish""Discard""ResetToTemplate"
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_performPolicyAction!path=reason&t=request)reason string

Required when action is `RequestChanges`. The reason for requesting changes.
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_performPolicyAction!path=overrideReason&t=request)overrideReason string

Required when action is `OverrideApprove`. The reason for overriding approval.

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

post/policies/{policyId}/actions

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

`{"action": "SubmitForApproval","reason": "string","overrideReason": "string"}`

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

`{"success": true,"newStatus": "NEEDS_APPROVAL","message": "Policy submitted for approval"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_getPolicyVersion)Get Policy Version

Get a specific Policy Version. Returns policy version details (default) or policy version HTML content based on Accept header.

🔒 Requires **Policies: List Policies** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_getPolicyVersion!in=path&path=policyId&t=request)policyId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_getPolicyVersion!in=path&path=policyVersionId&t=request)policyVersionId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policies/#tag/Policies/operation/PoliciesPublicV2Controller_getPolicyVersion!in=query&path=expand[]&t=request)expand[]Array of strings (PolicyVersionExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"owner""weekTimeFrameSlas""p3MatrixSlas""gracePeriodSlas""downloadUrl""downloadPdfUrl""languages"

Responses

200
Successful

get/policies/{policyId}/policy-versions/{policyVersionId}

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
  const policyId = 'YOUR_policyId_PARAMETER';
  const policyVersionId = 'YOUR_policyVersionId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/policies/${policyId}/policy-versions/${policyVersionId}`,
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

application/json

`{"id": 1,"approvedAt": "2025-07-01T16:45:55.246Z","changeSummary": "string","changesExplanation": "string","createdAt": "2025-07-01T16:45:55.246Z","current": true,"description": "string","policyVersionStatus": "PUBLISHED","publishedAt": "2025-07-01T16:45:55.246Z","renewalDate": "2020-07-06","subVersion": 0,"type": "BUILDER","updatedAt": "2025-07-01T16:45:55.246Z","version": 1,"gracePeriodSlas": [{"id": 1,"label": "Grace Period Review","gracePeriod": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"p3MatrixSlas": [{"id": 1,"label": "P3 Matrix Review","timeFrame": "1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"policy": {"id": 1,"name": "Acceptable Use Policy","description": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assignedTo": "ALL","policyStatus": "ACTIVE","renewalDate": "2025-07-01T16:45:55.246Z"},"requiresAcknowledgment": true,"weekTimeFrameSlas": [{"id": 1,"timeFrame": "1","label": "Weekly Review","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"downloadUrl": "https://s3.amazonaws.com/bucket/file.pdf?signature=...","downloadPdfUrl": "https://s3.amazonaws.com/bucket/file.pdf?signature=...","languages": [{"id": 1,"language": "en-US","default": false,"hasFile": true,"hasPdf": true,"hasHtml": false,"originalFileName": "information-security-policy-es.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`
