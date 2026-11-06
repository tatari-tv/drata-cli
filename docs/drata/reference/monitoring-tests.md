Title: Monitoring Tests

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/

Markdown Content:
Monitoring Tests are compliance tests used to determine whether a person, system, process, or organization is adhering to standards set by compliance controls within a framework. The [help docs](https://help.drata.com/en/articles/4790546-monitoring) have more information.

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors)List Monitoring Tests

Find Monitoring Tests matching the provided filters

🔒 Requires **Monitoring: List Monitors** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=expand[]&t=request)expand[]Array of strings (MonitorExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"controls""monitorInstances""disablingUser"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=checkResultStatus&t=request)checkResultStatus string (CheckResultStatusEnum)

Filter Monitors by result status

Enum:"READY""PASSED""FAILED""ERROR""PREAUDIT"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=checkStatus&t=request)checkStatus string (CheckStatusEnum)

Filter Monitors by system status

| Enum Value | **Description** |
| --- | --- |
| UNUSED | Test is not currently in use |
| NEW | Test has been newly added and not yet evaluated |
| ENABLED | Test is active and running |
| DISABLED | Test has been turned off |
| TESTING | Test is in a testing state |
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=type&t=request)type string (CheckTypeEnum)

Filter Monitors by check type (associated monitor instances)

Enum:"POLICY""IN_DRATA""AGENT""INFRASTRUCTURE""VERSION_CONTROL""IDENTITY""TICKETING""HRIS""OBSERVABILITY""CUSTOM"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=testSource&t=request)testSource string (TestSourceEnum)

Filter monitors by test source type (DRATA for built-in tests, CUSTOM for user-created tests, ACORN for compliance-as-code tests, EXTERNAL for external tests, DRATA_LIBRARY for Drata library tests)

| Enum Value | **Description** |
| --- | --- |
| DRATA | Built-in Drata test |
| CUSTOM | User-created test |
| EXTERNAL | External test |
| ACORN | Compliance-as-code test |
| DRATA_LIBRARY | Drata library test |
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=draft&t=request)draft boolean

Filter Monitors by draft state: true returns only drafts, false (the default) only published tests

Example:draft=false
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitors!in=query&path=name&t=request)name string<= 191 characters

Filter Monitors by name (prefix match, case-insensitive). Send it again with each `cursor` page.

Example:name=MFA

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

get/workspaces/{workspaceId}/monitoring-tests

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
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"data": [{"id": 1,"name": "A Version Control System is being Used","description": "Inspected Drata's version control system...","checkResultStatus": "PASSED","checkStatus": "ENABLED","testSource": "DRATA","draft": false,"testId": 1,"parentTestId": null,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","lastPassedAt": "2025-07-01T16:45:55.246Z","failedSince": "2025-07-01T16:45:55.246Z","monitorInstances": [{"id": 1,"checkResultStatus": "PASSED","collectionDescription": "Curl call to the website","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","checkTypes": ["INFRASTRUCTURE"]}],"disabledByUser": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createCustomMonitoringTest)Create Custom Monitoring Test

Create a custom Monitoring Test over Custom Connection data. The test is created as a **draft**: run, publish, and map it to controls in the Drata app. Validate the recipe first with the validate endpoint.

🔒 Requires **Monitoring: Create Custom Monitor** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createCustomMonitoringTest!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createCustomMonitoringTest!path=connectionIds&t=request)connectionIds Array of numbers<= 300 items

IDs of the Custom Connections in this workspace the test runs against, from `GET /custom-connections`. Required when `recipe.allConnections` is false and ignored when it is true. Each must be a connected, active Custom Connection assigned to this workspace, and together they must cover every provider in the recipe.
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createCustomMonitoringTest!path=name&t=request)name
required string<= 191 characters

Name of the custom Monitoring Test
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createCustomMonitoringTest!path=description&t=request)description string<= 30000 characters

Plain-language explanation of what the Monitoring Test checks, saved as the description of the evidence it collects. The Monitoring Test `description` in responses is generated by Drata and does not change.
required object

The test logic. Validate it first with `POST /workspaces/{workspaceId}/monitoring-tests/action-validate-recipe`.

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

409
The account has reached its custom test limit

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/workspaces/{workspaceId}/monitoring-tests

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

`{"connectionIds": [1],"name": "Active employees have a department","description": "Every active employee record must be assigned to a department.","recipe": {"assertion": "nofail","allConnections": true,"providers": [{"provider": "HR System","resources": [{"resource": "Employees","evaluator": {"all": [{"fact": "department","operator": "notEqual","value": ""},{"fact": "isActive","operator": "equal","value": true}]},"filteringCriteria": {"mode": "inclusion","evaluator": {"all": [{"fact": "isActive","operator": "equal","value": true}]}}}]}]}}`

Response samples

*   201
*   400
*   401
*   402
*   403
*   404
*   409
*   412
*   500

4 more

application/json

`{"id": 1,"name": "A Version Control System is being Used","description": "Inspected Drata's version control system...","checkResultStatus": "PASSED","checkStatus": "ENABLED","testSource": "DRATA","draft": false,"testId": 1,"parentTestId": null,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","lastPassedAt": "2025-07-01T16:45:55.246Z","failedSince": "2025-07-01T16:45:55.246Z","monitorInstances": [{"id": 1,"checkResultStatus": "PASSED","collectionDescription": "Curl call to the website","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","checkTypes": ["INFRASTRUCTURE"]}],"disabledByUser": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_getMonitor)Get Monitoring Test

Get a specific Monitoring Test by Test ID

🔒 Requires **Monitoring: Get Monitor** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_getMonitor!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_getMonitor!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field — `id` is an internal account-wide database key and is not accepted here.

Example:1

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_getMonitor!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_getMonitor!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_getMonitor!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_getMonitor!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_getMonitor!in=query&path=expand[]&t=request)expand[]Array of strings (MonitorExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"controls""monitorInstances""disablingUser"

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

get/workspaces/{workspaceId}/monitoring-tests/{testId}

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
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"id": 1,"name": "A Version Control System is being Used","description": "Inspected Drata's version control system...","checkResultStatus": "PASSED","checkStatus": "ENABLED","testSource": "DRATA","draft": false,"testId": 1,"parentTestId": null,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","lastPassedAt": "2025-07-01T16:45:55.246Z","failedSince": "2025-07-01T16:45:55.246Z","monitorInstances": [{"id": 1,"checkResultStatus": "PASSED","collectionDescription": "Curl call to the website","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","checkTypes": ["INFRASTRUCTURE"]}],"disabledByUser": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_updateMonitor)Update Monitoring Test

Update a Monitoring Test's name, description, and/or enabled state.

🔒 Requires **Monitoring: Update Monitor** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_updateMonitor!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_updateMonitor!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field — `id` is an internal account-wide database key and is not accepted here.

Example:1

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_updateMonitor!path=name&t=request)name string<= 191 characters

Updated name for the Monitoring Test.
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_updateMonitor!path=enabled&t=request)enabled boolean

Enable or disable the Monitoring Test. When set to false, downstream monitor instances stop running and a disable message of "Disabled via Public API" is recorded.
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_updateMonitor!path=description&t=request)description string or null<= 30000 characters

Updated description for the Monitoring Test. Pass null to clear.

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

409
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

put/workspaces/{workspaceId}/monitoring-tests/{testId}

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

`{"name": "Cloud Data Storage Exposure","enabled": true,"description": "string"}`

Response samples

*   200
*   400
*   401
*   403
*   404
*   409
*   412
*   500

3 more

application/json

`{"id": 1,"name": "A Version Control System is being Used","description": "Inspected Drata's version control system...","checkResultStatus": "PASSED","checkStatus": "ENABLED","testSource": "DRATA","draft": false,"testId": 1,"parentTestId": null,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","lastPassedAt": "2025-07-01T16:45:55.246Z","failedSince": "2025-07-01T16:45:55.246Z","monitorInstances": [{"id": 1,"checkResultStatus": "PASSED","collectionDescription": "Curl call to the website","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","checkTypes": ["INFRASTRUCTURE"]}],"disabledByUser": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_validateCustomTestRecipe)Validate Custom Monitoring Test Recipe

Check custom test logic (a recipe) against the Custom Connection data sources in this workspace without saving anything. Returns every error with its location; fix them and validate again until `isValid` is true, then create or update the test. Responds 200 whether or not the recipe is valid. API keys need the Create Custom Monitor permission to call this endpoint; OAuth clients need the `create:monitor-test` scope.

🔒 Requires **Monitoring: Create Custom Monitor** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_validateCustomTestRecipe!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: application/json
required

required object

The test logic to validate

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

post/workspaces/{workspaceId}/monitoring-tests/action-validate-recipe

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

`{"recipe": {"assertion": "nofail","allConnections": true,"providers": [{"provider": "HR System","resources": [{"resource": "Employees","evaluator": {"all": [{"fact": "department","operator": "notEqual","value": ""},{"fact": "isActive","operator": "equal","value": true}]},"filteringCriteria": {"mode": "inclusion","evaluator": {"all": [{"fact": "isActive","operator": "equal","value": true}]}}}]}]}}`

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

`{"isValid": false,"errors": [{"type": "invalidOperator","message": "Operator equal is not supported for fact department","location": "/providers/0/resources/0/evaluator/all/1/operator","provider": "HR System","resource": "Employees"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_updateCustomMonitoringTestRecipe)Update Custom Monitoring Test Recipe

Replace the test logic (recipe) of a custom Monitoring Test draft. Published tests cannot be edited here: create a draft of the test in the Drata app first.

🔒 Requires **Monitoring: Update Monitor** permission.

💎 Requires your account have the **Custom Connections and Tests** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_updateCustomMonitoringTestRecipe!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
required number or string

The `testId` field returned in the Monitoring Test response (workspace-scoped), or the draft's exact name prefixed with `name:`. A name matches custom drafts only. Do not pass the `id` field — `id` is an internal account-wide database key and is not accepted here.

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_updateCustomMonitoringTestRecipe!path=connectionIds&t=request)connectionIds Array of numbers<= 300 items

IDs of the Custom Connections in this workspace the test runs against, from `GET /custom-connections`. Required when `recipe.allConnections` is false and ignored when it is true. Each must be a connected, active Custom Connection assigned to this workspace, and together they must cover every provider in the recipe.
required object

The replacement test logic. Validate it first with `POST /workspaces/{workspaceId}/monitoring-tests/action-validate-recipe`.

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

409
The test is not a custom test, or it is published rather than a draft

412
You must accept the Drata terms and conditions to use the API

422
Multiple custom drafts match the provided name. Use the numeric testId.

500
Internal server error

put/workspaces/{workspaceId}/monitoring-tests/{testId}/recipe

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

`{"connectionIds": [1],"recipe": {"assertion": "nofail","allConnections": true,"providers": [{"provider": "HR System","resources": [{"resource": "Employees","evaluator": {"all": [{"fact": "department","operator": "notEqual","value": ""},{"fact": "isActive","operator": "equal","value": true}]},"filteringCriteria": {"mode": "inclusion","evaluator": {"all": [{"fact": "isActive","operator": "equal","value": true}]}}}]}]}}`

Response samples

*   200
*   400
*   401
*   402
*   403
*   404
*   409
*   412
*   422
*   500

5 more

application/json

`{"id": 1,"name": "A Version Control System is being Used","description": "Inspected Drata's version control system...","checkResultStatus": "PASSED","checkStatus": "ENABLED","testSource": "DRATA","draft": false,"testId": 1,"parentTestId": null,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","lastPassedAt": "2025-07-01T16:45:55.246Z","failedSince": "2025-07-01T16:45:55.246Z","monitorInstances": [{"id": 1,"checkResultStatus": "PASSED","collectionDescription": "Curl call to the website","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","checkTypes": ["INFRASTRUCTURE"]}],"disabledByUser": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"controls": [{"id": 1,"code": "AC-1","name": "Access Control","description": "Drata has implemented tools to monitor Drata's databases and notify appropriate personnel of any events or incidents based on\n                 predetermined criteria. Incidents are escalated per policy.","isReady": true,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorExclusions)List Monitoring Test Exclusions

Find Monitor Exclusions for a specific Monitoring Test

🔒 Requires **Monitoring: Get Monitor** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorExclusions!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorExclusions!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field — `id` is an internal account-wide database key and is not accepted here.

Example:1

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorExclusions!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorExclusions!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorExclusions!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorExclusions!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorExclusions!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorExclusions!in=query&path=expand[]&t=request)expand[]Array of strings (MonitorExclusionExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"connection""exclusionDesignator"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorExclusions!in=query&path=targetId&t=request)targetId string<= 30000 characters

Filter exclusions by target resource ID

Example:targetId=sg-0a1b2c3d4e5f67890

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

get/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions

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
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"data": [{"id": 1,"targetId": "sg-0a1b2c3d4e5f67890","targetName": "Production Security Group - Web Servers","exclusionReason": "Legacy system scheduled for decommission in Q2 2024 - approved by security team","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connection": {"id": "1","clientType": "GOOGLE","clientId": "drata.com","clientAlias": "My-connection-alias-1","state": "ACTIVE","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","connectedAt": "2025-07-01T16:45:55.246Z","failedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z"},"exclusionDesignator": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createMonitorExclusions)Create Monitoring Test Exclusions

Exclude failures of a Monitoring Test, with a reason. Pass the `id` and `connectionId` of each failure from List Monitoring Test Failures. Each item is reported in request order with its own `statusCode` and outcome. Returns 201 when every failure is excluded, and 207 when some are excluded and others are not. When none is excluded, returns 500 if any exclusion could not be saved, 422 if any failure is not a current failure of the test, or 409 otherwise, still with every item. Exclusions apply from the next test run. Returns 400 for codebase Monitoring Tests, and for tests that use Compliance Check Exclusions instead.

🔒 Requires **Monitoring: Exclude Monitoring Test Failures** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createMonitorExclusions!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createMonitorExclusions!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field; `id` is an internal account-wide database key and is not accepted here.

Example:1

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createMonitorExclusions!path=reason&t=request)reason
required string<= 30000 characters

Reason for the exclusion, applied to every failure in the request.
required Array of objects (MonitorExclusionTargetRequestPublicV2Dto) [ 1 .. 300 ] items

Failures to exclude. Each item is reported with its own outcome, so one that is not a current failure, is already excluded, or repeats an earlier item does not fail the batch as long as another failure is excluded.

Responses

201
Created

207
Multi-Status

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

409
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

422
Unprocessable Entity

500
Internal server error

post/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions

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

`{"reason": "Hosts the public website, so public access is intended.","exclusions": [{"id": "sg-0a1b2c3d4e5f67890","connectionId": 1}]}`

Response samples

*   201
*   207
*   400
*   401
*   403
*   404
*   409
*   412
*   422
*   500

5 more

application/json

`{"data": [{"error": {"message": "The failure is already excluded."},"statusCode": 201,"outcome": "EXCLUDED","data": {"id": "sg-0a1b2c3d4e5f67890","connectionId": 1,"exclusionIds": [10]}}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_deleteMonitorExclusions)Delete Monitoring Test Exclusions

Delete Monitoring Test Exclusions by ID, so their failures count against the test again from the next test run. Each ID is reported in request order with its own `statusCode` and outcome. Returns 200 when every ID is deleted, 207 when some are deleted and others are not, and 422 with every item when none is deleted. Returns 400 for codebase Monitoring Tests, and for tests that use Compliance Check Exclusions instead.

🔒 Requires **Monitoring: Re-include Monitoring Test Exclusions** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_deleteMonitorExclusions!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_deleteMonitorExclusions!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field; `id` is an internal account-wide database key and is not accepted here.

Example:1

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_deleteMonitorExclusions!path=exclusionIds&t=request)exclusionIds
required Array of numbers[ 1 .. 300 ] items

IDs of the exclusions to delete, from List Monitoring Test Exclusions, or from List Monitoring Test Codebase Exclusions for a codebase test. Each is reported with its own outcome, so an ID that is not an exclusion on this test, or repeats an earlier ID, does not fail the batch as long as another ID is deleted.

Responses

200
Successful

207
Multi-Status

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

post/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions/action-delete

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

`{"exclusionIds": [1,2]}`

Response samples

*   200
*   207
*   400
*   401
*   403
*   404
*   412
*   422
*   500

4 more

application/json

`{"data": [{"error": {"message": "The failure is already excluded."},"statusCode": 200,"outcome": "DELETED","data": {"exclusionId": 10}}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorCodebaseExclusions)List Monitoring Test Codebase Exclusions

List the Codebase Exclusions of a codebase Monitoring Test. Returns 400 for Monitoring Tests that are not codebase tests, and 404 for a Monitoring Test that is not in the workspace.

🔒 Requires **Monitoring: Get Monitor** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorCodebaseExclusions!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorCodebaseExclusions!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field; `id` is an internal account-wide database key and is not accepted here.

Example:1

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorCodebaseExclusions!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorCodebaseExclusions!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorCodebaseExclusions!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorCodebaseExclusions!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorCodebaseExclusions!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorCodebaseExclusions!in=query&path=expand[]&t=request)expand[]Array of strings (MonitorCodebaseExclusionExpandEnum)

List of subcollections and sub-objects to expand

Items Value:"exclusionDesignator"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorCodebaseExclusions!in=query&path=repositoryName&t=request)repositoryName string<= 191 characters

Filter Codebase Exclusions by the exact name of a Code Repository, as returned in the `repositoryName` field of Codebase Exclusions and Monitoring Test Failures.

Example:repositoryName=example-org/example-repo

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

get/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions

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
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"data": [{"id": 1,"findingId": "8000|example-config-id|a1b2c3d4|example-org/example-repo|main.tf|example_bucket|aws_s3_bucket|2","repositoryName": "example-org/example-repo","reason": "Bucket is intentionally public to serve static website assets","exclusionDesignatorId": 1,"exclusionDesignator": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createMonitorCodebaseExclusions)Create Monitoring Test Codebase Exclusions

Exclude Findings of a codebase Monitoring Test, with a reason. Pass the `findingId` of each failure from List Monitoring Test Failures. Each Finding is reported in request order with its own `statusCode` and outcome. Returns 201 when every Finding is excluded, and 207 when some are excluded and others are not. When none is excluded, returns 500 if any exclusion could not be saved, 422 if any Finding is not a current failure of the test or has no Code Repository, or 409 otherwise, still with every item. Returns 400 for Monitoring Tests that are not codebase tests. Codebase Exclusions are scoped to the `testId`, account-wide.

🔒 Requires **Monitoring: Exclude Monitoring Test Failures** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createMonitorCodebaseExclusions!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createMonitorCodebaseExclusions!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field; `id` is an internal account-wide database key and is not accepted here.

Example:1

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createMonitorCodebaseExclusions!path=reason&t=request)reason
required string<= 30000 characters

Reason for the exclusion, applied to every Finding in the request.
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_createMonitorCodebaseExclusions!path=findingIds&t=request)findingIds
required Array of strings[ 1 .. 300 ] items

The `findingId` of each Finding to exclude, from List Monitoring Test Failures. Treat it as an opaque ID for one test: it can change when the Finding ID format is upgraded, so read it from a recent response rather than storing it. Each is reported with its own outcome, so one that is not a current failure, is already excluded, or repeats an earlier ID does not fail the batch.

Responses

201
Created

207
Multi-Status

400
Malformed data and/or validation errors

401
Invalid Authorization

403
You are not allowed to perform this action

404
Not Found

409
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

422
Unprocessable Entity

500
Internal server error

post/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions

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

`{"reason": "The bucket hosts the public website, so public access is intended.","findingIds": ["1|example-config-id|a1b2c3d4|example-org/example-repo|main.tf|example_bucket|aws_s3_bucket|2"]}`

Response samples

*   201
*   207
*   400
*   401
*   403
*   404
*   409
*   412
*   422
*   500

5 more

application/json

`{"data": [{"error": {"message": "The failure is already excluded."},"statusCode": 201,"outcome": "EXCLUDED","data": {"findingId": "1|example-config-id|a1b2c3d4|example-org/example-repo|main.tf|example_bucket|aws_s3_bucket|2","exclusionIds": [10]}}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_deleteMonitorCodebaseExclusions)Delete Monitoring Test Codebase Exclusions

Delete Codebase Exclusions by ID, so their Findings count against the test again from the next test run. Each ID is reported in request order with its own `statusCode` and outcome. Returns 200 when every ID is deleted, 207 when some are deleted and others are not, and 422 with every item when none is deleted. Returns 400 for Monitoring Tests that are not codebase tests. Codebase Exclusions are scoped to the `testId`, account-wide.

🔒 Requires **Monitoring: Re-include Monitoring Test Exclusions** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_deleteMonitorCodebaseExclusions!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_deleteMonitorCodebaseExclusions!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field; `id` is an internal account-wide database key and is not accepted here.

Example:1

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_deleteMonitorCodebaseExclusions!path=exclusionIds&t=request)exclusionIds
required Array of numbers[ 1 .. 300 ] items

IDs of the exclusions to delete, from List Monitoring Test Exclusions, or from List Monitoring Test Codebase Exclusions for a codebase test. Each is reported with its own outcome, so an ID that is not an exclusion on this test, or repeats an earlier ID, does not fail the batch as long as another ID is deleted.

Responses

200
Successful

207
Multi-Status

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

post/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions/action-delete

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

`{"exclusionIds": [1,2]}`

Response samples

*   200
*   207
*   400
*   401
*   403
*   404
*   412
*   422
*   500

4 more

application/json

`{"data": [{"error": {"message": "The failure is already excluded."},"statusCode": 200,"outcome": "DELETED","data": {"exclusionId": 10}}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestFailures)List Monitoring Test Failures

Find Monitoring Test Failures for a specific Monitoring Test. By default, excludes failures that have been manually excluded unless includeExclusions is set to true. Each item carries a `status` of `OPEN` when it is still counted against the test, or `EXCLUDED` when it is not.

🔒 Requires **Monitoring: Get Monitor** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestFailures!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestFailures!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field — `id` is an internal account-wide database key and is not accepted here.

Example:1

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestFailures!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestFailures!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestFailures!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestFailures!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestFailures!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestFailures!in=query&path=includeExclusions&t=request)includeExclusions boolean

Default:false

Include excluded Monitor Test failures in the results. Excluded findings are returned with `status` set to `EXCLUDED`; findings still failing the test are returned with `status` set to `OPEN`.

Example:includeExclusions=false
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestFailures!in=query&path=expand[]&t=request)expand[]Array of strings (MonitorTestFailureExpandEnum)

List of subcollections and sub-objects to expand. Pass `tags` to include cloud-provider resource tags on each item when available. Tag resolution may add latency to the response. Tags are not present for every resource.

Items Value:"tags"

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

get/workspaces/{workspaceId}/monitoring-tests/{testId}/failures

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
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"data": [{"id": "sg-0a1b2c3d4e5f67890","status": "OPEN","providerName": "Amazon Web Services","resourceName": "example-web-server-sg","accountName": "Example Production Account","clientId": "123456789012","connectionId": 1,"subscriptionId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd","resourceArn": "arn:aws:ec2:us-east-1:123456789012:security-group/sg-0a1b2c3d4e5f67890","organizationalUnitId": "ou-root-123456789","region": "us-east-1","resourceType": "Microsoft.Network/loadBalancers","tags": {"cost-center": "cc-example-001","system.id": "example-system-id","environment": "production"},"cause": [{ }],"findingId": "8000|example-config-id|a1b2c3d4|example-org/example-repo|main.tf|example_bucket|aws_s3_bucket|2","repositoryName": "example-org/example-repo"}],"pagination": {"cursor": "string","totalCount": 0},"lastCheck": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestPasses)List Monitoring Test Passes

Find resources that are currently passing a specific Monitoring Test. Returns an empty page for tests that do not emit passing-resource data (see the Drata documentation for the per-test passing-data status).

🔒 Requires **Monitoring: Get Monitor** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestPasses!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestPasses!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field — `id` is an internal account-wide database key and is not accepted here.

Example:1

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestPasses!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestPasses!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestPasses!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestPasses!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestPasses!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_listMonitorTestPasses!in=query&path=expand[]&t=request)expand[]Array of strings (MonitorTestPassExpandEnum)

List of subcollections and sub-objects to expand. Pass `tags` to include cloud-provider resource tags on each item when available. Tag resolution may add latency to the response. Tags are not present for every resource.

Items Value:"tags"

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

get/workspaces/{workspaceId}/monitoring-tests/{testId}/passes

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
*   400
*   401
*   403
*   404
*   412
*   500

2 more

application/json

`{"data": [{"id": "sg-0a1b2c3d4e5f67890","providerName": "Amazon Web Services","resourceName": "example-web-server-sg","accountName": "Example Production Account","clientId": "123456789012","subscriptionId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd","resourceArn": "arn:aws:ec2:us-east-1:123456789012:security-group/sg-0a1b2c3d4e5f67890","organizationalUnitId": "ou-root-123456789","region": "us-east-1","resourceType": "Microsoft.Network/loadBalancers","tags": {"cost-center": "cc-example-001","system.id": "example-system-id","environment": "production"},"cause": [{ }]}],"pagination": {"cursor": "string","totalCount": 0},"lastCheck": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_retestMonitor)Run Monitoring Test

Run a Monitoring Test immediately instead of waiting for its schedule. The run is asynchronous: a 200 means the run was accepted, not that it passed. Poll the Get Monitoring Test endpoint until `checkStatus` leaves `TESTING` to read the result. Returns 409 if a run for this test is already in flight, and 429 if the test was re-run within the cooldown window that follows a completed run.

🔒 Requires **Monitoring: Trigger Monitor Test** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_retestMonitor!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Monitoring-Tests/#tag/Monitoring-Tests/operation/MonitorsPublicV2Controller_retestMonitor!in=path&path=testId&t=request)testId
required number

The `testId` field returned in the Monitoring Test response (workspace-scoped). Do not pass the `id` field — `id` is an internal account-wide database key and is not accepted here.

Example:1

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

409
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

429
The test was re-run within the cooldown window. Retry after the window elapses.

500
Internal server error

post/workspaces/{workspaceId}/monitoring-tests/{testId}/action-retest

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
*   400
*   401
*   403
*   404
*   409
*   412
*   429
*   500

4 more

application/json

`{"testId": 1,"checkStatus": "TESTING"}`
