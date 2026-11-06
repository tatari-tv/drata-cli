Title: Vendor Criteria Creations

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Vendor-Criteria-Creations/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Vendor Criteria Creation workflows track the agent-assisted creation of assessment criteria for Vendors within a Workspace.

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Criteria-Creations/#tag/Vendor-Criteria-Creations/operation/VendorCriteriaCreationsPublicV2Controller_listVendorCriteriaCreations)List Vendor Criteria Creations

Find Vendor Criteria Creation workflows in a Workspace matching the provided filters.

🔒 Requires **Vendors: List Vendor Criteria Creations** permission.

💎 Requires your account have the **Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Criteria-Creations/#tag/Vendor-Criteria-Creations/operation/VendorCriteriaCreationsPublicV2Controller_listVendorCriteriaCreations!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Criteria-Creations/#tag/Vendor-Criteria-Creations/operation/VendorCriteriaCreationsPublicV2Controller_listVendorCriteriaCreations!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Criteria-Creations/#tag/Vendor-Criteria-Creations/operation/VendorCriteriaCreationsPublicV2Controller_listVendorCriteriaCreations!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Criteria-Creations/#tag/Vendor-Criteria-Creations/operation/VendorCriteriaCreationsPublicV2Controller_listVendorCriteriaCreations!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Criteria-Creations/#tag/Vendor-Criteria-Creations/operation/VendorCriteriaCreationsPublicV2Controller_listVendorCriteriaCreations!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Criteria-Creations/#tag/Vendor-Criteria-Creations/operation/VendorCriteriaCreationsPublicV2Controller_listVendorCriteriaCreations!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Criteria-Creations/#tag/Vendor-Criteria-Creations/operation/VendorCriteriaCreationsPublicV2Controller_listVendorCriteriaCreations!in=query&path=statuses[]&t=request)statuses[]Array of strings (AgentWorkflowStatusEnum)

Filter by Workflow statuses

Items Enum:"PENDING""IN_PROGRESS""COMPLETED""FAILED""CANCELLED"

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

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/workspaces/{workspaceId}/vendor-criteria-creations

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/vendor-criteria-creations`,
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

`{"data": [{"id": 1,"workspaceId": 1,"status": "IN_PROGRESS","createdAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`
