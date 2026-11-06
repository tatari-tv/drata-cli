Title: Vendor Security Reviews

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Vendor-Security-Reviews/

Markdown Content:
API V2
API Documentation
API Reference
Drata API
Access Reviews
Assets
Audit Requests
Audits
Background Checks
Company
Compliance Check Exclusions
Control Notes
Control Owners
Control Tickets
Controls
Custom Connections
Custom Data Records
Custom Field Definitions
Device Documents
Devices
Events
Evidence
Evidence Library
Frameworks
Groups
HRIS User Identities
Infrastructure User Identities
Monitoring Test Tickets
Monitoring Tests
Personnel
Policies
Policy Languages
Procurement Connection Mappings
Risk Documents
Risk Library
Risk Notes
Risk Registers
Risk Tickets
Risks
Standard Connections
Tasks
Uploads
User Documents
User's Assigned Policies
Users and Roles
Vendor Criteria
Vendor Criteria Creations
Vendor Documents
Vendor Questionnaire Templates
Vendor Risk Levels
Vendor Security Reviews
GET
List Assessments 🧪
GET
Get Assessment 🧪
GET
Get Assessments Stats 🧪
GET
List Observations 🧪
POST
Create Observation 🧪
GET
Get Observation 🧪
PUT
Update Observation 🧪
DELETE
Delete Observation 🧪
GET
Get Security Review Residual Risk 🧪
GET
List Notes 🧪
POST
Create Note 🧪
GET
Get Note 🧪
PUT
Update Note 🧪
DELETE
Delete Note 🧪
GET
Get Vendor Security Review Workflow 🧪
GET
List Vendor Security Reviews
POST
Create Vendor Security Review
POST
Create Vendor Security Review with File
GET
List Vendor Security Reviews Across Vendors
GET
Get Vendor Security Review 🧪
PUT
Update Vendor Security Review 🧪
GET
Get Vendor Security Review Summary 🧪
POST
Upload Security Questionnaire
GET
List Security Review Questionnaires 🧪
POST
Upload Security Questionnaire
GET
List Security Review Actions 🧪
POST
Perform Security Review Action 🧪
POST
Start Agent Review on Security Review 🧪
POST
Confirm Agent Review Criteria 🧪
POST
Start Agent Review Assessment 🧪
DELETE
Stop Agent Review Assessment 🧪
POST
Generate Agent Review Follow-up Questionnaire 🧪
GET
Get Agent Review Follow-up Questionnaire 🧪
POST
Send Agent Review Follow-up Questionnaire 🧪
POST
Finalize Agent Review 🧪
POST
Add Agent Review Documents 🧪
POST
Collect Agent Review Public Trust Center Documents 🧪
GET
Get Agent Review Trust Center Access Request Requirements 🧪
POST
Request Agent Review Trust Center Access 🧪
POST
Provide Agent Review Trust Center Access Link 🧪
POST
Skip Agent Review Trust Center Access Request 🧪
Vendor Types
Vendors
Version Control User Identities
Workspaces
MCP Server
Recipes
Overview
Create an API key
Add MFA on IdP Evidence
Add Completed Security Training Evidence
Add HIPAA Training Evidence
Add Password Manager Evidence
Add Hard Drive Encryption Evidence
Add Anti-Virus Evidence
Add Auto Updates Evidence
Add Screensaver Lock Evidence
Work with Custom Fields
Work with Custom Connections
Skills
Agent Skills
Start Here
What needs attention today
Find the right skill
Compliance & Audit Readiness
Framework readiness
Control readiness
Controls needing work
Fix controls
Monitoring coverage
Evidence freshness
Evidence needing work
Fix evidence
Risk Management
Risk posture
Risks needing work
Fix risks
Third-Party & Vendor Risk
Vendor portfolio
Vendors needing review
Fix vendors
Personnel & Access Compliance
Workforce compliance
Reporting & Stakeholder Communications
Executive briefing
Custom Workflows
Overview
Notify Current Employees When Non-Compliant / Failure
Notify Admin When Personnel Is Out of Compliance by 5 Days
Notify and Create Task When Risk Residual Score Changes to a Higher Level
Notify Reminder to the Control Owner on Evidence Past Due
Notify Control Owners When Evidence Renewals Are Approaching
Create Task and Notify When Control Changes to "Not Ready"
Notify Control Owners When Evidence Is Unlinked From Their Control
Notify When Control Owner Is Updated
Notify Control Owner When an Artifact Is Added to Evidence
Create Task for Control Approvers to Review Newly Linked Evidence
Review Updated Risk Treatment Plan
Vendor Security Reviews

Vendor Security Reviews track the status of security reviews for Vendors. You can create a security review, upload questionnaires, and track the progress of the review. Each review produces Assessments, the per-criterion findings that carry an analysis summary, status, and severity. The help docs have more information.

List Assessments 🧪

Get a paginated list of Assessments for a given Vendor Security Review.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: List Assessments permission.

💎 Requires your account have the TPRM Agentic Assessment, AI Data Share feature. Contact your Account Manager for help upgrading.

🔒 Requires the company's AI features setting turned on. Otherwise it returns 403 AI_FORBIDDEN.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.

QUERY PARAMETERS
cursor
string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a pagination.cursor value that can be used in the subsequent request to retrieve the next page of results



size
number [ 1 .. 500 ]
Default: 50

Number of results to return



sort
string (SortTypeLimitedEnum)

Which field to sort by

Enum: "createdAt" "updatedAt"


sortDir
string (SortDirectionEnum)

The direction to sort the data

Enum: "ASC" "DESC"


includeTotalCount
boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example: includeTotalCount=false


criteriaName
string

Filters by criteria name prefix: returns Assessments whose criteria name starts with this text.

Example: criteriaName=security criteria


statuses[]
Array of strings (VendorAssessmentStatusEnum)

Filter Assessments by one or more current statuses.

Items Enum: "MET" "NOT_MET" "INCONCLUSIVE" "N_A" "PARTIALLY_MET"


severities[]
Array of strings (VendorAssessmentCriterionSeverityEnum)

Filter Assessments by one or more criterion severities.

Items Enum: "LOW" "MEDIUM" "HIGH"
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

You are not allowed to perform this action AI features are not enabled for this account.

404

The Vendor has no Vendor Security Review with the given securityReviewId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/assessments`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"data":
[
{
"id": 1,
"vendorSecurityReviewId": 5,
"vendorAssessmentCriteriaId": 123,
"criteriaName": "My Criteria",
"analysisSummary": "Analysis summary content",
"originalAgentStatus": "MET",
"currentStatus": "MET",
"severity": "HIGH",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
}
],
"pagination":
{
"cursor": "string",
"totalCount": 0
}
}
Get Assessment 🧪

Get one Assessment of a Vendor Security Review, including the Vendor Document excerpts the agent cited and the criterion requirements it tested.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Get Assessment permission.

💎 Requires your account have the TPRM Agentic Assessment, AI Data Share feature. Contact your Account Manager for help upgrading.

🔒 Requires the company's AI features setting turned on. Otherwise it returns 403 AI_FORBIDDEN.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.



assessmentId
required

number

The numeric ID of the Assessment.

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

You are not allowed to perform this action AI features are not enabled for this account.

404

The Vendor has no Vendor Security Review with the given securityReviewId, or the review has no Assessment with the given assessmentId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments/{assessmentId}
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const assessmentId = 'YOUR_assessmentId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/assessments/${assessmentId}`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"vendorSecurityReviewId": 5,
"vendorAssessmentCriteriaId": 123,
"criteriaName": "My Criteria",
"analysisSummary": "Analysis summary content",
"originalAgentStatus": "MET",
"currentStatus": "MET",
"severity": "HIGH",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"sources":
[
{
"documentId": 42,
"documentName": "SOC 2 Type II Report.pdf",
"documentExcerpt": "All customer data is encrypted at rest using AES-256.",
"pageNumber": 12,
"referencedOn": "2025-07-01T16:45:55.246Z"
}
],
"requirements":
[
{
"question": "Is customer data encrypted at rest?",
"explanation": "The SOC 2 report states that customer data is encrypted at rest with AES-256.",
"status": "MET",
"sources":
[
{
"documentId": 42,
"documentName": "SOC 2 Type II Report.pdf",
"documentExcerpt": "All customer data is encrypted at rest using AES-256.",
"pageNumber": 12,
"referencedOn": "2025-07-01T16:45:55.246Z"
}
]
}
]
}
Get Assessments Stats 🧪

Count the Assessments of a Vendor Security Review by current status: met, not met, partially met, inconclusive and not applicable.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Get Assessments Statistics permission.

💎 Requires your account have the TPRM Agentic Assessment, AI Data Share feature. Contact your Account Manager for help upgrading.

🔒 Requires the company's AI features setting turned on. Otherwise it returns 403 AI_FORBIDDEN.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.

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

You are not allowed to perform this action AI features are not enabled for this account.

404

The Vendor has no Vendor Security Review with the given securityReviewId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments-stats
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/assessments-stats`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
Copy
{
"met": 30,
"notMet": 5,
"partiallyMet": 4,
"inconclusive": 3,
"notApplicable": 1,
"total": 44
}
List Observations 🧪

Get a paginated list of Observations for a given Vendor Security Review.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Get Security Review Observations permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.

QUERY PARAMETERS
cursor
string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a pagination.cursor value that can be used in the subsequent request to retrieve the next page of results



size
number [ 1 .. 500 ]
Default: 50

Number of results to return



sort
string (SortTypeLimitedEnum)

Which field to sort by

Enum: "createdAt" "updatedAt"


sortDir
string (SortDirectionEnum)

The direction to sort the data

Enum: "ASC" "DESC"


includeTotalCount
boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example: includeTotalCount=false


vendorAssessmentId
number >= 1

Filter by the ID of the Vendor Assessment the Observations are related to

Example: vendorAssessmentId=345


expand[]
Array of strings (VendorSecurityReviewObservationExpandEnum)

List of sub-objects to expand

Items Value: "user"
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

The Vendor has no Vendor Security Review with the given securityReviewId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/observations
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/observations`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"data":
[
{
"id": 1,
"securityReviewId": 1,
"observation": "Vendor provided a SOC 2 Type II report covering all five trust service criteria.",
"source": "SOC 2 Type II report",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"userId": 1,
"vendorAssessmentId": 1,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
}
}
],
"pagination":
{
"cursor": "string",
"totalCount": 0
}
}
Create Observation 🧪

Record an Observation against a given Vendor Security Review, optionally related to the assessment of one of its criteria through vendorAssessmentId. A Vendor Security Review holds at most 250 Observations and Notes together; past it the request fails with 400.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Create Security Review Observation permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.

REQUEST BODY SCHEMA: application/json
required
observation
required

string <= 30000 characters

The free-text finding to record against the Vendor Security Review.



source
string <= 500 characters

Where the Observation was drawn from, such as the document or questionnaire it references.



vendorAssessmentId
number >= 1

ID of the Vendor Assessment to relate the Observation to: the assessment of one criterion of this Vendor Security Review, as listed by the Assessments endpoint. Fails with 422 when it is not an assessment of this Vendor Security Review.

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

The Vendor has no Vendor Security Review with the given securityReviewId

412

You must accept the Drata terms and conditions to use the API

422

The vendorAssessmentId is not an assessment of this Vendor Security Review.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/observations
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{
"observation": "No evidence of annual penetration testing.",
"source": "SOC 2 Type II report, section 4",
"vendorAssessmentId": 345
}
Response samples
201
400
401
402
403
4 more
404
412
422
500
4 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"securityReviewId": 1,
"observation": "Vendor provided a SOC 2 Type II report covering all five trust service criteria.",
"source": "SOC 2 Type II report",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"userId": 1,
"vendorAssessmentId": 1,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
}
}
Get Observation 🧪

Get an Observation of a given Vendor Security Review.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Get Security Review Observations permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.



observationId
required

number

The numeric ID of the Observation.

QUERY PARAMETERS
expand[]
Array of strings (VendorSecurityReviewObservationExpandEnum)

List of sub-objects to expand

Items Value: "user"
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

The Vendor has no Vendor Security Review with the given securityReviewId, or the Vendor Security Review has no Observation with the given observationId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const observationId = 'YOUR_observationId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/observations/${observationId}`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"securityReviewId": 1,
"observation": "Vendor provided a SOC 2 Type II report covering all five trust service criteria.",
"source": "SOC 2 Type II report",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"userId": 1,
"vendorAssessmentId": 1,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
}
}
Update Observation 🧪

Update the text or source of an Observation of a given Vendor Security Review. Omitted fields keep their value; a body with no field returns 204 and changes nothing.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Security Review Observation permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.



observationId
required

number

The numeric ID of the Observation.

REQUEST BODY SCHEMA: application/json
required
observation
string <= 30000 characters

The new free-text finding. Omit it to keep the current one; null and an empty string are rejected.



source
string or null <= 500 characters

The new source of the Observation. Omit it to keep the current one; send null to clear it. An empty string is rejected.

Responses
200

Successful

204

No Content

400

Malformed data and/or validation errors

401

Invalid Authorization

402

You must upgrade your plan to use this feature

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the user is not the author of the Observation: only its author can update it.

404

The Vendor has no Vendor Security Review with the given securityReviewId, or the Vendor Security Review has no Observation with the given observationId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

PUT
/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{
"observation": "No evidence of annual penetration testing in the last 24 months.",
"source": "SOC 2 Type II report, section 4"
}
Response samples
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"securityReviewId": 1,
"observation": "Vendor provided a SOC 2 Type II report covering all five trust service criteria.",
"source": "SOC 2 Type II report",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"userId": 1,
"vendorAssessmentId": 1,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
}
}
Delete Observation 🧪

Delete an Observation of a given Vendor Security Review.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Delete Security Review Observation permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.



observationId
required

number

The numeric ID of the Observation.

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

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the user is not the author of the Observation: only its author can delete it.

404

The Vendor has no Vendor Security Review with the given securityReviewId, or the Vendor Security Review has no Observation with the given observationId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

DELETE
/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const observationId = 'YOUR_observationId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/observations/${observationId}`,
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
400
401
402
403
404
2 more
412
500
2 more
application/json
Copy
{
"name": "string",
"statusCode": 0,
"message": "string",
"code": 0
}
Get Security Review Residual Risk 🧪

Return the Recommended Residual Risk evaluation for a Vendor Security Review: the deterministic score and Risk level, the confidence flag, and the supporting breakdown and persisted reasoning.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Get Security Review Residual Risk permission.

💎 Requires your account have the TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.

Responses
200

Successful

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account is not on the TPRM Pro Max plan.

403

The caller lacks the required permission or OAuth scope, has restricted vendor access that excludes this Vendor, or the feature is not enabled for the account.

404

The Vendor has no Security Review with the given securityReviewId.

412

You must accept the Drata terms and conditions to use the API

422

Unprocessable Entity

500

Internal server error

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/residual-risk
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/residual-risk`,
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
200
400
401
402
403
4 more
404
412
422
500
4 more
application/json
CopyExpand allCollapse all
{
"score": 0.67,
"recommendedRiskLevel":
{
"id": 3,
"name": "Major",
"position": 3
},
"confidence": "HIGH",
"exceedsInherentRisk": false,
"breakdown":
{
"conclusiveCount": 8,
"inconclusiveCount": 2,
"notApplicableCount": 1,
"scoredCount": 8,
"totalApplicableCount": 10,
"inconclusiveShare": 0.2,
"thresholds":
[
0.25,
0.5,
0.75,
1
],
"weightedGapTotal": 8,
"totalPossiblePoints": 12
},
"reason": "AWS is a Tier 1 vendor, so inherent risk is High. Of 10 applicable criteria, 8 are conclusive (5 met, 3 with gaps) and 2 are inconclusive and excluded. Strong, independently attested MFA and encryption controls support a recommended residual of Major.",
"confidenceCaveat": false,
"reasonGeneratedAt": "2025-07-01T16:45:55.246Z",
"stale": false
}
List Notes 🧪

Get a paginated list of Internal Notes for a given Vendor Security Review.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Get Security Review Notes permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.

QUERY PARAMETERS
cursor
string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a pagination.cursor value that can be used in the subsequent request to retrieve the next page of results



size
number [ 1 .. 500 ]
Default: 50

Number of results to return



sort
string (SortTypeLimitedEnum)

Which field to sort by

Enum: "createdAt" "updatedAt"


sortDir
string (SortDirectionEnum)

The direction to sort the data

Enum: "ASC" "DESC"


includeTotalCount
boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example: includeTotalCount=false


vendorAssessmentId
number >= 1

Filter by the ID of the Vendor Assessment the Notes are related to

Example: vendorAssessmentId=345


expand[]
Array of strings (VendorSecurityReviewObservationExpandEnum)

List of sub-objects to expand

Items Value: "user"
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

The Vendor has no Vendor Security Review with the given securityReviewId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/notes
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/notes`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"data":
[
{
"id": 1,
"securityReviewId": 1,
"note": "Procurement approved the exception until the next renewal.",
"source": "Procurement ticket PROC-1234",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"userId": 1,
"vendorAssessmentId": 345,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
}
}
],
"pagination":
{
"cursor": "string",
"totalCount": 0
}
}
Create Note 🧪

Record an Internal Note against a given Vendor Security Review, optionally related to the assessment of one of its criteria through vendorAssessmentId. A Vendor Security Review holds at most 250 Observations and Notes together; past it the request fails with 400.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Create Security Review Note permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.

REQUEST BODY SCHEMA: application/json
required
note
required

string <= 30000 characters

The free-text Internal Note to record against the Vendor Security Review.



source
string <= 500 characters

Where the Note was drawn from, such as the document or system it references.



vendorAssessmentId
number >= 1

ID of the Vendor Assessment to relate the Note to: the assessment of one criterion of this Vendor Security Review, as listed by the Assessments endpoint. Fails with 422 when it is not an assessment of this Vendor Security Review.

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

The Vendor has no Vendor Security Review with the given securityReviewId

412

You must accept the Drata terms and conditions to use the API

422

The vendorAssessmentId is not an assessment of this Vendor Security Review.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/notes
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{
"note": "Procurement approved the exception until the next renewal.",
"source": "Procurement ticket PROC-1234",
"vendorAssessmentId": 345
}
Response samples
201
400
401
402
403
4 more
404
412
422
500
4 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"securityReviewId": 1,
"note": "Procurement approved the exception until the next renewal.",
"source": "Procurement ticket PROC-1234",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"userId": 1,
"vendorAssessmentId": 345,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
}
}
Get Note 🧪

Get an Internal Note of a given Vendor Security Review.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Get Security Review Notes permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.



securityReviewNoteId
required

number

The numeric ID of the Note.

QUERY PARAMETERS
expand[]
Array of strings (VendorSecurityReviewObservationExpandEnum)

List of sub-objects to expand

Items Value: "user"
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

The Vendor has no Vendor Security Review with the given securityReviewId, or the Vendor Security Review has no Note with the given securityReviewNoteId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const securityReviewNoteId = 'YOUR_securityReviewNoteId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/notes/${securityReviewNoteId}`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"securityReviewId": 1,
"note": "Procurement approved the exception until the next renewal.",
"source": "Procurement ticket PROC-1234",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"userId": 1,
"vendorAssessmentId": 345,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
}
}
Update Note 🧪

Update the text or source of an Internal Note of a given Vendor Security Review. Omitted fields keep their value; a body with no field returns 204 and changes nothing.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Security Review Note permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.



securityReviewNoteId
required

number

The numeric ID of the Note.

REQUEST BODY SCHEMA: application/json
required
note
string <= 30000 characters

The new Internal Note text. Omit it to keep the current one; null and an empty string are rejected.



source
string or null <= 500 characters

The new source of the Note. Omit it to keep the current one; send null to clear it. An empty string is rejected.

Responses
200

Successful

204

No Content

400

Malformed data and/or validation errors

401

Invalid Authorization

402

You must upgrade your plan to use this feature

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the user is not the author of the Note: only its author can update it.

404

The Vendor has no Vendor Security Review with the given securityReviewId, or the Vendor Security Review has no Note with the given securityReviewNoteId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

PUT
/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{
"note": "Procurement extended the exception for one more quarter.",
"source": "Procurement ticket PROC-1234"
}
Response samples
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"securityReviewId": 1,
"note": "Procurement approved the exception until the next renewal.",
"source": "Procurement ticket PROC-1234",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"userId": 1,
"vendorAssessmentId": 345,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
}
}
Delete Note 🧪

Delete an Internal Note of a given Vendor Security Review.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Delete Security Review Note permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Vendor Security Review.



securityReviewNoteId
required

number

The numeric ID of the Note.

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

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the user is not the author of the Note: only its author can delete it.

404

The Vendor has no Vendor Security Review with the given securityReviewId, or the Vendor Security Review has no Note with the given securityReviewNoteId

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

DELETE
/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const securityReviewNoteId = 'YOUR_securityReviewNoteId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/notes/${securityReviewNoteId}`,
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
400
401
402
403
404
2 more
412
500
2 more
application/json
Copy
{
"name": "string",
"statusCode": 0,
"message": "string",
"code": 0
}
Get Vendor Security Review Workflow 🧪

Get the vendor assessment agent workflow for a Security Review. Returns a null workflow when no assessment workflow exists for the review.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Get Security Review Workflow permission.

💎 Requires your account have the TPRM Agentic Assessment, AI Data Share feature. Contact your Account Manager for help upgrading.

🔒 Requires the company's AI features setting turned on. Otherwise it returns 403 AI_FORBIDDEN.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

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

You are not allowed to perform this action AI features are not enabled for this account.

404

Not Found

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/workflow
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/workflow`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"workflow":
{
"id": 1,
"workspaceId": 1,
"vendorSecurityReviewId": 456,
"status": "NOT_STARTED",
"progress": 50,
"auditTrail":
[
{
"key": "PROCESS_DOCUMENTS",
"name": "Security review started",
"status": "COMPLETED",
"date": "2025-07-01T16:45:55.246Z"
}
],
"documents":
{
"data":
[
{
"id": 123,
"name": "SOC 2 Type II Report.pdf",
"type": "DOCUMENT",
"status": "PROCESSED"
}
],
"totalCount": 3
},
"runs":
{
"data":
[
{
"id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
"status": "COMPLETED",
"stats":
{
"met": 15,
"notMet": 3,
"inconclusive": 2,
"partiallyMet": 1,
"overallSecurityPosture": "Met"
},
"startedAt": "2025-07-01T16:45:55.246Z",
"completedAt": "2025-07-01T16:45:55.246Z",
"isPublicDocsOnly": true
}
],
"totalCount": 2
}
}
}
List Vendor Security Reviews

Get a paginated list of all Security Reviews for a given Vendor.

🔒 Requires Vendors: List Vendors permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number
QUERY PARAMETERS
cursor
string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a pagination.cursor value that can be used in the subsequent request to retrieve the next page of results



size
number [ 1 .. 500 ]
Default: 50

Number of results to return



sort
string (SortTypeLimitedEnum)

Which field to sort by

Enum: "createdAt" "updatedAt"


sortDir
string (SortDirectionEnum)

The direction to sort the data

Enum: "ASC" "DESC"


includeTotalCount
boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example: includeTotalCount=false


status[]
Array of strings (VendorSecurityReviewStatusEnum)

Filter by security review status

Items Enum: "NOT_YET_STARTED" "IN_PROGRESS" "COMPLETED" "NOT_REQUIRED"


type[]
Array of strings (VendorSecurityReviewTypeEnum)

Filter by security review type

Items Enum: "SECURITY" "SOC_REPORT" "UPLOAD_REPORT"


decision[]
Array of strings (VendorSecurityReviewDecisionEnum)

Filter by security review decision

Items Enum: "PENDING" "APPROVED" "APPROVED_WITH_CONDITIONS" "REJECTED"


expand[]
Array of strings (VendorSecurityReviewExpandEnum)

List of subcollections and sub-objects to expand

Items Enum: "user" "requesterUser"


createdAtFrom
string <date>

Filter to Security Reviews created on or after this value (inclusive).

Example: createdAtFrom=2020-07-06


createdAtTo
string <date>

Filter to Security Reviews created on or before this value (inclusive).

Example: createdAtTo=2020-07-06


updatedAtFrom
string <date>

Filter to Security Reviews last updated on or after this value (inclusive)

Example: updatedAtFrom=2020-07-06


updatedAtTo
string <date>

Filter to Security Reviews last updated on or before this value (inclusive).

Example: updatedAtTo=2020-07-06
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

GET
/vendors/{vendorId}/security-reviews
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"data":
[
{
"id": 1,
"requestedAt": "2025-07-01T16:45:55.246Z",
"reviewDeadlineAt": "2025-07-01T16:45:55.246Z",
"decision": "APPROVED",
"title": "string",
"note": "string",
"status": "NOT_YET_STARTED",
"type": "SECURITY",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"vendorId": 1,
"vendor":
{
"id": 1,
"name": "Acme"
},
"userId": 0,
"requesterUserId": 0,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"requesterUser":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"_links":
{
"self":
{
"href": "https://app.drata.com/risk/vendors/profile/1/overview"
}
},
"socReviewForm":
{
"readyToFinalize": false,
"reviewerInformation":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reviewer": "Jane Doe",
"reportIssueDate": "2020-07-06"
},
"complianceScope":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"certification": "SOC_2",
"scopeType": "TYPE_2",
"auditPeriod": "2020-07-06",
"auditPeriodRange":
{
"start": "2020-07-06",
"end": "2020-07-06"
},
"trustServiceCriteria":
[
"AVAILABILITY"
]
},
"reportOpinion":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "Schedule a follow-up review in 90 days."
},
"findings":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"findings":
[
{
"description": "Control objective was not met during the audit period."
}
],
"hasMaterialImpact": false
},
"endUserControls":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"userControls":
[
{
"description": "Users must enforce strong password policies.",
"inPlace": true
}
]
},
"servicesAndLocations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"services":
[
{
"name": "Cloud Hosting"
}
],
"locations":
[
{
"city": "Austin",
"stateOrCountry": "TX"
}
]
},
"cpaFirm":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"cpaFirm": "Deloitte",
"cpaProcedurePerformed": "Assessed reputation via public records."
},
"subserviceOrganizations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"subserviceOrganization": "SubCo Inc.",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "Reviewed available SOC reports."
}
}
}
],
"pagination":
{
"cursor": "string",
"totalCount": 0
}
}
Create Vendor Security Review

Create a new Security Review for a given Vendor.

🔒 Requires Vendors: Create Security Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number
REQUEST BODY SCHEMA: application/json
required
reviewDeadlineAt
required

string <date-time>

Vendor Security Review deadline date



securityReviewStatus
required

string

The status of the Security Review

Enum: "NOT_YET_STARTED" "IN_PROGRESS" "COMPLETED" "NOT_REQUIRED"


securityReviewType
required

string

The type of the Security Review

Enum: "SECURITY" "SOC_REPORT" "UPLOAD_REPORT"


requestedAt
string <date-time>

Vendor Security Review requested date



note
string or null <= 1000 characters

Vendor Security Review note



requesterUserId
number or null <= 1000000000

Optional. The user ID of the person that requested the Security Review. If not provided, defaults to: (1) the vendor's security owner if assigned and active, or (2) the platform admin user.



title
string or null <= 191 characters

Vendor Security Review title

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

POST
/vendors/{vendorId}/security-reviews
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{
"reviewDeadlineAt": "2025-07-01T16:45:55.246Z",
"securityReviewStatus": "NOT_YET_STARTED",
"securityReviewType": "SECURITY",
"requestedAt": "2025-07-01T16:45:55.246Z",
"note": "Security Review note",
"requesterUserId": 1,
"title": "Security Review title"
}
Response samples
201
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"requestedAt": "2025-07-01T16:45:55.246Z",
"reviewDeadlineAt": "2025-07-01T16:45:55.246Z",
"decision": "APPROVED",
"title": "string",
"note": "string",
"status": "NOT_YET_STARTED",
"type": "SECURITY",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"vendorId": 1,
"vendor":
{
"id": 1,
"name": "Acme"
},
"userId": 0,
"requesterUserId": 0,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"requesterUser":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"_links":
{
"self":
{
"href": "https://app.drata.com/risk/vendors/profile/1/overview"
}
},
"socReviewForm":
{
"readyToFinalize": false,
"reviewerInformation":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reviewer": "Jane Doe",
"reportIssueDate": "2020-07-06"
},
"complianceScope":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"certification": "SOC_2",
"scopeType": "TYPE_2",
"auditPeriod": "2020-07-06",
"auditPeriodRange":
{
"start": "2020-07-06",
"end": "2020-07-06"
},
"trustServiceCriteria":
[
"AVAILABILITY"
]
},
"reportOpinion":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "Schedule a follow-up review in 90 days."
},
"findings":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"findings":
[
{
"description": "Control objective was not met during the audit period."
}
],
"hasMaterialImpact": false
},
"endUserControls":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"userControls":
[
{
"description": "Users must enforce strong password policies.",
"inPlace": true
}
]
},
"servicesAndLocations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"services":
[
{
"name": "Cloud Hosting"
}
],
"locations":
[
{
"city": "Austin",
"stateOrCountry": "TX"
}
]
},
"cpaFirm":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"cpaFirm": "Deloitte",
"cpaProcedurePerformed": "Assessed reputation via public records."
},
"subserviceOrganizations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"subserviceOrganization": "SubCo Inc.",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "Reviewed available SOC reports."
}
}
}
Create Vendor Security Review with File

Create a new Security Review for a given Vendor with file upload in a single atomic operation.

🔒 Requires Vendors: Create Security Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number
REQUEST BODY SCHEMA: application/json
required
title
required

string <= 255 characters

The title of the security review



reviewDeadlineAt
required

string <date-time>

Vendor Security Review deadline date



requestedAt
string

The date when the security review was requested



securityReviewStatus
required

string

The status of the security review

Enum: "NOT_YET_STARTED" "IN_PROGRESS" "COMPLETED" "NOT_REQUIRED"


securityReviewType
required

string

The type of security review

Enum: "SECURITY" "SOC_REPORT" "UPLOAD_REPORT"


requesterUserId
number

Optional. The ID of the user requesting the security review. If not provided, defaults to: (1) the vendor's security owner if assigned and active, or (2) the platform admin user.



note
string <= 1000 characters

Optional notes for the security review



documentType
required

string

The type of document being uploaded

Enum: "COMPLIANCE_REPORT" "COMPLIANCE_REPORT_REVIEW" "BRIDGE_LETTER" "UPLOADED_COMPLIANCE_REPORT_REVIEW" "QUESTIONNAIRE_ATTACHMENT" "SOC_DOCUMENT" "QUESTIONNAIRE_REPORT" "TRUST_CENTER_PAGE"


file
required

string <binary>

File to upload. Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .gif, .jpg, .jpeg, .png, .csv

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

POST
/vendors/{vendorId}/security-reviews/with-file
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{
"title": "Q4 2024 Security Review",
"reviewDeadlineAt": "2025-07-01T16:45:55.246Z",
"requestedAt": "2024-01-01T00:00:00.000Z",
"securityReviewStatus": 3,
"securityReviewType": 1,
"requesterUserId": 123,
"note": "Additional notes about the security review",
"documentType": 1,
"file": "string"
}
Response samples
201
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"requestedAt": "2025-07-01T16:45:55.246Z",
"reviewDeadlineAt": "2025-07-01T16:45:55.246Z",
"decision": "APPROVED",
"title": "string",
"note": "string",
"status": "NOT_YET_STARTED",
"type": "SECURITY",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"vendorId": 1,
"vendor":
{
"id": 1,
"name": "Acme"
},
"userId": 0,
"requesterUserId": 0,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"requesterUser":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"_links":
{
"self":
{
"href": "https://app.drata.com/risk/vendors/profile/1/overview"
}
},
"socReviewForm":
{
"readyToFinalize": false,
"reviewerInformation":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reviewer": "Jane Doe",
"reportIssueDate": "2020-07-06"
},
"complianceScope":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"certification": "SOC_2",
"scopeType": "TYPE_2",
"auditPeriod": "2020-07-06",
"auditPeriodRange":
{
"start": "2020-07-06",
"end": "2020-07-06"
},
"trustServiceCriteria":
[
"AVAILABILITY"
]
},
"reportOpinion":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "Schedule a follow-up review in 90 days."
},
"findings":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"findings":
[
{
"description": "Control objective was not met during the audit period."
}
],
"hasMaterialImpact": false
},
"endUserControls":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"userControls":
[
{
"description": "Users must enforce strong password policies.",
"inPlace": true
}
]
},
"servicesAndLocations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"services":
[
{
"name": "Cloud Hosting"
}
],
"locations":
[
{
"city": "Austin",
"stateOrCountry": "TX"
}
]
},
"cpaFirm":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"cpaFirm": "Deloitte",
"cpaProcedurePerformed": "Assessed reputation via public records."
},
"subserviceOrganizations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"subserviceOrganization": "SubCo Inc.",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "Reviewed available SOC reports."
}
}
}
List Vendor Security Reviews Across Vendors

Get a paginated list of Security Reviews across all Vendors in the account, subject to the caller's vendor access. Supports the same filters as the per-vendor list plus optional vendorIds[] narrowing. When the caller has restricted vendor access, results are limited to the intersection of the vendors they are permitted to see and the (optional) vendorIds[] filter.

🔒 Requires Vendors: List Vendors permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
QUERY PARAMETERS
cursor
string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a pagination.cursor value that can be used in the subsequent request to retrieve the next page of results



size
number [ 1 .. 500 ]
Default: 50

Number of results to return



sort
string (SortTypeLimitedEnum)

Which field to sort by

Enum: "createdAt" "updatedAt"


sortDir
string (SortDirectionEnum)

The direction to sort the data

Enum: "ASC" "DESC"


includeTotalCount
boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example: includeTotalCount=false


status[]
Array of strings (VendorSecurityReviewStatusEnum)

Filter by security review status

Items Enum: "NOT_YET_STARTED" "IN_PROGRESS" "COMPLETED" "NOT_REQUIRED"


type[]
Array of strings (VendorSecurityReviewTypeEnum)

Filter by security review type

Items Enum: "SECURITY" "SOC_REPORT" "UPLOAD_REPORT"


decision[]
Array of strings (VendorSecurityReviewDecisionEnum)

Filter by security review decision

Items Enum: "PENDING" "APPROVED" "APPROVED_WITH_CONDITIONS" "REJECTED"


expand[]
Array of strings (VendorSecurityReviewsAcrossVendorsExpandEnum)

List of subcollections and sub-objects to expand

Items Enum: "user" "requesterUser" "vendor"


createdAtFrom
string <date>

Filter to Security Reviews created on or after this value (inclusive).

Example: createdAtFrom=2020-07-06


createdAtTo
string <date>

Filter to Security Reviews created on or before this value (inclusive).

Example: createdAtTo=2020-07-06


updatedAtFrom
string <date>

Filter to Security Reviews last updated on or after this value (inclusive)

Example: updatedAtFrom=2020-07-06


updatedAtTo
string <date>

Filter to Security Reviews last updated on or before this value (inclusive).

Example: updatedAtTo=2020-07-06


vendorIds[]
Array of numbers

Filter to Security Reviews for these Vendor IDs. When the caller has restricted vendor access, the effective set is the intersection of this parameter and their permitted vendors.

Example: vendorIds[]=1&vendorIds[]=2&vendorIds[]=3
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

GET
/vendor-security-reviews
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendor-security-reviews`,
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
200
400
401
402
403
2 more
412
500
2 more
application/json
CopyExpand allCollapse all
{
"data":
[
{
"id": 1,
"requestedAt": "2025-07-01T16:45:55.246Z",
"reviewDeadlineAt": "2025-07-01T16:45:55.246Z",
"decision": "APPROVED",
"title": "string",
"note": "string",
"status": "NOT_YET_STARTED",
"type": "SECURITY",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"vendorId": 1,
"vendor":
{
"id": 1,
"name": "Acme"
},
"userId": 0,
"requesterUserId": 0,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"requesterUser":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"_links":
{
"self":
{
"href": "https://app.drata.com/risk/vendors/profile/1/overview"
}
},
"socReviewForm":
{
"readyToFinalize": false,
"reviewerInformation":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reviewer": "Jane Doe",
"reportIssueDate": "2020-07-06"
},
"complianceScope":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"certification": "SOC_2",
"scopeType": "TYPE_2",
"auditPeriod": "2020-07-06",
"auditPeriodRange":
{
"start": "2020-07-06",
"end": "2020-07-06"
},
"trustServiceCriteria":
[
"AVAILABILITY"
]
},
"reportOpinion":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "Schedule a follow-up review in 90 days."
},
"findings":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"findings":
[
{
"description": "Control objective was not met during the audit period."
}
],
"hasMaterialImpact": false
},
"endUserControls":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"userControls":
[
{
"description": "Users must enforce strong password policies.",
"inPlace": true
}
]
},
"servicesAndLocations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"services":
[
{
"name": "Cloud Hosting"
}
],
"locations":
[
{
"city": "Austin",
"stateOrCountry": "TX"
}
]
},
"cpaFirm":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"cpaFirm": "Deloitte",
"cpaProcedurePerformed": "Assessed reputation via public records."
},
"subserviceOrganizations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"subserviceOrganization": "SubCo Inc.",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "Reviewed available SOC reports."
}
}
}
],
"pagination":
{
"cursor": "string",
"totalCount": 0
}
}
Get Vendor Security Review 🧪

Get a specific Security Review for a given Vendor. For SOC_REPORT reviews the response includes a socReview field with the SOC form data.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: List Vendors permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number


securityReviewId
required

number
QUERY PARAMETERS
expand[]
Array of strings (VendorSecurityReviewExpandEnum)

List of subcollections and sub-objects to expand

Items Enum: "user" "requesterUser"
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

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"requestedAt": "2025-07-01T16:45:55.246Z",
"reviewDeadlineAt": "2025-07-01T16:45:55.246Z",
"decision": "APPROVED",
"title": "string",
"note": "string",
"status": "NOT_YET_STARTED",
"type": "SECURITY",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"vendorId": 1,
"vendor":
{
"id": 1,
"name": "Acme"
},
"userId": 0,
"requesterUserId": 0,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"requesterUser":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"_links":
{
"self":
{
"href": "https://app.drata.com/risk/vendors/profile/1/overview"
}
},
"socReviewForm":
{
"readyToFinalize": false,
"reviewerInformation":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reviewer": "Jane Doe",
"reportIssueDate": "2020-07-06"
},
"complianceScope":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"certification": "SOC_2",
"scopeType": "TYPE_2",
"auditPeriod": "2020-07-06",
"auditPeriodRange":
{
"start": "2020-07-06",
"end": "2020-07-06"
},
"trustServiceCriteria":
[
"AVAILABILITY"
]
},
"reportOpinion":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "Schedule a follow-up review in 90 days."
},
"findings":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"findings":
[
{
"description": "Control objective was not met during the audit period."
}
],
"hasMaterialImpact": false
},
"endUserControls":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"userControls":
[
{
"description": "Users must enforce strong password policies.",
"inPlace": true
}
]
},
"servicesAndLocations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"services":
[
{
"name": "Cloud Hosting"
}
],
"locations":
[
{
"city": "Austin",
"stateOrCountry": "TX"
}
]
},
"cpaFirm":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"cpaFirm": "Deloitte",
"cpaProcedurePerformed": "Assessed reputation via public records."
},
"subserviceOrganizations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"subserviceOrganization": "SubCo Inc.",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "Reviewed available SOC reports."
}
}
}
Update Vendor Security Review 🧪

Update a Vendor Security Review. The title field applies to all security review types. The socForm field is only processed when securityReviewType is SOC_REPORT. The updatedAt field overrides the review's last-updated timestamp (surfaced as the vendor's last review date) and is the only field that may be set on a completed review.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Security Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number


securityReviewId
required

number
REQUEST BODY SCHEMA: application/json
required
title
string

The Security Review name.



socForm

object or null

SOC review form data. Only processed when securityReviewType is SOC_REPORT.



updatedAt
string <date-time>

Overrides the Security Review last-updated timestamp, which is surfaced as the vendor's last review date. Provided to backfill or correct historical review dates. This is the only field that may be set on a completed review. Note that the value is not preserved if the review is edited afterwards.

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

PUT
/vendors/{vendorId}/security-reviews/{securityReviewId}
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
CopyExpand allCollapse all
{
"title": "string",
"socForm":
{
"reviewerInformation":
{
"reviewerUserId": 42,
"reportIssueDate": "2020-07-06"
},
"complianceScope":
{
"certification": "SOC_2",
"scopeType": "TYPE_2",
"auditPeriod": "2020-07-06",
"auditPeriodRange":
{
"start": "2020-07-06",
"end": "2020-07-06"
},
"trustServiceCriteria":
[
"AVAILABILITY"
]
},
"reportOpinion":
{
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "string"
},
"findings":
{
"findings":
[
{
"description": "string"
}
],
"hasMaterialImpact": false
},
"endUserControls":
{
"userControls":
[
{
"description": "string",
"inPlace": true
}
]
},
"servicesAndLocations":
{
"services":
[
{
"name": "string"
}
],
"locations":
[
{
"city": "Austin",
"stateOrCountry": "TX"
}
]
},
"cpaFirm":
{
"cpaFirm": "Deloitte",
"cpaProcedurePerformed": "string"
},
"subserviceOrganizations":
{
"subserviceOrganization": "string",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "string"
}
},
"updatedAt": "2025-07-01T16:45:55.246Z"
}
Response samples
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"id": 1,
"requestedAt": "2025-07-01T16:45:55.246Z",
"reviewDeadlineAt": "2025-07-01T16:45:55.246Z",
"decision": "APPROVED",
"title": "string",
"note": "string",
"status": "NOT_YET_STARTED",
"type": "SECURITY",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"vendorId": 1,
"vendor":
{
"id": 1,
"name": "Acme"
},
"userId": 0,
"requesterUserId": 0,
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"requesterUser":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"_links":
{
"self":
{
"href": "https://app.drata.com/risk/vendors/profile/1/overview"
}
},
"socReviewForm":
{
"readyToFinalize": false,
"reviewerInformation":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reviewer": "Jane Doe",
"reportIssueDate": "2020-07-06"
},
"complianceScope":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"certification": "SOC_2",
"scopeType": "TYPE_2",
"auditPeriod": "2020-07-06",
"auditPeriodRange":
{
"start": "2020-07-06",
"end": "2020-07-06"
},
"trustServiceCriteria":
[
"AVAILABILITY"
]
},
"reportOpinion":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "Schedule a follow-up review in 90 days."
},
"findings":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"findings":
[
{
"description": "Control objective was not met during the audit period."
}
],
"hasMaterialImpact": false
},
"endUserControls":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"userControls":
[
{
"description": "Users must enforce strong password policies.",
"inPlace": true
}
]
},
"servicesAndLocations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"services":
[
{
"name": "Cloud Hosting"
}
],
"locations":
[
{
"city": "Austin",
"stateOrCountry": "TX"
}
]
},
"cpaFirm":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"cpaFirm": "Deloitte",
"cpaProcedurePerformed": "Assessed reputation via public records."
},
"subserviceOrganizations":
{
"isComplete": false,
"missingRequiredFields":
[
"reviewerInformation.reviewer"
],
"subserviceOrganization": "SubCo Inc.",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "Reviewed available SOC reports."
}
}
}
Get Vendor Security Review Summary 🧪

Get the status of a Vendor Security Review summary PDF. When status is READY the response includes document, a time-limited signed downloadUrl for the most recently generated summary PDF. Use the document createdAt to judge freshness: a reopened review that has not regenerated its summary may still surface the prior finalization PDF. GENERATING means the summary is still being produced; NOT_GENERATING means none is available (never requested, not eligible, or the last generation failed — these are not distinguished). Access requires the vendor read scope (read:vendor-security-review). This endpoint is a read-only status check; it does not itself generate or deliver the summary.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: List Vendors permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the vendor.



securityReviewId
required

number

The numeric ID of the security review.

Responses
200

Successful

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

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/summary/status
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/summary/status`,
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
200
401
402
403
404
2 more
412
500
2 more
application/json
CopyExpand allCollapse all
{
"status": "READY",
"document":
{
"id": 1,
"name": "AWS SOC 2 2025",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"type": "COMPLIANCE_REPORT",
"downloadUrl":
{
"signedUrl": "https://somedomain.com/filename.pdf?Signature=ABC123",
"fileBuffer":
{
"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="
},
"fileName": "Artifact 3.png"
}
}
}
Upload Security Questionnaire

🔒 Requires Vendors: Upload Security Questionnaire permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number
REQUEST BODY SCHEMA: multipart/form-data
required
files
required

Array of strings <binary>

Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .csv

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

POST
/vendors/{vendorId}/security-questionnaires
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import FormData from 'form-data';
import fetch from 'node-fetch';

async function run() {
  const form = new FormData();
  form.append('files[0]','string');

  const vendorId = 'YOUR_vendorId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-questionnaires`,
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
201
400
401
402
403
3 more
404
412
500
3 more
application/json
Copy
{
"id": 1,
"completedBy": "Acme",
"recipientEmail": "jdoe@company.com",
"isCompleted": "true",
"dateSent": "2025-07-01T16:45:55.246Z",
"isManualUpload": "true",
"responseId": 1,
"title": "Vendor Security Questionnaire",
"status": "SENT",
"completedAt": "2025-07-01T16:45:55.246Z",
"totalQuestions": 10,
"totalQuestionsAnswered": 10,
"reminderDate": "2025-07-01T16:45:55.246Z"
}
List Security Review Questionnaires 🧪

List the questionnaires that belong to a Vendor Security Review. Cursor-paginated. Archived or soft-deleted questionnaires are excluded from the response.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: List Vendor Questionnaire permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the vendor.



securityReviewId
required

number

The numeric ID of the security review.

QUERY PARAMETERS
cursor
string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a pagination.cursor value that can be used in the subsequent request to retrieve the next page of results



size
number [ 1 .. 500 ]
Default: 50

Number of results to return



sort
string (SortTypeLimitedEnum)

Which field to sort by

Enum: "createdAt" "updatedAt"


sortDir
string (SortDirectionEnum)

The direction to sort the data

Enum: "ASC" "DESC"


includeTotalCount
boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example: includeTotalCount=false
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

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/security-questionnaires`,
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
200
400
401
402
403
3 more
404
412
500
3 more
application/json
CopyExpand allCollapse all
{
"data":
[
{
"id": 1,
"completedBy": "Acme",
"recipientEmail": "jdoe@company.com",
"isCompleted": "true",
"dateSent": "2025-07-01T16:45:55.246Z",
"isManualUpload": "true",
"responseId": 1,
"title": "Vendor Security Questionnaire",
"status": "SENT",
"completedAt": "2025-07-01T16:45:55.246Z",
"totalQuestions": 10,
"totalQuestionsAnswered": 10,
"reminderDate": "2025-07-01T16:45:55.246Z"
}
],
"pagination":
{
"cursor": "string",
"totalCount": 0
}
}
Upload Security Questionnaire

Upload a Security Questionnaire to a Vendor for a specific Security Review.

🔒 Requires Vendors: Upload Security Questionnaire permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number


securityReviewId
required

number
REQUEST BODY SCHEMA: multipart/form-data
required
files
required

Array of strings <binary>

Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .csv

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

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import FormData from 'form-data';
import fetch from 'node-fetch';

async function run() {
  const form = new FormData();
  form.append('files[0]','string');

  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/security-questionnaires`,
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
201
400
401
402
403
3 more
404
412
500
3 more
application/json
Copy
{
"id": 1,
"completedBy": "Acme",
"recipientEmail": "jdoe@company.com",
"isCompleted": "true",
"dateSent": "2025-07-01T16:45:55.246Z",
"isManualUpload": "true",
"responseId": 1,
"title": "Vendor Security Questionnaire",
"status": "SENT",
"completedAt": "2025-07-01T16:45:55.246Z",
"totalQuestions": 10,
"totalQuestionsAnswered": 10,
"reminderDate": "2025-07-01T16:45:55.246Z"
}
List Security Review Actions 🧪

List available actions for a security review based on its current state (e.g. Finalize, Reopen). Supported for all security review types (SECURITY, SOC_REPORT, UPLOAD_REPORT). SOC_REPORT reviews additionally require the linked vendor report review to pass readiness validation before Finalize is offered.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: List Security Review Actions permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number


securityReviewId
required

number
Responses
200

Successful

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

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/actions
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/actions`,
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
200
401
402
403
404
2 more
412
500
2 more
application/json
CopyExpand allCollapse all
{
"data":
[
{
"action": "finalize"
}
]
}
Perform Security Review Action 🧪

Execute an action on a security review. Finalize marks the review as complete and records the reviewer decision (PENDING, APPROVED, APPROVED_WITH_CONDITIONS, or REJECTED); decision defaults to APPROVED when omitted, and PENDING completes the review without recording a conclusion. Reopen returns a completed review to in-progress and, when resetDecision is true, also clears the prior decision back to PENDING. Supported for all security review types (SECURITY, SOC_REPORT, UPLOAD_REPORT).

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Perform Security Review Action permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number


securityReviewId
required

number
REQUEST BODY SCHEMA: application/json
required
action
required

string

Action to perform on the security review.

Enum: "finalize" "reopen"


decision
string

Finalization decision. Only valid when action is finalize. Must be one of PENDING, APPROVED, APPROVED_WITH_CONDITIONS, or REJECTED — the same options the review page offers. PENDING completes the review without recording a conclusion; use the reopen action to return it to in-progress. When omitted on finalize, defaults to APPROVED for backward compatibility.

Enum: "PENDING" "APPROVED" "APPROVED_WITH_CONDITIONS" "REJECTED"


note
string <= 1000 characters

Optional reviewer note captured with the finalization. Only valid when action is finalize.



resetDecision
boolean

When true, reset the decision back to PENDING as part of the reopen. Only valid when action is reopen. Defaults to false, which preserves the prior decision on the reopened review.

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

The requested action is not available for the current state of this security review.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/actions
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{
"action": "finalize",
"decision": "APPROVED",
"note": "string",
"resetDecision": true
}
Response samples
200
400
401
402
403
4 more
404
412
422
500
4 more
application/json
Copy
{
"success": true,
"newStatus": "COMPLETED",
"message": "Security review finalized successfully"
}
Start Agent Review on Security Review 🧪

Start the AI agent on an existing Security Review, or get the workflow id of the agent already running on it. Creating a SECURITY Security Review with POST /vendors/{vendorId}/security-reviews starts the agent automatically; call this endpoint afterwards to get the workflow id, or to start the agent if the automatic start failed. When the agent already runs on the review, the same workflow is returned and nothing new starts. The next step is to confirm the criteria. The work runs in the background: a 202 means it was accepted, not finished. Read the outcome through the Assessments endpoints.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Start Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
object (AgentReviewActionRequestPublicV2Dto)
Responses
202

The agent is running on the review.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the AI agent is not available: the account needs TPRM_AGENT, AI_DATA_SHARE and AI turned on, and the Security Review must be a SECURITY review.

404

The Security Review does not belong to the Vendor.

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/agent
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{ }
Response samples
202
400
401
402
403
3 more
404
412
500
3 more
application/json
Copy
{
"securityReviewId": 1,
"workflowId": 1
}
Confirm Agent Review Criteria 🧪

Save the Vendor Criteria the agent assesses. Open only while the agent waits for criteria; otherwise it fails with 422. Send criterionIds to choose them; every ID must be a published criterion. Omit criterionIds to save the published criteria that match the Vendor impact level, inherent risk level and vendor type, the same set the Vendor Criteria list returns for the review. An empty array is rejected, and so is an empty default set. If access to the Vendor Trust Center was already granted, confirming can also start document collection.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
criterionIds
Array of numbers [ 1 .. 1000 ] items unique

IDs of the published Vendor Criteria the agent assesses. When omitted, the published criteria that match the vendor impact level, inherent risk level and vendor type are selected and saved. An empty array is rejected.

Responses
200

The criteria were saved.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

You are not allowed to perform this action

404

The Security Review does not belong to the Vendor, or has no agent review.

412

You must accept the Drata terms and conditions to use the API

422

Criteria confirmation is not open: the structured body with blockers and availableActions. A criterion is unknown or unpublished, or no default criteria match: the standard error body, with the reason in message.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/criteria-confirmations
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
CopyExpand allCollapse all
{
"criterionIds":
[
101,
102
]
}
Response samples
200
400
401
402
403
4 more
404
412
422
500
4 more
application/json
CopyExpand allCollapse all
{
"securityReviewId": 1,
"criterionIds":
[
101,
102
],
"source": "default"
}
Start Agent Review Assessment 🧪

Start the assessment, or re-run it when it already ran. Open once the criteria are confirmed and again after an assessment finishes; otherwise it fails with 422. The work runs in the background: a 202 means it was accepted, not finished. Read the outcome through the Assessments endpoints.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
object (AgentReviewActionRequestPublicV2Dto)
Responses
202

The assessment was accepted.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the AI agent is not available: the account needs TPRM_AGENT, AI_DATA_SHARE and AI turned on, and the Security Review must be a SECURITY review.

404

The Security Review does not belong to the Vendor, or has no agent review.

412

You must accept the Drata terms and conditions to use the API

422

The assessment cannot be started right now.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/assessment-runs
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{ }
Response samples
202
400
401
402
403
4 more
404
412
422
500
4 more
application/json
Copy
{
"securityReviewId": 1,
"workflowId": 1,
"action": "start_assessment"
}
Stop Agent Review Assessment 🧪

Stop the assessment that is running or preparing. Open only while an assessment runs or prepares; otherwise it fails with 422. The run ends as cancelled and nothing already recorded is deleted. Start or re-run the assessment again with POST .../assessment-runs. The work runs in the background: a 202 means it was accepted, not finished. Read the outcome through the Assessments endpoints.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
object (AgentReviewActionRequestPublicV2Dto)
Responses
202

The stop was accepted.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the AI agent is not available: the account needs TPRM_AGENT, AI_DATA_SHARE and AI turned on, and the Security Review must be a SECURITY review.

404

The Security Review does not belong to the Vendor, or has no agent review.

412

You must accept the Drata terms and conditions to use the API

422

No assessment is running or preparing.

500

Internal server error

DELETE
/vendors/{vendorId}/security-reviews/{securityReviewId}/assessment-runs/current
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{ }
Response samples
202
400
401
402
403
4 more
404
412
422
500
4 more
application/json
Copy
{
"securityReviewId": 1,
"workflowId": 1,
"action": "start_assessment"
}
Generate Agent Review Follow-up Questionnaire 🧪

Generate the follow-up questionnaire for the criteria the assessment did not meet. Open after an assessment finishes; otherwise it fails with 422. When every criterion was met, nothing is generated. Once generated, it can be regenerated only if a criterion result changed since then and the questionnaire was not sent: calling it again then replaces the questions of the draft, including any edits made to it in the Drata app. Otherwise a second call fails with 422. Once it is sent, a new one comes from re-running the assessment. The work runs in the background: a 202 means it was accepted, not finished. Read the outcome through the Assessments endpoints.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
object (AgentReviewActionRequestPublicV2Dto)
Responses
202

The generation was accepted.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the AI agent is not available: the account needs TPRM_AGENT, AI_DATA_SHARE and AI turned on, and the Security Review must be a SECURITY review.

404

The Security Review does not belong to the Vendor, or has no agent review.

412

You must accept the Drata terms and conditions to use the API

422

The follow-up questionnaire cannot be generated right now.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/follow-up-questionnaires
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{ }
Response samples
202
400
401
402
403
4 more
404
412
422
500
4 more
application/json
Copy
{
"securityReviewId": 1,
"workflowId": 1,
"action": "start_assessment"
}
Get Agent Review Follow-up Questionnaire 🧪

Read the follow-up questionnaire the agent generated for the Security Review: its status and questions. It has no side effects. Fails with 404 when no follow-up questionnaire was generated; generate it with POST .../follow-up-questionnaires.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: List Vendors permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

Responses
200

The follow-up questionnaire.

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the AI agent is not available: the account needs TPRM_AGENT, AI_DATA_SHARE and AI turned on, and the Security Review must be a SECURITY review.

404

The Security Review does not belong to the Vendor, has no agent review, or has no follow-up questionnaire.

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/follow-up-questionnaire
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/follow-up-questionnaire`,
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
200
401
402
403
404
2 more
412
500
2 more
application/json
CopyExpand allCollapse all
{
"securityReviewId": 1,
"workflowId": 1,
"followUpQuestionnaire":
{
"id": 1,
"name": "Vendor Assessment Follow-Up - Review 123",
"status": "DRAFT"
},
"questions":
[
{
"id": 1,
"question": "Do you encrypt data at rest?",
"type": "SHORT_ANSWER",
"required": true,
"criterionName": "Encryption at rest"
}
]
}
Send Agent Review Follow-up Questionnaire 🧪

Email the Vendor a link to the follow-up questionnaire of this Security Review. The questionnaire is the draft the agent generated; it cannot be chosen. Open after the follow-up questionnaire was generated and before it is sent; otherwise it fails with 422. The email is sent in the background: a 202 means it was accepted, not delivered. The Vendor receives a Vendor Hub link. When the Vendor answers, the agent re-runs the assessment automatically. When emailContent is omitted the default message of the email template is used.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
email
required

Array of strings <email> [ 1 .. 5 ] items unique

Email addresses that receive the follow-up questionnaire link. The addresses joined together must be at most 191 characters.



cc
Array of strings <email> <= 5 items unique

Email addresses copied on the email



bcc
Array of strings <email> <= 5 items unique

Email addresses blind copied on the email



emailSubject
string <= 191 characters

The email subject. When omitted, the Vendor email subject setting is used, or the default subject when the setting is empty.



emailContent
string <= 30000 characters

The email message. HTML is sanitized. When omitted or empty, the default message of the email template is used.

Responses
202

The send was accepted.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the AI agent is not available: the account needs TPRM_AGENT, AI_DATA_SHARE and AI turned on, and the Security Review must be a SECURITY review.

404

The Security Review does not belong to the Vendor, has no agent review, or has no follow-up questionnaire.

412

You must accept the Drata terms and conditions to use the API

422

The follow-up questionnaire cannot be sent right now: the structured body with blockers and availableActions. The questionnaire is no longer a draft: the standard error body, with the reason in message.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/follow-up-questionnaire-sends
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
CopyExpand allCollapse all
{
"email":
[
"security@vendor.com"
],
"cc":
[
"manager@drata.com"
],
"bcc":
[
"audit@drata.com"
],
"emailSubject": "Drata is conducting a security review of you.",
"emailContent": "Hi,\n\nWe'd like to conduct a security review and would like some information from you. Use this link to complete the questionnaire.\n\nThank you."
}
Response samples
202
400
401
402
403
4 more
404
412
422
500
4 more
application/json
Copy
{
"securityReviewId": 1,
"workflowId": 1,
"action": "start_assessment"
}
Finalize Agent Review 🧪

Complete the Security Review with a decision. Open only after the assessment completed and the residual risk was reviewed; otherwise it fails with 422. It marks the review completed with the decision and the optional note, and generates the executive summary in the background. The agent's "Finalize review and generate report" step completes asynchronously and best-effort after this call returns. The decision is required: there is no default decision, unlike the base-tier Security Review actions endpoint, which defaults to APPROVED. Fails with 400 when the review is already completed.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
decision
required

string

Decision recorded on the Security Review. Required: there is no default. PENDING completes the review without recording a conclusion.

Enum: "PENDING" "APPROVED" "APPROVED_WITH_CONDITIONS" "REJECTED"


note
string <= 1000 characters

Optional reviewer note captured with the finalization.

Responses
200

The Security Review was finalized.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the AI agent is not available: the account needs TPRM_AGENT, AI_DATA_SHARE and AI turned on, and the Security Review must be a SECURITY review.

404

The Security Review does not belong to the Vendor, or has no agent review.

412

You must accept the Drata terms and conditions to use the API

422

The agent does not offer the finalize action right now.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/finalizations
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{
"decision": "APPROVED",
"note": "string"
}
Response samples
200
400
401
402
403
4 more
404
412
422
500
4 more
application/json
Copy
{
"securityReviewId": 1,
"workflowId": 1,
"action": "finalize",
"decision": "APPROVED"
}
Add Agent Review Documents 🧪

Upload documents to a Security Review and notify its agent, so the next assessment run uses them. Use this endpoint, not the Vendor Documents upload, for a review the agent works on: only this one notifies the agent. Open whenever an agent review exists on the Security Review, whatever the agent is doing, so it can also unblock a review that stalled for lack of documents. Send 10 files at most as multipart/form-data in files, and optionally a type applied to all of them. Every file is validated and scanned before any is stored: when one is rejected, none is added. Fails with 404 when the Security Review has no agent review. Adding documents does not start an assessment; start or re-run it through the Assessment Runs endpoint.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: multipart/form-data
required
files
required

Array of strings <binary> [ 1 .. 10 ] items

Between 1 and 10 files. Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .gif, .jpg, .jpeg, .png, .csv. MIME type must match the file extension



type
string

The Vendor Document type applied to every uploaded file. Defaults to COMPLIANCE_REPORT.

Enum: "COMPLIANCE_REPORT" "COMPLIANCE_REPORT_REVIEW" "BRIDGE_LETTER" "UPLOADED_COMPLIANCE_REPORT_REVIEW" "QUESTIONNAIRE_ATTACHMENT" "SOC_DOCUMENT" "QUESTIONNAIRE_REPORT" "TRUST_CENTER_PAGE"
Responses
201

The documents were added and the agent was notified.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the AI agent is not available: the account needs TPRM_AGENT, AI_DATA_SHARE and AI turned on, and the Security Review must be a SECURITY review.

404

The Security Review does not belong to the Vendor, or has no agent review.

412

You must accept the Drata terms and conditions to use the API

413

The file was too large to upload

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/documents
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import FormData from 'form-data';
import fetch from 'node-fetch';

async function run() {
  const form = new FormData();
  form.append('files[0]','string');

  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/documents`,
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
201
400
401
402
403
4 more
404
412
413
500
4 more
application/json
CopyExpand allCollapse all
{
"securityReviewId": 1,
"workflowId": 1,
"documents":
[
{
"id": 1,
"name": "AWS SOC 2 2025",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"type": "COMPLIANCE_REPORT",
"downloadUrl":
{
"signedUrl": "https://somedomain.com/filename.pdf?Signature=ABC123",
"fileBuffer":
{
"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="
},
"fileName": "Artifact 3.png"
}
}
]
}
Collect Agent Review Public Trust Center Documents 🧪

Collect the documents the Vendor publishes on its Trust Center, which need no access, and assess them once the criteria are confirmed. Open while the Vendor has a supported Trust Center, access was not granted and no collection or assessment is underway; otherwise it fails with 422. To collect the private documents too, request access with POST .../access-requests. The work runs in the background: a 202 means it was accepted, not finished. Read the outcome through the Assessments endpoints.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
object (AgentReviewActionRequestPublicV2Dto)
Responses
202

The collection was accepted.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the AI agent is not available: the account needs TPRM_AGENT, AI_DATA_SHARE and AI turned on, and the Security Review must be a SECURITY review.

404

The Security Review does not belong to the Vendor, or has no agent review.

412

You must accept the Drata terms and conditions to use the API

422

Public documents cannot be collected right now.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/public-document-collections
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{ }
Response samples
202
400
401
402
403
4 more
404
412
422
500
4 more
application/json
Copy
{
"securityReviewId": 1,
"workflowId": 1,
"action": "start_assessment"
}
Get Agent Review Trust Center Access Request Requirements 🧪

Read the form the Vendor Trust Center asks to fill in to request access, to send with POST .../access-requests. It has no side effects. Open while the agent offers to request access; otherwise it fails with 422. The form describes the user the API key or OAuth token acts as, because the request is made in their name.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: List Vendors permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

Responses
200

The access request form.

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, or the user cannot access this Vendor.

404

The Security Review does not belong to the Vendor, has no agent review, or its agent review is no longer running.

409

The agent review is authenticated against the Trust Center as a different user.

412

You must accept the Drata terms and conditions to use the API

422

The agent does not offer to request access right now.

500

Internal server error

502

The Vendor Trust Center failed or did not answer. Nothing was changed.

GET
/vendors/{vendorId}/security-reviews/{securityReviewId}/access-request-requirements
Try it
Request samples
Node.js
JavaScript
curl
Python
C#
2 more
Java
PHP
2 more
Copy
import fetch from 'node-fetch';

async function run() {
  const vendorId = 'YOUR_vendorId_PARAMETER';
  const securityReviewId = 'YOUR_securityReviewId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/security-reviews/${securityReviewId}/access-request-requirements`,
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
200
401
402
403
404
5 more
409
412
422
500
502
5 more
application/json
CopyExpand allCollapse all
{
"securityReviewId": 1,
"workflowId": 1,
"memberLoginStatusCode": "email_not_recognized",
"fields":
[
{
"name": "firstName",
"label": "First name",
"type": "text",
"required": true,
"minLength": 1,
"maxLength": 64,
"options":
[
{
"value": "customer",
"label": "Customer"
}
],
"links":
[
"https://trust.vendor.example.com/terms"
]
}
]
}
Request Agent Review Trust Center Access 🧪

Ask the Vendor for access to the private documents of its Trust Center, so the agent collects them once access is granted. Open while the agent offers to request access; otherwise it fails with 422. The agent review is authenticated as the user the API key or OAuth token acts as. For a SafeBase Trust Center the request carries that user's email and the fields sent. For any other Trust Center the agent fills in the Vendor form with the fields sent, and uses the email of the user who created the review unless workEmail is sent. Read the fields with GET .../access-request-requirements. If the Trust Center rejects the request, nothing is kept, so it can be sent again. The work runs in the background: a 202 means it was accepted, not finished. Read the outcome through the Assessments endpoints.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
fields

object <= 50 properties

The access request form, by field name. Read the names, types and required fields from GET .../access-request-requirements. A checkbox takes a boolean, a select takes one of its options' value, and a multi-select takes a list of them. Omit it when that read lists no fields.

Responses
202

The access request was sent.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, or the user cannot access this Vendor.

404

The Security Review does not belong to the Vendor, has no agent review, or its agent review is no longer running.

409

The agent review is authenticated against the Trust Center as a different user.

412

You must accept the Drata terms and conditions to use the API

422

The agent does not offer to request access right now.

500

Internal server error

502

The Vendor Trust Center failed or did not answer. Nothing was changed.

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/access-requests
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
CopyExpand allCollapse all
{
"fields":
{
"firstName": "Jane",
"lastName": "Doe",
"companyName": "Acme Corporation",
"agreeToTerms": true
}
}
Response samples
202
400
401
402
403
6 more
404
409
412
422
500
502
6 more
application/json
Copy
{
"securityReviewId": 1,
"workflowId": 1,
"action": "start_assessment"
}
Provide Agent Review Trust Center Access Link 🧪

Give the agent a link that opens the Vendor Trust Center documents, such as one the Vendor shared, so it collects them without an access request. Open while the agent offers it: the Trust Center is not a SafeBase one, access was not granted and no collection is underway; otherwise it fails with 422. The link must open the Vendor Trust Center (its own host) or a known Trust Center platform, and resolve to public addresses only. It needs the AI agent, like the other steps that make the agent collect documents. The agent review stays authenticated as the user the API key or OAuth token acts as. The work runs in the background: a 202 means it was accepted, not finished. Read the outcome through the Assessments endpoints.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
accessLink
required

string <uri> <= 2048 characters

A link that opens the Vendor Trust Center documents without signing in, as the Vendor shared it. Treat it as a secret: anyone with it can read those documents.

Responses
202

The access link was accepted.

400

The body is invalid, or the access link does not open the Vendor Trust Center or a known Trust Center platform on a public address.

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, the user cannot access this Vendor, or the AI agent is not available: the account needs TPRM_AGENT, AI_DATA_SHARE and AI turned on, and the Security Review must be a SECURITY review.

404

The Security Review does not belong to the Vendor, has no agent review, or its agent review is no longer running.

409

The agent review is authenticated against the Trust Center as a different user.

412

You must accept the Drata terms and conditions to use the API

422

The agent does not accept an access link right now.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/access-links
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{
"accessLink": "https://trust.vendor.example.com/share?token=abc123"
}
Response samples
202
400
401
402
403
5 more
404
409
412
422
500
5 more
application/json
Copy
{
"securityReviewId": 1,
"workflowId": 1,
"action": "start_assessment"
}
Skip Agent Review Trust Center Access Request 🧪

Stop waiting on a Trust Center access request that is still pending, so documents can be uploaded with POST .../documents. The request itself stays open: if the Vendor grants access later, the agent still collects the documents. Open only while an access request is pending; otherwise it fails with 422. POST .../access-requests undoes the skip whenever the agent offers it again, which it does while the review is not authenticated against the Trust Center (a request made through this API authenticates it). The work runs in the background: a 202 means it was accepted, not finished. Read the outcome through the Assessments endpoints.

🧪 Note: This endpoint is in beta and may change.

🔒 Requires Vendors: Update Agent Review permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number

The numeric ID of the Vendor.



securityReviewId
required

number

The numeric ID of the Security Review.

REQUEST BODY SCHEMA: application/json
required
object (AgentReviewActionRequestPublicV2Dto)
Responses
202

The skip was accepted.

400

Malformed data and/or validation errors

401

Invalid Authorization

402

The account lacks a TPRM plan.

403

The API key or OAuth token lacks the permission or scope, or the user cannot access this Vendor.

404

The Security Review does not belong to the Vendor, or has no agent review.

412

You must accept the Drata terms and conditions to use the API

422

No access request is pending.

500

Internal server error

POST
/vendors/{vendorId}/security-reviews/{securityReviewId}/access-request-skips
Try it
Request samples
Payload
Node.js
JavaScript
curl
Python
3 more
C#
Java
PHP
3 more
application/json
Copy
{ }
Response samples
202
400
401
402
403
4 more
404
412
422
500
4 more
application/json
Copy
{
"securityReviewId": 1,
"workflowId": 1,
"action": "start_assessment"
}
Next to Vendor Types
