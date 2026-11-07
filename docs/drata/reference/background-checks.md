Title: Background Checks

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Background-Checks/

Markdown Content:
Background checks verify a person’s identity, history, and qualifications to ensure they meet legal, regulatory, or policy standards. The [help docs](https://help.drata.com/en/articles/5833999-background-check-management) have more information.

## [](https://developers.drata.com/openapi/reference/v2/tag/Background-Checks/#tag/Background-Checks/operation/BackgroundChecksPublicV2Controller_createBackgroundCheck)Create Manual Background Check

Create a Background Check record and mark the user as compliant for Background Check requirements.

🔒 Requires **Personnel & Users: Create Background Check** permission.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Background-Checks/#tag/Background-Checks/operation/BackgroundChecksPublicV2Controller_createBackgroundCheck!path=userId&t=request)userId
required number

The ID of the User for whom the Background Check is being uploaded
[](https://developers.drata.com/openapi/reference/v2/tag/Background-Checks/#tag/Background-Checks/operation/BackgroundChecksPublicV2Controller_createBackgroundCheck!path=url&t=request)url
required string<uri><= 768 characters

The URL of the Background Check document or report
[](https://developers.drata.com/openapi/reference/v2/tag/Background-Checks/#tag/Background-Checks/operation/BackgroundChecksPublicV2Controller_createBackgroundCheck!path=filedAt&t=request)filedAt
required string<date>

The date when this Background Check data was completed.

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

post/background-checks

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

`{"id": 1,"employmentStatus": "CURRENT_EMPLOYEE","userId": 1,"email": "john.doe@example.com","createdAt": "2023-01-01T00:00:00.000Z","updatedAt": "2023-01-01T00:00:00.000Z"}`
