Title: Risk Documents

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Risk Documents are supporting documents, evidence, or other materials that are associated with a risk.

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_listRiskDocuments)List Risk Documents

Retrieve all Documents associated with a specific Risk.

🔒 Requires **Risk Management: Get Risk Documents** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_listRiskDocuments!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_listRiskDocuments!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_listRiskDocuments!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_listRiskDocuments!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_listRiskDocuments!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_listRiskDocuments!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false

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

get/risk-registers/{riskRegisterId}/risks/{riskId}/documents

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
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}/documents`,
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

`{"data": [{"id": 1,"name": "Risk Assessment Document.pdf","createdAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_uploadRiskDocuments)Upload Risk Documents

Upload one or more Documents for a given Risk.

🔒 Requires **Risk Management: Upload Risk Document** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_uploadRiskDocuments!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")

##### Request Body schema: multipart/form-data
required

Upload one or more Documents for the Risk

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_uploadRiskDocuments!path=files&t=request)files
required Array of strings<binary>

Document files to upload (max 10 files, 25MB each)

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

413
The file was too large to upload

500
Internal server error

503
Third party system was unavailable

post/risk-registers/{riskRegisterId}/risks/{riskId}/documents

Request samples

*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

2 more

import FormData from 'form-data';
import fetch from 'node-fetch';

async function run() {
  const form = new FormData();
  form.append('files[0]','string');

  const riskRegisterId = 'YOUR_riskRegisterId_PARAMETER';
  const riskId = 'YOUR_riskId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}/documents`,
    {
      method: 'POST',
      headers: {
        Authorization: 'Bearer <YOUR_API_KEY_HERE>'
      },
      body: form
    }
  );

  const data = await resp.text();
  console.log(data);
}

run();

Response samples

*   201
*   400
*   401
*   402
*   403
*   404
*   412
*   413
*   500
*   503

5 more

application/json

`{"documents": [{"id": 1,"name": "Risk Assessment Document.pdf","createdAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_getRiskDocument)Get Risk Document

Retrieve a specific Risk Document by its ID.

🔒 Requires **Risk Management: Get Risk Documents** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_getRiskDocument!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_getRiskDocument!in=path&path=documentId&t=request)documentId
required number

The Document ID

Example:123

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

get/risk-registers/{riskRegisterId}/risks/{riskId}/documents/{documentId}

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
  const documentId = 'YOUR_documentId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}/documents/${documentId}`,
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

## [](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_deleteRiskDocument)Delete Risk Document

Delete a specific Risk Document using the provided Document ID.

🔒 Requires **Risk Management: Delete Risk Document** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_deleteRiskDocument!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")
[](https://developers.drata.com/openapi/reference/v2/tag/Risk-Documents/#tag/Risk-Documents/operation/RiskDocumentsPublicV2Controller_deleteRiskDocument!in=path&path=documentId&t=request)documentId
required number

The Document ID

Example:123

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

delete/risk-registers/{riskRegisterId}/risks/{riskId}/documents/{documentId}

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
  const documentId = 'YOUR_documentId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}/documents/${documentId}`,
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
