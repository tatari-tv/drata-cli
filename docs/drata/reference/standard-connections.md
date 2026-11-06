Title: Standard Connections

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Standard Connections let you integrate Drata with well-known third-party providers using Drata-defined credentials and data models, in contrast to Custom Connections, which use a user-defined schema.

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_listAwsConnections)List AWS Connections

Find AWS Connections matching the provided filters.

🔒 Requires **Connections: List Connections** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_listAwsConnections!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_listAwsConnections!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_listAwsConnections!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_listAwsConnections!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_listAwsConnections!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_listAwsConnections!in=query&path=expand[]&t=request)expand[]Array of strings (StandardConnectionExpandEnum)

List of sub-collections and sub-objects to expand

Items Value:"workspaces"

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

get/aws-connections

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
    `https://public-api.drata.com/public/v2/aws-connections`,
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

`{"data": [{"id": 1,"clientAlias": "My AWS Connection","roleArn": "arn:aws:iam::123456789012:role/DrataAuditRole","externalId": "b3b7f7b0-6e4b-4e2a-9c2f-3e2f6f8f1a1b","state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_createAwsConnection)Create AWS Connection

Create a new AWS Connection by providing the ARN of an IAM Role for Drata to assume. The response includes the External ID to configure in the role's trust policy.

🔒 Requires **Connections: Create Connection** permission.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_createAwsConnection!path=name&t=request)name
required string

Name for the AWS Connection
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_createAwsConnection!path=roleArn&t=request)roleArn
required string

The ARN of the IAM Role Drata will assume to access this AWS account. The role must trust Drata and be configured with the External ID returned in the response.
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_createAwsConnection!path=workspaceIds&t=request)workspaceIds Array of numbers

IDs of the Workspaces to assign this AWS Connection to. Omit to leave the connection unassigned, in which case it must be assigned in Drata before its monitoring tests can run against it.

Responses

201
Created

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

post/aws-connections

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

`{"name": "My AWS Connection","roleArn": "arn:aws:iam::123456789012:role/DrataAuditRole","workspaceIds": [1]}`

Response samples

*   201
*   400
*   401
*   403
*   412
*   500

1 more

application/json

`{"id": 1,"clientAlias": "My AWS Connection","roleArn": "arn:aws:iam::123456789012:role/DrataAuditRole","externalId": "b3b7f7b0-6e4b-4e2a-9c2f-3e2f6f8f1a1b","state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_getAwsConnection)Get AWS Connection

🔒 Requires **Connections: List Connections** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

Drata internal numeric ID (e.g. `10`) or AWS IAM Role ARN prefixed with `roleArn:` (e.g. `roleArn:arn:aws:iam::123456789012:role/DrataAuditRole`). Any `/` in the ARN must be percent-encoded as `%2F`, since it's part of a single URL path segment.

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_getAwsConnection!in=query&path=expand[]&t=request)expand[]Array of strings (StandardConnectionExpandEnum)

List of sub-collections and sub-objects to expand

Items Value:"workspaces"

Responses

200

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

get/aws-connections/{connectionId}

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
    `https://public-api.drata.com/public/v2/aws-connections/${connectionId}`,
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
*   404
*   412
*   500

1 more

application/json

`{"id": 1,"clientAlias": "My AWS Connection","roleArn": "arn:aws:iam::123456789012:role/DrataAuditRole","externalId": "b3b7f7b0-6e4b-4e2a-9c2f-3e2f6f8f1a1b","state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_updateAwsConnection)Update AWS Connection

Update an AWS Connection. Providing a new `roleArn` re-validates the connection via STS AssumeRole.

🔒 Requires **Connections: Update Connection** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

Drata internal numeric ID (e.g. `10`) or AWS IAM Role ARN prefixed with `roleArn:` (e.g. `roleArn:arn:aws:iam::123456789012:role/DrataAuditRole`). Any `/` in the ARN must be percent-encoded as `%2F`, since it's part of a single URL path segment.

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_updateAwsConnection!path=name&t=request)name string

Name for the AWS Connection
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_updateAwsConnection!path=roleArn&t=request)roleArn string

The ARN of the IAM Role Drata will assume to access this AWS account. Providing a new value re-validates the connection via STS AssumeRole.
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_updateAwsConnection!path=workspaceIds&t=request)workspaceIds Array of numbers

IDs of the Workspaces this AWS Connection should be assigned to. Replaces the current assignment: omit to leave it unchanged, or pass an empty array to unassign every Workspace.

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

put/aws-connections/{connectionId}

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

`{"name": "My Updated AWS Connection","roleArn": "arn:aws:iam::123456789012:role/DrataAuditRole","workspaceIds": [1]}`

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

`{"id": 1,"clientAlias": "My AWS Connection","roleArn": "arn:aws:iam::123456789012:role/DrataAuditRole","externalId": "b3b7f7b0-6e4b-4e2a-9c2f-3e2f6f8f1a1b","state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AwsConnectionsPublicV2Controller_deleteAwsConnection)Delete AWS Connection

🔒 Requires **Connections: Delete Connection** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

Drata internal numeric ID (e.g. `10`) or AWS IAM Role ARN prefixed with `roleArn:` (e.g. `roleArn:arn:aws:iam::123456789012:role/DrataAuditRole`). Any `/` in the ARN must be percent-encoded as `%2F`, since it's part of a single URL path segment.

Responses

204
No Content

401
Invalid Authorization

403
You are not allowed to perform this action

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

delete/aws-connections/{connectionId}

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
    `https://public-api.drata.com/public/v2/aws-connections/${connectionId}`,
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
*   403
*   412
*   500

application/json

`{"statusCode": 0,"message": "string","code": 0,"debugInfo": {"name": "string","message": "string","stack": "string"},"providerError": {"provider": "string","status": 0,"code": "string","message": "string","details": { }}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_listAzureConnections)List Azure Connections

Find Azure Connections matching the provided filters.

🔒 Requires **Connections: List Connections** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_listAzureConnections!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_listAzureConnections!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_listAzureConnections!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_listAzureConnections!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_listAzureConnections!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_listAzureConnections!in=query&path=expand[]&t=request)expand[]Array of strings (StandardConnectionExpandEnum)

List of sub-collections and sub-objects to expand

Items Value:"workspaces"

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

get/azure-connections

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
    `https://public-api.drata.com/public/v2/azure-connections`,
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

`{"data": [{"id": 1,"clientAlias": "My Azure Connection","tenantId": "72f988bf-86f1-41af-91ab-2d7cd011db47","applicationId": "4f812b7c-91d4-4b3a-9c17-6e2d6d5f9f3a","subscriptionId": "28ffcc9b-be6d-4bbf-b53a-057049debc38","useMicrosoftPrimaryEmail": false,"state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_createAzureConnection)Create Azure Connection

Create a new Azure Connection using an Azure App Registration (Service Principal) and its client secret.

🔒 Requires **Connections: Create Connection** permission.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_createAzureConnection!path=name&t=request)name
required string

Name for the Azure Connection
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_createAzureConnection!path=tenantId&t=request)tenantId
required string

The Azure Active Directory Tenant ID
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_createAzureConnection!path=applicationId&t=request)applicationId
required string

The Application (client) ID of the Azure App Registration
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_createAzureConnection!path=applicationSecret&t=request)applicationSecret
required string

The client secret for the Azure App Registration. Never returned in any response.
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_createAzureConnection!path=subscriptionId&t=request)subscriptionId
required string

The Azure Subscription ID to connect
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_createAzureConnection!path=useMicrosoftPrimaryEmail&t=request)useMicrosoftPrimaryEmail boolean

Default:false

How Drata maps Microsoft directory users to Personnel. `false` maps by UPN (User Principal Name), `true` maps by primary email address. Defaults to `false`.
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_createAzureConnection!path=workspaceIds&t=request)workspaceIds Array of numbers

IDs of the Workspaces to assign this Azure Connection to. Omit to leave the connection unassigned, in which case it must be assigned in Drata before its monitoring tests can run against it.

Responses

201
Created

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

post/azure-connections

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

`{"name": "My Azure Connection","tenantId": "72f988bf-86f1-41af-91ab-2d7cd011db47","applicationId": "4f812b7c-91d4-4b3a-9c17-6e2d6d5f9f3a","applicationSecret": "abc123~ExampleClientSecretValue","subscriptionId": "28ffcc9b-be6d-4bbf-b53a-057049debc38","useMicrosoftPrimaryEmail": false,"workspaceIds": [1]}`

Response samples

*   201
*   400
*   401
*   403
*   412
*   500

1 more

application/json

`{"id": 1,"clientAlias": "My Azure Connection","tenantId": "72f988bf-86f1-41af-91ab-2d7cd011db47","applicationId": "4f812b7c-91d4-4b3a-9c17-6e2d6d5f9f3a","subscriptionId": "28ffcc9b-be6d-4bbf-b53a-057049debc38","useMicrosoftPrimaryEmail": false,"state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_getAzureConnection)Get Azure Connection

🔒 Requires **Connections: List Connections** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

Drata internal numeric ID (e.g. `10`) or Azure Subscription ID prefixed with `subscriptionId:` (e.g. `subscriptionId:28ffcc9b-be6d-4bbf-b53a-057049debc38`).

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_getAzureConnection!in=query&path=expand[]&t=request)expand[]Array of strings (StandardConnectionExpandEnum)

List of sub-collections and sub-objects to expand

Items Value:"workspaces"

Responses

200

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

get/azure-connections/{connectionId}

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
    `https://public-api.drata.com/public/v2/azure-connections/${connectionId}`,
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
*   404
*   412
*   500

1 more

application/json

`{"id": 1,"clientAlias": "My Azure Connection","tenantId": "72f988bf-86f1-41af-91ab-2d7cd011db47","applicationId": "4f812b7c-91d4-4b3a-9c17-6e2d6d5f9f3a","subscriptionId": "28ffcc9b-be6d-4bbf-b53a-057049debc38","useMicrosoftPrimaryEmail": false,"state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_updateAzureConnection)Update Azure Connection

Update an Azure Connection. Providing any new credential fields re-validates the connection.

🔒 Requires **Connections: Update Connection** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

Drata internal numeric ID (e.g. `10`) or Azure Subscription ID prefixed with `subscriptionId:` (e.g. `subscriptionId:28ffcc9b-be6d-4bbf-b53a-057049debc38`).

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_updateAzureConnection!path=name&t=request)name string

Name for the Azure Connection
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_updateAzureConnection!path=tenantId&t=request)tenantId string

The Azure Active Directory Tenant ID
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_updateAzureConnection!path=applicationId&t=request)applicationId string

The Application (client) ID of the Azure App Registration
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_updateAzureConnection!path=applicationSecret&t=request)applicationSecret string

The client secret for the Azure App Registration. Providing a new value re-validates the connection.
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_updateAzureConnection!path=subscriptionId&t=request)subscriptionId string

The Azure Subscription ID to connect
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_updateAzureConnection!path=useMicrosoftPrimaryEmail&t=request)useMicrosoftPrimaryEmail boolean

How Drata maps Microsoft directory users to Personnel. `false` maps by UPN (User Principal Name), `true` maps by primary email address. Omit to leave it unchanged.
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_updateAzureConnection!path=workspaceIds&t=request)workspaceIds Array of numbers

IDs of the Workspaces this Azure Connection should be assigned to. Replaces the current assignment: omit to leave it unchanged, or pass an empty array to unassign every Workspace.

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

put/azure-connections/{connectionId}

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

`{"name": "My Updated Azure Connection","tenantId": "72f988bf-86f1-41af-91ab-2d7cd011db47","applicationId": "4f812b7c-91d4-4b3a-9c17-6e2d6d5f9f3a","applicationSecret": "abc123~ExampleClientSecretValue","subscriptionId": "28ffcc9b-be6d-4bbf-b53a-057049debc38","useMicrosoftPrimaryEmail": false,"workspaceIds": [1]}`

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

`{"id": 1,"clientAlias": "My Azure Connection","tenantId": "72f988bf-86f1-41af-91ab-2d7cd011db47","applicationId": "4f812b7c-91d4-4b3a-9c17-6e2d6d5f9f3a","subscriptionId": "28ffcc9b-be6d-4bbf-b53a-057049debc38","useMicrosoftPrimaryEmail": false,"state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/AzureConnectionsPublicV2Controller_deleteAzureConnection)Delete Azure Connection

🔒 Requires **Connections: Delete Connection** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

Drata internal numeric ID (e.g. `10`) or Azure Subscription ID prefixed with `subscriptionId:` (e.g. `subscriptionId:28ffcc9b-be6d-4bbf-b53a-057049debc38`).

Responses

204
No Content

401
Invalid Authorization

403
You are not allowed to perform this action

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

delete/azure-connections/{connectionId}

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
    `https://public-api.drata.com/public/v2/azure-connections/${connectionId}`,
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
*   403
*   412
*   500

application/json

`{"statusCode": 0,"message": "string","code": 0,"debugInfo": {"name": "string","message": "string","stack": "string"},"providerError": {"provider": "string","status": 0,"code": "string","message": "string","details": { }}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_listGcpConnections)List GCP Connections

Find GCP Connections matching the provided filters.

🔒 Requires **Connections: List Connections** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_listGcpConnections!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_listGcpConnections!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_listGcpConnections!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_listGcpConnections!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_listGcpConnections!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_listGcpConnections!in=query&path=expand[]&t=request)expand[]Array of strings (StandardConnectionExpandEnum)

List of sub-collections and sub-objects to expand

Items Value:"workspaces"

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

get/gcp-connections

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
    `https://public-api.drata.com/public/v2/gcp-connections`,
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

`{"data": [{"id": 1,"clientAlias": "My GCP Connection","projectId": "my-project","state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_createGcpConnection)Create GCP Connection

Create a new GCP Connection by providing a GCP service account key. The service account must be granted read access to the resources Drata will scan.

🔒 Requires **Connections: Create Connection** permission.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_createGcpConnection!path=name&t=request)name
required string<= 191 characters

Name for the GCP Connection
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_createGcpConnection!path=serviceAccountKey&t=request)serviceAccountKey
required string

The GCP service account key, as the raw JSON contents of the key file downloaded from GCP IAM (stringified). Must include at least `project_id`, `client_email`, and `private_key`. Never returned in any response.
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_createGcpConnection!path=workspaceIds&t=request)workspaceIds Array of numbers

IDs of the Workspaces to assign this GCP Connection to. Omit to leave the connection unassigned, in which case it must be assigned in Drata before its monitoring tests can run against it.

Responses

201
Created

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

post/gcp-connections

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

`{"name": "My GCP Connection","serviceAccountKey": "{\"type\":\"service_account\",\"project_id\":\"my-project\",...}","workspaceIds": [1]}`

Response samples

*   201
*   400
*   401
*   403
*   412
*   500

1 more

application/json

`{"id": 1,"clientAlias": "My GCP Connection","projectId": "my-project","state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_getGcpConnection)Get GCP Connection

🔒 Requires **Connections: List Connections** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

Drata internal numeric ID (e.g. `10`) or GCP Project ID prefixed with `projectId:` (e.g. `projectId:my-project`).

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_getGcpConnection!in=query&path=expand[]&t=request)expand[]Array of strings (StandardConnectionExpandEnum)

List of sub-collections and sub-objects to expand

Items Value:"workspaces"

Responses

200

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

get/gcp-connections/{connectionId}

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
    `https://public-api.drata.com/public/v2/gcp-connections/${connectionId}`,
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
*   404
*   412
*   500

1 more

application/json

`{"id": 1,"clientAlias": "My GCP Connection","projectId": "my-project","state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_updateGcpConnection)Update GCP Connection

Update a GCP Connection. Providing a new `serviceAccountKey` re-validates the connection.

🔒 Requires **Connections: Update Connection** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

Drata internal numeric ID (e.g. `10`) or GCP Project ID prefixed with `projectId:` (e.g. `projectId:my-project`).

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_updateGcpConnection!path=name&t=request)name string<= 191 characters

Name for the GCP Connection
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_updateGcpConnection!path=serviceAccountKey&t=request)serviceAccountKey string

The GCP service account key, as the raw JSON contents of the key file downloaded from GCP IAM (stringified). Providing a new value re-validates the connection.
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_updateGcpConnection!path=workspaceIds&t=request)workspaceIds Array of numbers

IDs of the Workspaces this GCP Connection should be assigned to. Replaces the current assignment: omit to leave it unchanged, or pass an empty array to unassign every Workspace.

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

put/gcp-connections/{connectionId}

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

`{"name": "My Updated GCP Connection","serviceAccountKey": "{\"type\":\"service_account\",\"project_id\":\"my-project\",...}","workspaceIds": [1]}`

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

`{"id": 1,"clientAlias": "My GCP Connection","projectId": "my-project","state": "ACTIVE","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GcpConnectionsPublicV2Controller_deleteGcpConnection)Delete GCP Connection

🔒 Requires **Connections: Delete Connection** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

required number or string

Drata internal numeric ID (e.g. `10`) or GCP Project ID prefixed with `projectId:` (e.g. `projectId:my-project`).

Responses

204
No Content

401
Invalid Authorization

403
You are not allowed to perform this action

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

delete/gcp-connections/{connectionId}

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
    `https://public-api.drata.com/public/v2/gcp-connections/${connectionId}`,
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
*   403
*   412
*   500

application/json

`{"statusCode": 0,"message": "string","code": 0,"debugInfo": {"name": "string","message": "string","stack": "string"},"providerError": {"provider": "string","status": 0,"code": "string","message": "string","details": { }}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_listGitlabConnections)List GitLab Connections

Find GitLab Connections matching the provided filters.

🔒 Requires **Connections: List Connections** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_listGitlabConnections!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_listGitlabConnections!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_listGitlabConnections!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_listGitlabConnections!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_listGitlabConnections!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_listGitlabConnections!in=query&path=expand[]&t=request)expand[]Array of strings (StandardConnectionExpandEnum)

List of sub-collections and sub-objects to expand

Items Value:"workspaces"

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

get/gitlab-connections

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
    `https://public-api.drata.com/public/v2/gitlab-connections`,
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

`{"data": [{"id": 1,"clientAlias": "My GitLab Connection","state": "ACTIVE","tokenExpiresAt": "2025-07-01T16:45:55.246Z","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_createGitlabConnection)Create GitLab Connection

Create a new GitLab.com Connection from a Personal Access Token. The token must carry at least the `read_api` scope and belong to a user with access to the groups Drata will scan. Self-managed GitLab is not supported by this endpoint.

🔒 Requires **Connections: Create Connection** permission.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_createGitlabConnection!path=name&t=request)name
required string<= 191 characters

Name for the GitLab Connection
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_createGitlabConnection!path=personalAccessToken&t=request)personalAccessToken
required string<= 191 characters

A GitLab.com Personal Access Token with at least the `read_api` scope, granted on a user with access to the groups Drata should scan. Never returned in any response.
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_createGitlabConnection!path=workspaceIds&t=request)workspaceIds Array of numbers

IDs of the Workspaces to assign this GitLab Connection to. Omit to leave the connection unassigned, in which case it must be assigned in Drata before its monitoring tests can run against it.

Responses

201
Created

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

post/gitlab-connections

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

`{"name": "My GitLab Connection","personalAccessToken": "glpat-xxxxxxxxxxxxxxxxxxxx","workspaceIds": [1]}`

Response samples

*   201
*   400
*   401
*   403
*   412
*   500

1 more

application/json

`{"id": 1,"clientAlias": "My GitLab Connection","state": "ACTIVE","tokenExpiresAt": "2025-07-01T16:45:55.246Z","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_getGitlabConnection)Get GitLab Connection

🔒 Requires **Connections: List Connections** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_getGitlabConnection!in=path&path=connectionId&t=request)connectionId
required number

Drata internal numeric ID (e.g. `10`).

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_getGitlabConnection!in=query&path=expand[]&t=request)expand[]Array of strings (StandardConnectionExpandEnum)

List of sub-collections and sub-objects to expand

Items Value:"workspaces"

Responses

200

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

get/gitlab-connections/{connectionId}

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
    `https://public-api.drata.com/public/v2/gitlab-connections/${connectionId}`,
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

`{"id": 1,"clientAlias": "My GitLab Connection","state": "ACTIVE","tokenExpiresAt": "2025-07-01T16:45:55.246Z","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_updateGitlabConnection)Update GitLab Connection

Update a GitLab Connection. This is a partial update: every field is optional, and an omitted field is left unchanged. Providing a new `personalAccessToken` re-validates the connection — for a Connection created through the GitLab OAuth flow in the Drata UI, this converts it to Personal Access Token authentication. A body that changes nothing returns a 204 with no content.

🔒 Requires **Connections: Update Connection** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_updateGitlabConnection!in=path&path=connectionId&t=request)connectionId
required number

Drata internal numeric ID (e.g. `10`).

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_updateGitlabConnection!path=name&t=request)name string<= 191 characters

Name for the GitLab Connection
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_updateGitlabConnection!path=personalAccessToken&t=request)personalAccessToken string<= 191 characters

A GitLab.com Personal Access Token with at least the `read_api` scope. Providing a new value re-validates the connection.
[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_updateGitlabConnection!path=workspaceIds&t=request)workspaceIds Array of numbers

IDs of the Workspaces this GitLab Connection should be assigned to. Replaces the current assignment: omit to leave it unchanged, or pass an empty array to unassign every Workspace.

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

put/gitlab-connections/{connectionId}

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

`{"name": "My Updated GitLab Connection","personalAccessToken": "glpat-xxxxxxxxxxxxxxxxxxxx","workspaceIds": [1]}`

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

`{"id": 1,"clientAlias": "My GitLab Connection","state": "ACTIVE","tokenExpiresAt": "2025-07-01T16:45:55.246Z","createdById": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","workspaces": [{"id": 1,"primary": false,"name": "Drata Automation","description": "Drata automates SOC 2 compliance","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_deleteGitlabConnection)Delete GitLab Connection

🔒 Requires **Connections: Delete Connection** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Standard-Connections/#tag/Standard-Connections/operation/GitlabConnectionsPublicV2Controller_deleteGitlabConnection!in=path&path=connectionId&t=request)connectionId
required number

Drata internal numeric ID (e.g. `10`).

Responses

204
No Content

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

delete/gitlab-connections/{connectionId}

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
    `https://public-api.drata.com/public/v2/gitlab-connections/${connectionId}`,
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
*   403
*   404
*   412
*   500

application/json

`{"statusCode": 0,"message": "string","code": 0,"debugInfo": {"name": "string","message": "string","stack": "string"},"providerError": {"provider": "string","status": 0,"code": "string","message": "string","details": { }}}`
