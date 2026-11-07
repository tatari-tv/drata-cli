Title: Custom Connections

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Custom Connections allow users to integrate external systems with Drata. CUSTOM connections push arbitrary JSON evidence records using a user-defined schema. MDM and HRIS connections use a fixed common model for device management and HR identity data respectively. The [help docs for custom connections and tests](https://help.drata.com/en/articles/11995676-part-1-custom-connections-and-tests) and [custom device connections](https://help.drata.com/en/articles/12014143-custom-device-connections) have more information.

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_listCustomConnections)List Custom Connections

Find Custom Connections matching the provided filters.

🔒 Requires **Custom Connections Management: List Custom Connections** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_listCustomConnections!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_listCustomConnections!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_listCustomConnections!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_listCustomConnections!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_listCustomConnections!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_listCustomConnections!in=query&path=expand[]&t=request)expand[]Array of strings (CustomConnectionExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"createdByUser""customResources""workspaces"
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_listCustomConnections!in=query&path=name&t=request)name string<= 191 characters

Filter Custom Connections by `clientAlias` (prefix match, case-insensitive). Send it again with each `cursor` page.

Example:name=HR System
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_listCustomConnections!in=query&path=workspaceId&t=request)workspaceId number

Return only Custom Connections linked to this Workspace. Send it again with each `cursor` page.

Example:workspaceId=1

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

get/custom-connections

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
    `https://public-api.drata.com/public/v2/custom-connections`,
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

`{"data": [{"id": 1,"clientAlias": "My Custom Connection","description": "Custom connection for managing Employee Data","providerTypes": ["MDM"],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","createdById": 1,"customResources": [{"id": 123,"name": "Employees","description": "Employee records for compliance tracking","schema": {"id": "string","type": "object","properties": {"name": {"type": "string"}},"required": ["name"],"additionalProperties": false,"selfRequired": true,},"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"}],"createdByUser": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_createCustomConnection)Create Custom Connection

Create a new Custom Connection. Supported provider types: CUSTOM, MDM, HRIS. CUSTOM connections require `schema` or `sampleData` and `displayNameKey`. MDM and HRIS connections use a fixed common model — `schema`, `sampleData`, and `displayNameKey` must not be provided.

🔒 Requires **Custom Connections Management: Create Custom Connection** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_createCustomConnection!path=name&t=request)name
required string

Name for the Custom Connection
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_createCustomConnection!path=providerTypes&t=request)providerTypes
required Array of strings

Provider types for the Custom Connection. Supported types: CUSTOM, MDM, HRIS. CUSTOM connections require a schema. MDM and HRIS use a fixed common model and do not accept schema fields.

Items Enum:"CUSTOM""MDM""HRIS"
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_createCustomConnection!path=workspaceIds&t=request)workspaceIds Array of numbers

Required for `CUSTOM` connections. IDs of the Workspaces to associate with this Custom Connection. Not applicable for `MDM` or `HRIS` connections, which are global.
required object

Required for `CUSTOM` connections. Not applicable for `MDM` or `HRIS` connections, which use a fixed common model.The JSON Schema definition for the data records this Custom Connection will store. Provide either `schema` (a valid JSON Schema object) or `sampleData` (a representative record) — not both. When `sampleData` is provided, the schema will be inferred automatically.
required object

Required for `CUSTOM` connections. Not applicable for `MDM` or `HRIS` connections, which use a fixed common model.A sample JSON object representing a single data record. The structure will be used to infer the JSON Schema for this Custom Connection resource. Provide either `sampleData` or `schema` — not both.
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_createCustomConnection!path=description&t=request)description string or null

Optional description for the Custom Connection resource.
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_createCustomConnection!path=displayNameKey&t=request)displayNameKey string

Required for `CUSTOM` connections. Not applicable for `MDM` or `HRIS` connections, which use a fixed common model.The key from the schema that identifies which field should be used as the human-readable display name for each record. For example, if your schema has `name`, `employeeId`, and `department` fields and you set `displayNameKey` to `"name"`, records will be labeled by their `name` value wherever they appear in the Drata UI. Must be a top-level key present in `schema.properties`.

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

post/custom-connections

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

`{"name": "My Custom Connection","providerTypes": ["CUSTOM"],"workspaceIds": [1],"schema": {"type": "object","properties": {"employeeId": {"type": "string"},"name": {"type": "string"},"department": {"type": "string"},"startDate": {"type": "string"},"isActive": {"type": "boolean"},"metadata": {"type": "object","properties": {"lastReview": {"type": "string"}},"additionalProperties": true}},"additionalProperties": true},"sampleData": {"employeeId": "EMP-001","name": "Jane Smith","department": "Engineering","startDate": "2024-01-15T00:00:00Z","isActive": true,"metadata": {"lastReview": "2025-03-01T00:00:00Z"}},"description": "A connection for syncing employee records from our internal HRIS.","displayNameKey": "name"}`

Response samples

*   201
*   400
*   401
*   402
*   403
*   412
*   500

2 more

application/json

`{"id": 1,"clientAlias": "My Custom Connection","description": "Custom connection for managing Employee Data","providerTypes": ["MDM"],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","createdById": 1,"customResources": [{"id": 123,"name": "Employees","description": "Employee records for compliance tracking","schema": {"id": "string","type": "object","properties": {"name": {"type": "string"}},"required": ["name"],"additionalProperties": false,"selfRequired": true,},"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"}],"createdByUser": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_getCustomConnection)Get Custom Connection

🔒 Requires **Custom Connections Management: List Custom Connections** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_getCustomConnection!in=path&path=connectionId&t=request)connectionId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_getCustomConnection!in=query&path=expand[]&t=request)expand[]Array of strings (CustomConnectionExpandEnum)

List of sub-collections and sub-objects to expand

Items Enum:"createdByUser""customResources""workspaces"

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

get/custom-connections/{connectionId}

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
    `https://public-api.drata.com/public/v2/custom-connections/${connectionId}`,
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

`{"id": 1,"clientAlias": "My Custom Connection","description": "Custom connection for managing Employee Data","providerTypes": ["MDM"],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","createdById": 1,"customResources": [{"id": 123,"name": "Employees","description": "Employee records for compliance tracking","schema": {"id": "string","type": "object","properties": {"name": {"type": "string"}},"required": ["name"],"additionalProperties": false,"selfRequired": true,},"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"}],"createdByUser": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_updateCustomConnection)Update Custom Connection

Update Custom Connection details

🔒 Requires **Custom Connections Management: Update Custom Connection** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_updateCustomConnection!in=path&path=connectionId&t=request)connectionId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_updateCustomConnection!path=clientAlias&t=request)clientAlias string or null

Alias for the Custom Connection
[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_updateCustomConnection!path=description&t=request)description string

Description for Custom Connection

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

put/custom-connections/{connectionId}

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

`{"clientAlias": "My Updated Custom Connection","description": "Updated description for Custom Connection"}`

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

`{"id": 1,"clientAlias": "My Custom Connection","description": "Custom connection for managing Employee Data","providerTypes": ["MDM"],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","createdById": 1,"customResources": [{"id": 123,"name": "Employees","description": "Employee records for compliance tracking","schema": {"id": "string","type": "object","properties": {"name": {"type": "string"}},"required": ["name"],"additionalProperties": false,"selfRequired": true,},"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"}],"createdByUser": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_deleteCustomConnection)Delete Custom Connection

🔒 Requires **Custom Connections Management: Delete Custom Connection** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_deleteCustomConnection!in=path&path=connectionId&t=request)connectionId
required number

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

delete/custom-connections/{connectionId}

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
    `https://public-api.drata.com/public/v2/custom-connections/${connectionId}`,
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
*   412
*   500

application/json

`{"statusCode": 0,"message": "string","code": 0,"debugInfo": {"name": "string","message": "string","stack": "string"},"providerError": {"provider": "string","status": 0,"code": "string","message": "string","details": { }}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_getCustomConnectionResourceAttributes)Get Custom Connection Resource Attributes

List the fields of a Custom Connection resource that custom Monitoring Test logic can evaluate, with each field's types and supported operators. Also returns the exact `provider` and `resource` strings to use in the recipe, and the JSON Schema of a valid evaluator.

🔒 Requires **Custom Connections Management: List Custom Connections** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Custom-Connections/#tag/Custom-Connections/operation/CustomConnectionsPublicV2Controller_getCustomConnectionResourceAttributes!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
required number or string

The `id` of the Custom Resource, from `customResources` on the Custom Connection (`expand[]=customResources`), or its exact name prefixed with `name:`
required number or string

The `id` of the Custom Connection, from `GET /custom-connections`, or its exact `clientAlias` prefixed with `name:`

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
Multiple Custom Connections or Custom Resources match the provided name. Use the numeric ID.

500
Internal server error

get/workspaces/{workspaceId}/custom-connections/{connectionId}/resources/{resourceId}/attributes

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
  const resourceId = 'YOUR_resourceId_PARAMETER';
  const connectionId = 'YOUR_connectionId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/custom-connections/${connectionId}/resources/${resourceId}/attributes`,
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

`{"provider": "HR System","resource": "Employees","properties": [{"name": "manager.email","types": ["string"],"arrayTypes": ["object"],"operators": [{"operator": "notEqual","valueTypes": ["string","fact"]}]}],"evaluatorSchema": {"type": "object","oneOf": [{"type": "object","properties": {"all": {"type": "array","minItems": 1}},"required": ["all"],"additionalProperties": false},{"type": "object","properties": {"any": {"type": "array","minItems": 1}},"required": ["any"],"additionalProperties": false}]}}`
