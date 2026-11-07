Title: Frameworks

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Frameworks/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Frameworks are collections of controls that are used to assess compliance with specific standards or regulations. The [help docs](https://help.drata.com/en/articles/5329593-frameworks) have more information.

## [](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks)List Frameworks

Find Frameworks matching the provided filters.

🔒 Requires **Frameworks: List Frameworks** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks!in=query&path=name&t=request)name string

Filter Frameworks by name (partial match)

Example:name=SOC 2
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks!in=query&path=excludeIds&t=request)excludeIds Array of numbers

Array of Framework IDs to exclude from results

Example:excludeIds=1&excludeIds=2&excludeIds=3
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks!in=query&path=isReady&t=request)isReady boolean

Filter Frameworks by ready state

Example:isReady=true
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworks!in=query&path=isEnabled&t=request)isEnabled boolean

Filter Frameworks by enabled state

Example:isEnabled=true

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

get/workspaces/{workspaceId}/frameworks

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/frameworks`,
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

`{"data": [{"id": 123,"name": "SOC 2","description": "Report on controls at a Service Organization","slug": "soc-2","tag": "SOC_2","pill": "SOC 2","numInScopeControls": 42,"numInScopeRequirements": 42,"numReadyInScopeRequirements": 35,"isReady": true,"isEnabled": true,"color": "#174880","bgColor": "#E8F4FF","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_createFramework)Create Custom Framework

Create a new custom compliance Framework.

🔒 Requires **workspaces-post-framework** permission.

💎 Requires your account have the **Custom Frameworks** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_createFramework!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_createFramework!path=name&t=request)name
required string<= 191 characters

Framework display name
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_createFramework!path=shortName&t=request)shortName
required string<= 16 characters

Short label used as a pill/badge in the UI
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_createFramework!path=description&t=request)description
required string<= 30000 characters

Framework description

Responses

201
Created

400
Malformed data and/or validation errors

401
Invalid Authorization

402
The required account entitlement is not enabled.

403
You are not allowed to perform this action

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/workspaces/{workspaceId}/frameworks

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

`{"name": "My Custom Framework 2024","shortName": "MCF-24","description": "A custom compliance framework for internal security controls."}`

Response samples

*   201
*   400
*   401
*   402
*   403
*   412
*   500

2 more

application/json

`{"id": 123,"name": "SOC 2","description": "Report on controls at a Service Organization","slug": "soc-2","tag": "SOC_2","pill": "SOC 2","numInScopeControls": 42,"numInScopeRequirements": 42,"numReadyInScopeRequirements": 35,"isReady": true,"isEnabled": true,"color": "#174880","bgColor": "#E8F4FF","activeLogo": "https://cdn.acme.com/frameworks/soc2-active.svg","inactiveLogo": "https://cdn.acme.com/frameworks/soc2-inactive.svg","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy)List Framework Requirements

Use `GET /workspaces/{workspaceId}/frameworks/{frameworkId}/requirements` instead.

Find Framework Requirements matching the provided filters. Use `expand[]=controls` to include a limited subcollection of controls for each requirement. Use `expand[]=customFields` to include custom field values for each requirement.

🔒 Requires **Frameworks: List Framework Requirements** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=expand[]&t=request)expand[]Array of strings (FrameworkRequirementsExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"controls""customFields""framework"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=isReady&t=request)isReady boolean

Filter Requirements that are or are not ready

Example:isReady=false
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=isInScope&t=request)isInScope boolean

Filter Requirements that are or are not in scope (not archived)

Example:isInScope=true
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=frameworkTag&t=request)frameworkTag Array of strings (FrameworkTagEnum)

Filter data by Requirements associated with these framework tags

Items Enum:"NONE""SOC_2""ISO27001""CCPA""GDPR""HIPAA""PCI""SCF""NIST80053""NISTCSF""CMMC""NIST800171""MSSSPA""FFIEC""ISO27701""COBIT""SOX_ITGC""ISO270012022""CCM""CYBER_ESSENTIALS""ISO270172015""ISO270182019""FEDRAMP""NISTAI""PCI4""NISTCSF2""NIS2""DORA""ISO420012023""DRATA_ESSENTIALS""NIST800171R3""CIS8""CYBER_ESSENTIALS_32""FEDRAMP20X""HITRUST""MSSSPA11""ESSENTIAL_EIGHT""NYDFS""TISAX""ISO270182025""CCPA2026""ISO277012025""CPS230""CYFUN""AIUC_1""EU_CRA""AIUC_1_26Q3""AIUC_1_26Q4""CUSTOM""ALL"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=excludeIds&t=request)excludeIds Array of numbers

Exclude Requirements by array of IDs

Example:
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_getFrameworkRequirementsLegacy!in=query&path=frameworkSlug&t=request)frameworkSlug string

Filter data by Requirements associated with these framework slugs. This parameter is intended to be used only for custom frameworks

Example:frameworkSlug=slug

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

get/workspaces/{workspaceId}/framework-requirements

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/framework-requirements`,
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

`{"data": [{"createdAt": "2025-07-01T16:45:55.246Z","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkId": 13,"frameworkName": "SOC 2","frameworkPill": "SOC 2","frameworkSlug": "SOC 2","frameworkTag": "SOC_2","id": 58,"name": "PI1.2","updatedAt": "2025-07-01T16:45:55.246Z","additionalInfo": "The entity demonstrates a commitment to integrity.","additionalInfo2": "The entity demonstrates a commitment to ethical values.","additionalInfo3": "The entity demonstrates a commitment to enforcing policies.","archivedAt": "2025-07-01T16:45:55.246Z","externalId": "SOC 2","longDescription": "The entity demonstrates a commitment to integrity and ethical values.","rationale": "This requirement is not needed.","category": "General Rules","controls": {"data": [{"id": 1,"code": "DCF-1001","isReady": true,"controlNumber": 1,"archivedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 47},"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirementLegacy)Update Framework Requirement

Use `PUT /workspaces/{workspaceId}/frameworks/{frameworkId}/requirements/{requirementId}` instead. The replacement endpoint supports both Custom Field and structural-field updates.

🔒 Requires **Frameworks: Update Framework Requirement** permission.

💎 Requires your account have the **Custom Fields and Formulas** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirementLegacy!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirementLegacy!in=path&path=frameworkRequirementId&t=request)frameworkRequirementId
required number

##### Request Body schema: application/json
required

Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Requirement.

💎 Requires your account have the Custom Fields and Formulas feature. Contact your Account Manager for help upgrading.

Responses

200
Successful

400
Malformed data and/or validation errors

401
Invalid Authorization

402
The required account entitlement is not enabled.

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

put/workspaces/{workspaceId}/framework-requirements/{frameworkRequirementId}

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

`{"customFields": [{"id": 1,"name": "Compliance Status","value": "Security & IT"}]}`

Response samples

*   200
*   400
*   401
*   402
*   403
*   404
*   412
*   500

3 more

application/json

`{"createdAt": "2025-07-01T16:45:55.246Z","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkId": 13,"frameworkName": "SOC 2","frameworkPill": "SOC 2","frameworkSlug": "SOC 2","frameworkTag": "SOC_2","id": 58,"name": "PI1.2","updatedAt": "2025-07-01T16:45:55.246Z","additionalInfo": "The entity demonstrates a commitment to integrity.","additionalInfo2": "The entity demonstrates a commitment to ethical values.","additionalInfo3": "The entity demonstrates a commitment to enforcing policies.","archivedAt": "2025-07-01T16:45:55.246Z","externalId": "SOC 2","longDescription": "The entity demonstrates a commitment to integrity and ethical values.","rationale": "This requirement is not needed.","category": "General Rules","controls": {"data": [{"id": 1,"code": "DCF-1001","isReady": true,"controlNumber": 1,"archivedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 47},"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements)List Framework Requirements

List Requirements scoped to the specified Framework.

🔒 Requires **Frameworks: List Framework Requirements** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=path&path=frameworkId&t=request)frameworkId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=query&path=expand[]&t=request)expand[]Array of strings (FrameworkRequirementsExpandEnum)

List of subcollections and sub-objects to expand. `controls` includes a limited subcollection of Controls mapped to each Requirement. `customFields` includes Custom Field values for each Requirement. `framework` includes a compact Framework object on each Requirement.

Items Enum:"controls""customFields""framework"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=query&path=isReady&t=request)isReady boolean

Filter Requirements that are or are not ready

Example:isReady=false
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=query&path=isInScope&t=request)isInScope boolean

Filter Requirements that are or are not in scope (not archived)

Example:isInScope=true
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirements!in=query&path=excludeIds&t=request)excludeIds Array of numbers

Exclude Requirements by array of IDs

Example:

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

get/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements

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
  const frameworkId = 'YOUR_frameworkId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/frameworks/${frameworkId}/requirements`,
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

`{"data": [{"id": 58,"code": "A.5.1.1","name": "Policies for information security","description": "A set of policies for information security shall be defined, approved, published...","category": "General Rules","additionalInfo": "Refer to ISO 27001:2022 Annex A.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","framework": {"id": 13,"name": "NIST AI RMF","description": "The NIST AI Risk Management Framework (AI RMF) is a set of suggestions and guidelines for incorporating trustworthiness considerations in the design, development, deployment, and use of AI systems.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","enabledAt": "2025-07-01T16:45:55.246Z","shortName": "SOC 2","tag": "SOC_2","deletedAt": "2025-07-01T16:45:55.246Z"},"controls": [{"id": 1,"code": "DCF-1001","isReady": true,"controlNumber": 1,"archivedAt": "2025-07-01T16:45:55.246Z"}],"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_createFrameworkRequirements)Create Framework Requirements

Create or update one or more Requirements on a custom Framework. Duplicate codes within a batch and unknown Control IDs cause the entire request to fail.

🔒 Requires **workspaces-post-framework-requirements** permission.

💎 Requires your account have the **Custom Frameworks** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_createFrameworkRequirements!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_createFrameworkRequirements!in=path&path=frameworkId&t=request)frameworkId
required number

##### Request Body schema: application/json
required

required Array of objects (CreateFrameworkRequirementRequestPublicV2Dto)

One or more Requirements to create on the Framework. Duplicate codes within a batch and unknown Control IDs cause the entire request to fail.

Responses

201
Created

400
Malformed data and/or validation errors

401
Invalid Authorization

402
The required account entitlement is not enabled.

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

422
Unprocessable Entity

500
Internal server error

post/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements

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

`{"data": [{"code": "A.5.1.1","name": "Policies for information security","description": "A set of policies for information security shall be defined, approved, published...","category": "General Rules","additionalInfo": "Refer to ISO 27001:2022 Annex A.","controlIds": [42,57]}]}`

Response samples

*   201
*   400
*   401
*   402
*   403
*   404
*   412
*   422
*   500

4 more

application/json

`{"data": [{"id": 58,"code": "A.5.1.1","name": "Policies for information security","description": "A set of policies for information security shall be defined, approved, published...","category": "General Rules","additionalInfo": "Refer to ISO 27001:2022 Annex A.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","framework": {"id": 13,"name": "NIST AI RMF","description": "The NIST AI Risk Management Framework (AI RMF) is a set of suggestions and guidelines for incorporating trustworthiness considerations in the design, development, deployment, and use of AI systems.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","enabledAt": "2025-07-01T16:45:55.246Z","shortName": "SOC 2","tag": "SOC_2","deletedAt": "2025-07-01T16:45:55.246Z"},"controls": [{"id": 1,"code": "DCF-1001","isReady": true,"controlNumber": 1,"archivedAt": "2025-07-01T16:45:55.246Z"}],"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirementControls)List Controls for a Framework Requirement

Retrieve a cursor-paginated list of all Controls mapped to the specified Framework Requirement. This is the full, paginated companion to the capped `expand[]=controls` subcollection returned by the Requirements list endpoints. Note: results are ordered by Control ID, which differs from the `expand[]=controls` preview ordering (isReady, then controlNumber).

🔒 Requires **Frameworks: List Framework Requirements** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirementControls!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirementControls!in=path&path=frameworkId&t=request)frameworkId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirementControls!in=path&path=requirementId&t=request)requirementId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirementControls!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirementControls!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirementControls!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirementControls!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_listFrameworkRequirementControls!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false

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

get/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements/{requirementId}/controls

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
  const frameworkId = 'YOUR_frameworkId_PARAMETER';
  const requirementId = 'YOUR_requirementId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/frameworks/${frameworkId}/requirements/${requirementId}/controls`,
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

`{"data": [{"id": 1,"code": "DCF-1001","isReady": true,"controlNumber": 1,"archivedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirement)Update Framework Requirement

Update a custom Framework Requirement, including core fields, control mappings, and Custom Field values.

🔒 Requires **Frameworks: Update Framework Requirement** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirement!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirement!in=path&path=frameworkId&t=request)frameworkId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirement!in=path&path=requirementId&t=request)requirementId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirement!path=code&t=request)code string<= 191 characters

Unique code for this Requirement within the Framework
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirement!path=name&t=request)name string<= 30000 characters

Human-readable name for this Requirement
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirement!path=description&t=request)description string<= 30000 characters

Detailed description of the Requirement
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirement!path=category&t=request)category string<= 191 characters

Category label for grouping Requirements within the Framework
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirement!path=additionalInfo&t=request)additionalInfo string<= 30000 characters

Additional information for this Requirement
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFrameworkRequirement!path=controlIds&t=request)controlIds Array of numbers

Control IDs to map to this Requirement. Providing an empty array clears all mappings. Omitting the field leaves existing mappings unchanged. All IDs must exist within the workspace.
Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Requirement.

💎 Requires your account have the Custom Fields and Formulas feature. Contact your Account Manager for help upgrading.

Responses

200
Successful

204
No Content

400
Malformed data and/or validation errors

401
Invalid Authorization

402
The required account entitlement is not enabled.

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

422
Unprocessable Entity

500
Internal server error

put/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements/{requirementId}

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

`{"code": "A.5.1.1","name": "Policies for information security","description": "A set of policies for information security shall be defined, approved, published...","category": "General Rules","additionalInfo": "Refer to ISO 27001:2022 Annex A.","controlIds": [42,57],"customFields": [{"id": 1,"name": "Compliance Status","value": "Security & IT"}]}`

Response samples

*   200
*   400
*   401
*   402
*   403
*   404
*   412
*   422
*   500

4 more

application/json

`{"id": 58,"code": "A.5.1.1","name": "Policies for information security","description": "A set of policies for information security shall be defined, approved, published...","category": "General Rules","additionalInfo": "Refer to ISO 27001:2022 Annex A.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","framework": {"id": 13,"name": "NIST AI RMF","description": "The NIST AI Risk Management Framework (AI RMF) is a set of suggestions and guidelines for incorporating trustworthiness considerations in the design, development, deployment, and use of AI systems.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","enabledAt": "2025-07-01T16:45:55.246Z","shortName": "SOC 2","tag": "SOC_2","deletedAt": "2025-07-01T16:45:55.246Z"},"controls": [{"id": 1,"code": "DCF-1001","isReady": true,"controlNumber": 1,"archivedAt": "2025-07-01T16:45:55.246Z"}],"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFramework)Update Custom Framework

Update an existing custom compliance Framework.

🔒 Requires **workspaces-put-framework** permission.

💎 Requires your account have the **Custom Frameworks** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFramework!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFramework!in=path&path=frameworkId&t=request)frameworkId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFramework!path=name&t=request)name string<= 191 characters

The title of the Framework
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFramework!path=shortName&t=request)shortName string<= 16 characters

Short label used as a pill/badge in the UI
[](https://developers.drata.com/openapi/reference/v2/tag/Frameworks/#tag/Frameworks/operation/FrameworksPublicV2Controller_updateFramework!path=description&t=request)description string<= 30000 characters

Framework description

Responses

200
Successful

400
Malformed data and/or validation errors

401
Invalid Authorization

402
The required account entitlement is not enabled.

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

422
Unprocessable Entity

500
Internal server error

put/workspaces/{workspaceId}/frameworks/{frameworkId}

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

`{"name": "My Updated Framework 2024","shortName": "MUF-24","description": "Updated description for the framework."}`

Response samples

*   200
*   400
*   401
*   402
*   403
*   404
*   412
*   422
*   500

4 more

application/json

`{"id": 123,"name": "SOC 2","description": "Report on controls at a Service Organization","slug": "soc-2","tag": "SOC_2","pill": "SOC 2","numInScopeControls": 42,"numInScopeRequirements": 42,"numReadyInScopeRequirements": 35,"isReady": true,"isEnabled": true,"color": "#174880","bgColor": "#E8F4FF","activeLogo": "https://cdn.acme.com/frameworks/soc2-active.svg","inactiveLogo": "https://cdn.acme.com/frameworks/soc2-inactive.svg","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}`
