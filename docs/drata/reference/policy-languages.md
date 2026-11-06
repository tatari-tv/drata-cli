Title: Policy Languages

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Policy Languages let an organization publish the same Policy Version in several languages. Settings endpoints expose the languages an organization has configured and which one is the default; Policy Language Versions expose the per-language variants of a Policy Version, including which content formats are available for each.

## [](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageSettings)List Policy Language Settings

List the organization's configured Policy languages and which one is the default. When the `POLICY_LANGUAGES_ENABLED` setting is off, returns `400` (`VALIDATION`) asking you to enable Policy languages in your organization settings.

🔒 Requires **Policies: List Policies** permission.

Security

**bearer** or **oauth2**

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

get/policy-language-settings

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
    `https://public-api.drata.com/public/v2/policy-language-settings`,
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

`{"data": [{"language": "en-US","default": true}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageVersions)List Policy Language Versions

List Policy Version language variants. Narrow to a single Policy Version with `policyVersionId`, or to every Version of a Policy with `policyId`. Scope to the parent Version status with `statuses[]` — `statuses[]=PUBLISHED` excludes variants attached to draft Versions. One row per language variant, so a Policy Version with several languages does not multiply rows in other collections. This is the endpoint to use when you need languages across multiple Policy Versions — the policy-version list endpoint does not expand them. Downloads are not available here: use `GET /policy-language-versions/{policyLanguageVersionId}` with `expand[]=downloadUrl` or `expand[]=downloadPdfUrl`. This is a content read, so the `POLICY_LANGUAGES_ENABLED` setting does not gate it — existing variants stay listable.

🔒 Requires **Policies: List Policies** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageVersions!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageVersions!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageVersions!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageVersions!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageVersions!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageVersions!in=query&path=policyVersionId&t=request)policyVersionId number

Filter to the language variants belonging to a single Policy Version. Omit to list every language variant in the organization.

Example:policyVersionId=1
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageVersions!in=query&path=policyId&t=request)policyId number

Filter to the language variants across every Version of a single Policy. Use this to retrieve languages for a whole Policy in one call rather than one request per Policy Version.

Example:policyId=1
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageVersions!in=query&path=statuses[]&t=request)statuses[]Array of strings (PolicyVersionStatusEnum)

Filter to the language variants whose parent Policy Version has one of these statuses. Use `statuses[]=PUBLISHED` to scope to published Versions only — the collection is otherwise unfiltered by status and includes variants attached to draft Versions.

Items Enum:"NEEDS_APPROVAL""APPROVED""PUBLISHED""DRAFT""DISCARDED"
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyLanguageVersions!in=query&path=expand[]&t=request)expand[]Array of strings (PolicyLanguageVersionListExpandEnum)

List of subcollections and sub-objects to expand. Downloads (`downloadUrl`, `downloadPdfUrl`) are available only on `GET /policy-language-versions/{policyLanguageVersionId}`.

Items Enum:"policyVersion""policy"

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

get/policy-language-versions

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
    `https://public-api.drata.com/public/v2/policy-language-versions`,
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

`{"data": [{"id": 1,"language": "en-US","default": false,"hasFile": true,"hasPdf": true,"hasHtml": false,"originalFileName": "information-security-policy-es.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","policyVersion": {"id": 1,"version": 1,"current": true,"type": "UPLOADED","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","renewalDate": "2025-07-01T16:45:55.246Z","subVersion": 1,"description": "string","approvedAt": "2025-07-01T16:45:55.246Z","changesExplanation": "string","publishedAt": "2025-07-01T16:45:55.246Z","requiresAcknowledgment": true},"policy": {"id": 1,"name": "Acceptable Use Policy","description": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assignedTo": "ALL","policyStatus": "ACTIVE","renewalDate": "2025-07-01T16:45:55.246Z"},}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyVersionLanguages)List Policy Version Languages

List the language variants of a single Policy Version. The version-scoped view of `/policy-language-versions` — the rows are the same, addressed through the Policy and Policy Version that own them. Use this when you already hold both identifiers; use `/policy-language-versions` when you need language rows across several Versions or Policies in one call, or when you want the parent relations expanded. Narrow by the parent Version status with `statuses[]`. Returns `404` when `policyVersionId` is not a Version of `policyId`, rather than an empty list that would be indistinguishable from a Version with no variants. This is a content read, so the `POLICY_LANGUAGES_ENABLED` setting does not gate it.

🔒 Requires **Policies: List Policies** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyVersionLanguages!in=path&path=policyId&t=request)policyId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyVersionLanguages!in=path&path=policyVersionId&t=request)policyVersionId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyVersionLanguages!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyVersionLanguages!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyVersionLanguages!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyVersionLanguages!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyVersionLanguages!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_listPolicyVersionLanguages!in=query&path=statuses[]&t=request)statuses[]Array of strings (PolicyVersionStatusEnum)

Filter to the language variants whose parent Policy Version has one of these statuses. Because the path already names a single Policy Version, this acts as an assertion about that Version: `statuses[]=PUBLISHED` returns its languages when it is published and an empty list when it is not. Omit to return the languages regardless of the Version status.

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

get/policies/{policyId}/policy-versions/{policyVersionId}/languages

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
    `https://public-api.drata.com/public/v2/policies/${policyId}/policy-versions/${policyVersionId}/languages`,
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

`{"data": [{"id": 1,"language": "en-US","default": false,"hasFile": true,"hasPdf": true,"hasHtml": false,"originalFileName": "information-security-policy-es.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_getPolicyLanguageVersion)Get Policy Language Version

Get a single Policy Version language variant by its identifier, as returned in the `id` of a `/policy-language-versions` row. Returns the variant's metadata (default), or its authored HTML content when `Accept: text/html` is sent.

Downloads: `expand[]=downloadUrl` returns a short-lived signed URL for the uploaded document in its original format, and `expand[]=downloadPdfUrl` one for the PDF rendering. Either returns `null` — not an error — when this language has no content of that kind, matching `hasFile` / `hasPdf` on the same response. Requesting either is recorded as a Policy download against the selected language; reading the HTML is not. These two expands are available only here, never on the `/policy-language-versions` collection.

Returns `404` when the identifier is unknown, when the variant belongs to another organization, or when the variant has been deleted — those three stay collapsed so an integrator cannot probe another tenant's id space. `Accept: text/html` also returns `404` when this language has no authored HTML: the body is the entire response, so an empty one could be mistaken for content. Note this is stricter than `GET /policies/{policyId}/policy-versions/{policyVersionId}`, which answers `200` with an empty body for a Policy Version with no HTML.

Variants attached to draft Policy Versions are readable here — HTML and metadata reflect the current draft content. For a BUILDER-type variant, though, the PDF is not rendered until the version is PUBLISHED: `hasPdf` is `false` and `expand[]=downloadPdfUrl` returns `null` pre-publish, per the same not-an-error contract above. Scope to published content with `statuses[]=PUBLISHED` on the collection.

🔒 Requires **Policies: List Policies** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_getPolicyLanguageVersion!in=path&path=policyLanguageVersionId&t=request)policyLanguageVersionId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Policy-Languages/#tag/Policy-Languages/operation/PolicyLanguagesPublicV2Controller_getPolicyLanguageVersion!in=query&path=expand[]&t=request)expand[]Array of strings (PolicyLanguageVersionExpandEnum)

List of subcollections and sub-objects to expand. `downloadUrl` and `downloadPdfUrl` return short-lived signed URLs for this language variant and are available only on this route, not on the `/policy-language-versions` collection; requesting either is an audited download.

Items Enum:"policyVersion""policy""downloadUrl""downloadPdfUrl"

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

406
None of the media types in the Accept header can be produced

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/policy-language-versions/{policyLanguageVersionId}

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
  const policyLanguageVersionId = 'YOUR_policyLanguageVersionId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/policy-language-versions/${policyLanguageVersionId}`,
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
*   406
*   412
*   500

3 more

application/json

`{"id": 1,"language": "en-US","default": false,"hasFile": true,"hasPdf": true,"hasHtml": false,"originalFileName": "information-security-policy-es.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","policyVersion": {"id": 1,"version": 1,"current": true,"type": "UPLOADED","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","renewalDate": "2025-07-01T16:45:55.246Z","subVersion": 1,"description": "string","approvedAt": "2025-07-01T16:45:55.246Z","changesExplanation": "string","publishedAt": "2025-07-01T16:45:55.246Z","requiresAcknowledgment": true},"policy": {"id": 1,"name": "Acceptable Use Policy","description": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assignedTo": "ALL","policyStatus": "ACTIVE","renewalDate": "2025-07-01T16:45:55.246Z"},"downloadUrl": "https://example.com/signed-url","downloadPdfUrl": "https://example.com/signed-url"}`
