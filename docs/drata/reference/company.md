Title: Company

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Company/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
The Company tracks essential information about your organization. The [help docs](https://help.drata.com/en/articles/8283910) have more information on the purpose of each field.

## [](https://developers.drata.com/openapi/reference/v2/tag/Company/#tag/Company/operation/CompaniesPublicV2Controller_getCompany)Get Company

🔒 Requires **Company Settings: Get Company Settings** permission.

Security

**bearer** or **oauth2**

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

get/company

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
    `https://public-api.drata.com/public/v2/company`,
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

`{"accountId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd","domain": "domain.com","name": "Acme","legalName": "Acme Inc.","description": "We make tools","logoUrl": "https://domain.com/logo.png","securityTraining": "DRATA_PROVIDED","hipaaTraining": "DRATA_PROVIDED","backgroundCheck": "CERTN","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","agentEnabled": false,"manualUploadEnabled": false,"entitlements": [{"name": "string","description": "string","type": "string","featureId": 0,"metadata": { }}],"language": "ENGLISH_US"}`
