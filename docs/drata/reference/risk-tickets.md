Title: Risk Tickets

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Risk Tickets are remediation items in a connected ticket management system, linked to a Risk so that the work treating that Risk is recorded against it.

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_getRiskTickets)List Risk Tickets

List the remediation Tickets linked to a Risk. Each Ticket reflects the state Drata holds, not a live read from the connected system.

🔒 Requires **Risk Management: List Risk Tickets** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_getRiskTickets!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_getRiskTickets!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_getRiskTickets!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_getRiskTickets!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_getRiskTickets!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_getRiskTickets!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_getRiskTickets!in=query&path=isDone&t=request)isDone boolean

Filter by completion state. Omit to return both open and completed Tickets.

Example:isDone=false

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

get/risk-registers/{riskRegisterId}/risks/{riskId}/tickets

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
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}/tickets`,
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

`{"data": [{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectionId": 42,"lastSyncedAt": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_linkRiskTicket)Link Risk Ticket

Link an existing Ticket to a Risk by its URL. The provider is resolved from the URL, so a Connection for that provider must already exist.

🔒 Requires **Risk Management: Link Risk Ticket** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_linkRiskTicket!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_linkRiskTicket!path=ticketUrl&t=request)ticketUrl
required string

URL of an existing Ticket in a connected ticket management system. The provider is resolved from the URL, so the Connection for that provider must already exist.

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

409
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/risk-registers/{riskRegisterId}/risks/{riskId}/tickets

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
*   402
*   403
*   404
*   409
*   412
*   500

4 more

application/json

`{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectionId": 42,"lastSyncedAt": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_createRiskTicket)Create Risk Ticket

Create a new Ticket in a connected ticket management system and link it to a Risk. Requires a Connection for the destination system; to attach a Ticket that already exists, link it instead.

🔒 Requires **Ticket Management: Create Ticket** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_createRiskTicket!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_createRiskTicket!path=connectionId&t=request)connectionId
required number

ID of the Connection to create the Ticket through. The Connection determines which ticket management system receives it.
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_createRiskTicket!path=projectId&t=request)projectId string

ID of the destination project, for providers that use projects
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_createRiskTicket!path=issueTypeId&t=request)issueTypeId string

ID of the issue type to create, for providers that use issue types
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_createRiskTicket!path=listId&t=request)listId string

ID of the destination list, for providers that use lists
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_createRiskTicket!path=teamId&t=request)teamId string

ID of the destination team, for providers that use teams
required Array of objects (TicketCreateFieldPublicV2Dto)

Field values for the new Ticket. Which fields are required is determined by the destination project and issue type in the connected system.
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_createRiskTicket!path=useRichContent&t=request)useRichContent boolean

Monitoring Test Tickets only: populate the Ticket body from the failing test result rather than from the supplied fields alone. Ignored for other Tickets.

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

500
Internal server error

post/risk-registers/{riskRegisterId}/risks/{riskId}/tickets/action-create

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
*   402
*   403
*   404
*   412
*   500

3 more

application/json

`{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectionId": 42,"lastSyncedAt": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_unlinkRiskTicket)Unlink Risk Ticket

Remove the association between a Ticket and a Risk. The Ticket itself is not changed in the connected system.

🔒 Requires **Risk Management: Unlink Risk Ticket** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_unlinkRiskTicket!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Tickets/#tag/Risk-Tickets/operation/RiskTicketsPublicV2Controller_unlinkRiskTicket!in=path&path=ticketId&t=request)ticketId
required number

The Ticket ID

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

delete/risk-registers/{riskRegisterId}/risks/{riskId}/tickets/{ticketId}

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
  const ticketId = 'YOUR_ticketId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}/tickets/${ticketId}`,
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
