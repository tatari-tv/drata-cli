Title: Vendor Risk Levels

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Vendor-Risk-Levels/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Vendor Risk Levels are the vendor risk-score labels (e.g. Critical, High) used to classify the inherent and residual risk of your Vendors.

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Risk-Levels/#tag/Vendor-Risk-Levels/operation/VendorRiskLevelsPublicV2Controller_listVendorRiskLevels)List Vendor Risk Levels

List all vendor risk levels configured for your account. The list is ordered by the inherited v2 default (by id, newest first), not by position. Clients that need severity order should sort the returned page by position.

🔒 Requires **Vendors: List Vendor Risk Levels** permission.

💎 Requires your account have the **TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Risk-Levels/#tag/Vendor-Risk-Levels/operation/VendorRiskLevelsPublicV2Controller_listVendorRiskLevels!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Risk-Levels/#tag/Vendor-Risk-Levels/operation/VendorRiskLevelsPublicV2Controller_listVendorRiskLevels!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Risk-Levels/#tag/Vendor-Risk-Levels/operation/VendorRiskLevelsPublicV2Controller_listVendorRiskLevels!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Risk-Levels/#tag/Vendor-Risk-Levels/operation/VendorRiskLevelsPublicV2Controller_listVendorRiskLevels!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Risk-Levels/#tag/Vendor-Risk-Levels/operation/VendorRiskLevelsPublicV2Controller_listVendorRiskLevels!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false

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

get/vendor-risk-levels

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
    `https://public-api.drata.com/public/v2/vendor-risk-levels`,
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

`{"data": [{"id": 12,"name": "Critical","position": 1,"isSystem": false}],"pagination": {"cursor": "string","totalCount": 0}}`
