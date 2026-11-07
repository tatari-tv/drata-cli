Title: Vendor Documents

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Vendor Documents provide compliance-related documentation, such as bridge letters, questionnaires, and SOC reports.

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocuments)List Vendor Documents

Get a list of Documents for a given Vendor.

🔒 Requires **Vendors: List Vendors** permission.

💎 Requires your account have the **TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocuments!in=path&path=vendorId&t=request)vendorId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocuments!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocuments!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocuments!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocuments!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocuments!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocuments!in=query&path=type&t=request)type string (VendorDocumentTypeEnum)

Vendor Document type

Enum:"COMPLIANCE_REPORT""COMPLIANCE_REPORT_REVIEW""BRIDGE_LETTER""UPLOADED_COMPLIANCE_REPORT_REVIEW""QUESTIONNAIRE_ATTACHMENT""SOC_DOCUMENT""QUESTIONNAIRE_REPORT""TRUST_CENTER_PAGE"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocuments!in=query&path=expand[]&t=request)expand[]Array of strings (VendorDocumentExpandEnum)

List of subcollections and sub-objects to expand

Items Value:"downloadUrl"

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

get/vendors/{vendorId}/documents

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
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/documents`,
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

`{"data": [{"id": 1,"name": "AWS SOC 2 2025","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","type": "COMPLIANCE_REPORT","downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="},"fileName": "Artifact 3.png"}}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_uploadVendorDocument)Upload Vendor Document

Upload a Vendor Document.

🔒 Requires **Vendors: Upload Vendor Report** permission.

💎 Requires your account have the **TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_uploadVendorDocument!in=path&path=vendorId&t=request)vendorId
required number

##### Request Body schema: multipart/form-data
required

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_uploadVendorDocument!path=file&t=request)file
required string<binary>

Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .gif, .jpg, .jpeg, .png, .csv. MIME type must match the file extension
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_uploadVendorDocument!path=securityReviewId&t=request)securityReviewId number or null

Security Review ID, when set this will attach the document to this security review
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_uploadVendorDocument!path=type&t=request)type string

Vendor Document type

Enum:"COMPLIANCE_REPORT""COMPLIANCE_REPORT_REVIEW""BRIDGE_LETTER""UPLOADED_COMPLIANCE_REPORT_REVIEW""QUESTIONNAIRE_ATTACHMENT""SOC_DOCUMENT""QUESTIONNAIRE_REPORT""TRUST_CENTER_PAGE"

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

412
You must accept the Drata terms and conditions to use the API

413
The file was too large to upload

500
Internal server error

503
Third party system was unavailable

post/vendors/{vendorId}/documents

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
  form.append('file','string');

  const vendorId = 'YOUR_vendorId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/documents`,
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
*   412
*   413
*   500
*   503

4 more

application/json

`{"id": 1,"name": "AWS SOC 2 2025","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","type": "COMPLIANCE_REPORT","downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="},"fileName": "Artifact 3.png"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocument)Get Vendor Document

🔒 Requires **Vendors: Get Vendor** permission.

💎 Requires your account have the **TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocument!in=path&path=vendorId&t=request)vendorId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Documents/#tag/Vendor-Documents/operation/VendorDocumentsPublicV2Controller_getVendorDocument!in=path&path=documentId&t=request)documentId
required number

Responses

200

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

get/vendors/{vendorId}/documents/{documentId}

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
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const documentId = 'YOUR_documentId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/documents/${documentId}`,
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
*   401
*   402
*   403
*   404
*   412
*   500

2 more

application/json

`{"id": 1,"name": "AWS SOC 2 2025","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","type": "COMPLIANCE_REPORT","downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="},"fileName": "Artifact 3.png"}}`
