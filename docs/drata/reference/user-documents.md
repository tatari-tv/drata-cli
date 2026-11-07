Title: User Documents

URL Source: https://developers.drata.com/openapi/reference/v2/tag/User-Documents/

Markdown Content:
User Documents allow you to provide manual evidence of User and Personnel compliance.

## [](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_listUserDocuments)List User Documents

Find User Documents matching the provided filters.

🔒 Requires **Personnel & Users: List User Documents** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_listUserDocuments!in=path&path=userId&t=request)userId
required number

A Drata integer ID or an email address of the form 'email:value'.

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_listUserDocuments!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_listUserDocuments!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_listUserDocuments!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_listUserDocuments!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_listUserDocuments!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_listUserDocuments!in=query&path=expand[]&t=request)expand[]Array of strings (UserDocumentExpandEnum)

List of subcollections and sub-objects to expand

Items Value:"downloadUrl"
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_listUserDocuments!in=query&path=name&t=request)name string

Filter documents by name

Example:name=Security training
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_listUserDocuments!in=query&path=type&t=request)type string

Filter documents by their type

Enum:"SEC_TRAINING""PASSWORD_MANAGER_EVIDENCE""AUTO_UPDATES_EVIDENCE""HARD_DRIVE_ENCRYPTION_EVIDENCE""ANTIVIRUS_EVIDENCE""LOCK_SCREEN_EVIDENCE""MFA_EVIDENCE""HIPAA_TRAINING_EVIDENCE""OFFBOARDING_EVIDENCE""NIST_AI_TRAINING_EVIDENCE"

Example:type=SEC_TRAINING

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

get/users/{userId}/documents

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
  const userId = 'YOUR_userId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/users/${userId}/documents`,
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

`{"data": [{"id": 1,"userId": 0,"downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="},"fileName": "Artifact 3.png"},"name": "Security Training","type": "SEC_TRAINING","renewalDate": "2026-10-27","createdAt": "2020-07-06","updatedAt": "2020-07-06"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_uploadUserDocument)Upload User Document

Upload a User Document.

🔒 Requires **Personnel & Users: Create User Document** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_uploadUserDocument!in=path&path=userId&t=request)userId
required number

A Drata integer ID or an email address of the form 'email:value'.

##### Request Body schema:

multipart/form-data

required

[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_uploadUserDocument!ct=multipart/form-data&path=type&t=request)type
required string

The user document type

Enum:"MFA_EVIDENCE""SEC_TRAINING""HIPAA_TRAINING_EVIDENCE""OFFBOARDING_EVIDENCE""NIST_AI_TRAINING_EVIDENCE"
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_uploadUserDocument!ct=multipart/form-data&path=file&t=request)file string<binary>

Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .gif, .jpg, .jpeg, .png
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_uploadUserDocument!ct=multipart/form-data&path=base64File&t=request)base64File string

JSON string with external evidence in Base64 format.
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_uploadUserDocument!ct=multipart/form-data&path=completionDate&t=request)completionDate string<date-time>

The date when the compliance was completed
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_uploadUserDocument!ct=multipart/form-data&path=fileKey&t=request)fileKey string<= 768 characters

The `objectKey` returned by `POST /upload-urls`, after the file has been PUT to the presigned URL. Use this instead of `file` or `base64File` when the document was uploaded out of band — it lifts the request-body size limits that bind the inline forms. Mutually exclusive with both of them.

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

413
The file was too large to upload

500
Internal server error

503
Third party system was unavailable

post/users/{userId}/documents

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

multipart/form-data

No sample

Response samples

*   201
*   400
*   401
*   403
*   404
*   412
*   413
*   500
*   503

4 more

application/json

`{"id": 1,"userId": 0,"downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="},"fileName": "Artifact 3.png"},"name": "Security Training","type": "SEC_TRAINING","fileUrl": "http://localhost:5000/download/documents/1","renewalDate": "2026-10-27","createdAt": "2020-07-06","updatedAt": "2020-07-06"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_getUserDocument)Get User Document

Get the full detail of a User Document.

🔒 Requires **Personnel & Users: List User Documents** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_getUserDocument!in=path&path=documentId&t=request)documentId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_getUserDocument!in=path&path=userId&t=request)userId
required number

A Drata integer ID or an email address of the form 'email:value'.

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_getUserDocument!in=query&path=expand[]&t=request)expand[]Array of strings (UserDocumentExpandEnum)

List of subcollections and sub-objects to expand

Items Value:"downloadUrl"

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

get/users/{userId}/documents/{documentId}

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
  const documentId = 'YOUR_documentId_PARAMETER';
  const userId = 'YOUR_userId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/users/${userId}/documents/${documentId}`,
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

`{"id": 1,"userId": 0,"downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="},"fileName": "Artifact 3.png"},"name": "Security Training","type": "SEC_TRAINING","fileUrl": "http://localhost:5000/download/documents/1","renewalDate": "2026-10-27","createdAt": "2020-07-06","updatedAt": "2020-07-06"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_deleteUserDocument)Delete User Document

🔒 Requires **Personnel & Users: Delete User Document** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_deleteUserDocument!in=path&path=documentId&t=request)documentId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/User-Documents/#tag/User-Documents/operation/UserDocumentsPublicV2Controller_deleteUserDocument!in=path&path=userId&t=request)userId
required number

A Drata integer ID or an email address of the form 'email:value'

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

delete/users/{userId}/documents/{documentId}

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
  const documentId = 'YOUR_documentId_PARAMETER';
  const userId = 'YOUR_userId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/users/${userId}/documents/${documentId}`,
    {
      method: 'DELETE',
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

*   400
*   401
*   403
*   404
*   412
*   500

1 more

application/json

`{"name": "string","statusCode": 0,"message": "string","code": 0}`
