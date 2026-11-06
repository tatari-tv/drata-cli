Title: Risk Notes

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Risk Notes allow you to provide additional information about Risks.

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNotes)List Risk Notes

Find Risk Notes matching the provided filters.

🔒 Requires **Risk Management: Get Risk Note** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNotes!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNotes!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNotes!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNotes!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNotes!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNotes!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNotes!in=query&path=expand[]&t=request)expand[]Array of strings (RiskNotesExpandEnum)

List of subcollections and sub-objects to expand

Items Value:"owner"

Responses

200

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

get/risk-registers/{riskRegisterId}/risks/{riskId}/notes

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
  const riskId = 'YOUR_riskId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}/notes`,
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

`{"data": [{"id": 123,"comment": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_createRiskNote)Create Risk Note

Create a new Note for a specific Risk.

🔒 Requires **Risk Management: Create Risk Note** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_createRiskNote!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID.

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_createRiskNote!path=comment&t=request)comment
required string<= 768 characters

The comment content for the note

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

post/risk-registers/{riskRegisterId}/risks/{riskId}/notes

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

`{"comment": "This is a note about the risk assessment findings."}`

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

`{"id": 123,"comment": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNote)Get Risk Note

Get a Note associated with a given Risk.

🔒 Requires **Risk Management: Get Risk Note** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNote!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID.

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNote!in=path&path=noteId&t=request)noteId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_getRiskNote!in=query&path=expand[]&t=request)expand[]Array of strings (RiskNotesExpandEnum)

List of subcollections and sub-objects to expand

Items Value:"owner"

Responses

200

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

get/risk-registers/{riskRegisterId}/risks/{riskId}/notes/{noteId}

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
  const riskId = 'YOUR_riskId_PARAMETER';
  const noteId = 'YOUR_noteId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}/notes/${noteId}`,
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

`{"id": 123,"comment": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_updateRiskNote)Update Risk Note

Update a specific Risk Note using the provided Note ID.

🔒 Requires **Risk Management: Update Risk Note** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_updateRiskNote!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_updateRiskNote!in=path&path=noteId&t=request)noteId
required number

The Note ID

Example:123

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_updateRiskNote!path=comment&t=request)comment
required string<= 768 characters

The comment content for the note

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

put/risk-registers/{riskRegisterId}/risks/{riskId}/notes/{noteId}

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

`{"comment": "This is a note about the risk assessment findings."}`

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

`{"id": 123,"comment": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_deleteRiskNote)Delete Risk Note

Delete a specific Risk Note using the provided Note ID.

🔒 Requires **Risk Management: Delete Risk Note** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_deleteRiskNote!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Notes/#tag/Risk-Notes/operation/RiskNotesPublicV2Controller_deleteRiskNote!in=path&path=noteId&t=request)noteId
required number

The Note ID

Example:123

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

delete/risk-registers/{riskRegisterId}/risks/{riskId}/notes/{noteId}

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
  const riskId = 'YOUR_riskId_PARAMETER';
  const noteId = 'YOUR_noteId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}/notes/${noteId}`,
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
