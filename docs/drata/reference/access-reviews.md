Title: Access Reviews

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
User Access Reviews (UAR) attest who has access to which Application. Completing a review is an ordered flow: create an Application, load its user set, conduct a review (which opens a review period and snapshots each Application into it), complete every Application in that period, and only then complete the period itself — completing the period fails while any Application in it is still incomplete. Only one review period may be active on an account at a time.

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplications)List Access Review Applications

🔒 Requires **Access Review: List Access Review Applications** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplications!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplications!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplications!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplications!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplications!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false

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

get/access-review-applications

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
    `https://public-api.drata.com/public/v2/access-review-applications`,
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

`{"data": [{"id": 1,"name": "AWS","source": "MANUALLY_ADDED","reviewers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_createApplication)Create Access Review Application

Create a manually-managed Access Review Application (a system whose user access will be reviewed). Load its user set separately via PUT .../access-review-applications/:applicationId/users.

🔒 Requires **Access Review: Create Access Review Application** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_createApplication!path=name&t=request)name
required string<= 191 characters

Application name
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_createApplication!path=websiteUrl&t=request)websiteUrl string<= 191 characters

Vendor URL
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_createApplication!path=reviewersIds&t=request)reviewersIds
required Array of numbers<= 300 items

IDs of Drata Users to assign as the default reviewers of this Application. Look up valid IDs via GET /public/v2/users.

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
An Application with this name and website URL already exists.

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/access-review-applications

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

`{"name": "AWS","websiteUrl": "https://acme.com","reviewersIds": [1,2,3]}`

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

`{"id": 1,"name": "AWS","websiteUrl": "https://acme.com","source": "MANUALLY_ADDED","reviewers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_getApplication)Get Access Review Application

🔒 Requires **Access Review: Get Access Review Application** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_getApplication!in=path&path=applicationId&t=request)applicationId
required number

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

get/access-review-applications/{applicationId}

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
  const applicationId = 'YOUR_applicationId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/access-review-applications/${applicationId}`,
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

`{"id": 1,"name": "AWS","websiteUrl": "https://acme.com","source": "MANUALLY_ADDED","reviewers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_updateApplication)Update Access Review Application

Update an Access Review Application's name, vendor URL, or default reviewers. Omitted fields are left unchanged. Name and vendor URL can be changed only on manually-managed Applications; reviewers can be changed on any Application.

🔒 Requires **Access Review: Update Access Review Application** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_updateApplication!in=path&path=applicationId&t=request)applicationId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_updateApplication!path=name&t=request)name string<= 191 characters

Application name. Omit to leave it unchanged.
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_updateApplication!path=websiteUrl&t=request)websiteUrl string or null<= 191 characters

Vendor URL. Omit to leave it unchanged, or pass null to clear it.
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_updateApplication!path=reviewersIds&t=request)reviewersIds Array of numbers<= 300 items

IDs of Drata Users to assign as the default reviewers of this Application. Replaces the current reviewers: omit to leave them unchanged. An Application must keep at least one reviewer, so an empty array is rejected.

Responses

200
Successful

204
No fields were provided, so nothing was changed.

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
An Application with this name and website URL already exists.

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

put/access-review-applications/{applicationId}

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

`{"name": "AWS","websiteUrl": "https://acme.com","reviewersIds": [1,2,3]}`

Response samples

*   200
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

`{"id": 1,"name": "AWS","websiteUrl": "https://acme.com","source": "MANUALLY_ADDED","reviewers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_deleteApplication)Delete Access Review Application

Delete a manually-managed Access Review Application. It is also removed from any review period in which it has not been completed. The identity records (name and email) of its loaded users are retained rather than deleted.

🔒 Requires **Access Review: Delete Access Review Application** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_deleteApplication!in=path&path=applicationId&t=request)applicationId
required number

Responses

204
No Content

400
Only manually-managed Applications can be deleted.

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

delete/access-review-applications/{applicationId}

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
  const applicationId = 'YOUR_applicationId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/access-review-applications/${applicationId}`,
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

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplicationUsers)List Access Review Application Users

List the user set currently loaded into an Access Review Application (see PUT .../access-review-applications/:applicationId/users).

🔒 Requires **Access Review: List Access Review Application Users** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplicationUsers!in=path&path=applicationId&t=request)applicationId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplicationUsers!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplicationUsers!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplicationUsers!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplicationUsers!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listApplicationUsers!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false

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

get/access-review-applications/{applicationId}/users

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
  const applicationId = 'YOUR_applicationId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/access-review-applications/${applicationId}/users`,
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

`{"data": [{"id": 1,"identityId": "1a2b3c","username": "jane.doe","email": "jane.doe@acme.com","roles": ["Admin","ReadOnly"],"hasAdminAccess": false,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_loadApplicationUsers)Create or Update Access Review Application Users

Load the full set of reviewable users for a manually-managed Access Review Application. Calling this again treats the submitted list as the complete set: users omitted from it are removed from the Application and from its open review, and their identity record (name and email) is retained in a disconnected state rather than deleted. The request is rejected while the Application has been completed within its still-open review period, or once reviewers have recorded decisions in it; after that review period is completed, users can be loaded again (for example, to prepare the Application for its next review cycle).

🔒 Requires **Access Review: Load Access Review Application User Set** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_loadApplicationUsers!in=path&path=applicationId&t=request)applicationId
required number

##### Request Body schema: application/json
required

required Array of objects (AccessReviewUserIdentityRequestPublicV2Dto) <= 4000 items

The Application's full set of reviewable users. Users omitted from this list are removed from the Application and its open review; their identity record (name and email) is retained in a disconnected state, not deleted.

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
This Application has already been completed while its review period is still open, or reviewers have already recorded decisions in it. Once that review period is completed, users can be loaded again.

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

put/access-review-applications/{applicationId}/users

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

`{"identities": [{"identityId": "1a2b3c","username": "jane.doe","email": "jane.doe@acme.com","roles": ["Admin","ReadOnly"]}]}`

Response samples

*   200
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

`{"savedCount": 3}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriods)List Access Reviews

List review periods, optionally filtered to active or completed ones.

🔒 Requires **Access Review: List Access Reviews** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriods!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriods!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriods!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriods!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriods!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriods!in=query&path=status&t=request)status string (AccessReviewPeriodStatusEnum)

Filter by review period status: `ACTIVE` periods have not been completed yet, `COMPLETED` periods have. Omit to return both.

Enum:"ACTIVE""COMPLETED"

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

get/access-review-periods

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
    `https://public-api.drata.com/public/v2/access-review-periods`,
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

`{"data": [{"id": 1,"startingDate": "2019-08-24T14:15:22Z","endingDate": "2019-08-24T14:15:22Z","completedAt": "2019-08-24T14:15:22Z","applications": [{"id": 1,"applicationId": 1,"status": "NOT_STARTED"}]}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_conductReview)Conduct Access Review

Start a review period, scoping in one or more Applications and assigning reviewers. Only one review period may be active on the account at a time.

🔒 Requires **Access Review: Conduct Access Review** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_conductReview!path=startingDate&t=request)startingDate
required string<date-time>

Starting date for the review period
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_conductReview!path=endingDate&t=request)endingDate
required string<date-time>

Ending date for the review period
required Array of objects (AccessReviewPeriodApplicationRequestPublicV2Dto) <= 300 items

The Applications to review in this period, each with its own reviewer(s). Only one review period may be active on the account at a time.

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
Another review period is already active on this account.

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/access-review-periods

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

`{"startingDate": "2025-07-01T16:45:55.246Z","endingDate": "2025-07-01T16:45:55.246Z","applications": [{"applicationId": 1,"reviewersIds": [1,2,3]}]}`

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

`{"id": 1,"startingDate": "2019-08-24T14:15:22Z","endingDate": "2019-08-24T14:15:22Z","completedAt": "2019-08-24T14:15:22Z","applications": [{"id": 1,"applicationId": 1,"status": "NOT_STARTED"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_getPeriod)Get Access Review

Get a review period and the Applications scoped into it.

🔒 Requires **Access Review: Get Access Review** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_getPeriod!in=path&path=periodId&t=request)periodId
required number

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

get/access-review-periods/{periodId}

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
  const periodId = 'YOUR_periodId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/access-review-periods/${periodId}`,
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

`{"id": 1,"startingDate": "2019-08-24T14:15:22Z","endingDate": "2019-08-24T14:15:22Z","completedAt": "2019-08-24T14:15:22Z","applications": [{"id": 1,"applicationId": 1,"status": "NOT_STARTED"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_getPeriodApplication)Get Application in Access Review

Get one Application within a review period, with its reviewers and counts of its users by review decision.

🔒 Requires **Access Review: Get Application in Access Review** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_getPeriodApplication!in=path&path=periodId&t=request)periodId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_getPeriodApplication!in=path&path=reviewAppId&t=request)reviewAppId
required number

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

get/access-review-periods/{periodId}/applications/{reviewAppId}

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
  const periodId = 'YOUR_periodId_PARAMETER';
  const reviewAppId = 'YOUR_reviewAppId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/access-review-periods/${periodId}/applications/${reviewAppId}`,
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

`{"id": 1,"applicationId": 1,"status": "IN_PROGRESS","completedAt": "2025-07-01T16:45:55.246Z","reviewers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"userCounts": {"notReviewed": 3,"approved": 10,"rejected": 1,"outOfScope": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_updatePeriodApplication)Update Application Reviewers in Access Review

Reassign the reviewers of one Application within a review period. New reviewers are also added to the Application's default reviewers. Rejected once the Application or its review period has been completed.

🔒 Requires **Access Review: Update Application Reviewers in Access Review** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_updatePeriodApplication!in=path&path=periodId&t=request)periodId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_updatePeriodApplication!in=path&path=reviewAppId&t=request)reviewAppId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_updatePeriodApplication!path=reviewersIds&t=request)reviewersIds Array of numbers<= 300 items

IDs of Drata Users to review this Application in the review period. Replaces the current reviewers: omit to leave them unchanged. At least one reviewer is required, so an empty array is rejected. New reviewers are also added to the Application's default reviewers.

Responses

200
Successful

204
No fields were provided, so nothing was changed.

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
The Application or its review period has already been completed.

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

put/access-review-periods/{periodId}/applications/{reviewAppId}

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

`{"reviewersIds": [1,2,3]}`

Response samples

*   200
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

`{"id": 1,"applicationId": 1,"status": "IN_PROGRESS","completedAt": "2025-07-01T16:45:55.246Z","reviewers": [{"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"userCounts": {"notReviewed": 3,"approved": 10,"rejected": 1,"outOfScope": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriodApplicationUsers)List Application Users in Access Review

List the users under review in one Application within a review period, with each user's review decision.

🔒 Requires **Access Review: List Application Users in Access Review** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriodApplicationUsers!in=path&path=periodId&t=request)periodId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriodApplicationUsers!in=path&path=reviewAppId&t=request)reviewAppId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriodApplicationUsers!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriodApplicationUsers!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriodApplicationUsers!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriodApplicationUsers!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_listPeriodApplicationUsers!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false

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

get/access-review-periods/{periodId}/applications/{reviewAppId}/users

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
  const periodId = 'YOUR_periodId_PARAMETER';
  const reviewAppId = 'YOUR_reviewAppId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/access-review-periods/${periodId}/applications/${reviewAppId}/users`,
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

`{"data": [{"id": 1,"identityId": "1a2b3c","username": "jane.doe","email": "jane.doe@acme.com","roles": ["Admin","ReadOnly"],"status": "APPROVED","completedBy": {"id": 1,"email": "email@example.com","firstName": "Sally","lastName": "Smith","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"},"lastRejectedAt": "2025-07-01T16:45:55.246Z","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_getPeriodEvidencePackage)Download Access Review Evidence Package

Get a short-lived signed URL for the evidence package ZIP of a review period. The package is generated in the background as Applications are completed, and covers every Application in the period.

🔒 Requires **Access Review: Download Access Review Evidence Package** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_getPeriodEvidencePackage!in=path&path=periodId&t=request)periodId
required number

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
The review period does not exist, or its evidence package is not ready.

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/access-review-periods/{periodId}/evidence-package

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
  const periodId = 'YOUR_periodId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/access-review-periods/${periodId}/evidence-package`,
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

`{"signedUrl": "https://somedomain.com/filename.pdf?Signature=ABC123","fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="},"fileName": "Artifact 3.png"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_performPeriodApplicationAction)Perform Access Review Application Action

`complete`: mark one Application within a review period as reviewed. Every Application in a period must be completed before the period itself can be completed. If any of the Application's users are still NOT_REVIEWED, the request is rejected with 409 unless `markUnreviewedUsersOutOfScope` is `true`, which marks them OUT_OF_SCOPE, recorded as completed by the calling API key's user; users who already have a decision keep it. That sweep runs asynchronously after this call returns; confirm it through `userCounts` on GET .../access-review-periods/:periodId/applications/:reviewAppId.

🔒 Requires **Access Review: Complete Access Review Application** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_performPeriodApplicationAction!in=path&path=periodId&t=request)periodId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_performPeriodApplicationAction!in=path&path=reviewAppId&t=request)reviewAppId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_performPeriodApplicationAction!path=action&t=request)action
required string

Action to execute: `complete` marks the Application within the review period as reviewed.

Value:"complete"
[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_performPeriodApplicationAction!path=markUnreviewedUsersOutOfScope&t=request)markUnreviewedUsersOutOfScope boolean

Required when any of the Application's users are still NOT_REVIEWED: set to `true` to mark them OUT_OF_SCOPE on completion. Without it, the request is rejected with 409 while unreviewed users remain.

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
This Application has already been completed, or still has NOT_REVIEWED users and `markUnreviewedUsersOutOfScope` was not set.

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/access-review-periods/{periodId}/applications/{reviewAppId}/actions

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

`{"action": "complete","markUnreviewedUsersOutOfScope": true}`

Response samples

*   200
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

`{"id": 1,"applicationId": 1,"status": "NOT_STARTED"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_performPeriodAction)Perform Access Review Action

`complete`: complete a review period. Every Application scoped into the period must already be completed (see POST .../applications/:reviewAppId/actions).

🔒 Requires **Access Review: Complete Access Review** permission.

💎 Requires your account have the **Access Review** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_performPeriodAction!in=path&path=periodId&t=request)periodId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Access-Reviews/#tag/Access-Reviews/operation/AccessReviewPublicV2Controller_performPeriodAction!path=action&t=request)action
required string

Action to execute: `complete` completes the review period. Every Application scoped into the period must already be completed.

Value:"complete"

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
Cannot complete a review period with incomplete Applications, or the review period has already been completed.

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/access-review-periods/{periodId}/actions

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
*   409
*   412
*   500

4 more

application/json

`{"id": 1,"startingDate": "2019-08-24T14:15:22Z","endingDate": "2019-08-24T14:15:22Z","completedAt": "2019-08-24T14:15:22Z","applications": [{"id": 1,"applicationId": 1,"status": "NOT_STARTED"}]}`
