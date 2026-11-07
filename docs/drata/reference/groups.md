Title: Groups

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Groups/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Groups are collections of Users that can be used to manage permissions and access to resources.

## [](https://developers.drata.com/openapi/reference/v2/tag/Groups/#tag/Groups/operation/GroupsPublicV2Controller_listGroups)List Groups

Lists personnel groups (teams / departments) for the account, optionally filtered by a case-insensitive name substring via `q`. The resolved group `id` can then be used to filter personnel via `groupIds[]` on the personnel endpoints. Reuses the Personnel read permission/scope.

🔒 Requires **Personnel & Users: List Personnel** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Groups/#tag/Groups/operation/GroupsPublicV2Controller_listGroups!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Groups/#tag/Groups/operation/GroupsPublicV2Controller_listGroups!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Groups/#tag/Groups/operation/GroupsPublicV2Controller_listGroups!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Groups/#tag/Groups/operation/GroupsPublicV2Controller_listGroups!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Groups/#tag/Groups/operation/GroupsPublicV2Controller_listGroups!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Groups/#tag/Groups/operation/GroupsPublicV2Controller_listGroups!in=query&path=q&t=request)q string<= 191 characters

Filter groups by a case-insensitive name substring (e.g. "Engineering"). Omit to list all groups.

Responses

200
Successful

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

get/groups

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
    `https://public-api.drata.com/public/v2/groups`,
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

`{"data": [{"id": 1,"name": "Engineering Team","externalId": "external-group-123","source": "GOOGLE","connectionId": 1,"createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z"}],"pagination": {"cursor": "string","totalCount": 0}}`
