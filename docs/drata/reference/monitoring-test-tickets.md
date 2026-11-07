Title: Monitoring Test Tickets

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Monitoring Test Tickets are remediation items in a connected ticket management system, linked to a Monitoring Test so that the work fixing a failing test is tracked against it.

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_getMonitoringTestTickets)List Monitoring Test Tickets

List the remediation Tickets linked to a Monitoring Test. Returns the state Drata holds; use the sync action to refresh it from the connected system.

🔒 Requires **Monitoring: List Monitoring Test Tickets** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_getMonitoringTestTickets!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_getMonitoringTestTickets!in=path&path=testId&t=request)testId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_getMonitoringTestTickets!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_getMonitoringTestTickets!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_getMonitoringTestTickets!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_getMonitoringTestTickets!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_getMonitoringTestTickets!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_getMonitoringTestTickets!in=query&path=isDone&t=request)isDone boolean

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

get/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets

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
  const testId = 'YOUR_testId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/monitoring-tests/${testId}/tickets`,
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

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_linkMonitoringTestTicket)Link Monitoring Test Ticket

Link an existing Ticket to a Monitoring Test by its URL. The provider is resolved from the URL, so a Connection for that provider must already exist.

🔒 Requires **Monitoring: Link Monitoring Test Ticket** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_linkMonitoringTestTicket!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_linkMonitoringTestTicket!in=path&path=testId&t=request)testId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_linkMonitoringTestTicket!path=ticketUrl&t=request)ticketUrl
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

post/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets

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

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_createMonitoringTestTicket)Create Monitoring Test Ticket

Create a new Ticket in a connected ticket management system and link it to a Monitoring Test. Requires a Connection for the destination system; to attach a Ticket that already exists, link it instead.

🔒 Requires **Ticket Management: Create Ticket** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_createMonitoringTestTicket!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_createMonitoringTestTicket!in=path&path=testId&t=request)testId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_createMonitoringTestTicket!path=connectionId&t=request)connectionId
required number

ID of the Connection to create the Ticket through. The Connection determines which ticket management system receives it.
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_createMonitoringTestTicket!path=projectId&t=request)projectId string

ID of the destination project, for providers that use projects
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_createMonitoringTestTicket!path=issueTypeId&t=request)issueTypeId string

ID of the issue type to create, for providers that use issue types
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_createMonitoringTestTicket!path=listId&t=request)listId string

ID of the destination list, for providers that use lists
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_createMonitoringTestTicket!path=teamId&t=request)teamId string

ID of the destination team, for providers that use teams
required Array of objects (TicketCreateFieldPublicV2Dto)

Field values for the new Ticket. Which fields are required is determined by the destination project and issue type in the connected system.
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_createMonitoringTestTicket!path=useRichContent&t=request)useRichContent boolean

Monitoring Test Tickets only: populate the Ticket body from the failing test result rather than from the supplied fields alone. Ignored for other Tickets.

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

503
Third party system was unavailable

post/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets/action-create

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

`{"connectionId": 42,"projectId": "10000","issueTypeId": "10001","listId": "10002","teamId": "10003","fields": [{"field": "summary","type": "string","value": "Enable MFA for the two remaining admin accounts"}],"useRichContent": false}`

Response samples

*   201
*   400
*   401
*   403
*   404
*   412
*   500
*   503

3 more

application/json

`{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectionId": 42,"lastSyncedAt": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_unlinkMonitoringTestTicket)Unlink Monitoring Test Ticket

Remove the association between a Ticket and a Monitoring Test. The Ticket itself is not changed in the connected system.

🔒 Requires **Monitoring: Unlink Monitoring Test Ticket** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_unlinkMonitoringTestTicket!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_unlinkMonitoringTestTicket!in=path&path=testId&t=request)testId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Test-Tickets/#tag/Monitoring-Test-Tickets/operation/MonitoringTestTicketsPublicV2Controller_unlinkMonitoringTestTicket!in=path&path=ticketId&t=request)ticketId
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

delete/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets/{ticketId}

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
  const testId = 'YOUR_testId_PARAMETER';
  const ticketId = 'YOUR_ticketId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/monitoring-tests/${testId}/tickets/${ticketId}`,
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
