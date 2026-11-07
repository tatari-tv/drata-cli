Title: Custom Data Records

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Custom Data Records are JSON evidence records pushed to a Custom Connection resource. Use the session management endpoints to batch upload records and track the status of bulk operations. You can create monitors on this data to provide continuous compliance. The [help docs](https://help.drata.com/en/articles/11995676-part-1-custom-connections-and-tests) have more information.

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listCustomDataRecords)List Custom Data Records

List Custom Data Records matching the provided filters.

🔒 Requires **Custom Connections Management: List Custom Connections** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listCustomDataRecords!in=path&path=connectionId&t=request)connectionId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listCustomDataRecords!in=path&path=resourceId&t=request)resourceId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listCustomDataRecords!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listCustomDataRecords!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listCustomDataRecords!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listCustomDataRecords!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listCustomDataRecords!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listCustomDataRecords!in=query&path=sessionId&t=request)sessionId string[ 3 .. 64 ] characters

Filter records by session ID.

Example:sessionId=session-abc-123

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

get/custom-connections/{connectionId}/resources/{resourceId}/records

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
  const resourceId = 'YOUR_resourceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/custom-connections/${connectionId}/resources/${resourceId}/records`,
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

`{"data": [{"id": "employee-001","attributes": {"id": "employee-001","name": "John Doe","department": "Engineering","status": "Active"},"sessionId": "session-abc-123","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_createCustomData)Upsert Custom Data Records

Create or update Custom Data records for a Custom Connection.

🔒 Requires **Custom Connections Data: Create Custom Connection Data** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

The `id` of the Custom Resource, or its exact name prefixed with `name:`. A name matches the resources of the Custom Connection given by `connectionId`.
required number or string

The `id` of the Custom Connection, or its exact `clientAlias` prefixed with `name:`

##### Request Body schema: application/json
required

required Array of objects or object

The Custom Data record(s) to be created or updated. Can be a single object or an array of objects for bulk upload. Records with matching IDs will be updated; new records will be created. Records will be added grouped under ACTIVE session if one exists.

Responses

200
Successful

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

422
Multiple Custom Connections or Custom Resources match the provided name. Use the numeric ID.

500
Internal server error

post/custom-connections/{connectionId}/resources/{resourceId}/records

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

`{"data": [{"id": "employee-001","name": "John","last_name": "Doe","department": "Engineering","status": "Active"},{"id": "employee-002","name": "Jane","last_name": "Smith","department": "Sales","status": "Active"}]}`

Response samples

*   200
*   201
*   400
*   401
*   402
*   403
*   404
*   412
*   422
*   500

5 more

application/json

`{"id": "fd52f04f-a030-44ea-a5af-40919194ab7e","createdAt": "2020-07-06 12:00:00.000000","updatedAt": "2020-07-06 13:00:00.000000","statusCode": 200,"data": {"YOUR": "DATA","PROPERTIES": {"WILL": "BE_HERE"}}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listSessions)List Custom Data Sessions

List Custom Data Sessions matching the provided filters.

🔒 Requires **Custom Connections Management: List Custom Connections** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listSessions!in=path&path=connectionId&t=request)connectionId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listSessions!in=path&path=resourceId&t=request)resourceId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listSessions!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listSessions!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listSessions!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listSessions!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listSessions!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_listSessions!in=query&path=status&t=request)status string (CustomDataSessionStatusEnum)

Filter sessions by status

Enum:"IN_PROGRESS""ACTIVE""CANCELED""ARCHIVED"

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

get/custom-connections/{connectionId}/resources/{resourceId}/sessions

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
  const resourceId = 'YOUR_resourceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/custom-connections/${connectionId}/resources/${resourceId}/sessions`,
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

`{"data": [{"id": 1,"sessionId": "session-abc-123","status": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","activatedAt": "2025-07-01T16:45:55.246Z","canceledAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_uploadSessionRecords)Upsert Custom Data Records with Session Management

Insert or update Custom Data records in batches using Session Management.

🔒 Requires **Custom Connections Data: Create Custom Connection Data** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_uploadSessionRecords!in=path&path=connectionId&t=request)connectionId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_uploadSessionRecords!in=path&path=resourceId&t=request)resourceId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_uploadSessionRecords!in=path&path=sessionId&t=request)sessionId
required string

##### Request Body schema: application/json
required

required Array of objects or object

The Custom Data record(s) to be created or updated within a session. Can be a single object or an array of objects for bulk upload. Records remain inactive until the session is completed via the session action endpoint.

Responses

200
Successful

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

post/custom-connections/{connectionId}/resources/{resourceId}/sessions/{sessionId}

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

`{"data": [{"id": "employee-001","name": "John","last_name": "Doe","department": "Engineering","status": "Active"},{"id": "employee-002","name": "Jane","last_name": "Smith","department": "Sales","status": "Active"}]}`

Response samples

*   200
*   201
*   400
*   401
*   402
*   403
*   404
*   412
*   500

4 more

application/json

`{"id": "fd52f04f-a030-44ea-a5af-40919194ab7e","createdAt": "2020-07-06 12:00:00.000000","updatedAt": "2020-07-06 13:00:00.000000","statusCode": 200,"data": {"YOUR": "DATA","PROPERTIES": {"WILL": "BE_HERE"}}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_performSessionAction)Perform Session Action

Perform an action on a Custom Data record Session

🔒 Requires **Custom Connections Data: Create Custom Connection Data** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_performSessionAction!in=path&path=connectionId&t=request)connectionId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_performSessionAction!in=path&path=resourceId&t=request)resourceId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_performSessionAction!in=path&path=sessionId&t=request)sessionId
required string

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_performSessionAction!path=action&t=request)action
required string

The action to perform on the session.

*   `complete`: Mark session as complete, sets data as active and **permanently deletes all data that is not part of the session**
*   `cancel`: Cancels session and discard data associated with that session

Enum:"complete""cancel"

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

post/custom-connections/{connectionId}/resources/{resourceId}/sessions/{sessionId}/actions

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

`{"action": "complete"}`

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

`{"sessionId": "upload-2024-03-13","status": "ACTIVE","action": "complete","connectionId": 123,"resourceId": 456}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_updateCustomData)Update Custom Data Record

Update an existing Custom Data record by ID.

🔒 Requires **Custom Connections Data: Create and Update Custom Connection Data** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_updateCustomData!in=path&path=connectionId&t=request)connectionId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_updateCustomData!in=path&path=resourceId&t=request)resourceId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_updateCustomData!in=path&path=recordId&t=request)recordId
required string

##### Request Body schema: application/json
required

required object

The custom data to update

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

413
The file was too large to upload

500
Internal server error

put/custom-connections/{connectionId}/resources/{resourceId}/records/{recordId}

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

`{"data": {"employeeId": "EMP-001","name": "Jane Smith","department": "Engineering","startDate": "2024-01-15T00:00:00Z","isActive": true,"accessLevel": 3,"tags": ["engineer","remote"],"metadata": {"lastReview": "2025-03-01T00:00:00Z"}}}`

Response samples

*   200
*   400
*   401
*   402
*   403
*   404
*   412
*   413
*   500

4 more

application/json

`{"id": "employee-001","attributes": {"id": "employee-001","name": "John Doe","department": "Engineering","status": "Active"},"sessionId": "session-abc-123","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_deleteCustomData)Delete Custom Connection Data Record

Delete a Custom Data Record by ID.

🔒 Requires **Custom Connections Data: Delete Custom Connection Data** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_deleteCustomData!in=path&path=connectionId&t=request)connectionId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_deleteCustomData!in=path&path=resourceId&t=request)resourceId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Data-Records/#tag/Custom-Data-Records/operation/CustomDataRecordsPublicV2Controller_deleteCustomData!in=path&path=recordId&t=request)recordId
required string

Responses

204
No Content

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

delete/custom-connections/{connectionId}/resources/{resourceId}/records/{recordId}

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
  const resourceId = 'YOUR_resourceId_PARAMETER';
  const recordId = 'YOUR_recordId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/custom-connections/${connectionId}/resources/${resourceId}/records/${recordId}`,
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

*   401
*   402
*   403
*   404
*   412
*   500

1 more

application/json

`{"statusCode": 0,"message": "string","code": 0,"debugInfo": {"name": "string","message": "string","stack": "string"},"providerError": {"provider": "string","status": 0,"code": "string","message": "string","details": { }}}`
