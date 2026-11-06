Title: Vendor Questionnaire Templates

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Vendor Questionnaire Templates are the questionnaires you can send to a Vendor. List them to find the `questionnaireId` a send request takes, and list a template's questions to see what the Vendor will be asked.

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplates)List Vendor Questionnaire Templates

List the Questionnaire Templates that can be sent to a Vendor. Only ACTIVE templates are returned unless `status` is DRAFT. Pass a template `id` as `questionnaireId` when sending a Questionnaire to a Vendor.

🔒 Requires **Vendors: List Vendor Questionnaire** permission.

💎 Requires your account have the **TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplates!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplates!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplates!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplates!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplates!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplates!in=query&path=title&t=request)title string<= 191 characters

Filters by title prefix match: returns Questionnaire Templates whose title starts with this text.

Example:title=Security
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplates!in=query&path=status&t=request)status string (VendorQuestionnaireTemplateStatusEnum)

Filter Questionnaire Templates by status. Defaults to ACTIVE, the templates ready to send, which includes the default Questionnaire Template. Pass DRAFT to list unfinished templates.

Enum:"ACTIVE""DRAFT"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplates!in=query&path=riskLevels[]&t=request)riskLevels[]Array of strings (VendorQuestionnaireTemplateRiskLevelEnum)

Filter Questionnaire Templates by the Vendor risk levels they target. Returns templates matching any of the given levels.

Items Enum:"NONE""LOW""MODERATE""HIGH"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplates!in=query&path=categories[]&t=request)categories[]Array of strings (VendorQuestionnaireTemplateCategoryEnum)

Filter Questionnaire Templates by the Vendor categories they target. Returns templates matching any of the given categories.

Items Enum:"ENGINEERING""PRODUCT""MARKETING""CS""SALES""FINANCE""HR""ADMINISTRATIVE""SECURITY""LEGAL""INFORMATION_TECHNOLOGY"

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

get/vendor-questionnaire-templates

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
    `https://public-api.drata.com/public/v2/vendor-questionnaire-templates`,
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

`{"data": [{"id": 1,"title": "Security Assessment","status": "ACTIVE","isDefault": false,"type": "SECURITY_ASSESSMENT","riskLevels": ["HIGH"],"categories": ["SECURITY"],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplateQuestions)List Vendor Questionnaire Template Questions

List the questions a Questionnaire Template asks, in the order the Vendor will see them.

🔒 Requires **Vendors: List Vendor Questionnaire** permission.

💎 Requires your account have the **TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplateQuestions!in=path&path=questionnaireId&t=request)questionnaireId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplateQuestions!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplateQuestions!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplateQuestions!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplateQuestions!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data. Defaults to ascending so questions are returned in the order the Vendor was asked them.

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Questionnaire-Templates/#tag/Vendor-Questionnaire-Templates/operation/VendorQuestionnaireTemplatesPublicV2Controller_listVendorQuestionnaireTemplateQuestions!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

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

get/vendor-questionnaire-templates/{questionnaireId}/questions

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
  const questionnaireId = 'YOUR_questionnaireId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendor-questionnaire-templates/${questionnaireId}/questions`,
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

`{"data": [{"id": 1,"question": "Is there a publicly available URL for your Privacy Policy?","type": "YES_NO","required": true,"choices": ["Yes","No"],"allowMultipleSelection": false,"allowOtherChoice": false}],"pagination": {"cursor": "string","totalCount": 0}}`
