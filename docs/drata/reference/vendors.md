Title: Vendors

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Vendors/

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
Vendor Types
Vendors
GET
List Vendors
POST
Create Vendor
GET
Search Vendors
GET
Get Vendor Statistics
GET
Get Vendor
PUT
Update Vendor
DELETE
Remove Vendor
GET
List Vendor Questionnaires
POST
Send Questionnaire to Vendor
GET
List Vendor Questionnaire Questions
GET
Get Vendor Questionnaire
GET
Get Vendor Trust Center Overview
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
Vendors

Vendors are third-parties that your organization is working with. Drata allows you to track and review risks associated with these third-parties. The help docs have more information.

List Vendors

Find Vendors matching the provided filters.

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


category
string (VendorCategoryEnum)

The category of the Vendors

Enum: "ENGINEERING" "PRODUCT" "MARKETING" "CS" "SALES" "FINANCE" "HR" "ADMINISTRATIVE" "SECURITY" "LEGAL" "INFORMATION_TECHNOLOGY" "NONE"


categories[]
Array of strings (VendorCategoryEnum)

Filter to Vendors in any of these categories (business unit). Multi-select is OR.

Items Enum: "ENGINEERING" "PRODUCT" "MARKETING" "CS" "SALES" "FINANCE" "HR" "ADMINISTRATIVE" "SECURITY" "LEGAL" "INFORMATION_TECHNOLOGY" "NONE"


expand[]
Array of strings (VendorExpandEnum)

List of subcollections and sub-objects to expand

Items Enum: "customFields" "documents" "integrations" "lastQuestionnaire" "latestSecurityReviews" "reviews" "vendorUser" "vendorRelationshipContact" "dataAccessedOrProcessed" "scheduleConfiguration" "customVendorType" "inherentRiskLevel" "residualRiskLevel"


impactLevel
string (VendorImpactLevelEnum)

Overall Vendor impact level

Enum: "INSIGNIFICANT" "MINOR" "MODERATE" "MAJOR" "CRITICAL" "UNSCORED"


renewalDate
string <date>

Vendor renewal date (date-only YYYY-MM-DD, matched exactly)

Example: renewalDate=2020-07-06


renewalScheduleType
string (RenewalScheduleTypeEnum)

Vendor renewal schedule type

Enum: "ONE_MONTH" "TWO_MONTHS" "THREE_MONTHS" "SIX_MONTHS" "ONE_YEAR" "CUSTOM" "NONE"


risk
string (VendorRiskEnum)

Filter data to Vendors of this risk level

Enum: "NONE" "LOW" "MODERATE" "HIGH"


status
string (VendorStatusEnum)

The status of the Vendors

Enum: "PROSPECTIVE" "ACTIVE" "ARCHIVED" "APPROVED" "REJECTED" "FLAGGED" "ON_HOLD" "OFFBOARDED" "UNDER_REVIEW" "NONE"


statuses[]
Array of strings (VendorStatusFilterEnum)

Filter to Vendors in any of these statuses (multi-select is OR). NONE matches Vendors with no status set. CURRENT matches the current Vendor book: every Vendor that is not PROSPECTIVE or ARCHIVED, including those with no status.

Items Enum: "PROSPECTIVE" "ACTIVE" "ARCHIVED" "APPROVED" "REJECTED" "FLAGGED" "ON_HOLD" "OFFBOARDED" "UNDER_REVIEW" "NONE" "CURRENT"


type
string (VendorTypeEnum)

Vendor type

Enum: "VENDOR" "SUPPLIER" "CONTRACTOR" "PARTNER" "OTHER" "NONE"


types[]
Array of strings (VendorTypeEnum)

Filter to Vendors of any of these types (multi-select is OR)

Items Enum: "VENDOR" "SUPPLIER" "CONTRACTOR" "PARTNER" "OTHER" "NONE"


isSubProcessor
boolean

Filter to sub-processor vendors (true) or non-sub-processors (false)

Example: isSubProcessor=true


vendorTypeIds[]
Array of numbers

Filter to Vendors of any of these vendor-type ids (the tenant-defined vendor-type taxonomy from vendor settings; resolve names via GET /vendor-types). Multi-select is OR.

Example: vendorTypeIds[]=1&vendorTypeIds[]=2


nextReviewDeadlineStatus[]
Array of strings (NextReviewDeadlineStatusEnum)

Filter by next-review-deadline status: NO_RENEWAL / DUE_SOON / OVERDUE (multi-select is OR)

Items Enum: "NO_RENEWAL" "DUE_SOON" "OVERDUE"


actionRequired
boolean

Filter to "action required" vendors: current (not prospective/archived), with a directory entry and an owner email, whose renewal is due soon or overdue

Example: actionRequired=true


vendorUserIds[]
Array of numbers

Filter to Vendors whose security owner is one of these user IDs. Mirrors the security-owner filter in the app.

Example: vendorUserIds[]=1&vendorUserIds[]=2&vendorUserIds[]=3
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

GET
/vendors
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
    `https://public-api.drata.com/public/v2/vendors`,
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
"name": "Acme",
"category": "ENGINEERING",
"risk": "MODERATE",
"type": "CONTRACTOR",
"critical": false,
"status": "ACTIVE",
"location": "USA",
"url": "https://acme.com",
"privacyUrl": "config.get('swagger.examples.url')/privacy",
"termsUrl": "config.get('swagger.examples.url')/terms-of-service",
"trustCenterUrl": "https://trust.example.com",
"trustCenterProvider": "DRATA",
"servicesProvided": "Perform security scans once a month",
"dataStored": "Resulting reports of security scans",
"hasPii": true,
"passwordPolicy": "USERNAME_PASSWORD",
"passwordRequiresMinLength": true,
"passwordMinLength": 8,
"passwordRequiresNumber": true,
"passwordRequiresSymbol": true,
"passwordMfaEnabled": true,
"contactAtVendor": "John Doe",
"contactsEmail": "jdoe@company.com",
"notes": "Meeting once a month to adjust contract",
"logoUrl": "https://cdn-prod.imgpilot.com/logo.png",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"isSubProcessor": false,
"isSubProcessorActive": false,
"archivedAt": "2025-07-01T16:45:55.246Z",
"archivedAs": "ACTIVE",
"renewalDate": "2020-07-06",
"renewalScheduleType": "ONE_YEAR",
"renewalDateStatus": "COMPLETED",
"confirmedAt": "2025-07-01T16:45:55.246Z",
"sharedAccountId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd",
"isDrataUser": false,
"events": 4,
"integrations":
[
{
"id": 1,
"name": "Acme"
}
],
"cost": "1088",
"operationalImpact": "CRITICAL",
"environmentAccess": "READ_ONLY",
"impactLevel": "INSIGNIFICANT",
"dataAccessedOrProcessedList":
[
"string"
],
"inherentRiskLevelId": 5,
"residualRiskLevelId": 5,
"inherentRiskLevel":
{
"id": 12,
"name": "Critical",
"position": 1,
"isSystem": false
},
"residualRiskLevel":
{
"id": 12,
"name": "Critical",
"position": 1,
"isSystem": false
},
"vendorTypeId": 1,
"customVendorType":
{
"id": 1,
"name": "Vendor"
},
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
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
],
"lastQuestionnaire":
{
"vendorId": 1,
"sendAt": "2025-07-01T16:45:55.246Z",
"sentEmail": "jdoe@company.com",
"file": "questionnaire.pdf",
"respondedAt": "2025-07-01T16:45:55.246Z",
"responseId": 1,
"isManualUpload": true,
"completedBy": "Acme"
},
"latestSecurityReviews":
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
}
}
],
"vendorRelationshipContact":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"reviews":
[
{
"id": 1,
"updatedAt": "2025-07-01T16:45:55.246Z",
"reviewer": "John Doe",
"reviewDate": "2025-07-01T16:45:55.246Z",
"reportIssueDate": "2025-07-01T16:45:55.246Z",
"socReport": "SOC_1",
"socReportType1": true,
"socReportType2": true,
"socType1StartDate": "2025-07-01T16:45:55.246Z",
"socType1EndDate": "2025-07-01T16:45:55.246Z",
"socType2StartDate": "2025-07-01T16:45:55.246Z",
"socType2EndDate": "2025-07-01T16:45:55.246Z",
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "User must proceed to...",
"hasMaterialImpact": true,
"cpaFirm": "CPA firm name",
"cpaProcedurePerformed": "The following procedures were performed...",
"subserviceOrganization": "Subservice Inc.",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "The following procedures were performed...",
"trustServiceCategories":
[
{
"id": 1,
"category": "AVAILABILITY"
}
],
"userControls":
[
{
"id": 1,
"name": "End User Control 1",
"inPlace": true
}
],
"services":
[
{
"id": 1,
"name": "Service 1"
}
],
"locations":
[
{
"id": 1,
"city": "San Diego",
"stateCountry": "CA"
}
],
"findings":
[
{
"id": 1,
"description": "Finding 1"
}
]
}
],
"customFields":
[
{
"customFieldId": 1,
"name": "Stakeholders",
"value": "Security & IT"
}
],
"_links":
{
"self":
{
"href": "https://app.drata.com/risk/vendors/profile/1/overview"
}
},
"catalogProductCandidateId": 1
}
],
"pagination":
{
"cursor": "string",
"totalCount": 0
}
}
Create Vendor

Create a new Vendor

🔒 Requires Vendors: Create Vendor permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
REQUEST BODY SCHEMA: application/json
required
name
required

string <= 191 characters

The name of the Vendor



hasPii
boolean
Default: false

Indicates whether this Vendor stores any type of Personally Identifiable Information (PII)



passwordRequiresNumber
boolean
Default: false

Indicates whether a password requires numbers



passwordRequiresSymbol
boolean
Default: false

Indicates whether a password requires non-alpha-numeric characters



passwordMfaEnabled
boolean
Default: false

Indicates whether multi-factor authentication is enabled for this Vendor



passwordRequiresMinLength
boolean
Default: false

Indicates whether there is a minimum length requirement for password



isSubProcessor
boolean
Default: false

Indicates whether this Vendor is considered a subprocessor



isSubProcessorActive
boolean
Deprecated
Default: false

[Deprecated] Indicates whether this subprocessor is active



category
string or null

The type of Vendor

Enum: "ENGINEERING" "PRODUCT" "MARKETING" "CS" "SALES" "FINANCE" "HR" "ADMINISTRATIVE" "SECURITY" "LEGAL" "INFORMATION_TECHNOLOGY" "NONE"


risk
string

The level of risk associated with customer data

Enum: "NONE" "LOW" "MODERATE" "HIGH"


status
string or null

The status of Vendor

Enum: "PROSPECTIVE" "ACTIVE" "ARCHIVED" "APPROVED" "REJECTED" "FLAGGED" "ON_HOLD" "OFFBOARDED" "UNDER_REVIEW" "NONE"


critical
boolean or null

Indicates if the Vendor is considered critical



userId
number or null <= 1000000000

The user ID of the person responsible for Vendor compliance



url
string or null <uri> <= 768 characters

Vendor URL



privacyUrl
string <uri> <= 768 characters

Vendor Privacy Policy URL



termsUrl
string <uri> <= 768 characters

Vendor Terms of Use URL



trustCenterUrl
string <uri> <= 768 characters

Vendor Trust Center URL



servicesProvided
string or null <= 30000 characters

Description of the services provided by the Vendor



dataStored
string or null <= 30000 characters

Description of the type of data the Vendor stores



location
string <= 30000 characters

Location where the Vendor services are provided



passwordPolicy
string or null

The Vendor password policy

Enum: "USERNAME_PASSWORD" "SSO" "LDAP" "NONE" "NOT_APPLICABLE" "SCIM" "OTHER"


passwordMinLength
number or null [ 6 .. 12 ]

Minimum character length required for a password



contactAtVendor
string or null <= 191 characters

Name of the corresponding account manager for this Vendor



contactEmail
string or null <= 191 characters

Semicolon-separated list of vendor contact email addresses



notes
string <= 30000 characters

Additional notes for Vendor



renewalDate
string or null

Vendor renewal date



renewalScheduleType
string or null

Vendor renewal schedule type

Enum: "ONE_MONTH" "TWO_MONTHS" "THREE_MONTHS" "SIX_MONTHS" "ONE_YEAR" "CUSTOM" "NONE"


confirmed
boolean or null

Indicate if all Vendor data is confirmed



type
string or null

Vendor type identifier

Enum: "VENDOR" "SUPPLIER" "CONTRACTOR" "PARTNER" "OTHER" "NONE"


accountId
string <= 36 characters

Account Id



operationalImpact
string or null

Vendor level of operational impact

Enum: "NONE" "LOW" "NORMAL" "IMPORTANT" "CRITICAL"


environmentAccess
string or null

Vendor environment access privileges

Enum: "NO" "READ_ONLY" "READ_WRITE"


impactLevel
string or null

Vendor overall impact level

Enum: "INSIGNIFICANT" "MINOR" "MODERATE" "MAJOR" "CRITICAL" "UNSCORED"


residualRiskLevelId
number or null <= 1000000000

Custom residual risk level ID. Only honored when custom vendor risk levels are enabled.



inherentRiskLevelId
number or null <= 1000000000

Custom inherent risk level ID. Only honored when custom vendor risk levels are enabled.



dataAccessedOrProcessedList
Array of strings or null (VendorDataAccessedOrProcessedEnum) <= 300 items

List of data accessed or processed enum type

Enum: "GENERAL" "PUBLIC" "CONTROLLED_UNCLASSIFIED" "FINANCIAL" "PROPRIETARY" "EMPLOYEE_PERSONNEL" "PERSONAL_IDENTIFIABLE_INFORMATION" "PROTECTED_HEALTH_INFORMATION" "OTHER_PERSONAL_OR_SENSITIVE" "CARDHOLDER_DATA"


integrations
Array of numbers

List of vendor IDs



cost
string or null

Annual Contract Value for the Vendor in Cents unit



customFields

Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Vendor.

💎 Requires your account have the Custom Fields and Formulas feature. Contact your Account Manager for help upgrading.



vendorRelationshipContact

object or null

The Vendor relationship contact. Pass null to leave it unset (no contact will be assigned).

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

500

Internal server error

POST
/vendors
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
"name": "Acme",
"hasPii": true,
"passwordRequiresNumber": true,
"passwordRequiresSymbol": true,
"passwordMfaEnabled": true,
"passwordRequiresMinLength": true,
"isSubProcessor": false,
"isSubProcessorActive": false,
"category": "ENGINEERING",
"risk": "MODERATE",
"status": "UNDER_REVIEW",
"critical": false,
"userId": 1,
"url": "https://acme.com",
"privacyUrl": "https://acme.com/privacy",
"termsUrl": "https://acme.com/terms",
"trustCenterUrl": "https://trust.example.com",
"servicesProvided": "Perform security scans once a month",
"dataStored": "resulting reports of security scans",
"location": "San Diego",
"passwordPolicy": "USERNAME_PASSWORD",
"passwordMinLength": 8,
"contactAtVendor": "John Doe",
"contactEmail": "jdoe@company.com;jane@company.com",
"notes": "Meeting once a month to adjust contract",
"renewalDate": "2025-07-01T16:45:55.246Z",
"renewalScheduleType": "ONE_YEAR",
"confirmed": true,
"type": "VENDOR",
"accountId": 36,
"operationalImpact": "IMPORTANT",
"environmentAccess": "READ_ONLY",
"impactLevel": "INSIGNIFICANT",
"residualRiskLevelId": 1,
"inherentRiskLevelId": 1,
"dataAccessedOrProcessedList":
[
"FINANCIAL",
"GENERAL"
],
"integrations":
[
1,
2,
3
],
"cost": "1088",
"customFields":
[
{
"id": 1,
"name": "Compliance Status",
"value": "Security & IT"
}
],
"vendorRelationshipContact":
{
"userId": 1
}
}
Response samples
201
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
"id": 1,
"name": "Acme",
"category": "ENGINEERING",
"risk": "MODERATE",
"type": "CONTRACTOR",
"critical": false,
"status": "ACTIVE",
"location": "USA",
"url": "https://acme.com",
"privacyUrl": "config.get('swagger.examples.url')/privacy",
"termsUrl": "config.get('swagger.examples.url')/terms-of-service",
"trustCenterUrl": "https://trust.example.com",
"trustCenterProvider": "DRATA",
"servicesProvided": "Perform security scans once a month",
"dataStored": "Resulting reports of security scans",
"hasPii": true,
"passwordPolicy": "USERNAME_PASSWORD",
"passwordRequiresMinLength": true,
"passwordMinLength": 8,
"passwordRequiresNumber": true,
"passwordRequiresSymbol": true,
"passwordMfaEnabled": true,
"contactAtVendor": "John Doe",
"contactsEmail": "jdoe@company.com",
"notes": "Meeting once a month to adjust contract",
"logoUrl": "https://cdn-prod.imgpilot.com/logo.png",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"isSubProcessor": false,
"isSubProcessorActive": false,
"archivedAt": "2025-07-01T16:45:55.246Z",
"archivedAs": "ACTIVE",
"renewalDate": "2020-07-06",
"renewalScheduleType": "ONE_YEAR",
"renewalDateStatus": "COMPLETED",
"confirmedAt": "2025-07-01T16:45:55.246Z",
"sharedAccountId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd",
"isDrataUser": false,
"events": 4,
"integrations":
[
{
"id": 1,
"name": "Acme"
}
],
"cost": "1088",
"operationalImpact": "CRITICAL",
"environmentAccess": "READ_ONLY",
"impactLevel": "INSIGNIFICANT",
"dataAccessedOrProcessedList":
[
"string"
],
"inherentRiskLevelId": 5,
"residualRiskLevelId": 5,
"inherentRiskLevel":
{
"id": 12,
"name": "Critical",
"position": 1,
"isSystem": false
},
"residualRiskLevel":
{
"id": 12,
"name": "Critical",
"position": 1,
"isSystem": false
},
"vendorTypeId": 1,
"customVendorType":
{
"id": 1,
"name": "Vendor"
},
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
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
],
"lastQuestionnaire":
{
"vendorId": 1,
"sendAt": "2025-07-01T16:45:55.246Z",
"sentEmail": "jdoe@company.com",
"file": "questionnaire.pdf",
"respondedAt": "2025-07-01T16:45:55.246Z",
"responseId": 1,
"isManualUpload": true,
"completedBy": "Acme"
},
"latestSecurityReviews":
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
}
}
],
"vendorRelationshipContact":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"reviews":
[
{
"id": 1,
"updatedAt": "2025-07-01T16:45:55.246Z",
"reviewer": "John Doe",
"reviewDate": "2025-07-01T16:45:55.246Z",
"reportIssueDate": "2025-07-01T16:45:55.246Z",
"socReport": "SOC_1",
"socReportType1": true,
"socReportType2": true,
"socType1StartDate": "2025-07-01T16:45:55.246Z",
"socType1EndDate": "2025-07-01T16:45:55.246Z",
"socType2StartDate": "2025-07-01T16:45:55.246Z",
"socType2EndDate": "2025-07-01T16:45:55.246Z",
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "User must proceed to...",
"hasMaterialImpact": true,
"cpaFirm": "CPA firm name",
"cpaProcedurePerformed": "The following procedures were performed...",
"subserviceOrganization": "Subservice Inc.",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "The following procedures were performed...",
"trustServiceCategories":
[
{
"id": 1,
"category": "AVAILABILITY"
}
],
"userControls":
[
{
"id": 1,
"name": "End User Control 1",
"inPlace": true
}
],
"services":
[
{
"id": 1,
"name": "Service 1"
}
],
"locations":
[
{
"id": 1,
"city": "San Diego",
"stateCountry": "CA"
}
],
"findings":
[
{
"id": 1,
"description": "Finding 1"
}
]
}
],
"customFields":
[
{
"customFieldId": 1,
"name": "Stakeholders",
"value": "Security & IT"
}
],
"_links":
{
"self":
{
"href": "https://app.drata.com/risk/vendors/profile/1/overview"
}
},
"catalogProductCandidateId": 1
}
Search Vendors

Vendor listing with full-text search via q and a richer filter, sort, and facet surface than GET /vendors. Results are eventually consistent; use GET /vendors if read-your-writes freshness is required.

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
string (VendorSortEnum)

Which field to sort by

Enum: "createdAt" "updatedAt" "name" "renewalDate"


sortDir
string (SortDirectionEnum)

The direction to sort the data. Defaults to DESC (newest/highest first); an empty value falls back to that same default.

Enum: "ASC" "DESC"


includeTotalCount
boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null). The count is capped at 10,000; larger result sets report 10000.

Example: includeTotalCount=false


expand[]
Array of strings (VendorSearchExpandEnum)

List of subcollections and sub-objects to expand

Items Enum: "customFields" "documents" "latestSecurityReviews" "vendorUser" "lastQuestionnaire"


facets[]
Array of strings (VendorSearchFacetEnum)

Filter dimensions for which to return value counts in the response. Only honored on the first page (when cursor is null). Each requested facet adds one terms aggregation to the OpenSearch query. Bounded enum facets are enriched with zero-count entries; unbounded facets (like vendorTypeId) return only values present in the result set.

Items Enum: "status" "category" "vendorTypeId" "risk" "impactLevel" "isSubProcessor" "securityReviewStatus" "securityReviewDecision" "scheduledQuestionnaireStatus"


q
string <= 191 characters

Free-text search across the Vendor name and servicesProvided. Matches whole words anywhere in those fields, case-insensitive and typo-tolerant, and completes a partial trailing word. Results are relevance-ranked.

Example: q=Acme


status[]
Array of strings (VendorStatusEnum)

Filter to Vendors in any of these statuses (multi-select is OR). When not supplied, PROSPECTIVE Vendors are excluded by default.

Items Enum: "PROSPECTIVE" "ACTIVE" "ARCHIVED" "APPROVED" "REJECTED" "FLAGGED" "ON_HOLD" "OFFBOARDED" "UNDER_REVIEW" "NONE"


category[]
Array of strings (VendorCategoryEnum)

Filter to Vendors in any of these categories (business unit; multi-select is OR).

Items Enum: "ENGINEERING" "PRODUCT" "MARKETING" "CS" "SALES" "FINANCE" "HR" "ADMINISTRATIVE" "SECURITY" "LEGAL" "INFORMATION_TECHNOLOGY" "NONE"


vendorTypeIds[]
Array of numbers

Filter to Vendors of any of these vendor-type ids (the tenant-defined vendor-type taxonomy; resolve names via GET /vendor-types). Multi-select is OR.

Example: vendorTypeIds[]=1&vendorTypeIds[]=2


risk
string (VendorRiskEnum)

Filter to Vendors of this risk level

Enum: "NONE" "LOW" "MODERATE" "HIGH"


impactLevel
string (VendorImpactLevelEnum)

Overall Vendor impact level

Enum: "INSIGNIFICANT" "MINOR" "MODERATE" "MAJOR" "CRITICAL" "UNSCORED"


isSubProcessor
boolean

Filter to sub-processor vendors (true) or non-sub-processors (false)

Example: isSubProcessor=true


renewalDateGte
string <date>

Lower bound for the Vendor renewal date, as a calendar date (YYYY-MM-DD, no time or zone). Compared date-to-date, so the bound is inclusive of the whole day.

Example: renewalDateGte=2020-07-06


renewalDateLte
string <date>

Upper bound for the Vendor renewal date, as a calendar date (YYYY-MM-DD, no time or zone). Compared date-to-date, so the bound is inclusive of the whole day.

Example: renewalDateLte=2020-07-06


renewalScheduleType
string (RenewalScheduleTypeEnum)

Vendor renewal schedule type

Enum: "ONE_MONTH" "TWO_MONTHS" "THREE_MONTHS" "SIX_MONTHS" "ONE_YEAR" "CUSTOM" "NONE"


nextReviewDeadlineStatus[]
Array of strings (NextReviewDeadlineStatusEnum)

Filter by next-review-deadline status (multi-select is OR). These are computed at query time from renewalScheduleType and the renewal date, not stored: DUE_SOON and OVERDUE are relative to today, and NO_RENEWAL covers both a disabled recurring schedule and no renewal date set.

Items Enum: "NO_RENEWAL" "DUE_SOON" "OVERDUE"


actionRequired
boolean

Filter to "action required" vendors: current (not PROSPECTIVE/ARCHIVED), with a directory entry and an owner email, whose renewal date is on or before the end of the due-soon window (today + the standard renewal horizon). Independent of renewalScheduleType: it matches purely on the renewal date, so it can include vendors that nextReviewDeadlineStatus[] would classify as NO_RENEWAL.

Example: actionRequired=true


vendorUserIds[]
Array of numbers

Filter to Vendors whose security owner is one of these User ids (resolvable via GET /users), matched against the Vendor security owner. Same dimension as vendorUserIds[] on GET /vendors.

Example: vendorUserIds[]=1&vendorUserIds[]=2&vendorUserIds[]=3


securityReviewStatus[]
Array of strings (VendorSecurityReviewStatusEnum)

Filter Vendors by one or more security review statuses (multi-select is OR).

Items Enum: "NOT_YET_STARTED" "IN_PROGRESS" "COMPLETED" "NOT_REQUIRED"


securityReviewDecision[]
Array of strings (VendorSecurityReviewDecisionEnum)

Filter Vendors by one or more security review decisions (multi-select is OR).

Items Enum: "PENDING" "APPROVED" "APPROVED_WITH_CONDITIONS" "REJECTED"


scheduledQuestionnaireStatus[]
Array of strings (ScheduledQuestionnaireStatusEnum)

Filter Vendors by scheduled questionnaire status: ENABLED / DISABLED (multi-select is OR).

Items Enum: "ENABLED" "DISABLED"


customFields

Array of objects (CustomFieldFilterPublicV2Dto)

Filter Vendors by Custom Field equality. Repeat the parameter with bracketed indices, e.g. customFields[0][customFieldId]=42&customFields[0][value]=Financial (or customFields[0][name]=Tier in place of customFieldId). Multiple entries are AND-ed.



customRangeFields

Array of objects (CustomRangeFieldFilterPublicV2Dto)

Filter Vendors by Custom Field range. Repeat the parameter with bracketed indices, e.g. customRangeFields[0][customFieldId]=99&customRangeFields[0][gte]=5&customRangeFields[0][lte]=10 (or customRangeFields[0][name]=Score in place of customFieldId). At least one of gte / lte must be supplied per entry. Multiple entries are AND-ed.

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
/vendors-search
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
    `https://public-api.drata.com/public/v2/vendors-search`,
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
"name": "Acme",
"category": "ENGINEERING",
"risk": "MODERATE",
"type": "CONTRACTOR",
"critical": false,
"status": "ACTIVE",
"location": "USA",
"url": "https://acme.com",
"privacyUrl": "config.get('swagger.examples.url')/privacy",
"termsUrl": "config.get('swagger.examples.url')/terms-of-service",
"trustCenterUrl": "https://trust.example.com",
"trustCenterProvider": "DRATA",
"servicesProvided": "Perform security scans once a month",
"dataStored": "Resulting reports of security scans",
"hasPii": true,
"passwordPolicy": "USERNAME_PASSWORD",
"passwordRequiresMinLength": true,
"passwordMinLength": 8,
"passwordRequiresNumber": true,
"passwordRequiresSymbol": true,
"passwordMfaEnabled": true,
"contactAtVendor": "John Doe",
"contactsEmail": "jdoe@company.com",
"notes": "Meeting once a month to adjust contract",
"logoUrl": "https://cdn-prod.imgpilot.com/logo.png",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"isSubProcessor": false,
"archivedAt": "2025-07-01T16:45:55.246Z",
"renewalDate": "2020-07-06",
"renewalScheduleType": "ONE_YEAR",
"renewalDateStatus": "COMPLETED",
"confirmedAt": "2025-07-01T16:45:55.246Z",
"sharedAccountId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd",
"isDrataUser": false,
"cost": "1088",
"operationalImpact": "CRITICAL",
"environmentAccess": "READ_ONLY",
"impactLevel": "INSIGNIFICANT",
"vendorUser":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"lastQuestionnaire":
{
"vendorId": 1,
"sendAt": "2025-07-01T16:45:55.246Z",
"sentEmail": "jdoe@company.com",
"file": "questionnaire.pdf",
"respondedAt": "2025-07-01T16:45:55.246Z",
"responseId": 1,
"isManualUpload": true,
"completedBy": "Acme"
},
"documents":
{
"data":
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
],
"totalCount": 3
},
"latestSecurityReviews":
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
}
}
],
"totalCount": 3
},
"customFields":
{
"data":
[
{
"customFieldId": 1,
"name": "Stakeholders",
"value": "Security & IT"
}
],
"totalCount": 8
}
}
],
"pagination":
{
"cursor": "string",
"totalCount": 0
},
"facets":
{
"status":
[
{
"value": "ACTIVE",
"count": 42
},
{
"value": "UNDER_REVIEW",
"count": 8
},
{
"value": "ARCHIVED",
"count": 3
}
]
}
}
Get Vendor Statistics

Retrieve vendor statistics for specified scopes. At least one expand parameter is required. By default, archived vendors are excluded unless includeArchived is set to true.

🔒 Requires Vendors: Get Vendors Statistics permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
QUERY PARAMETERS
expand[]
required

Array of strings (VendorStatsExpandEnum)

List of stat scopes to include. At least one scope is required.

Items Enum: "reminder" "hasPii" "businessUnits" "passwordPolicy" "status" "isCritical" "isSubProcessor" "type" "risk" "impactLevel"


includeArchived
boolean
Default: false

Whether to include archived vendors in statistics. Defaults to false (excludes archived vendors).

Example: includeArchived=false
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

GET
/vendors-stats
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
  const query = new URLSearchParams({'expand[]': 'reminder'}).toString();

  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors-stats?${query}`,
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
"reminder":
[
{
"key": "RENEWAL_DUE",
"count": 1
},
{
"key": "RENEWAL_DUE_SOON",
"count": 1
}
],
"hasPii":
[
{
"key": true,
"count": 1
},
{
"key": false,
"count": 1
}
],
"businessUnits":
[
{
"key": "ENGINEERING",
"count": 118
},
{
"key": "PRODUCT",
"count": 2
}
],
"passwordPolicy":
[
{
"key": "USERNAME_PASSWORD",
"count": 118
},
{
"key": "SSO",
"count": 32
}
],
"status":
[
{
"key": "ACTIVE",
"count": 118
},
{
"key": "UNDER_REVIEW",
"count": 20
}
],
"isCritical":
[
{
"key": "Yes",
"count": 10
},
{
"key": "No",
"count": 20
}
],
"isSubProcessor":
[
{
"key": true,
"count": 10
},
{
"key": false,
"count": 20
}
],
"type":
[
{
"key": "CONTRACTOR",
"count": 15
},
{
"key": "VENDOR",
"count": 45
}
],
"risk":
[
{
"key": "HIGH",
"count": 15
},
{
"key": "MODERATE",
"count": 50
},
{
"key": "LOW",
"count": 45
},
{
"key": "NONE",
"count": 10
}
],
"impactLevel":
[
{
"key": "CRITICAL",
"count": 7
},
{
"key": "MAJOR",
"count": 12
},
{
"key": "UNSCORED",
"count": 0
}
]
}
Get Vendor

🔒 Requires Vendors: Get Vendor permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number
QUERY PARAMETERS
expand[]
Array of strings (VendorExpandEnum)

List of subcollections and sub-objects to expand

Items Enum: "customFields" "documents" "integrations" "lastQuestionnaire" "latestSecurityReviews" "reviews" "vendorUser" "vendorRelationshipContact" "dataAccessedOrProcessed" "scheduleConfiguration" "customVendorType" "inherentRiskLevel" "residualRiskLevel"
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

GET
/vendors/{vendorId}
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
    `https://public-api.drata.com/public/v2/vendors/${vendorId}`,
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
"id": 1,
"name": "Acme",
"category": "ENGINEERING",
"risk": "MODERATE",
"type": "CONTRACTOR",
"critical": false,
"status": "ACTIVE",
"location": "USA",
"url": "https://acme.com",
"privacyUrl": "config.get('swagger.examples.url')/privacy",
"termsUrl": "config.get('swagger.examples.url')/terms-of-service",
"trustCenterUrl": "https://trust.example.com",
"trustCenterProvider": "DRATA",
"servicesProvided": "Perform security scans once a month",
"dataStored": "Resulting reports of security scans",
"hasPii": true,
"passwordPolicy": "USERNAME_PASSWORD",
"passwordRequiresMinLength": true,
"passwordMinLength": 8,
"passwordRequiresNumber": true,
"passwordRequiresSymbol": true,
"passwordMfaEnabled": true,
"contactAtVendor": "John Doe",
"contactsEmail": "jdoe@company.com",
"notes": "Meeting once a month to adjust contract",
"logoUrl": "https://cdn-prod.imgpilot.com/logo.png",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"isSubProcessor": false,
"isSubProcessorActive": false,
"archivedAt": "2025-07-01T16:45:55.246Z",
"archivedAs": "ACTIVE",
"renewalDate": "2020-07-06",
"renewalScheduleType": "ONE_YEAR",
"renewalDateStatus": "COMPLETED",
"confirmedAt": "2025-07-01T16:45:55.246Z",
"sharedAccountId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd",
"isDrataUser": false,
"events": 4,
"integrations":
[
{
"id": 1,
"name": "Acme"
}
],
"cost": "1088",
"operationalImpact": "CRITICAL",
"environmentAccess": "READ_ONLY",
"impactLevel": "INSIGNIFICANT",
"dataAccessedOrProcessedList":
[
"string"
],
"inherentRiskLevelId": 5,
"residualRiskLevelId": 5,
"inherentRiskLevel":
{
"id": 12,
"name": "Critical",
"position": 1,
"isSystem": false
},
"residualRiskLevel":
{
"id": 12,
"name": "Critical",
"position": 1,
"isSystem": false
},
"vendorTypeId": 1,
"customVendorType":
{
"id": 1,
"name": "Vendor"
},
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
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
],
"lastQuestionnaire":
{
"vendorId": 1,
"sendAt": "2025-07-01T16:45:55.246Z",
"sentEmail": "jdoe@company.com",
"file": "questionnaire.pdf",
"respondedAt": "2025-07-01T16:45:55.246Z",
"responseId": 1,
"isManualUpload": true,
"completedBy": "Acme"
},
"latestSecurityReviews":
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
}
}
],
"vendorRelationshipContact":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"reviews":
[
{
"id": 1,
"updatedAt": "2025-07-01T16:45:55.246Z",
"reviewer": "John Doe",
"reviewDate": "2025-07-01T16:45:55.246Z",
"reportIssueDate": "2025-07-01T16:45:55.246Z",
"socReport": "SOC_1",
"socReportType1": true,
"socReportType2": true,
"socType1StartDate": "2025-07-01T16:45:55.246Z",
"socType1EndDate": "2025-07-01T16:45:55.246Z",
"socType2StartDate": "2025-07-01T16:45:55.246Z",
"socType2EndDate": "2025-07-01T16:45:55.246Z",
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "User must proceed to...",
"hasMaterialImpact": true,
"cpaFirm": "CPA firm name",
"cpaProcedurePerformed": "The following procedures were performed...",
"subserviceOrganization": "Subservice Inc.",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "The following procedures were performed...",
"trustServiceCategories":
[
{
"id": 1,
"category": "AVAILABILITY"
}
],
"userControls":
[
{
"id": 1,
"name": "End User Control 1",
"inPlace": true
}
],
"services":
[
{
"id": 1,
"name": "Service 1"
}
],
"locations":
[
{
"id": 1,
"city": "San Diego",
"stateCountry": "CA"
}
],
"findings":
[
{
"id": 1,
"description": "Finding 1"
}
]
}
],
"customFields":
[
{
"customFieldId": 1,
"name": "Stakeholders",
"value": "Security & IT"
}
],
"_links":
{
"self":
{
"href": "https://app.drata.com/risk/vendors/profile/1/overview"
}
},
"catalogProductCandidateId": 1
}
Update Vendor

Update Vendor details

🔒 Requires Vendors: Update Vendor permission.

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
name
string <= 191 characters

The name of the Vendor



hasPii
boolean

Indicates whether this Vendor stores any type of Personally Identifiable Information (PII)



passwordRequiresNumber
boolean

Indicates whether a password requires numbers



passwordRequiresSymbol
boolean

Indicates whether a password requires non-alpha-numeric characters



passwordMfaEnabled
boolean

Indicates whether multi-factor authentication is enabled for this Vendor



passwordRequiresMinLength
boolean

Indicates whether there is a minimum length requirement for password



isSubProcessor
boolean

Indicates whether this Vendor is considered a subprocessor



isSubProcessorActive
boolean
Deprecated

[Deprecated] Indicates whether this subprocessor is active



category
string or null

The type of Vendor

Enum: "ENGINEERING" "PRODUCT" "MARKETING" "CS" "SALES" "FINANCE" "HR" "ADMINISTRATIVE" "SECURITY" "LEGAL" "INFORMATION_TECHNOLOGY" "NONE"


risk
string

The level of risk associated with customer data

Enum: "NONE" "LOW" "MODERATE" "HIGH"


status
string or null

The status of Vendor

Enum: "PROSPECTIVE" "ACTIVE" "ARCHIVED" "APPROVED" "REJECTED" "FLAGGED" "ON_HOLD" "OFFBOARDED" "UNDER_REVIEW" "NONE"


critical
boolean or null

Indicates if the Vendor is considered critical



userId
number or null <= 1000000000

The user ID of the person responsible for Vendor compliance



url
string or null <uri> <= 768 characters

Vendor URL



privacyUrl
string <uri> <= 768 characters

Vendor Privacy Policy URL



termsUrl
string <uri> <= 768 characters

Vendor Terms of Use URL



trustCenterUrl
string <uri> <= 768 characters

Vendor Trust Center URL



servicesProvided
string or null <= 30000 characters

Description of the services provided by the Vendor



dataStored
string or null <= 30000 characters

Description of the type of data the Vendor stores



location
string <= 30000 characters

Location where the Vendor services are provided



passwordPolicy
string or null

The Vendor password policy

Enum: "USERNAME_PASSWORD" "SSO" "LDAP" "NONE" "NOT_APPLICABLE" "SCIM" "OTHER"


passwordMinLength
number or null [ 6 .. 12 ]

Minimum character length required for a password



contactAtVendor
string or null <= 191 characters

Name of the corresponding account manager for this Vendor



contactsEmail
string or null <= 191 characters

Semicolon-separated list of vendor contact email addresses



notes
string <= 30000 characters

Additional notes for Vendor



renewalDate
string or null

Vendor renewal date



renewalScheduleType
string or null

Vendor renewal schedule type

Enum: "ONE_MONTH" "TWO_MONTHS" "THREE_MONTHS" "SIX_MONTHS" "ONE_YEAR" "CUSTOM" "NONE"


confirmed
boolean or null

Indicate if all Vendor data is confirmed



type
string or null

Vendor type identifier

Enum: "VENDOR" "SUPPLIER" "CONTRACTOR" "PARTNER" "OTHER" "NONE"


vendorTypeId
number or null <= 1000000000

Custom vendor type ID. Only honored when custom vendor types are enabled. Mutually exclusive with "type".



accountId
string <= 36 characters

Account Id



operationalImpact
string or null

Vendor level of operational impact

Enum: "NONE" "LOW" "NORMAL" "IMPORTANT" "CRITICAL"


environmentAccess
string or null

Vendor environment access privileges

Enum: "NO" "READ_ONLY" "READ_WRITE"


impactLevel
string or null

Vendor overall impact level

Enum: "INSIGNIFICANT" "MINOR" "MODERATE" "MAJOR" "CRITICAL" "UNSCORED"


residualRiskLevelId
number or null <= 1000000000

Custom residual risk level ID. Only honored when custom vendor risk levels are enabled.



inherentRiskLevelId
number or null <= 1000000000

Custom inherent risk level ID. Only honored when custom vendor risk levels are enabled.



dataAccessedOrProcessedList
Array of strings or null (VendorDataAccessedOrProcessedEnum) <= 300 items

List of data accessed or processed enum type

Enum: "GENERAL" "PUBLIC" "CONTROLLED_UNCLASSIFIED" "FINANCIAL" "PROPRIETARY" "EMPLOYEE_PERSONNEL" "PERSONAL_IDENTIFIABLE_INFORMATION" "PROTECTED_HEALTH_INFORMATION" "OTHER_PERSONAL_OR_SENSITIVE" "CARDHOLDER_DATA"


integrations
Array of numbers

List of vendor IDs



cost
string or null

Annual Contract Value for the Vendor in cents unit



customFields

Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Vendor.

💎 Requires your account have the Custom Fields and Formulas feature. Contact your Account Manager for help upgrading.



vendorRelationshipContact

object or null

The Vendor relationship contact. Pass null to clear the currently assigned contact.

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
/vendors/{vendorId}
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
"name": "Acme",
"hasPii": true,
"passwordRequiresNumber": true,
"passwordRequiresSymbol": true,
"passwordMfaEnabled": true,
"passwordRequiresMinLength": true,
"isSubProcessor": false,
"isSubProcessorActive": false,
"category": "ENGINEERING",
"risk": "MODERATE",
"status": "UNDER_REVIEW",
"critical": false,
"userId": 1,
"url": "https://acme.com",
"privacyUrl": "https://acme.com/privacy",
"termsUrl": "https://acme.com/terms",
"trustCenterUrl": "https://trust.example.com",
"servicesProvided": "Perform security scans once a month",
"dataStored": "resulting reports of security scans",
"location": "San Diego",
"passwordPolicy": "USERNAME_PASSWORD",
"passwordMinLength": 8,
"contactAtVendor": "John Doe",
"contactsEmail": "jdoe@company.com;jane@company.com",
"notes": "Meeting once a month to adjust contract",
"renewalDate": "2025-07-01T16:45:55.246Z",
"renewalScheduleType": "ONE_YEAR",
"confirmed": true,
"type": "VENDOR",
"vendorTypeId": 1,
"accountId": 36,
"operationalImpact": "IMPORTANT",
"environmentAccess": "READ_ONLY",
"impactLevel": "INSIGNIFICANT",
"residualRiskLevelId": 1,
"inherentRiskLevelId": 1,
"dataAccessedOrProcessedList":
[
"FINANCIAL",
"GENERAL"
],
"integrations":
[
1,
2,
3
],
"cost": "1088",
"customFields":
[
{
"id": 1,
"name": "Compliance Status",
"value": "Security & IT"
}
],
"vendorRelationshipContact":
{
"userId": 1
}
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
"name": "Acme",
"category": "ENGINEERING",
"risk": "MODERATE",
"type": "CONTRACTOR",
"critical": false,
"status": "ACTIVE",
"location": "USA",
"url": "https://acme.com",
"privacyUrl": "config.get('swagger.examples.url')/privacy",
"termsUrl": "config.get('swagger.examples.url')/terms-of-service",
"trustCenterUrl": "https://trust.example.com",
"trustCenterProvider": "DRATA",
"servicesProvided": "Perform security scans once a month",
"dataStored": "Resulting reports of security scans",
"hasPii": true,
"passwordPolicy": "USERNAME_PASSWORD",
"passwordRequiresMinLength": true,
"passwordMinLength": 8,
"passwordRequiresNumber": true,
"passwordRequiresSymbol": true,
"passwordMfaEnabled": true,
"contactAtVendor": "John Doe",
"contactsEmail": "jdoe@company.com",
"notes": "Meeting once a month to adjust contract",
"logoUrl": "https://cdn-prod.imgpilot.com/logo.png",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z",
"isSubProcessor": false,
"isSubProcessorActive": false,
"archivedAt": "2025-07-01T16:45:55.246Z",
"archivedAs": "ACTIVE",
"renewalDate": "2020-07-06",
"renewalScheduleType": "ONE_YEAR",
"renewalDateStatus": "COMPLETED",
"confirmedAt": "2025-07-01T16:45:55.246Z",
"sharedAccountId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd",
"isDrataUser": false,
"events": 4,
"integrations":
[
{
"id": 1,
"name": "Acme"
}
],
"cost": "1088",
"operationalImpact": "CRITICAL",
"environmentAccess": "READ_ONLY",
"impactLevel": "INSIGNIFICANT",
"dataAccessedOrProcessedList":
[
"string"
],
"inherentRiskLevelId": 5,
"residualRiskLevelId": 5,
"inherentRiskLevel":
{
"id": 12,
"name": "Critical",
"position": 1,
"isSystem": false
},
"residualRiskLevel":
{
"id": 12,
"name": "Critical",
"position": 1,
"isSystem": false
},
"vendorTypeId": 1,
"customVendorType":
{
"id": 1,
"name": "Vendor"
},
"user":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
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
],
"lastQuestionnaire":
{
"vendorId": 1,
"sendAt": "2025-07-01T16:45:55.246Z",
"sentEmail": "jdoe@company.com",
"file": "questionnaire.pdf",
"respondedAt": "2025-07-01T16:45:55.246Z",
"responseId": 1,
"isManualUpload": true,
"completedBy": "Acme"
},
"latestSecurityReviews":
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
}
}
],
"vendorRelationshipContact":
{
"id": 1,
"email": "email@example.com",
"firstName": "Sally",
"lastName": "Smith",
"createdAt": "2025-07-01T16:45:55.246Z",
"updatedAt": "2025-07-01T16:45:55.246Z"
},
"reviews":
[
{
"id": 1,
"updatedAt": "2025-07-01T16:45:55.246Z",
"reviewer": "John Doe",
"reviewDate": "2025-07-01T16:45:55.246Z",
"reportIssueDate": "2025-07-01T16:45:55.246Z",
"socReport": "SOC_1",
"socReportType1": true,
"socReportType2": true,
"socType1StartDate": "2025-07-01T16:45:55.246Z",
"socType1EndDate": "2025-07-01T16:45:55.246Z",
"socType2StartDate": "2025-07-01T16:45:55.246Z",
"socType2EndDate": "2025-07-01T16:45:55.246Z",
"reportOpinion": "UNQUALIFIED",
"encompassBusinessNeeds": true,
"followUpActivity": "User must proceed to...",
"hasMaterialImpact": true,
"cpaFirm": "CPA firm name",
"cpaProcedurePerformed": "The following procedures were performed...",
"subserviceOrganization": "Subservice Inc.",
"subserviceOrganizationUsingInclusiveMethod": true,
"subserviceOrganizationProcedurePerformed": "The following procedures were performed...",
"trustServiceCategories":
[
{
"id": 1,
"category": "AVAILABILITY"
}
],
"userControls":
[
{
"id": 1,
"name": "End User Control 1",
"inPlace": true
}
],
"services":
[
{
"id": 1,
"name": "Service 1"
}
],
"locations":
[
{
"id": 1,
"city": "San Diego",
"stateCountry": "CA"
}
],
"findings":
[
{
"id": 1,
"description": "Finding 1"
}
]
}
],
"customFields":
[
{
"customFieldId": 1,
"name": "Stakeholders",
"value": "Security & IT"
}
],
"_links":
{
"self":
{
"href": "https://app.drata.com/risk/vendors/profile/1/overview"
}
},
"catalogProductCandidateId": 1
}
Remove Vendor

🔒 Requires Vendors: Delete Vendor permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number
Responses
204

No Content

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

DELETE
/vendors/{vendorId}
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
    `https://public-api.drata.com/public/v2/vendors/${vendorId}`,
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
401
402
403
412
500
application/json
CopyExpand allCollapse all
{
"statusCode": 0,
"message": "string",
"code": 0,
"debugInfo":
{
"name": "string",
"message": "string",
"stack": "string"
},
"providerError":
{
"provider": "string",
"status": 0,
"code": "string",
"message": "string",
"details": { }
}
}
List Vendor Questionnaires

Get Questionnaires sent to a Vendor.

🔒 Requires Vendors: List Vendor Questionnaire permission.

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

404

Not Found

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/questionnaires
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
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/questionnaires`,
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
"title": "Vendor Security Questionnaire",
"recipientEmail": "wc@example.com",
"isCompleted": true,
"completedBy": "Acme",
"responseId": 1,
"dateSent": "2025-07-01T16:45:55.246Z",
"isManualUpload": false
}
],
"pagination":
{
"cursor": "string",
"totalCount": 0
}
}
Send Questionnaire to Vendor

Send a Questionnaire to a Vendor by email.

🔒 Requires Vendors: Send Questionnaire to Vendor permission.

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
email
required

string <email> <= 191 characters

The email address to receive the Questionnaire



questionnaireId
required

number

ID of the Questionnaire Template to send. List the available templates with GET /vendor-questionnaire-templates.



emailSubject
string <= 191 characters

The email subject for the vendor questionnaire



emailContent
required

string <= 30000 characters

The email content for the Vendor



securityReviewId
required

number

Security Review ID to associate the Questionnaire with. When provided, the sent Questionnaire will be linked to the specified security review as a document of type QUESTIONNAIRE. List Security Reviews with GET /vendor-security-reviews or GET /vendors/{vendorId}/security-reviews.

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
/vendors/{vendorId}/questionnaires
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
"email": "vendor@example.com",
"questionnaireId": 1,
"emailSubject": "Drata is conducting a security review of you.",
"emailContent": "Hi,\n\nWe'd like to conduct a security review and would like some information from you. Use this link to complete the questionnaire.\n\nThank you.",
"securityReviewId": 1
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
Copy
{
"id": 1,
"title": "Vendor Security Questionnaire",
"recipientEmail": "wc@example.com",
"isCompleted": true,
"completedBy": "Acme",
"responseId": 1,
"dateSent": "2025-07-01T16:45:55.246Z",
"isManualUpload": false
}
List Vendor Questionnaire Questions

Get the questions of a Questionnaire sent to a Vendor. Each question includes its configuration (type, whether it is required, whether it allows multiple or "other" choices, and whether it has a conditional follow-up) and the answer the Vendor gave; questions the Vendor did not answer are returned with a null answer.

🔒 Requires Vendors: List Vendor Questionnaire permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number


questionnaireId
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

The direction to sort the data. Defaults to ascending so questions are returned in the order the Vendor was asked them.

Enum: "ASC" "DESC"


includeTotalCount
boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example: includeTotalCount=false
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

404

Not Found

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/questionnaires/{questionnaireId}/questions
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
  const questionnaireId = 'YOUR_questionnaireId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/questionnaires/${questionnaireId}/questions`,
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
"question": "Is there a publicly available URL for your Privacy Policy?",
"type": "YES_NO",
"required": true,
"allowMultipleSelection": false,
"allowOtherChoice": false,
"followUp": false,
"answer": "Yes, you can get to this right here!",
"choices":
[
"Yes",
"No"
],
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
Get Vendor Questionnaire

Get a specific Questionnaire sent to a Vendor.

🔒 Requires Vendors: List Vendor Questionnaire permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number


questionnaireId
required

number
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

404

Not Found

412

You must accept the Drata terms and conditions to use the API

500

Internal server error

GET
/vendors/{vendorId}/questionnaires/{questionnaireId}
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
  const questionnaireId = 'YOUR_questionnaireId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/questionnaires/${questionnaireId}`,
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
"id": 1,
"title": "Vendor Security Questionnaire",
"recipientEmail": "wc@example.com",
"isCompleted": true,
"completedBy": "Acme",
"responseId": 1,
"dateSent": "2025-07-01T16:45:55.246Z",
"isManualUpload": false
}
Get Vendor Trust Center Overview

Get the Trust Center overview summary for a given Vendor.

🔒 Requires Vendors: Get Vendor permission.

💎 Requires your account have the TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max feature. Contact your Account Manager for help upgrading.

Security
bearer or oauth2
Request
PATH PARAMETERS
vendorId
required

number
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

502

The upstream Trust Center provider could not be reached.

GET
/vendors/{vendorId}/trust-overview
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
    `https://public-api.drata.com/public/v2/vendors/${vendorId}/trust-overview`,
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
500
502
4 more
application/json
Copy
{
"overview": "Acme maintains a public trust center with current compliance reports and security documentation."
}
Next to Version Control User Identities
