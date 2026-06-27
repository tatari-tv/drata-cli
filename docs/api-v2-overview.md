Title: Drata API

URL Source: https://developers.drata.com/openapi/reference/v2/overview/

Markdown Content:
Download OpenAPI specification:[Download](https://developers.drata.com/openapi/reference/v2/overview/)

### What's New in V2

*   Support for Custom Fields
*   Get and acknowledge user's assigned policies
*   Payloads are streamlined to include only the essential information. You can expand related objects and collections using the `expand` query parameter.
*   Cursor-based pagination for greater efficiency and stability with large datasets

### Getting Started

Please visit our help article to learn how to [create an API key](https://help.drata.com/en/articles/6695964).

### Upgrading From Version 1

API V2 is designed to provide a faster an more flexible way of accessing Drata. There are several differences from V1:

*   The listing endpoints now use **cursor-based pagination**. This provides faster responses and ensures that all records can be retrieved even, if they are changed during the process.

```
async function fetchAll() {
  // The first request doesn't send a value for the cursor.
  let cursor = undefined;
  do {
    const query = new URLSearchParams({ cursor });
    const resp = await fetch(
      `https://public-api.drata.com/public/v2/users?${query}`,
      {
        method: 'GET',
        headers: { Authorization: 'Bearer <YOUR_API_KEY_HERE>' }
      }
    );
    const data = await resp.json();
    console.log(data);

    // If there's a cursor value returned, then there are more results.
    // Pass that back as a query parameter to get the next page of data.
    cursor = data?.pagination?.cursor;
  } while (cursor)
}
fetchAll();
```

*   You'll notice smaller responses. Most endpoints accept an `expand` query parameter that let's you specify which related objects and collections you want to expand.

## [](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets)Assets

Assets let you build an inventory of policies, personnel and computer infrastructure. The [help docs](https://help.drata.com/en/collections/10485424) have more information.

## [](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets)List Assets

Find Assets by search terms and filters.

🔒 Requires **Assets: List Assets** permission.

Security

**bearer**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Default:false

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=expand[]&t=request)expand[]Array of strings (AssetExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"device""assetClassTypes""complianceChecks""customFields""identifiers""owner"
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=assetClassType&t=request)assetClassType string (AssetClassTypeEnum)

Filter assets by class type. This filter is only supported in conjunction with expand[]=assetClassTypes.

Enum:"HARDWARE""POLICY""DOCUMENT""PERSONNEL""SOFTWARE""CODE""CONTAINER""COMPUTE""NETWORKING""DATABASE""STORAGE"
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=assetType&t=request)assetType string (AssetTypeEnum)

Filter assets by type

Enum:"PHYSICAL""VIRTUAL"
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=assetProvider&t=request)assetProvider string (AssetProviderEnum)

Filter assets by provider

Enum:"DRATA_DEV""AGENT""DRATA""GOOGLE""MICROSOFT_365""JAMF""INTUNE""OKTA_IDENTITY""KANDJI""JUMPCLOUD""HEXNODE_UEM""RIPPLING""AWS""MERGEDEV_ONELOGIN""MERGEDEV_JUMPCLOUD""WORKSPACE_ONE""CSV_IDP""AWS_GOV_CLOUD""AZURE""GCP""MICROSOFT_365_GCC_HIGH""MERGEDEV_CYBERARK""KOLIDE""MERGEDEV_PINGONE""AWS_ORG_UNITS""INTUNE_GCC_HIGH""CUSTOM_XFA""NINJAONE"
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=userId&t=request)userId number<= 1000000000

Filter data by user ID associated to the Assets

Example:userId=1
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_listAssets!in=query&path=employmentStatus&t=request)employmentStatus string (EmploymentStatusEnum)

Filter data by personnel of this employment status

| Enum Value | **Description** |
| --- | --- |
| CURRENT_EMPLOYEE | Current Employee |
| FORMER_EMPLOYEE | Former Employee |
| CURRENT_CONTRACTOR | Current Contractor |
| FORMER_CONTRACTOR | Former Contractor |
| FUTURE_HIRE | Future Hire – Based on the HRIS data |
| UNKNOWN | Unknown – The personnel did not match an HRIS record |
| OUT_OF_SCOPE | Out of Scope – Manually marked as out of scope |
| SERVICE_ACCOUNT | Service Account – Automatically marked as out of scope |
| SPECIAL_FORMER_EMPLOYEE | Special Former Employee – Deprecated status for manually created personnel |
| SPECIAL_FORMER_CONTRACTOR | Special Former Contractor – Deprecated status for manually created personnel |

Responses

200
Successful

401
Invalid Authorization

403
You are not allowed to perform this action

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/assets

Request samples

*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

2 more

Response samples

*   200
*   401
*   403
*   412
*   500

application/json

`{"data": [{"id": 1,"name": "MacBook Pro 16-inch","description": "MacBook Pro 16-inch with 14-Core CPU 32-Core GPU 36GB Unified Memory 1TB SSD Storage","assetType": "PHYSICAL","assetProvider": "AGENT","removedAt": "2025-07-01T16:45:55.246Z","assetClassTypes": "AssetClassTypeResponsePublicV2Dto[]","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","jobTitle": "CEO","drataTermsAgreedAt": "2025-07-01T16:45:55.246Z","roles": ["ROLE","ANOTHER_ROLE"],"backgroundChecks": [{"id": 0,"userId": 0,"status": "OK","caseId": "abc123","caseInvitationId": "abc123","manualCheckDate": "2020-07-06","manuallyCheckUrl": "url.com","type": "CERTN","source": "DRATA","reportData": "string","outOfScopeReason": "abc123","outOfScopeAt": "2025-07-01T16:45:55.246Z","invitationEmail": "email@email.com","linkedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"documents": [{"id": 1,"userId": 0,"downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="}},"name": "Security Training","type": "SEC_TRAINING","renewalDate": "2026-10-27","createdAt": "2020-07-06","updatedAt": "2020-07-06"}],"identities": [{"id": 1,"identityId": "1a2b3c","username": "johndoe","connectedAt": "2025-07-01T16:45:55.246Z","disconnectedAt": "2025-07-01T16:45:55.246Z","hasMfa": true,"userId": 1,"connectionId": 1,"connection": {"id": "1","clientType": "GOOGLE","clientId": "drata.com","clientAlias": "My-connection-alias-1","state": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectedAt": "2025-07-01T16:45:55.246Z","failedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"},"email": "johndoe@example.com","secondaryEmail": "johndoe@test.com","firstName": "John","lastName": "Doe","startedAt": "Fri Jun 26 2026","separatedAt": "Fri Jun 26 2026","isContractor": true,"jobTitle": "Engineer","managerId": "x00jk12-2312","managerName": "Jose Díaz","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]},"notes": "string","assetReferenceType": "PERSONNEL","associatedId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","device": {"id": 1,"assetId": 123,"osVersion": "MacOS 10.15.6","serialNumber": "C02T6CDJGTFL","model": "MacBook Pro","macAddress": "65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E","lastCheckedAt": "2025-07-01T16:45:55.246Z","sourceType": "AGENT","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z","appsCount": 20,"isDeviceCompliant": false,"complianceChecks": [ ],"identifiers": [ ],"documents": [ ],"screenLockTime": 30,"screenLockExplanation": {"minutesIdleBeforeSleep": "2","minutesSleepingBeforePasswordIsRequired": "2"},"antivirusEnabled": true,"antivirusExplanation": "No matching app was found","autoUpdateEnabled": true,"autoUpdateExplanation": "No compliances found","passwordManagerEnabled": true,"passwordManagerExplanation": {"passwordManagerApps": ["1password 7"]},"encryptionEnabled": false,"encryptionExplanation": "No encryption provided","firewallEnabled": true,"firewallExplanation": "{}","asset": {"id": 1,"name": "MacBook Pro - Space Black 16-inch","description": "MacBook Pro Space Black - with 16-inch Liquid Retina XDR display","assetType": "PHYSICAL","assetProvider": "AGENT","approvedAt": "2025-07-01T16:45:55.246Z","removedAt": "2025-07-01T16:45:55.246Z","company": "Acme, Inc","notes": "string","assetReferenceType": "PERSONNEL","uniqueId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353"},"userId": 1,"personnelId": 1,"externalId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd"},"externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353","customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset)Create Asset

Manually add a new Asset to the Account.

🔒 Requires **Assets: Create Asset** permission.

Security

**bearer**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset!path=name&t=request)name
required string<= 191 characters

The Asset name
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset!path=description&t=request)description
required string<= 191 characters

The Asset description
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset!path=notes&t=request)notes string<= 191 characters

The Asset notes
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset!path=assetClassTypes&t=request)assetClassTypes
required Array of strings (AssetClassTypeEnum)

The Asset class types

Items Enum:"HARDWARE""POLICY""DOCUMENT""PERSONNEL""SOFTWARE""CODE""CONTAINER""COMPUTE""NETWORKING""DATABASE""STORAGE"
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset!path=assetType&t=request)assetType
required string

The Asset type

Enum:"PHYSICAL""VIRTUAL"
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset!path=ownerId&t=request)ownerId
required number

The owner ID
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset!path=associatedId&t=request)associatedId string

An ID associated with this Asset
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset!path=removedAt&t=request)removedAt string<date-time>

Date the Asset was removed
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset!path=externalId&t=request)externalId string

An externally sourced unique identifier for a virtual Asset
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_createAsset!path=externalOwnerId&t=request)externalOwnerId string

Used to track the source of virtual Assets, typically an account ID
Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Asset.

💎 Requires your account have the Custom Fields and Formulas feature. Contact your CSM for help upgrading.

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

422
Unprocessable Entity

500
Internal server error

post/assets

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

`{"name": "string","description": "string","notes": "string","assetClassTypes": ["HARDWARE","PERSONNEL"],"assetType": "PHYSICAL","ownerId": 1,"associatedId": "C02T6CDJGTFL","removedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353","customFields": [{"id": 1,"name": "Compliance Status","value": "Security & IT"}]}`

Response samples

*   201
*   400
*   401
*   403
*   404
*   412
*   422
*   500

3 more

application/json

`{"id": 1,"name": "MacBook Pro 16-inch","description": "MacBook Pro 16-inch with 14-Core CPU 32-Core GPU 36GB Unified Memory 1TB SSD Storage","assetType": "PHYSICAL","assetProvider": "AGENT","removedAt": "2025-07-01T16:45:55.246Z","assetClassTypes": "AssetClassTypeResponsePublicV2Dto[]","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","jobTitle": "CEO","drataTermsAgreedAt": "2025-07-01T16:45:55.246Z","roles": ["ROLE","ANOTHER_ROLE"],"backgroundChecks": [{"id": 0,"userId": 0,"status": "OK","caseId": "abc123","caseInvitationId": "abc123","manualCheckDate": "2020-07-06","manuallyCheckUrl": "url.com","type": "CERTN","source": "DRATA","reportData": "string","outOfScopeReason": "abc123","outOfScopeAt": "2025-07-01T16:45:55.246Z","invitationEmail": "email@email.com","linkedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"documents": [{"id": 1,"userId": 0,"downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="}},"name": "Security Training","type": "SEC_TRAINING","renewalDate": "2026-10-27","createdAt": "2020-07-06","updatedAt": "2020-07-06"}],"identities": [{"id": 1,"identityId": "1a2b3c","username": "johndoe","connectedAt": "2025-07-01T16:45:55.246Z","disconnectedAt": "2025-07-01T16:45:55.246Z","hasMfa": true,"userId": 1,"connectionId": 1,"connection": {"id": "1","clientType": "GOOGLE","clientId": "drata.com","clientAlias": "My-connection-alias-1","state": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectedAt": "2025-07-01T16:45:55.246Z","failedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"},"email": "johndoe@example.com","secondaryEmail": "johndoe@test.com","firstName": "John","lastName": "Doe","startedAt": "Fri Jun 26 2026","separatedAt": "Fri Jun 26 2026","isContractor": true,"jobTitle": "Engineer","managerId": "x00jk12-2312","managerName": "Jose Díaz","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]},"notes": "string","assetReferenceType": "PERSONNEL","associatedId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","device": {"id": 1,"assetId": 123,"osVersion": "MacOS 10.15.6","serialNumber": "C02T6CDJGTFL","model": "MacBook Pro","macAddress": "65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E","lastCheckedAt": "2025-07-01T16:45:55.246Z","sourceType": "AGENT","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z","appsCount": 20,"isDeviceCompliant": false,"complianceChecks": [ ],"identifiers": [ ],"documents": [ ],"screenLockTime": 30,"screenLockExplanation": {"minutesIdleBeforeSleep": "2","minutesSleepingBeforePasswordIsRequired": "2"},"antivirusEnabled": true,"antivirusExplanation": "No matching app was found","autoUpdateEnabled": true,"autoUpdateExplanation": "No compliances found","passwordManagerEnabled": true,"passwordManagerExplanation": {"passwordManagerApps": ["1password 7"]},"encryptionEnabled": false,"encryptionExplanation": "No encryption provided","firewallEnabled": true,"firewallExplanation": "{}","asset": {"id": 1,"name": "MacBook Pro - Space Black 16-inch","description": "MacBook Pro Space Black - with 16-inch Liquid Retina XDR display","assetType": "PHYSICAL","assetProvider": "AGENT","approvedAt": "2025-07-01T16:45:55.246Z","removedAt": "2025-07-01T16:45:55.246Z","company": "Acme, Inc","notes": "string","assetReferenceType": "PERSONNEL","uniqueId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353"},"userId": 1,"personnelId": 1,"externalId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd"},"externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353","customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}`

## [](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_getAsset)Get Asset

🔒 Requires **Assets: List Assets** permission.

Security

**bearer**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_getAsset!in=path&path=assetId&t=request)assetId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_getAsset!in=query&path=expand[]&t=request)expand[]Array of strings (AssetExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"device""assetClassTypes""complianceChecks""customFields""identifiers""owner"

Responses

200
Successful

401
Invalid Authorization

403
You are not allowed to perform this action

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/assets/{assetId}

Request samples

*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

2 more

Response samples

*   200
*   401
*   403
*   412
*   500

application/json

`{"id": 1,"name": "MacBook Pro 16-inch","description": "MacBook Pro 16-inch with 14-Core CPU 32-Core GPU 36GB Unified Memory 1TB SSD Storage","assetType": "PHYSICAL","assetProvider": "AGENT","removedAt": "2025-07-01T16:45:55.246Z","assetClassTypes": "AssetClassTypeResponsePublicV2Dto[]","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","jobTitle": "CEO","drataTermsAgreedAt": "2025-07-01T16:45:55.246Z","roles": ["ROLE","ANOTHER_ROLE"],"backgroundChecks": [{"id": 0,"userId": 0,"status": "OK","caseId": "abc123","caseInvitationId": "abc123","manualCheckDate": "2020-07-06","manuallyCheckUrl": "url.com","type": "CERTN","source": "DRATA","reportData": "string","outOfScopeReason": "abc123","outOfScopeAt": "2025-07-01T16:45:55.246Z","invitationEmail": "email@email.com","linkedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"documents": [{"id": 1,"userId": 0,"downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="}},"name": "Security Training","type": "SEC_TRAINING","renewalDate": "2026-10-27","createdAt": "2020-07-06","updatedAt": "2020-07-06"}],"identities": [{"id": 1,"identityId": "1a2b3c","username": "johndoe","connectedAt": "2025-07-01T16:45:55.246Z","disconnectedAt": "2025-07-01T16:45:55.246Z","hasMfa": true,"userId": 1,"connectionId": 1,"connection": {"id": "1","clientType": "GOOGLE","clientId": "drata.com","clientAlias": "My-connection-alias-1","state": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectedAt": "2025-07-01T16:45:55.246Z","failedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"},"email": "johndoe@example.com","secondaryEmail": "johndoe@test.com","firstName": "John","lastName": "Doe","startedAt": "Fri Jun 26 2026","separatedAt": "Fri Jun 26 2026","isContractor": true,"jobTitle": "Engineer","managerId": "x00jk12-2312","managerName": "Jose Díaz","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]},"notes": "string","assetReferenceType": "PERSONNEL","associatedId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","device": {"id": 1,"assetId": 123,"osVersion": "MacOS 10.15.6","serialNumber": "C02T6CDJGTFL","model": "MacBook Pro","macAddress": "65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E","lastCheckedAt": "2025-07-01T16:45:55.246Z","sourceType": "AGENT","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z","appsCount": 20,"isDeviceCompliant": false,"complianceChecks": [ ],"identifiers": [ ],"documents": [ ],"screenLockTime": 30,"screenLockExplanation": {"minutesIdleBeforeSleep": "2","minutesSleepingBeforePasswordIsRequired": "2"},"antivirusEnabled": true,"antivirusExplanation": "No matching app was found","autoUpdateEnabled": true,"autoUpdateExplanation": "No compliances found","passwordManagerEnabled": true,"passwordManagerExplanation": {"passwordManagerApps": ["1password 7"]},"encryptionEnabled": false,"encryptionExplanation": "No encryption provided","firewallEnabled": true,"firewallExplanation": "{}","asset": {"id": 1,"name": "MacBook Pro - Space Black 16-inch","description": "MacBook Pro Space Black - with 16-inch Liquid Retina XDR display","assetType": "PHYSICAL","assetProvider": "AGENT","approvedAt": "2025-07-01T16:45:55.246Z","removedAt": "2025-07-01T16:45:55.246Z","company": "Acme, Inc","notes": "string","assetReferenceType": "PERSONNEL","uniqueId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353"},"userId": 1,"personnelId": 1,"externalId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd"},"externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353","customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}`

## [](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_updateAsset)Update Asset

Update Asset

🔒 Requires **Assets: Update Asset** permission.

Security

**bearer**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_updateAsset!in=path&path=assetId&t=request)assetId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_updateAsset!path=name&t=request)name string<= 191 characters

The Asset name
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_updateAsset!path=description&t=request)description string<= 191 characters

The Asset description
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_updateAsset!path=notes&t=request)notes string<= 191 characters

The Asset notes
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_updateAsset!path=ownerId&t=request)ownerId number

The ID of the user to set as Asset owner
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_updateAsset!path=assetClassTypes&t=request)assetClassTypes Array of strings

The Asset class types

Items Enum:"HARDWARE""POLICY""DOCUMENT""PERSONNEL""SOFTWARE""CODE""CONTAINER""COMPUTE""NETWORKING""DATABASE""STORAGE"
[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_updateAsset!path=assetType&t=request)assetType string

The Asset type

Enum:"PHYSICAL""VIRTUAL"
Array of objects (CustomFieldSubmitRequestPublicV2Dto)

Custom Fields for the Asset.

💎 Requires your account have the Custom Fields and Formulas feature. Contact your CSM for help upgrading.

Responses

200
Successful

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

put/assets/{assetId}

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

`{"name": "string","description": "string","notes": "string","ownerId": 0,"assetClassTypes": ["HARDWARE","PERSONNEL"],"assetType": "PHYSICAL","customFields": [{"id": 1,"name": "Compliance Status","value": "Security & IT"}]}`

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

`{"id": 1,"name": "MacBook Pro 16-inch","description": "MacBook Pro 16-inch with 14-Core CPU 32-Core GPU 36GB Unified Memory 1TB SSD Storage","assetType": "PHYSICAL","assetProvider": "AGENT","removedAt": "2025-07-01T16:45:55.246Z","assetClassTypes": "AssetClassTypeResponsePublicV2Dto[]","owner": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","jobTitle": "CEO","drataTermsAgreedAt": "2025-07-01T16:45:55.246Z","roles": ["ROLE","ANOTHER_ROLE"],"backgroundChecks": [{"id": 0,"userId": 0,"status": "OK","caseId": "abc123","caseInvitationId": "abc123","manualCheckDate": "2020-07-06","manuallyCheckUrl": "url.com","type": "CERTN","source": "DRATA","reportData": "string","outOfScopeReason": "abc123","outOfScopeAt": "2025-07-01T16:45:55.246Z","invitationEmail": "email@email.com","linkedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"documents": [{"id": 1,"userId": 0,"downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="}},"name": "Security Training","type": "SEC_TRAINING","renewalDate": "2026-10-27","createdAt": "2020-07-06","updatedAt": "2020-07-06"}],"identities": [{"id": 1,"identityId": "1a2b3c","username": "johndoe","connectedAt": "2025-07-01T16:45:55.246Z","disconnectedAt": "2025-07-01T16:45:55.246Z","hasMfa": true,"userId": 1,"connectionId": 1,"connection": {"id": "1","clientType": "GOOGLE","clientId": "drata.com","clientAlias": "My-connection-alias-1","state": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectedAt": "2025-07-01T16:45:55.246Z","failedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"},"email": "johndoe@example.com","secondaryEmail": "johndoe@test.com","firstName": "John","lastName": "Doe","startedAt": "Fri Jun 26 2026","separatedAt": "Fri Jun 26 2026","isContractor": true,"jobTitle": "Engineer","managerId": "x00jk12-2312","managerName": "Jose Díaz","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]},"notes": "string","assetReferenceType": "PERSONNEL","associatedId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","device": {"id": 1,"assetId": 123,"osVersion": "MacOS 10.15.6","serialNumber": "C02T6CDJGTFL","model": "MacBook Pro","macAddress": "65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E","lastCheckedAt": "2025-07-01T16:45:55.246Z","sourceType": "AGENT","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z","appsCount": 20,"isDeviceCompliant": false,"complianceChecks": [ ],"identifiers": [ ],"documents": [ ],"screenLockTime": 30,"screenLockExplanation": {"minutesIdleBeforeSleep": "2","minutesSleepingBeforePasswordIsRequired": "2"},"antivirusEnabled": true,"antivirusExplanation": "No matching app was found","autoUpdateEnabled": true,"autoUpdateExplanation": "No compliances found","passwordManagerEnabled": true,"passwordManagerExplanation": {"passwordManagerApps": ["1password 7"]},"encryptionEnabled": false,"encryptionExplanation": "No encryption provided","firewallEnabled": true,"firewallExplanation": "{}","asset": {"id": 1,"name": "MacBook Pro - Space Black 16-inch","description": "MacBook Pro Space Black - with 16-inch Liquid Retina XDR display","assetType": "PHYSICAL","assetProvider": "AGENT","approvedAt": "2025-07-01T16:45:55.246Z","removedAt": "2025-07-01T16:45:55.246Z","company": "Acme, Inc","notes": "string","assetReferenceType": "PERSONNEL","uniqueId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353"},"userId": 1,"personnelId": 1,"externalId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd"},"externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353","customFields": [{"customFieldId": 1,"name": "Stakeholders","value": "Security & IT"}]}`

## [](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_deleteAsset)Remove Asset

Remove a virtual or manually-added Asset. This is an unrecoverable operation.

🔒 Requires **Assets: Delete Asset** permission.

Security

**bearer**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/overview/#tag/Assets/operation/AssetsPublicV2Controller_deleteAsset!in=path&path=assetId&t=request)assetId
required number

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

delete/assets/{assetId}

Request samples

*   Node.js
*   JavaScript
*   curl
*   Python
*   C#
*   Java
*   PHP

2 more

Response samples

*   400
*   401
*   403
*   404
*   412
*   500

1 more

application/json

`{"name": "string","statusCode": 0,"message": "string","code": 0,"debugInfo": {"name": "string","message": "string","stack": "string"}}`
