Title: Tasks

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Tasks/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Tasks are individual units of work that can be assigned to users.

## [](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks)List Tasks

Find Tasks matching the provided filters.

🔒 Requires **Tasks: List and Get Tasks** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=sort&t=request)sort string (TaskSortEnum)

Which field to sort by

Enum:"createdAt""updatedAt""dueDate"
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=expand[]&t=request)expand[]Array of strings (TaskExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"assignee""createdBy""controls""risks""policies""schedule"
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=status&t=request)status string (TaskStatus)

Filter by Task status

| Enum Value | **Description** |
| --- | --- |
| PAST_DUE | Past Due – Due date has passed and task is not completed |
| COMPLETED | Completed – Task has been marked as complete |
| INCOMPLETE | Incomplete – Task is not yet due and not completed |
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=assigneeId&t=request)assigneeId number

Filter by assigned User ID

Example:assigneeId=456
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=createdById&t=request)createdById number

Filter by User ID who created the task

Example:createdById=123
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=dueDateFrom&t=request)dueDateFrom string<date>

Filter Tasks with due date on or after this value (inclusive)

Example:dueDateFrom=2020-07-06
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=dueDateTo&t=request)dueDateTo string<date>

Filter Tasks with due date on or before this value (inclusive)

Example:dueDateTo=2020-07-06
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=controlId&t=request)controlId number

Filter by linked Control ID

Example:controlId=101
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=taskType&t=request)taskType string (TaskTypeEnum)

Filter by Task type

Enum:"POLICY_RENEWALS""EVIDENCE""VENDOR""EXTERNAL_EVIDENCE""REPORT""GENERAL""CONTROL""RISK""CONTROL_APPROVALS""POLICY_APPROVALS"
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=title&t=request)title string

Filter by Task title (prefix search)

Example:title=Review SOC 2
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_listTasks!in=query&path=description&t=request)description string

Filter by Task description (prefix search)

Example:description=quarterly review

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

get/workspaces/{workspaceId}/tasks

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/tasks`,
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

`{"data": [{"id": 123,"title": "Review SOC 2 controls","description": "Quarterly review of all SOC 2 Type II controls","status": "INCOMPLETE","taskType": "GENERAL","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assigneeId": 456,"createdById": 789,"isRecurring": false,"assignee": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"createdBy": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"controls": {"data": [[ ]],"totalCount": 0},"risks": {"data": [[ ]],"totalCount": 0},"policies": {"data": [[ ]],"totalCount": 0},"schedule": {"type": "monthly","interval": 1,"start": "2020-07-06","end": "2020-07-06","dayOfMonth": 15,"weekNumber": "SECOND_WEEK","weekday": "FR","daysOfWeek": ["MO","TH"],"month": 3,"day": 20}}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask)Create Task

Create a new Task.

Pass `configurationId` with `dueDate` to create the Task for one occurrence of a recurring Task, giving it an ID the other Task endpoints accept. Upcoming Tasks returns both values on any row whose `sourceId` is `null`.

Every other property comes from the recurring Task, so none may be sent with `configurationId`, and the occurrence is removed from the recurrence permanently.

🔒 Requires **Tasks: Create Task** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask!path=configurationId&t=request)configurationId number

Create this Task from an occurrence of a recurring Task, as returned by `configurationId` on Upcoming Tasks. `dueDate` then selects which occurrence, and every other property is taken from the recurring Task, so none may be sent.

This is a short-lived handle rather than a durable identifier: it is replaced whenever the recurring Task is edited, so list Upcoming Tasks and create from the value in that response rather than caching it.
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask!path=title&t=request)title string<= 255 characters

Task title. Required unless `configurationId` is supplied.
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask!path=description&t=request)description string<= 768 characters

Task description
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask!path=dueDate&t=request)dueDate
required string<date>

Due date. With `configurationId` this selects the occurrence to create and must be a date that recurring Task actually falls on.
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask!path=assigneeId&t=request)assigneeId number

ID of the User to assign the task to
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask!path=taskType&t=request)taskType string

Default:"GENERAL"

Task type

Enum:"POLICY_RENEWALS""EVIDENCE""VENDOR""EXTERNAL_EVIDENCE""REPORT""GENERAL""CONTROL""RISK""CONTROL_APPROVALS""POLICY_APPROVALS"
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask!path=controlIds&t=request)controlIds Array of numbers

IDs of Controls to link to this Task. Can be combined with `riskIds` and `policyIds`.
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask!path=riskIds&t=request)riskIds Array of numbers

IDs of Risks to link to this Task. Can be combined with `controlIds` and `policyIds`.
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_createTask!path=policyIds&t=request)policyIds Array of numbers

IDs of Policies to link to this Task. Can be combined with `controlIds` and `riskIds`.

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

409
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/workspaces/{workspaceId}/tasks

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

`{"configurationId": 42,"title": "Review access controls","description": "Quarterly review of access control policies","dueDate": "2020-07-06","assigneeId": 456,"taskType": "GENERAL","controlIds": [101,102],"riskIds": [201],"policyIds": [301,302]}`

Response samples

*   201
*   400
*   401
*   403
*   404
*   409
*   412
*   500

3 more

application/json

`{"id": 123,"title": "Review SOC 2 controls","description": "Quarterly review of all SOC 2 Type II controls","status": "INCOMPLETE","taskType": "GENERAL","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assigneeId": 456,"createdById": 789,"isRecurring": false,"assignee": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"createdBy": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"controls": {"data": [[ ]],"totalCount": 0},"risks": {"data": [[ ]],"totalCount": 0},"policies": {"data": [[ ]],"totalCount": 0},"schedule": {"type": "monthly","interval": 1,"start": "2020-07-06","end": "2020-07-06","dayOfMonth": 15,"weekNumber": "SECOND_WEEK","weekday": "FR","daysOfWeek": ["MO","TH"],"month": 3,"day": 20}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_getTask)Get Task

Get details for a specific Task.

🔒 Requires **Tasks: List and Get Tasks** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_getTask!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_getTask!in=path&path=taskId&t=request)taskId
required number

The Task ID

Example:123

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_getTask!in=query&path=expand[]&t=request)expand[]Array of strings (TaskExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"assignee""createdBy""controls""risks""policies""schedule"

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

get/workspaces/{workspaceId}/tasks/{taskId}

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
  const taskId = 'YOUR_taskId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/tasks/${taskId}`,
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

`{"id": 123,"title": "Review SOC 2 controls","description": "Quarterly review of all SOC 2 Type II controls","status": "INCOMPLETE","taskType": "GENERAL","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assigneeId": 456,"createdById": 789,"isRecurring": false,"assignee": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"createdBy": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"controls": {"data": [[ ]],"totalCount": 0},"risks": {"data": [[ ]],"totalCount": 0},"policies": {"data": [[ ]],"totalCount": 0},"schedule": {"type": "monthly","interval": 1,"start": "2020-07-06","end": "2020-07-06","dayOfMonth": 15,"weekNumber": "SECOND_WEEK","weekday": "FR","daysOfWeek": ["MO","TH"],"month": 3,"day": 20}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_updateTask)Update Task

Update an existing Task.

🔒 Requires **Tasks: Update Task** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_updateTask!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_updateTask!in=path&path=taskId&t=request)taskId
required number

The Task ID

Example:123

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_updateTask!path=title&t=request)title string<= 255 characters

Task title
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_updateTask!path=description&t=request)description string or null<= 768 characters

Task description. Set to null to clear.
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_updateTask!path=dueDate&t=request)dueDate string<date>

Due date
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_updateTask!path=assigneeId&t=request)assigneeId number or null

ID of the User who should complete the task. Set to null to unassign.
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_updateTask!path=controlIds&t=request)controlIds Array of numbers

IDs of Controls to link to this task. Associates the task with compliance controls, making it visible in control detail views and filterable by control. Replaces all existing control links. Pass `[]` to clear all links. Omit to leave unchanged.
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_updateTask!path=riskIds&t=request)riskIds Array of numbers

IDs of Risks to link. Replaces all existing links. Pass `[]` to clear. Omit to leave unchanged.
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_updateTask!path=policyIds&t=request)policyIds Array of numbers

IDs of Policies to link. Replaces all existing links. Pass `[]` to clear. Omit to leave unchanged.

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

put/workspaces/{workspaceId}/tasks/{taskId}

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

`{"title": "string","description": "string","dueDate": "2020-07-06","assigneeId": 0,"controlIds": [0],"riskIds": [0],"policyIds": [0]}`

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

`{"id": 123,"title": "Review SOC 2 controls","description": "Quarterly review of all SOC 2 Type II controls","status": "INCOMPLETE","taskType": "GENERAL","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assigneeId": 456,"createdById": 789,"isRecurring": false,"assignee": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"createdBy": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"controls": {"data": [[ ]],"totalCount": 0},"risks": {"data": [[ ]],"totalCount": 0},"policies": {"data": [[ ]],"totalCount": 0},"schedule": {"type": "monthly","interval": 1,"start": "2020-07-06","end": "2020-07-06","dayOfMonth": 15,"weekNumber": "SECOND_WEEK","weekday": "FR","daysOfWeek": ["MO","TH"],"month": 3,"day": 20}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_performTaskAction)Perform Task Action

Perform an action on a Task: complete or uncomplete.

🔒 Requires **Tasks: Update Task** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_performTaskAction!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_performTaskAction!in=path&path=taskId&t=request)taskId
required number

The Task ID

Example:123

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/TasksPublicV2Controller_performTaskAction!path=action&t=request)action
required string

Action to execute: `complete` marks the task as completed, `uncomplete` marks it as incomplete

Enum:"complete""uncomplete"

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

post/workspaces/{workspaceId}/tasks/{taskId}/actions

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
*   403
*   404
*   412
*   500

2 more

application/json

`{"id": 123,"title": "Review SOC 2 controls","description": "Quarterly review of all SOC 2 Type II controls","status": "INCOMPLETE","taskType": "GENERAL","dueDate": "2020-07-06","completedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","assigneeId": 456,"createdById": 789,"isRecurring": false,"assignee": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"createdBy": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"controls": {"data": [[ ]],"totalCount": 0},"risks": {"data": [[ ]],"totalCount": 0},"policies": {"data": [[ ]],"totalCount": 0},"schedule": {"type": "monthly","interval": 1,"start": "2020-07-06","end": "2020-07-06","dayOfMonth": 15,"weekNumber": "SECOND_WEEK","weekday": "FR","daysOfWeek": ["MO","TH"],"month": 3,"day": 20}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks)List Upcoming Tasks

Find Upcoming Tasks for a workspace. Returns Drata-assigned tasks sourced from policy renewals, vendor reviews, external evidence due dates, library document renewals, control approvals, and custom tasks.

Occurrences of recurring Tasks are included. Those with no Task created for them yet have a `null``sourceId` and a `configurationId`. Create the Task by sending that `configurationId` and the row's `dueDate` to `POST /workspaces/{workspaceId}/tasks`. The `dueDateTo` filter bounds how far ahead occurrences are projected; without it they reach six months out.

**Note:** Vendor and policy renewal tasks are account-level and will appear identically across all workspaces for a given account.

🔒 Requires **Tasks: List and Get Tasks** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks!in=path&path=workspaceId&t=request)workspaceId
required number

The Workspace ID associated to the Account

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks!in=query&path=sort&t=request)sort string (UpcomingTaskSortEnum)

Which field to sort by

Value:"dueDate"
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on the first page (when cursor is null) and when `dueDateTo` is supplied, since a recurrence with no end date has no finite number of occurrences to count.

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks!in=query&path=status&t=request)status string (TaskStatus)

Filter by Task status

| Enum Value | **Description** |
| --- | --- |
| PAST_DUE | Past Due – Due date has passed and task is not completed |
| COMPLETED | Completed – Task has been marked as complete |
| INCOMPLETE | Incomplete – Task is not yet due and not completed |
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks!in=query&path=taskType&t=request)taskType string (TaskTypeEnum)

Filter by Task type

Enum:"POLICY_RENEWALS""EVIDENCE""VENDOR""EXTERNAL_EVIDENCE""REPORT""GENERAL""CONTROL""RISK""CONTROL_APPROVALS""POLICY_APPROVALS"
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks!in=query&path=dueDateFrom&t=request)dueDateFrom string<date>

Filter Tasks with due date on or after this value (inclusive)

Example:dueDateFrom=2020-07-06
[](https://developers.drata.com/openapi/reference/v2/tag/Tasks/#tag/Tasks/operation/UpcomingTasksPublicV2Controller_listUpcomingTasks!in=query&path=dueDateTo&t=request)dueDateTo string<date>

Filter Tasks with due date on or before this value (inclusive)

Example:dueDateTo=2020-07-06

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

get/workspaces/{workspaceId}/upcoming-tasks

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
    `https://public-api.drata.com/public/v2/workspaces/${workspaceId}/upcoming-tasks`,
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

`{"data": [{"sourceId": 123,"name": "Acme Corp Vendor Review","taskType": "VENDOR","dueDate": "2020-07-06","status": "INCOMPLETE","completedAt": "2025-07-01T16:45:55.246Z","assigneeIds": [456,789],"configurationId": 42}],"pagination": {"cursor": "string","totalCount": 0}}`
