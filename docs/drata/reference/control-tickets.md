Title: Control Tickets

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Control Tickets are remediation items in a connected ticket management system, linked to a Control so that the work closing a gap is recorded alongside the Control it affects.

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_getControlTickets)List Control Tickets

List the remediation Tickets linked to a Control. Returns the state Drata holds; it does not call the connected system.

🔒 Requires **Controls: List Control Tickets** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_getControlTickets!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_getControlTickets!in=path&path=controlId&t=request)controlId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_getControlTickets!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_getControlTickets!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_getControlTickets!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_getControlTickets!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_getControlTickets!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_getControlTickets!in=query&path=isDone&t=request)isDone boolean

Filter by completion state. Omit to return both open and completed Tickets.

Example:isDone=false

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

get/workspaces/{workspaceId}/controls/{controlId}/tickets

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls/${controlId}/tickets`,
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

`{"data": [{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectionId": 42,"lastSyncedAt": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_linkControlTicket)Link Control Ticket

Link an existing Ticket to a Control by its URL. The provider is resolved from the URL, so a Connection for that provider must already exist.

🔒 Requires **Controls: Link Control Ticket** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_linkControlTicket!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_linkControlTicket!in=path&path=controlId&t=request)controlId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_linkControlTicket!path=ticketUrl&t=request)ticketUrl
required string

URL of an existing Ticket in a connected ticket management system. The provider is resolved from the URL, so the Connection for that provider must already exist.

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

post/workspaces/{workspaceId}/controls/{controlId}/tickets

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

`{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectionId": 42,"lastSyncedAt": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_unlinkControlTicket)Unlink Control Ticket

Remove the association between a Ticket and a Control. The Ticket itself is not changed in the connected system.

🔒 Requires **Controls: Unlink Control Ticket** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_unlinkControlTicket!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_unlinkControlTicket!in=path&path=controlId&t=request)controlId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Control-Tickets/#tag/Control-Tickets/operation/ControlTicketsPublicV2Controller_unlinkControlTicket!in=path&path=ticketId&t=request)ticketId
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

delete/workspaces/{workspaceId}/controls/{controlId}/tickets/{ticketId}

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
  const ticketId = 'YOUR_ticketId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls/${controlId}/tickets/${ticketId}`,
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
