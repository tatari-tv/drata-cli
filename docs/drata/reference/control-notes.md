Title: Control Notes

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Control Notes allow you to provide additional information about Controls.

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNotes)List Control Notes

Find Control Notes matching the provided filters.

🔒 Requires **Controls: Get Control Note** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNotes!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNotes!in=path&path=controlId&t=request)controlId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNotes!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNotes!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNotes!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNotes!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNotes!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNotes!in=query&path=excludeIds&t=request)excludeIds Array of strings

Exclude Control Notes by IDs

Example:
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNotes!in=query&path=expand[]&t=request)expand[]Array of strings (ControlNotesExpandEnum)

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

get/workspaces/{workspaceId}/controls/{controlId}/notes

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
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls/${controlId}/notes`,
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

`{"data": [{"id": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd","ownerId": 60,"comment": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_createControlNote)Create Control Note

Create a Note for a given Control.

🔒 Requires **Controls: Create Control Note** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_createControlNote!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_createControlNote!in=path&path=controlId&t=request)controlId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_createControlNote!path=comment&t=request)comment
required string<= 191 characters

The text of the Note

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

post/workspaces/{workspaceId}/controls/{controlId}/notes

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

`{"comment": "string"}`

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

`{"id": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd","ownerId": 60,"comment": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNote)Get Control Note

Get a Note associated with a given Control.

🔒 Requires **Controls: Get Control Note** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNote!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNote!in=path&path=controlId&t=request)controlId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNote!in=path&path=noteId&t=request)noteId
required string

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_getControlNote!in=query&path=expand[]&t=request)expand[]Array of strings (ControlNotesExpandEnum)

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

get/workspaces/{workspaceId}/controls/{controlId}/notes/{noteId}

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
  const noteId = 'YOUR_noteId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls/${controlId}/notes/${noteId}`,
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

`{"id": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd","ownerId": 60,"comment": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_updateNote)Update Control Note

Update a Note for a given Control.

🔒 Requires **Controls: Update Control Note** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_updateNote!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_updateNote!in=path&path=controlId&t=request)controlId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_updateNote!in=path&path=noteId&t=request)noteId
required string

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_updateNote!path=comment&t=request)comment
required string<= 191 characters

The text of the note

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

put/workspaces/{workspaceId}/controls/{controlId}/notes/{noteId}

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

`{"comment": "Note comment"}`

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

`{"id": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd","ownerId": 60,"comment": "string","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_deleteNote)Delete Control Note

Delete a Note for a given Control.

🔒 Requires **Controls: Delete Control Note** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_deleteNote!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_deleteNote!in=path&path=controlId&t=request)controlId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Notes/#tag/Control-Notes/operation/ControlNotesPublicV2Controller_deleteNote!in=path&path=noteId&t=request)noteId
required string

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

delete/workspaces/{workspaceId}/controls/{controlId}/notes/{noteId}

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
  const noteId = 'YOUR_noteId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls/${controlId}/notes/${noteId}`,
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

*   401
*   403
*   404
*   412
*   500

application/json

`{"statusCode": 0,"message": "string","code": 0,"debugInfo": {"name": "string","message": "string","stack": "string"},"providerError": {"provider": "string","status": 0,"code": "string","message": "string","details": { }}}`
