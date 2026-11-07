Title: Risk Library

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
The Risk Library is a collection of Risks that can be copied into a Risk Register. The [help docs](https://help.drata.com/en/articles/13371089-drata-s-risk-library-new-experience) have more information.

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary)List Risk Libraries

Find Library Risks by search terms and filters.

🔒 Requires **Risk Management: List Risks** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary!in=query&path=riskId&t=request)riskId string

Filter by Risk ID. Searches for risks whose ID starts with the provided value (e.g., "AA" matches "AA-01", "AA-02").

Example:riskId=AA-01
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary!in=query&path=title&t=request)title string

Filter by Title. Searches for risks whose title contains the provided text anywhere in the field (case-insensitive).

Example:title=Activity Log Evaluation
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary!in=query&path=description&t=request)description string

Filter by Description. Searches for risks whose description contains the provided text anywhere in the field (case-insensitive).

Example:description=Unauthorized System Access
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_listRiskLibrary!in=query&path=expand[]&t=request)expand[]Array of strings (RiskLibraryExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"controls""categories"

Responses

200

400
Malformed data and/or validation errors

401
Invalid Authorization

402
You must upgrade your plan to use this feature

403
You are not allowed to perform this action

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/risk-registers/{riskRegisterId}/risk-library

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
  const riskRegisterId = 'YOUR_riskRegisterId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risk-library`,
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
*   402
*   403
*   412
*   500

2 more

application/json

`{"data": [{"id": 1,"riskId": "AC-04","title": "Password Management - Password Cracking","description": "An attacker attempts to gain access to organizational information by guessing passwords.","controls": [{"id": 10,"code": "AC-2","name": "Account Management","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"categories": [{"id": 1,"name": "Access Control","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}],"pagination": {"cursor": "string"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_getRiskLibraryItem)Get Risk Library Item by ID.

Get Risk Library

🔒 Requires **Risk Management: Get Risk** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_getRiskLibraryItem!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk Library ID or string Risk Library ID prefixed with `riskId:`

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_getRiskLibraryItem!in=query&path=expand[]&t=request)expand[]Array of strings (RiskLibraryExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"controls""categories"

Responses

200
Successful

400
Malformed data and/or validation errors

401
Invalid Authorization

402
You must upgrade your plan to use this feature

403
You are not allowed to perform this action

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/risk-registers/{riskRegisterId}/risk-library/{riskLibraryId}

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
  const riskRegisterId = 'YOUR_riskRegisterId_PARAMETER';
  const riskLibraryId = 'YOUR_riskLibraryId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risk-library/${riskLibraryId}`,
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
*   402
*   403
*   404
*   412
*   500

3 more

application/json

`{"id": 1,"riskId": "AC-04","title": "Password Management - Password Cracking","description": "An attacker attempts to gain access to organizational information by guessing passwords.","controls": [{"id": 10,"code": "AC-2","name": "Account Management","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"categories": [{"id": 1,"name": "Access Control","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_copyRiskLibrary)Copies Risk Library Items to a Risk Register.

Copy Risk Library Items to Register

🔒 Requires **Risk Management: Create Risk** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_copyRiskLibrary!path=bulkActionType&t=request)bulkActionType
required string

The copy action to perform on from Risk Library to the Risk Register.

Enum:"COPY_BY_IDS""COPY_BY_GROUPS"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_copyRiskLibrary!path=riskIds&t=request)riskIds
required Array of strings

An array of the Risk IDs that you want to copy from Risk Library to the Risk Register.
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_copyRiskLibrary!path=riskGroups&t=request)riskGroups
required Array of strings

An array of the Risk Groups that you want to copy from Risk Library to the Risk Register.

Items Enum:"WIZARD_CORE""AI_DEVELOPMENT""AI_USE""PHYSICAL_SITE""CLOUD_ENVIRONMENT""UNSECURE_DEVICES""DEVICE_DELIVERY""SOFTWARE_DEVELOPMENT""REGULATORY_REQUIREMENTS"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Library/#tag/Risk-Library/operation/RiskLibraryPublicV2Controller_copyRiskLibrary!path=registerId&t=request)registerId
required number

The Register ID to copy the Risks to.

Responses

201

400
Malformed data and/or validation errors

401
Invalid Authorization

402
You must upgrade your plan to use this feature

403
You are not allowed to perform this action

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/risk-library/action-copy

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

`{"bulkActionType": "COPY_BY_IDS","riskIds": ["AA-01","AA-02","AA-03"],"riskGroups": ["CLOUD_ENVIRONMENT"],"registerId": 1}`

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

`{"data": [{"id": 100,"riskId": "AC-04","title": "Password Management - Password Cracking","description": "An attacker attempts to gain access to organizational information by guessing passwords.","registerId": 5}]}`
