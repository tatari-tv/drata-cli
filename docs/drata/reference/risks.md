Title: Risks

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Risks/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Risks are potential events that could impact the security, reputation, and financial health of a company.

## [](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks)List Risks

Find Risks matching the provided filters.

🔒 Requires **Risk Management: List Risks** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=expand[]&t=request)expand[]Array of strings (RiskExpandV2Enum)

List of subcollections and sub-objects to expand

Items Enum:"owners""reviewers""controls""categories""documents""notes""tickets""tasks""customFields"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=title&t=request)title string

Filter Risks by title. Performs a case-insensitive partial match on the Risk title field.

Example:title=Activity Log Evaluation - Unauthorized System Access
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=status&t=request)status string (RiskStatusTypeEnum)

Filter Risks by status

Enum:"ACTIVE""ARCHIVED""CLOSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=treatmentPlan&t=request)treatmentPlan string (RiskTreatmentPlanEnum)

Filter Risks by their treatment strategy. Treatment plans define how the organization addresses each Risk: ACCEPT (acknowledge and monitor), AVOID (eliminate the Risk source), MITIGATE (reduce impact/likelihood), TRANSFER (shift to third party via insurance/contracts), or UNTREATED (no action taken yet).

Enum:"UNTREATED""ACCEPT""TRANSFER""AVOID""MITIGATE"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=minScore&t=request)minScore number or null[ 1 .. 25 ]

Filter Risks by minimum calculated Risk score (impact × likelihood). Score range depends on organization settings (1-25 with thresholds 1-4=low, 5-9=medium, 10-16=high, 17-25=critical)

Example:minScore=5
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=maxScore&t=request)maxScore number or null[ 1 .. 25 ]

Filter Risks by maximum calculated Risk score (impact × likelihood). Score range depends on organization settings (1-25 with thresholds 1-4=low, 5-9=medium, 10-16=high, 17-25=critical)

Example:maxScore=16
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_listRisks!in=query&path=vendorId&t=request)vendorId number or null

Filter Risks by vendor/third-party provider. Specify the unique vendor identifier to return only Risks associated with that specific vendor, such as Risks related to vendor security assessments, data processing agreements, or third-party service dependencies.

Example:vendorId=1

Responses

200

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

get/risk-registers/{riskRegisterId}/risks

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
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks`,
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
*   403
*   412
*   500

application/json

`{"data": [{"id": 1,"riskId": "AC-04","title": "Password Management - Password Cracking","description": "An attacker attempts to gain access to organizational information by guessing of passwords.","treatmentPlan": "UNTREATED","treatmentDetails": "Implementing multi-factor authentication and password complexity requirements to reduce likelihood of successful password attacks.","anticipatedCompletionDate": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z","identifiedAt": "2025-07-01T16:45:55.246Z","impact": 5,"likelihood": 5,"score": 25,"residualImpact": 3,"residualLikelihood": 3,"residualScore": 9,"status": "ACTIVE","type": "INTERNAL","vendorId": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"categories": [{"id": 1,"name": "Access Control","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"owners": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"reviewers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"documents": [{"id": 1,"name": "Risk Assessment Report Q4 2023.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"notes": [{"id": 1,"comment": "This Risk has been reviewed and approved by the security team. Implementation timeline updated.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"tickets": [{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"tasks": [{"id": 1,"title": "Review quarterly security policies","description": "Conduct a comprehensive review of all security policies to ensure compliance with current regulations.","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"riskRegister": {"id": 1,"name": "Default Risk Register","description": "Primary risk register for organizational risks"},"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk)Create Risk

Create a new custom Risk in the Risk Register.

🔒 Requires **Risk Management: Create Risk** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=title&t=request)title
required string<= 191 characters

Descriptive title that clearly identifies the nature and scope of the Risk
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=description&t=request)description
required string<= 768 characters

Comprehensive description of the Risk scenario, potential threats, business impact, and affected systems or processes
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=identifiedAt&t=request)identifiedAt string or null<date>

Date when the Risk was first identified (accepts date-only or full ISO8601 format)
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=impact&t=request)impact number[ 1 .. 10 ]

Impact score representing the potential severity of consequences if the Risk materializes. Default is 1-5 scale (where 5 is most severe), but can be configured from 3x3 up to 10x10 matrices.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=likelihood&t=request)likelihood number[ 1 .. 10 ]

Likelihood score representing the probability of the Risk occurring. Default is 1-5 scale (where 5 is most likely), but can be configured from 3x3 up to 10x10 matrices.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=treatmentPlan&t=request)treatmentPlan string

Default:0

Strategy for addressing the Risk: ACCEPT (acknowledge and monitor), AVOID (eliminate source), MITIGATE (reduce impact/likelihood), TRANSFER (shift to third party), UNTREATED (no action taken)

Enum:"UNTREATED""ACCEPT""TRANSFER""AVOID""MITIGATE"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=treatmentDetails&t=request)treatmentDetails string<= 30000 characters

Detailed explanation of the specific actions, controls, or measures being implemented to address the Risk. Cannot be provided when treatmentPlan is UNTREATED
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=anticipatedCompletionDate&t=request)anticipatedCompletionDate string<date-time>

Target date for completing Risk treatment activities (accepts date-only or full ISO8601 format). Cannot be provided when treatmentPlan is UNTREATED
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=completionDate&t=request)completionDate string<date>

Actual date when Risk treatment was completed (accepts date-only or full ISO8601 format). Cannot be provided when treatmentPlan is UNTREATED
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=residualImpact&t=request)residualImpact number[ 1 .. 10 ]

Residual impact score after treatment implementation. Default is 1-5 scale, but can be configured from 3x3 up to 10x10 matrices. Cannot be provided when treatmentPlan is UNTREATED
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=residualLikelihood&t=request)residualLikelihood number[ 1 .. 10 ]

Residual likelihood score after treatment implementation. Default is 1-5 scale, but can be configured from 3x3 up to 10x10 matrices. Cannot be provided when treatmentPlan is UNTREATED
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=status&t=request)status string

Default:"ACTIVE"

Current lifecycle status of the Risk: ACTIVE (being monitored/treated and considered applicable to your organization), CLOSED (resolved/completed), or ARCHIVED (not applicable to your organization).

Enum:"ACTIVE""ARCHIVED""CLOSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_createRisk!path=vendorId&t=request)vendorId number

Identifier of the Vendor this Risk relates to. Supplying a Vendor creates an external Risk; omitting it creates an internal Risk
Array of objects (CategoryRequestPublicV2Dto)

Risk categories for classification and reporting purposes
Array of objects (RiskOwnerRequestPublicV2Dto)

Users responsible for managing and monitoring this Risk
Array of objects (ReviewerRequestPublicV2Dto)

Users responsible for reviewing and approving Risk treatment plans. Cannot be provided when treatmentPlan is UNTREATED
Array of objects (RiskControlRequestPublicV2Dto)

Security controls implemented to mitigate this Risk
Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Risk

Responses

201
Created

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

409
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

422
Unprocessable Entity

500
Internal server error

post/risk-registers/{riskRegisterId}/risks

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

`{"title": "Password Management - Weak Password Policies","description": "Weak password policies may allow unauthorized access to organizational systems and data through password-based attacks such as brute force, dictionary attacks, or credential stuffing.","identifiedAt": "2020-07-06","impact": 4,"likelihood": 3,"treatmentPlan": "MITIGATE","treatmentDetails": "Implement multi-factor authentication and enforce strong password complexity requirements across all systems.","anticipatedCompletionDate": "2020-07-06","completionDate": "2020-07-06","residualImpact": 2,"residualLikelihood": 2,"status": "ACTIVE","vendorId": 1,"categories": [{"id": 1}],"owners": [{"id": 1}],"reviewers": [{"id": 1}],"controls": [{"id": 1}],"customFields": [{"id": 1,"name": "Compliance Status","value": "Security & IT"}]}`

Response samples

*   201
*   400
*   401
*   403
*   409
*   412
*   422
*   500

3 more

application/json

`{"id": 1,"riskId": "AC-04","title": "Password Management - Password Cracking","description": "An attacker attempts to gain access to organizational information by guessing of passwords.","treatmentPlan": "UNTREATED","treatmentDetails": "Implementing multi-factor authentication and password complexity requirements to reduce likelihood of successful password attacks.","anticipatedCompletionDate": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z","identifiedAt": "2025-07-01T16:45:55.246Z","impact": 5,"likelihood": 5,"score": 25,"residualImpact": 3,"residualLikelihood": 3,"residualScore": 9,"status": "ACTIVE","type": "INTERNAL","vendorId": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"categories": [{"id": 1,"name": "Access Control","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"owners": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"reviewers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"documents": [{"id": 1,"name": "Risk Assessment Report Q4 2023.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"notes": [{"id": 1,"comment": "This Risk has been reviewed and approved by the security team. Implementation timeline updated.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"tickets": [{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"tasks": [{"id": 1,"title": "Review quarterly security policies","description": "Conduct a comprehensive review of all security policies to ensure compliance with current regulations.","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"riskRegister": {"id": 1,"name": "Default Risk Register","description": "Primary risk register for organizational risks"},"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks)Search Risks by Risk Register

Search Risks in a Risk Register with rich filtering, sorting, faceting, and expand. Supports cursor-based pagination.

🔒 Requires **Risk Management: List Risks** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=sort&t=request)sort string (RiskSortEnum)

Which field to sort by. Highest residual risk first (with `sortDir=DESC`, the default) is the natural default view for a risk register.

Enum:"residualScore""inherentScore""anticipatedCompletionDate""completedDate""title""riskId""status""vendorName""createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data.

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=expand[]&t=request)expand[]Array of strings (RiskSearchExpandEnum)

List of subcollections and sub-objects to expand.

Items Enum:"owners""reviewers""controls""categories""documents""notes""tickets""tasks""customFields"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=facets[]&t=request)facets[]Array of strings (RiskSearchInRegisterFacetEnum)

Filter dimensions for which to return value counts in the response. Only honored on the first page (when `cursor` is null). Each requested facet adds one terms aggregation to the search query. Bounded enum facets (`status`, `treatmentPlan`, `type`) are enriched with zero-count entries; unbounded facets (`categories`, `controls`, `ownerEmails`, `reviewerEmails`, `vendorNames`) return only values present in the result set.

Items Enum:"status""treatmentPlan""type""categories""controls""ownerEmails""reviewerEmails""vendorNames"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=q&t=request)q string<= 191 characters

Free-text search across `title`, `description`, `riskId`, and `vendorName`. Fuzzy, prefix-aware, and relevance-ranked.

Example:q=unauthorized access
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=status[]&t=request)status[]Array of strings (RiskStatusTypeEnum)

Filter Risks by one or more workflow statuses.

Items Enum:"ACTIVE""ARCHIVED""CLOSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=treatmentPlan[]&t=request)treatmentPlan[]Array of strings (RiskTreatmentPlanEnum)

Filter Risks by treatment plan.

Items Enum:"UNTREATED""ACCEPT""TRANSFER""AVOID""MITIGATE"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=type[]&t=request)type[]Array of strings (RiskVersionTypeEnum)

Filter Risks by version type.

Items Enum:"INTERNAL""EXTERNAL"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=inherentScoreGte&t=request)inherentScoreGte number

Filter Risks by minimum inherent (pre-mitigation) score (inclusive).
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=inherentScoreLte&t=request)inherentScoreLte number

Filter Risks by maximum inherent (pre-mitigation) score (inclusive).
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=residualScoreGte&t=request)residualScoreGte number

Filter Risks by minimum residual (post-mitigation) score (inclusive).
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=residualScoreLte&t=request)residualScoreLte number

Filter Risks by maximum residual (post-mitigation) score (inclusive).
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=anticipatedCompletionDateGte&t=request)anticipatedCompletionDateGte string<date-time>

Filter Risks with anticipated completion date on or after this ISO 8601 timestamp.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=anticipatedCompletionDateLte&t=request)anticipatedCompletionDateLte string<date-time>

Filter Risks with anticipated completion date on or before this ISO 8601 timestamp.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=completedDateGte&t=request)completedDateGte string<date-time>

Filter Risks with actual completion date on or after this ISO 8601 timestamp.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=completedDateLte&t=request)completedDateLte string<date-time>

Filter Risks with actual completion date on or before this ISO 8601 timestamp.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=categories[]&t=request)categories[]Array of strings

Filter Risks by exact category name membership.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=controls[]&t=request)controls[]Array of strings

Filter Risks by exact control code membership (e.g. `AC-04`).
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=ownerEmails[]&t=request)ownerEmails[]Array of strings

Filter Risks by owner email membership.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=reviewerEmails[]&t=request)reviewerEmails[]Array of strings

Filter Risks by reviewer email membership.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisks!in=query&path=vendorNames[]&t=request)vendorNames[]Array of strings

Filter Risks by exact vendor name membership. Each Risk is indexed with a single vendor name (the first associated vendor), so a Risk matches if that vendor name is in the supplied list.
Array of objects (CustomFieldFilterPublicV2Dto)

Filter Risks by Custom Field equality. Repeat the parameter with bracketed indices, e.g. `customFields[0][customFieldId]=42&customFields[0][value]=Financial` (or `customFields[0][name]=Category` in place of `customFieldId`). Multiple entries are AND-ed.
Array of objects (CustomRangeFieldFilterPublicV2Dto)

Filter Risks by Custom Field range. Repeat the parameter with bracketed indices, e.g. `customRangeFields[0][customFieldId]=99&customRangeFields[0][gte]=5&customRangeFields[0][lte]=10` (or `customRangeFields[0][name]=Score` in place of `customFieldId`). At least one of `gte` / `lte` must be supplied per entry. Multiple entries are AND-ed.

Responses

200

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

404
The specified Risk Register does not exist or is not accessible to the caller.

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/risk-registers/{riskRegisterId}/risks-search

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
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks-search`,
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

`{"data": [{"id": 1,"riskRegisterId": 1,"riskId": "AC-04","title": "Password Management - Password Cracking","description": "An attacker attempts to gain access to organizational information by guessing of passwords.","treatmentPlan": "UNTREATED","treatmentDetails": "string","anticipatedCompletionDate": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z","identifiedAt": "2025-07-01T16:45:55.246Z","impact": 0,"likelihood": 0,"score": 0,"residualImpact": 0,"residualLikelihood": 0,"residualScore": 0,"status": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","owners": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 3},"reviewers": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 2},"controls": {"data": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 4},"categories": {"data": [{"id": 1,"name": "Access Control","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 2},"documents": {"data": [{"id": 1,"name": "Risk Assessment Report Q4 2023.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 5},"notes": {"data": [{"id": 1,"comment": "This Risk has been reviewed and approved by the security team. Implementation timeline updated.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 6},"tickets": {"data": [{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 3},"tasks": {"data": [{"id": 1,"title": "Review quarterly security policies","description": "Conduct a comprehensive review of all security policies to ensure compliance with current regulations.","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 4},"customFields": {"data": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}],"totalCount": 8}}],"pagination": {"cursor": "string","totalCount": 0},"facets": {"status": [{"value": "ACTIVE","count": 42},{"value": "CLOSED","count": 8},{"value": "ARCHIVED","count": 3}]}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters)Search Risks

Search Risks across every Risk Register the caller can access, with rich filtering, sorting, faceting, and expand.

🔒 Requires **Risk Management: List Risks** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=sort&t=request)sort string (RiskSortEnum)

Which field to sort by. Highest residual risk first (with `sortDir=DESC`, the default) is the natural default view for a risk register.

Enum:"residualScore""inherentScore""anticipatedCompletionDate""completedDate""title""riskId""status""vendorName""createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data.

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=expand[]&t=request)expand[]Array of strings (RiskSearchExpandEnum)

List of subcollections and sub-objects to expand.

Items Enum:"owners""reviewers""controls""categories""documents""notes""tickets""tasks""customFields"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=facets[]&t=request)facets[]Array of strings (RiskSearchFacetEnum)

Filter dimensions for which to return value counts in the response. Only honored on the first page (when `cursor` is null). Each requested facet adds one terms aggregation to the search query. Bounded enum facets (`status`, `treatmentPlan`, `type`) are enriched with zero-count entries; unbounded facets (`categories`, `controls`, `ownerEmails`, `reviewerEmails`, `vendorNames`, `riskRegisterId`) return only values present in the result set.

Items Enum:"status""treatmentPlan""type""categories""controls""ownerEmails""reviewerEmails""vendorNames""riskRegisterId"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=q&t=request)q string<= 191 characters

Free-text search across `title`, `description`, `riskId`, and `vendorName`. Fuzzy, prefix-aware, and relevance-ranked.

Example:q=unauthorized access
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=status[]&t=request)status[]Array of strings (RiskStatusTypeEnum)

Filter Risks by one or more workflow statuses.

Items Enum:"ACTIVE""ARCHIVED""CLOSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=treatmentPlan[]&t=request)treatmentPlan[]Array of strings (RiskTreatmentPlanEnum)

Filter Risks by treatment plan.

Items Enum:"UNTREATED""ACCEPT""TRANSFER""AVOID""MITIGATE"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=type[]&t=request)type[]Array of strings (RiskVersionTypeEnum)

Filter Risks by version type.

Items Enum:"INTERNAL""EXTERNAL"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=inherentScoreGte&t=request)inherentScoreGte number

Filter Risks by minimum inherent (pre-mitigation) score (inclusive).
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=inherentScoreLte&t=request)inherentScoreLte number

Filter Risks by maximum inherent (pre-mitigation) score (inclusive).
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=residualScoreGte&t=request)residualScoreGte number

Filter Risks by minimum residual (post-mitigation) score (inclusive).
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=residualScoreLte&t=request)residualScoreLte number

Filter Risks by maximum residual (post-mitigation) score (inclusive).
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=anticipatedCompletionDateGte&t=request)anticipatedCompletionDateGte string<date-time>

Filter Risks with anticipated completion date on or after this ISO 8601 timestamp.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=anticipatedCompletionDateLte&t=request)anticipatedCompletionDateLte string<date-time>

Filter Risks with anticipated completion date on or before this ISO 8601 timestamp.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=completedDateGte&t=request)completedDateGte string<date-time>

Filter Risks with actual completion date on or after this ISO 8601 timestamp.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=completedDateLte&t=request)completedDateLte string<date-time>

Filter Risks with actual completion date on or before this ISO 8601 timestamp.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=categories[]&t=request)categories[]Array of strings

Filter Risks by exact category name membership.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=controls[]&t=request)controls[]Array of strings

Filter Risks by exact control code membership (e.g. `AC-04`).
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=ownerEmails[]&t=request)ownerEmails[]Array of strings

Filter Risks by owner email membership.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=reviewerEmails[]&t=request)reviewerEmails[]Array of strings

Filter Risks by reviewer email membership.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=vendorNames[]&t=request)vendorNames[]Array of strings

Filter Risks by exact vendor name membership. Each Risk is indexed with a single vendor name (the first associated vendor), so a Risk matches if that vendor name is in the supplied list.
Array of objects (CustomFieldFilterPublicV2Dto)

Filter Risks by Custom Field equality. Repeat the parameter with bracketed indices, e.g. `customFields[0][customFieldId]=42&customFields[0][value]=Financial` (or `customFields[0][name]=Category` in place of `customFieldId`). Multiple entries are AND-ed.
Array of objects (CustomRangeFieldFilterPublicV2Dto)

Filter Risks by Custom Field range. Repeat the parameter with bracketed indices, e.g. `customRangeFields[0][customFieldId]=99&customRangeFields[0][gte]=5&customRangeFields[0][lte]=10` (or `customRangeFields[0][name]=Score` in place of `customFieldId`). At least one of `gte` / `lte` must be supplied per entry. Multiple entries are AND-ed.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_searchRisksAcrossRegisters!in=query&path=riskRegisterId[]&t=request)riskRegisterId[]Array of numbers

Narrow results to Risks in this subset of Risk Registers. Each ID must be a Risk Register the caller can access; unknown or inaccessible IDs are silently filtered out. Omit to search every Risk Register accessible to the caller.

Responses

200

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

get/risks-search

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
    `https://public-api.drata.com/public/v2/risks-search`,
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

`{"data": [{"id": 1,"riskRegisterId": 1,"riskId": "AC-04","title": "Password Management - Password Cracking","description": "An attacker attempts to gain access to organizational information by guessing of passwords.","treatmentPlan": "UNTREATED","treatmentDetails": "string","anticipatedCompletionDate": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z","identifiedAt": "2025-07-01T16:45:55.246Z","impact": 0,"likelihood": 0,"score": 0,"residualImpact": 0,"residualLikelihood": 0,"residualScore": 0,"status": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","owners": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 3},"reviewers": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 2},"controls": {"data": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 4},"categories": {"data": [{"id": 1,"name": "Access Control","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 2},"documents": {"data": [{"id": 1,"name": "Risk Assessment Report Q4 2023.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 5},"notes": {"data": [{"id": 1,"comment": "This Risk has been reviewed and approved by the security team. Implementation timeline updated.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 6},"tickets": {"data": [{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 3},"tasks": {"data": [{"id": 1,"title": "Review quarterly security policies","description": "Conduct a comprehensive review of all security policies to ensure compliance with current regulations.","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 4},"customFields": {"data": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}],"totalCount": 8}}],"pagination": {"cursor": "string","totalCount": 0},"facets": {"status": [{"value": "ACTIVE","count": 42},{"value": "CLOSED","count": 8},{"value": "ARCHIVED","count": 3}]}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_getRisk)Get Risk

Get detail for a Risk item.

🔒 Requires **Risk Management: Get Risk** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_getRisk!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_getRisk!in=query&path=expand[]&t=request)expand[]Array of strings (RiskExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"owners""reviewers""controls""categories""documents""notes""tickets""tasks""customFields"

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

get/risk-registers/{riskRegisterId}/risks/{riskId}

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
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}`,
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
*   403
*   412
*   500

application/json

`{"id": 1,"riskId": "AC-04","title": "Password Management - Password Cracking","description": "An attacker attempts to gain access to organizational information by guessing of passwords.","treatmentPlan": "UNTREATED","treatmentDetails": "Implementing multi-factor authentication and password complexity requirements to reduce likelihood of successful password attacks.","anticipatedCompletionDate": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z","identifiedAt": "2025-07-01T16:45:55.246Z","impact": 5,"likelihood": 5,"score": 25,"residualImpact": 3,"residualLikelihood": 3,"residualScore": 9,"status": "ACTIVE","type": "INTERNAL","vendorId": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"categories": [{"id": 1,"name": "Access Control","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"owners": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"reviewers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"documents": [{"id": 1,"name": "Risk Assessment Report Q4 2023.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"notes": [{"id": 1,"comment": "This Risk has been reviewed and approved by the security team. Implementation timeline updated.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"tickets": [{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"tasks": [{"id": 1,"title": "Review quarterly security policies","description": "Conduct a comprehensive review of all security policies to ensure compliance with current regulations.","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"riskRegister": {"id": 1,"name": "Default Risk Register","description": "Primary risk register for organizational risks"},"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk)Update Risk

Update an existing Risk.

🔒 Requires **Risk Management: Update Risk** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=title&t=request)title string<= 191 characters

Descriptive title that clearly identifies the nature and scope of the Risk
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=description&t=request)description string<= 768 characters

Comprehensive description of the Risk scenario, potential threats, business impact, and affected systems or processes
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=identifiedAt&t=request)identifiedAt string or null<date>

Date when the Risk was first identified
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=impact&t=request)impact number[ 1 .. 10 ]

Impact score representing the potential severity of consequences if the Risk materializes. Default is 1-5 scale (where 5 is most severe), but can be configured from 3x3 up to 10x10 matrices.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=likelihood&t=request)likelihood number[ 1 .. 10 ]

Likelihood score representing the probability of the Risk occurring. Default is 1-5 scale (where 5 is most likely), but can be configured from 3x3 up to 10x10 matrices.
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=treatmentPlan&t=request)treatmentPlan string

Strategy for addressing the Risk: ACCEPT (acknowledge and monitor), AVOID (eliminate source), MITIGATE (reduce impact/likelihood), TRANSFER (shift to third party), UNTREATED (no action taken)

Enum:"UNTREATED""ACCEPT""TRANSFER""AVOID""MITIGATE"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=treatmentDetails&t=request)treatmentDetails string<= 30000 characters

Detailed explanation of the specific actions, controls, or measures being implemented to address the Risk. Cannot be provided when treatmentPlan is UNTREATED
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=anticipatedCompletionDate&t=request)anticipatedCompletionDate string<date>

Target date for completing Risk treatment activities. Cannot be provided when treatmentPlan is UNTREATED
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=completionDate&t=request)completionDate string<date>

Actual date when Risk treatment was completed. Cannot be provided when treatmentPlan is UNTREATED
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=residualImpact&t=request)residualImpact number[ 1 .. 10 ]

Residual impact score after treatment implementation. Default is 1-5 scale, but can be configured from 3x3 up to 10x10 matrices. Cannot be provided when treatmentPlan is UNTREATED
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=residualLikelihood&t=request)residualLikelihood number[ 1 .. 10 ]

Residual likelihood score after treatment implementation. Default is 1-5 scale, but can be configured from 3x3 up to 10x10 matrices. Cannot be provided when treatmentPlan is UNTREATED
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=status&t=request)status string

Current status of the Risk in the Risk management lifecycle

Enum:"ACTIVE""ARCHIVED""CLOSED"
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_updateRisk!path=vendorId&t=request)vendorId number or null

Identifier of the Vendor this Risk relates to. Supplying a Vendor makes this an external Risk; null detaches the Vendor and makes the Risk internal
Array of objects (CategoryRequestPublicV2Dto)

Risk categories for classification and reporting purposes
Array of objects (RiskOwnerRequestPublicV2Dto)

Users responsible for managing and monitoring this Risk
Array of objects (ReviewerRequestPublicV2Dto)

Users responsible for reviewing and approving Risk treatment plans. Cannot be provided when treatmentPlan is UNTREATED
Array of objects (RiskControlRequestPublicV2Dto)

Security controls implemented to mitigate this Risk
Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Risk

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

422
Unprocessable Entity

500
Internal server error

put/risk-registers/{riskRegisterId}/risks/{riskId}

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

`{"title": "Password Management - Weak Password Policies","description": "Weak password policies may allow unauthorized access to organizational systems and data through password-based attacks such as brute force, dictionary attacks, or credential stuffing.","identifiedAt": "2020-07-06","impact": 4,"likelihood": 3,"treatmentPlan": "MITIGATE","treatmentDetails": "Implement multi-factor authentication and enforce strong password complexity requirements across all systems.","anticipatedCompletionDate": "2020-07-06","completionDate": "2024-12-01","residualImpact": 2,"residualLikelihood": 2,"status": "ACTIVE","vendorId": 1,"categories": [{"id": 1}],"owners": [{"id": 1}],"reviewers": [{"id": 1}],"controls": [{"id": 1}],"customFields": [{"id": 1,"name": "Compliance Status","value": "Security & IT"}]}`

Response samples

*   200
*   400
*   401
*   403
*   404
*   412
*   422
*   500

3 more

application/json

`{"id": 1,"riskId": "AC-04","title": "Password Management - Password Cracking","description": "An attacker attempts to gain access to organizational information by guessing of passwords.","treatmentPlan": "UNTREATED","treatmentDetails": "Implementing multi-factor authentication and password complexity requirements to reduce likelihood of successful password attacks.","anticipatedCompletionDate": "2025-07-01T16:45:55.246Z","completionDate": "2025-07-01T16:45:55.246Z","identifiedAt": "2025-07-01T16:45:55.246Z","impact": 5,"likelihood": 5,"score": 25,"residualImpact": 3,"residualLikelihood": 3,"residualScore": 9,"status": "ACTIVE","type": "INTERNAL","vendorId": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"categories": [{"id": 1,"name": "Access Control","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"owners": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"reviewers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"documents": [{"id": 1,"name": "Risk Assessment Report Q4 2023.pdf","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"notes": [{"id": 1,"comment": "This Risk has been reviewed and approved by the security team. Implementation timeline updated.","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"tickets": [{"id": 1,"externalTicketId": "ENG-11245","isDone": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"tasks": [{"id": 1,"title": "Review quarterly security policies","description": "Conduct a comprehensive review of all security policies to ensure compliance with current regulations.","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"riskRegister": {"id": 1,"name": "Default Risk Register","description": "Primary risk register for organizational risks"},"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_deleteRisk)Delete Risk

Delete a Risk from the Risk Register.

🔒 Requires **risk-management-delete** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_deleteRisk!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1
required number or string

An integer Risk ID or string risk identifier (e.g., "RISK-001")

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

delete/risk-registers/{riskRegisterId}/risks/{riskId}

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
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/risks/${riskId}`,
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

## [](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_getRiskInsights)Get Risk Insights

Retrieve comprehensive analytics and dashboard data for a Risk Register, including risk posture distribution, treatment overview, risk heatmap, category breakdown, scoring progress, and historical trends over time.

🔒 Requires **Risk Management: List Insights** permission.

💎 Requires your account have the **Risk Management Pro** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_getRiskInsights!in=path&path=riskRegisterId&t=request)riskRegisterId
required number

The Risk Register ID

Example:1

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_getRiskInsights!in=query&path=ownerIds[]&t=request)ownerIds[]Array of numbers

Filter insights to only include Risks where the owner matches the specified user IDs

Example:ownerIds[]=1&ownerIds[]=2&ownerIds[]=3
[](https://developers.drata.com/openapi/reference/v2/tag/Risks/#tag/Risks/operation/RiskManagementPublicV2Controller_getRiskInsights!in=query&path=status[]&t=request)status[]Array of strings (RiskStatusTypeEnum)

Filter insights by Risk Status. ACTIVE includes Risks currently being managed, CLOSED includes Risks that have been resolved or accepted. ARCHIVED includes Risks that are not applicable to your organization.

Items Enum:"ACTIVE""ARCHIVED""CLOSED"

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

422
Unprocessable Entity

500
Internal server error

get/risk-registers/{riskRegisterId}/insights

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
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/risk-registers/${riskRegisterId}/insights`,
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
*   422
*   500

4 more

application/json

`{"riskPosture": [{"id": 1,"title": "Low","count": 5}],"treatmentOverview": {"untreated": 5,"accept": 23,"transfer": 13,"avoid": 10,"mitigate": 33},"riskHeatmap": [{"impact": 2,"likelihood": 2,"total": 11}],"categoryBreakdown": [{"category": {"id": 1,"name": "Access Control","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"severity": [{"id": 1,"title": "Low","count": 5}]}],"scored": 63,"remaining": 8,"riskOverTime": [{"period": "2026-02","severityCounts": [{"id": 1,"title": "Low","count": 5}]}]}`
