Title: Device Documents

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/

Markdown Content:
Device Documents allow you to provide manual evidence of Devices compliance. Using the Drata Agent or an MDM connection automatically provides this information.

## [](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_getDeviceDocuments)List Device Documents

Get a list of Documents for a given Device and Document type.

🔒 Requires **Devices: Manage Device Documents** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_getDeviceDocuments!in=path&path=deviceId&t=request)deviceId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_getDeviceDocuments!in=query&path=type&t=request)type string (DeviceDocumentTypeEnum)

Filter documents by their type

Enum:"PASSWORD_MANAGER_EVIDENCE""AUTO_UPDATES_EVIDENCE""HARD_DRIVE_ENCRYPTION_EVIDENCE""ANTIVIRUS_EVIDENCE""LOCK_SCREEN_EVIDENCE"
[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_getDeviceDocuments!in=query&path=expand[]&t=request)expand[]Array of strings (DeviceDocumentExpandEnum)

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

404
Not Found

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

get/devices/{deviceId}/documents

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

`{"documents": [{"id": 1,"type": "PASSWORD_MANAGER_EVIDENCE","name": "Password Manager Evidence","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="},"fileName": "Artifact 3.png"}}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_uploadDocumentForDevice)Upload Device Document

Upload a new Device compliance Document for a given Device.

🔒 Requires **Devices: Manage Device Documents** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_uploadDocumentForDevice!in=path&path=deviceId&t=request)deviceId
required number

##### Request Body schema:

multipart/form-data

required

[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_uploadDocumentForDevice!ct=multipart/form-data&path=type&t=request)type
required string

The device document type

Enum:"PASSWORD_MANAGER_EVIDENCE""AUTO_UPDATES_EVIDENCE""HARD_DRIVE_ENCRYPTION_EVIDENCE""ANTIVIRUS_EVIDENCE""LOCK_SCREEN_EVIDENCE"
[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_uploadDocumentForDevice!ct=multipart/form-data&path=file&t=request)file string<binary>

Accepted file extensions: .pdf, .docx, .odt, .doc, .xlsx, .ods, .pptx, .odp, .gif, .jpg, .jpeg, .png
[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_uploadDocumentForDevice!ct=multipart/form-data&path=base64File&t=request)base64File string

JSON string with external evidence in Base64 format.

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

post/devices/{deviceId}/documents

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

`{"id": 1,"type": "PASSWORD_MANAGER_EVIDENCE","name": "Password Manager Evidence","fileUrl": "http://localhost:5000/download/device-documents/1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="},"fileName": "Artifact 3.png"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_getDeviceDocument)Get Device Document

🔒 Requires **Devices: Manage Device Documents** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_getDeviceDocument!in=path&path=deviceId&t=request)deviceId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_getDeviceDocument!in=path&path=documentId&t=request)documentId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_getDeviceDocument!in=query&path=expand[]&t=request)expand[]Array of strings (DeviceDocumentExpandEnum)

List of subcollections and sub-objects to expand

Items Value:"downloadUrl"

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

get/devices/{deviceId}/documents/{documentId}

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

`{"id": 1,"type": "PASSWORD_MANAGER_EVIDENCE","name": "Password Manager Evidence","fileUrl": "http://localhost:5000/download/device-documents/1","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","downloadUrl": {"fileBuffer": {"buffer": "RXhhbXBsZSB0ZXh0IGNvbnRlbnQ="},"fileName": "Artifact 3.png"}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_deleteDeviceDocument)Delete Device Document

Removes a specific Device Document using the provided Document ID

🔒 Requires **Devices: Manage Device Documents** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_deleteDeviceDocument!in=path&path=deviceId&t=request)deviceId
required number
[](https://developers.drata.com/openapi/reference/v2/tag/Device-Documents/#tag/Device-Documents/operation/DeviceDocumentsPublicV2Controller_deleteDeviceDocument!in=path&path=documentId&t=request)documentId
required number

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

delete/devices/{deviceId}/documents/{documentId}

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

*   400
*   401
*   403
*   404
*   412
*   500

1 more

application/json

`{"name": "string","statusCode": 0,"message": "string","code": 0}`
