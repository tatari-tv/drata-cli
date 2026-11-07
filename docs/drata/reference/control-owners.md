Title: Control Owners

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Control Owners are the Users responsible for Controls. They ensure the right evidence is associated, that any automated tests are passing, and help prepare for an audit.

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl)List Control Owners

Find Control Owners matching the provided filters.

🔒 Requires **Controls: Get Control** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl!in=path&path=controlId&t=request)controlId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl!in=query&path=frameworkSlug&t=request)frameworkSlug string

Filter data by Users associated with this Control Requirement Framework slug

Example:frameworkSlug=soc2
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl!in=query&path=includeUserIds[]&t=request)includeUserIds[]Array of numbers or null<= 300 items

User IDs of the Control Owners to return

Example:includeUserIds[]=1&includeUserIds[]=2&includeUserIds[]=3
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_getControlOwnersForAControl!in=query&path=excludeUserIds&t=request)excludeUserIds Array of numbers

User IDs of the Control Owners to exclude

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

get/workspaces/{workspaceId}/controls/{controlId}/owners

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
  const controlId = 'YOUR_controlId_PARAMETER';
  const workspaceId = 'YOUR_workspaceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls/${controlId}/owners`,
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

`{"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_createControlOwner)Add Control Owner

🔒 Requires **Controls: Manage Control Owners** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_createControlOwner!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_createControlOwner!in=path&path=controlId&t=request)controlId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_createControlOwner!path=ownerId&t=request)ownerId
required number

Owner ID

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

post/workspaces/{workspaceId}/controls/{controlId}/owners

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

`{"ownerId": 56}`

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

`{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_modifyControlOwners)Modify Control Owners

Modify all Owners for a given Control

🔒 Requires **Controls: Manage Control Owners** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_modifyControlOwners!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_modifyControlOwners!in=path&path=controlId&t=request)controlId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_modifyControlOwners!path=ownerUserIds&t=request)ownerUserIds
required Array of numbers

User IDs of the Control's owners

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

put/workspaces/{workspaceId}/controls/{controlId}/owners

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

`{"ownerUserIds": [1,2,3]}`

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

`{"ownerIds": [0]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_deleteControlOwner)Delete Control Owner

🔒 Requires **Controls: Manage Control Owners** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_deleteControlOwner!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_deleteControlOwner!in=path&path=controlId&t=request)controlId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Owners/#tag/Control-Owners/operation/ControlOwnersPublicV2Controller_deleteControlOwner!in=path&path=ownerId&t=request)ownerId
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

delete/workspaces/{workspaceId}/controls/{controlId}/owners/{ownerId}

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
  const controlId = 'YOUR_controlId_PARAMETER';
  const ownerId = 'YOUR_ownerId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls/${controlId}/owners/${ownerId}`,
    {
      method: 'DELETE',
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

`{"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`
