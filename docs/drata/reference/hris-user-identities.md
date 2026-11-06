Title: HRIS User Identities

URL Source: https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
HR user identity records for a Custom HRIS connection. Use the batch upsert endpoint to submit employee records keyed by your own `identityId`, then use the update endpoint to reflect employment changes (e.g. set `separatedAt` when an employee leaves). Deleting a record is intended for data submitted in error, not for offboarding. Changes are not applied immediately — they are incorporated the next time identity sync runs, which happens nightly by default. A sync can also be triggered manually from the Personnel page.

## [](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_listCustomHrisUserIdentities)List HRIS User Identities

Returns a cursor-paginated list of all active HR user identities for the specified custom HRIS connection.

🔒 Requires **HRIS User Identities: List HRIS User Identities** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_listCustomHrisUserIdentities!in=path&path=connectionId&t=request)connectionId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_listCustomHrisUserIdentities!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_listCustomHrisUserIdentities!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_listCustomHrisUserIdentities!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_listCustomHrisUserIdentities!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_listCustomHrisUserIdentities!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false

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

get/custom-connections/{connectionId}/hris-user-identities

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
  const connectionId = 'YOUR_connectionId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/custom-connections/${connectionId}/hris-user-identities`,
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

`{"data": [{"id": 1001,"identityId": "emp-001","email": "jane.doe@acme.com","secondaryEmail": null,"firstName": "Jane","lastName": "Doe","jobTitle": "Software Engineer","managerId": "emp-050","managerName": "Bob Manager","startedAt": "2022-03-01T00:00:00.000Z","separatedAt": null,"isContractor": false,"createdAt": "2026-03-26T10:00:00.000Z","updatedAt": "2026-03-26T10:00:00.000Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities)Create or Update HRIS User Identities

Submit one or more HR user identities for a custom HRIS connection. Each record is keyed by the customer-defined `identityId`: if a record with that `identityId` already exists for this connection it is updated, otherwise a new one is created. Invalid items are reported per-item and do not prevent valid items from being processed.

Partial updates are supported — omitted fields are left unchanged, while an explicit `null` clears a field.

`separatedAt` is write-once: once a separation date is stored, a different non-null value is ignored. To change a stored separation date, submit `"separatedAt": null` first to clear it, then send the new date in a subsequent request.

A record whose submitted values already match what is stored returns a per-item `{ "statusCode": 200, "data": null }` — this is a success (nothing changed), not an error.

🔒 Requires **HRIS User Identities: Create or Update HRIS User Identities** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!in=path&path=connectionId&t=request)connectionId
required number

##### Request Body schema: application/json
required

List of HR user identity records to upsert. Duplicate `identityId` values within a batch are reported as per-item errors.

Array

([ 1 .. 1000 ] items)

[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=identityId&t=request)identityId
required string

Customer-defined stable identifier for this record (e.g. employee ID, HRIS record ID). Used as the upsert key for batch submissions.
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=email&t=request)email string or null<email><= 191 characters

Primary email address. Used to match this record to an existing Drata user during identity sync.
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=secondaryEmail&t=request)secondaryEmail string or null<email><= 191 characters

Secondary email address.
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=firstName&t=request)firstName string or null<= 191 characters

First name.
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=lastName&t=request)lastName string or null<= 191 characters

Last name.
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=jobTitle&t=request)jobTitle string or null<= 191 characters

Job title.
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=managerId&t=request)managerId string or null<= 191 characters

The customer's own identifier for this person's manager (e.g. the manager's identityId in the customer's HRIS system). Not a Drata User ID.
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=managerName&t=request)managerName string or null<= 191 characters

Manager's full name.
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=startedAt&t=request)startedAt string or null<date-time>

Employment start date.
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=separatedAt&t=request)separatedAt string or null<date-time>

Employment end date. Set `separatedAt` to indicate the employee has left rather than deleting the record. Write-once on an existing record: once a separation date is stored, a different non-null `separatedAt` in a later submission is ignored — only `null` is applied (which clears/reactivates the record). To change a stored separation date, first submit `"separatedAt": null` to clear it, then submit the new date in a subsequent request. (This guard also applies when re-activating a previously separated record.)
[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_createCustomHrisUserIdentities!path=isContractor&t=request)isContractor boolean or null

If `true`, this person is a `CURRENT_CONTRACTOR`; `false` means `CURRENT_EMPLOYEE`. `null` means employment type is `UNKNOWN` — the person will appear as `UNKNOWN` in Drata until an explicit value is provided.

Responses

200
At least one record was updated (mix of creates and updates).

201
All records were newly created.

207
Some records succeeded and some failed. Per-item `statusCode` indicates each outcome.

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

post/custom-connections/{connectionId}/hris-user-identities

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

`[{"identityId": "emp-001","email": "jane.doe@acme.com","secondaryEmail": "jane.doe@personal.com","firstName": "Jane","lastName": "Doe","jobTitle": "Software Engineer","managerId": "emp-050","managerName": "Bob Manager","startedAt": "2022-03-01T00:00:00.000Z","separatedAt": "2024-12-31T00:00:00.000Z","isContractor": false}]`

Response samples

*   200
*   201
*   207
*   400
*   401
*   402
*   403
*   404
*   412
*   500

5 more

application/json

`[{"statusCode": 201,"data": {"id": 1001,"identityId": "emp-001","email": "jane.doe@acme.com","secondaryEmail": null,"firstName": "Jane","lastName": "Doe","jobTitle": "Software Engineer","managerId": "emp-050","managerName": "Bob Manager","startedAt": "2022-03-01T00:00:00.000Z","separatedAt": null,"isContractor": false,"createdAt": "2026-03-26T10:00:00.000Z","updatedAt": "2026-03-26T10:00:00.000Z"},"error": { }}]`

## [](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_getCustomHrisUserIdentity)Get HRIS User Identity

Returns a single HR user identity. The `:userIdentityId` path parameter accepts two forms: a Drata-assigned internal numeric ID (e.g. `1001`), or a customer-assigned identityId prefixed with `identity-id:` (e.g. `identity-id:emp-001`).

🔒 Requires **HRIS User Identities: Get HRIS User Identity** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_getCustomHrisUserIdentity!in=path&path=connectionId&t=request)connectionId
required number
required number or string

Drata internal numeric ID (e.g. `1001`) or customer-assigned identityId prefixed with `identity-id:` (e.g. `identity-id:emp-001`)

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

get/custom-connections/{connectionId}/hris-user-identities/{userIdentityId}

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
  const connectionId = 'YOUR_connectionId_PARAMETER';
  const userIdentityId = 'YOUR_userIdentityId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/custom-connections/${connectionId}/hris-user-identities/${userIdentityId}`,
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

`{"id": 1001,"identityId": "emp-001","email": "jane.doe@acme.com","secondaryEmail": null,"firstName": "Jane","lastName": "Doe","jobTitle": "Software Engineer","managerId": "emp-050","managerName": "Bob Manager","startedAt": "2022-03-01T00:00:00.000Z","separatedAt": null,"isContractor": false,"createdAt": "2026-03-26T10:00:00.000Z","updatedAt": "2026-03-26T10:00:00.000Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_deleteCustomHrisUserIdentity)Delete HRIS User Identity

Soft-deletes an HR user identity. The `:userIdentityId` path parameter accepts two forms: a Drata-assigned internal numeric ID (e.g. `1001`), or a customer-assigned identityId prefixed with `identity-id:` (e.g. `identity-id:emp-001`). This endpoint is intended for removing records submitted in error (e.g. test data, duplicates, or employees who should never have been included). To indicate that an employee has left, re-submit the record via the batch upsert endpoint with the `separatedAt` field set — this preserves employment history and allows identity sync to correctly classify the person as `FORMER_EMPLOYEE` or `FORMER_CONTRACTOR`. If the deleted record was previously matched to a Drata user, that user's employment status will move to `UNKNOWN` at the next identity sync. A deleted record can be restored by re-submitting it via the batch upsert endpoint with the same `identityId`.

🔒 Requires **HRIS User Identities: Delete HRIS User Identity** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/HRIS-User-Identities/#tag/HRIS-User-Identities/operation/CustomHrisUserIdentitiesPublicV2Controller_deleteCustomHrisUserIdentity!in=path&path=connectionId&t=request)connectionId
required number
required number or string

Drata internal numeric ID (e.g. `1001`) or customer-assigned identityId prefixed with `identity-id:` (e.g. `identity-id:emp-001`)

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

delete/custom-connections/{connectionId}/hris-user-identities/{userIdentityId}

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
  const connectionId = 'YOUR_connectionId_PARAMETER';
  const userIdentityId = 'YOUR_userIdentityId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/custom-connections/${connectionId}/hris-user-identities/${userIdentityId}`,
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
