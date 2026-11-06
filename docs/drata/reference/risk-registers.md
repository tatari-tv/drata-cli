Title: Risk Registers

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Risk Registers are a collection of Risks. They are used to organize and manage Risks.

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_listRiskRegisters)List Risk Registers

Retrieve all Risk Registers associated with the account.

🔒 Requires **Risk Management: List Risk Registers** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_listRiskRegisters!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_listRiskRegisters!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_listRiskRegisters!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_listRiskRegisters!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_listRiskRegisters!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_listRiskRegisters!in=query&path=name&t=request)name string<= 191 characters

Search query for Register name

Example:name=security
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_listRiskRegisters!in=query&path=ownerIds[]&t=request)ownerIds[]Array of numbers<= 300 items

User IDs of the Register Owners to filter by

Example:ownerIds[]=1&ownerIds[]=2&ownerIds[]=3
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_listRiskRegisters!in=query&path=workspaceIds[]&t=request)workspaceIds[]Array of numbers<= 300 items

Workspace IDs to filter Risk Registers by

Example:workspaceIds[]=1&workspaceIds[]=2
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_listRiskRegisters!in=query&path=expand[]&t=request)expand[]Array of strings (RiskRegisterExpandEnum)

List of subcollections and sub-objects to expand

Items Value:"workspaces"

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

get/risk-registers

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
    `https://public-api.drata.com/public/v2/risk-registers`,
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

`{"data": [ ],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_createRiskRegister)Create Risk Register

Create Risk Register.

🔒 Requires **Risk Management: Create Risk Register** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_createRiskRegister!path=name&t=request)name
required string<= 191 characters

Unique name for the Risk Register
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_createRiskRegister!path=description&t=request)description string or null<= 60000 characters

Description for the Risk Register
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_createRiskRegister!path=ownerIds&t=request)ownerIds Array of numbers

List of User ids to be assigned to the Risk Register as owners
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_createRiskRegister!path=workspaceIds&t=request)workspaceIds Array of numbers

List of Workspace IDs to associate with the Risk Register

Responses

201
Created

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

422
Unprocessable Entity

500
Internal server error

post/risk-registers

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

`{"name": "Default Register","description": "Register for all Risks","ownerIds": [1,2,3],"workspaceIds": [1,2]}`

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

`{"id": 1,"name": "Default Register","description": "Default Register for all Risks","owners": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"workspaces": [{"id": 1,"name": "Drata"}],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_getRiskRegister)Get Risk Register

Get Risk Register details

🔒 Requires **Risk Management: Get Risk Register** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_getRiskRegister!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1

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

get/risk-registers/{riskRegisterId}

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
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}`,
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

`{"id": 1,"name": "Default Register","description": "Default Register for all Risks","owners": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"workspaces": [{"id": 1,"name": "Drata"}],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_updateRiskRegister)Update Risk Register

Update the Risk Register details

🔒 Requires **Risk Management: Update Risk Register** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_updateRiskRegister!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_updateRiskRegister!path=description&t=request)description string or null<= 60000 characters

Description for the Risk Register
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_updateRiskRegister!path=ownerIds&t=request)ownerIds Array of numbers

List of User ids to be assigned to the Risk Register as owners
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_updateRiskRegister!path=workspaceIds&t=request)workspaceIds Array of numbers

List of Workspace IDs to associate with the Risk Register
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_updateRiskRegister!path=name&t=request)name string<= 191 characters

Unique name for the Risk Register

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

422
Unprocessable Entity

500
Internal server error

put/risk-registers/{riskRegisterId}

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

`{"description": "Register for all Risks","ownerIds": [1,2,3],"workspaceIds": [1,2],"name": "Default Register"}`

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

`{"id": 1,"name": "Default Register","description": "Default Register for all Risks","owners": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"workspaces": [{"id": 1,"name": "Drata"}],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_deleteRiskRegister)Delete Risk Register

Deletes a Risk Register

🔒 Requires **Risk Management: Delete Risk Register** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Registers/#tag/Risk-Registers/operation/RiskRegisterPublicV2Controller_deleteRiskRegister!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1

Responses

204
No Content

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

delete/risk-registers/{riskRegisterId}

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
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}`,
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
*   402
*   403
*   404
*   412
*   500

2 more

application/json

`{"name": "string","statusCode": 0,"message": "string","code": 0}`
