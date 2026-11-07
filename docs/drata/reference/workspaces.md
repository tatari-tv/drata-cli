Title: Workspaces

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Workspaces/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Workspaces allow you to represent different products or business lines that have different compliance requirements. Each Workspace can have its own Frameworks and Controls. The [help docs](https://help.drata.com/en/articles/6598264) have more information on how create and manage Workspaces.

## [](https://developers.drata.com/openapi/reference/v2/tag/Workspaces/#tag/Workspaces/operation/WorkspacesPublicV2Controller_listWorkspaces)List Workspaces

🔒 Requires **Workspaces: List Workspaces** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Workspaces/#tag/Workspaces/operation/WorkspacesPublicV2Controller_listWorkspaces!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Workspaces/#tag/Workspaces/operation/WorkspacesPublicV2Controller_listWorkspaces!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Workspaces/#tag/Workspaces/operation/WorkspacesPublicV2Controller_listWorkspaces!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Workspaces/#tag/Workspaces/operation/WorkspacesPublicV2Controller_listWorkspaces!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Workspaces/#tag/Workspaces/operation/WorkspacesPublicV2Controller_listWorkspaces!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Workspaces/#tag/Workspaces/operation/WorkspacesPublicV2Controller_listWorkspaces!in=query&path=expand[]&t=request)expand[]Array of strings (WorkspaceExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"frameworks""personnelScope"

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

get/workspaces

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
    `https://public-api.drata.com/public/v2/workspaces`,
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

`{"data": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","howItWorks": "Connect your systems to the Drata Autopilot system and sit back and relax!","frameworks": [{"id": 13,"name": "NIST AI RMF","description": "The NIST AI Risk Management Framework (AI RMF) is a set of suggestions and guidelines for incorporating trustworthiness considerations in the design, development, deployment, and use of AI systems.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","enabledAt": "2025-07-01T16:45:55.246Z"}],"scopedPersonnelCount": 42}],"pagination": {"cursor": "string","totalCount": 0}}`
