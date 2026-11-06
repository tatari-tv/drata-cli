Title: Controls

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Controls/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Controls are a strategic measure or safeguard that an organization puts in place to protect its assets and meet the requirements of specific compliance frameworks.

## [](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls)List Controls

Find Controls matching the provided filters.

🔒 Requires **Controls: List Controls** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=expand[]&t=request)expand[]Array of strings (ControlsExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"customFields""evidenceIds""flags""frameworkTags""owners""requirements""testIds""topics"
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=isMonitored&t=request)isMonitored boolean

Filter Controls that have or do not have a Monitor

Example:isMonitored=false
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=isReady&t=request)isReady boolean

Filter Controls that are or are not ready

Example:isReady=false
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=hasEvidence&t=request)hasEvidence boolean

Filter to Controls with or without Evidence

Example:hasEvidence=true
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=hasPolicy&t=request)hasPolicy boolean

Filter to Controls with or without Policy

Example:hasPolicy=true
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=hasPassingTest&t=request)hasPassingTest boolean

Filter to Controls with at least one passing Test

Example:hasPassingTest=true
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=ticketStatus&t=request)ticketStatus string

Only include Controls with an associated Task Management Ticket

Enum:"IN_PROGRESS""ARCHIVED"

Example:ticketStatus=0
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=policyId&t=request)policyId number

Filter Controls associated with a specific Policy ID

Example:policyId=123
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=isEnabled&t=request)isEnabled boolean

Filter Controls that are or are not enabled

Example:isEnabled=true
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=isArchived&t=request)isArchived boolean

Filter Controls that are or are not archived

Example:isArchived=false
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControls!in=query&path=requiredApprovals&t=request)requiredApprovals string (FilterRequiredApprovalsEnum)

Filter Controls by the status of their required approval. CHANGES_REQUESTED, NEEDS_APPROVAL and PREPARE_FOR_APPROVERS identify Controls that are not ready because their approval is still in progress; APPROVED and NO_REVIEW do not block readiness. Approval status only applies to enabled Controls, so this filter cannot be combined with isEnabled=false.

Enum:"PREPARE_FOR_APPROVERS""NEEDS_APPROVAL""APPROVED""CHANGES_REQUESTED""NO_REVIEW"

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

get/workspaces/{workspaceId}/controls

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls`,
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

`{"data": [{"id": "123","name": "Databases Monitored and Alarmed","code": "DCF-1002","slug": "databases-monitored-and-alarmed","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n             predetermined criteria. Incidents are escalated per policy.","question": "Does the organization implement tools to monitor its databases and notify appropriate personnel of incidents based on predetermined\n             criteria?","activity": "Ensure tools are implemented to monitor databases","archivedAt": "2025-07-01T16:45:55.246Z","rationale": "string","frameworkTags": ["SOC_2"],"topics": [1,2],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","controlTemplateId": "123","flags": {"hasEvidence": true,"hasPolicy": true,"isReady": "true","hasTicket": "true","hasOwner": false,"isMonitored": false},"owners": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"requirements": [{"createdAt": "2025-07-01T16:45:55.246Z","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkName": "SOC 2","frameworkPill": "SOC 2","frameworkSlug": "SOC 2","frameworkTag": "SOC_2","id": "58","name": "PI1.2","updatedAt": "2025-07-01T16:45:55.246Z","additionalInfo": "The entity demonstrates a commitment to integrity.","additionalInfo2": "The entity demonstrates a commitment to ethical values.","additionalInfo3": "The entity demonstrates a commitment to enforcing policies.","archivedAt": "2025-07-01T16:45:55.246Z","externalId": "SOC 2","longDescription": "The entity demonstrates a commitment to integrity and ethical values.","rationale": "This requirement is not needed."}],"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}],"testIds": {"passing": [1,2,3],"failing": [4],"monitored": [1,2,3,5]},"evidenceIds": {"validExternalEvidenceIds": [10,11],"invalidExternalEvidenceIds": [12],"validReportIds": [20,21],"invalidReportIds": [22]},"_links": {"self": {}}}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl)Create Control

Create a new custom Control

🔒 Requires **Controls: Create Control** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema:

multipart/form-data

required

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=question&t=request)question string<= 30000 characters

The question of the Control
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=activity&t=request)activity string<= 30000 characters

The activity of the Control
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=reportIds&t=request)reportIds Array of numbers

Array of Report IDs
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=policyIds&t=request)policyIds Array of numbers

Array of Policy IDs
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=requirementIds&t=request)requirementIds Array of numbers

Array of Requirement IDs
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=ownersIds&t=request)ownersIds Array of numbers

Array of User IDs to make owners of the Control
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=testIds&t=request)testIds Array of numbers

Array of Control Test IDs
Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Control.

💎 Requires your account have the Custom Fields and Formulas feature. Contact your Account Manager for help upgrading.
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=name&t=request)name
required string<= 191 characters

The name of the Control
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=description&t=request)description
required string<= 30000 characters

The description of the Control
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=code&t=request)code
required string<= 20 characters

The Control code
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=externalEvidence&t=request)externalEvidence Array of strings<binary>

External evidence files. Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .gif, .jpg, .jpeg, .png
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=externalEvidenceMetadata&t=request)externalEvidenceMetadata
required string

JSON string of metadata of uploaded evidence
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_createControl!ct=multipart/form-data&path=base64Files&t=request)base64Files
required string

JSON string with array of external evidence in Base64 format.

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

413
The file was too large to upload

500
Internal server error

503
Third party system was unavailable

post/workspaces/{workspaceId}/controls

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

multipart/form-data

No sample

Response samples

*   201
*   400
*   401
*   403
*   404
*   412
*   413
*   500
*   503

4 more

application/json

`{"id": "123","name": "Databases Monitored and Alarmed","code": "DCF-1002","slug": "databases-monitored-and-alarmed","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n             predetermined criteria. Incidents are escalated per policy.","question": "Does the organization implement tools to monitor its databases and notify appropriate personnel of incidents based on predetermined\n             criteria?","activity": "Ensure tools are implemented to monitor databases","archivedAt": "2025-07-01T16:45:55.246Z","rationale": "string","frameworkTags": ["SOC_2"],"topics": [1,2],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","controlTemplateId": "123","flags": {"hasEvidence": true,"hasPolicy": true,"isReady": "true","hasTicket": "true","hasOwner": false,"isMonitored": false},"owners": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"requirements": [{"createdAt": "2025-07-01T16:45:55.246Z","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkName": "SOC 2","frameworkPill": "SOC 2","frameworkSlug": "SOC 2","frameworkTag": "SOC_2","id": "58","name": "PI1.2","updatedAt": "2025-07-01T16:45:55.246Z","additionalInfo": "The entity demonstrates a commitment to integrity.","additionalInfo2": "The entity demonstrates a commitment to ethical values.","additionalInfo3": "The entity demonstrates a commitment to enforcing policies.","archivedAt": "2025-07-01T16:45:55.246Z","externalId": "SOC 2","longDescription": "The entity demonstrates a commitment to integrity and ethical values.","rationale": "This requirement is not needed."}],"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}],"testIds": {"passing": [1,2,3],"failing": [4],"monitored": [1,2,3,5]},"evidenceIds": {"validExternalEvidenceIds": [10,11],"invalidExternalEvidenceIds": [12],"validReportIds": [20,21],"invalidReportIds": [22]},"_links": {"self": {}}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControlById)Get Control

Get all the information for a specific Control

🔒 Requires **Controls: Get Control** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControlById!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
required number or string

An integer control ID or a control code prefixed with `code:`

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControlById!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControlById!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControlById!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControlById!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getControlById!in=query&path=expand[]&t=request)expand[]Array of strings (ControlExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"customFields""evidenceIds""flags""frameworkTags""owners""requirements""testIds""topics"

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

get/workspaces/{workspaceId}/controls/{controlId}

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
  const controlId = 'YOUR_controlId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls/${controlId}`,
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

`{"id": "123","name": "Databases Monitored and Alarmed","code": "DCF-1002","slug": "databases-monitored-and-alarmed","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n             predetermined criteria. Incidents are escalated per policy.","question": "Does the organization implement tools to monitor its databases and notify appropriate personnel of incidents based on predetermined\n             criteria?","activity": "Ensure tools are implemented to monitor databases","archivedAt": "2025-07-01T16:45:55.246Z","rationale": "string","frameworkTags": ["SOC_2"],"topics": [1,2],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","controlTemplateId": "123","flags": {"hasEvidence": true,"hasPolicy": true,"isReady": "true","hasTicket": "true","hasOwner": false,"isMonitored": false},"owners": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"requirements": [{"createdAt": "2025-07-01T16:45:55.246Z","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkName": "SOC 2","frameworkPill": "SOC 2","frameworkSlug": "SOC 2","frameworkTag": "SOC_2","id": "58","name": "PI1.2","updatedAt": "2025-07-01T16:45:55.246Z","additionalInfo": "The entity demonstrates a commitment to integrity.","additionalInfo2": "The entity demonstrates a commitment to ethical values.","additionalInfo3": "The entity demonstrates a commitment to enforcing policies.","archivedAt": "2025-07-01T16:45:55.246Z","externalId": "SOC 2","longDescription": "The entity demonstrates a commitment to integrity and ethical values.","rationale": "This requirement is not needed."}],"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}],"testIds": {"passing": [1,2,3],"failing": [4],"monitored": [1,2,3,5]},"evidenceIds": {"validExternalEvidenceIds": [10,11],"invalidExternalEvidenceIds": [12],"validReportIds": [20,21],"invalidReportIds": [22]},"_links": {"self": {}}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl)Modify Control

🔒 Requires **Controls: Update Control** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
required number or string

An integer control ID or a control code prefixed with `code:`

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!path=name&t=request)name string<= 191 characters

The name of the Control
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!path=description&t=request)description string<= 30000 characters

The description of the Control
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!path=question&t=request)question string<= 30000 characters

The question of the Control
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!path=code&t=request)code string<= 20 characters

The Control code
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!path=activity&t=request)activity string<= 30000 characters

The activity of the Control
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!path=reportIds&t=request)reportIds Array of numbers

Array of Report IDs
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!path=policyIds&t=request)policyIds Array of numbers

Array of Policy IDs
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!path=requirementIds&t=request)requirementIds Array of numbers

Array of Requirement IDs
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!path=ownersIds&t=request)ownersIds Array of numbers

Array of User IDs to make owners of the Control
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_modifyControl!path=testIds&t=request)testIds Array of numbers

Array of Control Test IDs
Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Control.

💎 Requires your account have the Custom Fields and Formulas feature. Contact your Account Manager for help upgrading.

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

put/workspaces/{workspaceId}/controls/{controlId}

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

`{"name": "Quarterly User Access Reviews","description": "Access to critical systems is reviewed on a quarterly basis to ensure that only authorized users retain access.","question": "A very good question","code": "DRA-69","activity": "On a quarterly schedule, generate access reports from identity providers.","reportIds": [1,2,3],"policyIds": [1,2,3],"requirementIds": [1,2,3],"ownersIds": [1,2,3],"testIds": [1,2,3],"customFields": [{"id": 1,"name": "Compliance Status","value": "Security & IT"}]}`

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

`{"id": "123","name": "Databases Monitored and Alarmed","code": "DCF-1002","slug": "databases-monitored-and-alarmed","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n             predetermined criteria. Incidents are escalated per policy.","question": "Does the organization implement tools to monitor its databases and notify appropriate personnel of incidents based on predetermined\n             criteria?","activity": "Ensure tools are implemented to monitor databases","archivedAt": "2025-07-01T16:45:55.246Z","rationale": "string","frameworkTags": ["SOC_2"],"topics": [1,2],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","controlTemplateId": "123","flags": {"hasEvidence": true,"hasPolicy": true,"isReady": "true","hasTicket": "true","hasOwner": false,"isMonitored": false},"owners": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"requirements": [{"createdAt": "2025-07-01T16:45:55.246Z","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkName": "SOC 2","frameworkPill": "SOC 2","frameworkSlug": "SOC 2","frameworkTag": "SOC_2","id": "58","name": "PI1.2","updatedAt": "2025-07-01T16:45:55.246Z","additionalInfo": "The entity demonstrates a commitment to integrity.","additionalInfo2": "The entity demonstrates a commitment to ethical values.","additionalInfo3": "The entity demonstrates a commitment to enforcing policies.","archivedAt": "2025-07-01T16:45:55.246Z","externalId": "SOC 2","longDescription": "The entity demonstrates a commitment to integrity and ethical values.","rationale": "This requirement is not needed."}],"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}],"testIds": {"passing": [1,2,3],"failing": [4],"monitored": [1,2,3,5]},"evidenceIds": {"validExternalEvidenceIds": [10,11],"invalidExternalEvidenceIds": [12],"validReportIds": [20,21],"invalidReportIds": [22]},"_links": {"self": {}}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements)List Requirements for Control

Find Control Requirements matching the provided filters.

🔒 Requires **Controls: Get Control** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements!in=path&path=controlId&t=request)controlId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements!in=query&path=frameworkTag&t=request)frameworkTag Array of strings

Filter data by controls associated with these framework tags

Items Enum:"NONE""SOC_2""ISO27001""CCPA""GDPR""HIPAA""PCI""SCF""NIST80053""NISTCSF""CMMC""NIST800171""MSSSPA""FFIEC""ISO27701""COBIT""SOX_ITGC""ISO270012022""CCM""CYBER_ESSENTIALS""ISO270172015""ISO270182019""FEDRAMP""NISTAI""PCI4""NISTCSF2""NIS2""DORA""ISO420012023""DRATA_ESSENTIALS""NIST800171R3""CIS8""CYBER_ESSENTIALS_32""FEDRAMP20X""HITRUST""MSSSPA11""ESSENTIAL_EIGHT""NYDFS""TISAX""ISO270182025""CCPA2026""ISO277012025""CPS230""CYFUN""AIUC_1""EU_CRA""AIUC_1_26Q3""AIUC_1_26Q4""CUSTOM""ALL"

Example:frameworkTag=SOC_2&frameworkTag=ISO27001
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements!in=query&path=excludeIds&t=request)excludeIds Array of numbers

Exclude requirements by array of IDs

Example:
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_getMappedRequirements!in=query&path=frameworkSlug&t=request)frameworkSlug string

Filter data by controls associated with these framework slugs. This parameter is intended to be used only for custom frameworks

Example:frameworkSlug=slug

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

get/workspaces/{workspaceId}/controls/{controlId}/requirements

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
  const controlId = 'YOUR_controlId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls/${controlId}/requirements`,
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

`{"data": [{"createdAt": "2025-07-01T16:45:55.246Z","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkName": "SOC 2","frameworkPill": "SOC 2","frameworkSlug": "SOC 2","frameworkTag": "SOC_2","id": "58","name": "PI1.2","updatedAt": "2025-07-01T16:45:55.246Z","additionalInfo": "The entity demonstrates a commitment to integrity.","additionalInfo2": "The entity demonstrates a commitment to ethical values.","additionalInfo3": "The entity demonstrates a commitment to enforcing policies.","archivedAt": "2025-07-01T16:45:55.246Z","externalId": "SOC 2","longDescription": "The entity demonstrates a commitment to integrity and ethical values.","rationale": "This requirement is not needed."}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_resetControlRequirementMappings)Reset Control Requirement Mappings

Reset multiple controls to their original requirement mappings from control templates. Custom controls cannot be reset. Returns success/failure status for each control.

🔒 Requires **Controls: Create Control** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_resetControlRequirementMappings!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_resetControlRequirementMappings!path=controlIds&t=request)controlIds
required Array of numbers<= 100 items

Array of control IDs to reset requirement mappings for (max 100)

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

post/workspaces/{workspaceId}/controls/action-reset-mappings

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

`{"controlIds": [123,456,789]}`

Response samples

*   200
*   400
*   401
*   403
*   412
*   500

1 more

application/json

`{"results": [{"controlId": 123,"success": true,"error": "Cannot reset requirement mappings for Custom Control"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_performControlAction)Perform Control Action

Perform an action on a Control: `markOutOfScope` archives it, `markInScope` restores it. Returns 204 with no body when the action is already in the desired state.

🔒 Requires **Controls: Update Control** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_performControlAction!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_performControlAction!in=path&path=controlId&t=request)controlId
required number

The Control ID

Example:123

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_performControlAction!path=action&t=request)action
required string

Action to perform: `markOutOfScope` archives the control, `markInScope` restores it

Enum:"markInScope""markOutOfScope"
[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_performControlAction!path=rationale&t=request)rationale string<= 191 characters

Required when action is `markOutOfScope`. Business rationale for archiving the control.

Responses

200
Successful

204
Control is already in the requested state; no change was made.

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

post/workspaces/{workspaceId}/controls/{controlId}/actions

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

`{"action": "markOutOfScope","rationale": "string"}`

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

`{"id": "123","name": "Databases Monitored and Alarmed","code": "DCF-1002","slug": "databases-monitored-and-alarmed","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n             predetermined criteria. Incidents are escalated per policy.","question": "Does the organization implement tools to monitor its databases and notify appropriate personnel of incidents based on predetermined\n             criteria?","activity": "Ensure tools are implemented to monitor databases","archivedAt": "2025-07-01T16:45:55.246Z","rationale": "string","frameworkTags": ["SOC_2"],"topics": [1,2],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","controlTemplateId": "123","flags": {"hasEvidence": true,"hasPolicy": true,"isReady": "true","hasTicket": "true","hasOwner": false,"isMonitored": false},"owners": {"data": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"totalCount": 15},"requirements": [{"createdAt": "2025-07-01T16:45:55.246Z","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkName": "SOC 2","frameworkPill": "SOC 2","frameworkSlug": "SOC 2","frameworkTag": "SOC_2","id": "58","name": "PI1.2","updatedAt": "2025-07-01T16:45:55.246Z","additionalInfo": "The entity demonstrates a commitment to integrity.","additionalInfo2": "The entity demonstrates a commitment to ethical values.","additionalInfo3": "The entity demonstrates a commitment to enforcing policies.","archivedAt": "2025-07-01T16:45:55.246Z","externalId": "SOC 2","longDescription": "The entity demonstrates a commitment to integrity and ethical values.","rationale": "This requirement is not needed."}],"customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}],"testIds": {"passing": [1,2,3],"failing": [4],"monitored": [1,2,3,5]},"evidenceIds": {"validExternalEvidenceIds": [10,11],"invalidExternalEvidenceIds": [12],"validReportIds": [20,21],"invalidReportIds": [22]},"_links": {"self": {}}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_compareControlRequirements)Compare Control Requirements

Compare requirements for multiple controls between tenant mappings and global template mappings.

🔒 Requires **Controls: Get Control** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_compareControlRequirements!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Controls/#tag/Controls/operation/ControlsPublicV2Controller_compareControlRequirements!in=query&path=controlIds[]&t=request)controlIds[]
required Array of numbers[ 1 .. 300 ] items

Array of Control IDs to compare requirements for

Example:controlIds[]=1&controlIds[]=2&controlIds[]=3

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

get/workspaces/{workspaceId}/controls-requirement-comparison

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
  const query = new URLSearchParams({'controlIds[]': '1,2,3'}).toString();

  const workspaceId = 'YOUR_workspaceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/controls-requirement-comparison?${query}`,
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

`{"data": [{"controlId": 123,"alignedRequirements": [{"id": 123,"name": "CC1.1","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkTag": "SOC_2","isOutOfScope": false}],"userMappedRequirements": [{"id": 123,"name": "CC1.1","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkTag": "SOC_2","isOutOfScope": false}],"templateMappedRequirements": [{"id": 123,"name": "CC1.1","description": "The entity demonstrates a commitment to integrity and ethical values.","frameworkTag": "SOC_2","isOutOfScope": false}]}]}`
