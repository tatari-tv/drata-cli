Title: Devices

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Devices/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Devices are computers used by personnel. The data is provided by the Drata Agent or an MDM connection.

## [](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices)List Devices

🔒 Requires **Devices: List Devices** permission.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=externalId&t=request)externalId string<= 191 characters

An externally sourced unique identifier for a device
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=expand[]&t=request)expand[]Array of strings (DeviceExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"asset""complianceChecks""documents""identifiers"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=macAddress&t=request)macAddress string

The device MAC address

Example:macAddress=65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=serialNumber&t=request)serialNumber string

The device serial number

Example:serialNumber=NKRTSPY456
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=sourceType&t=request)sourceType string (MobileDeviceManagementSourceTypeEnum)

Device source type

Enum:"AGENT""JAMF""INTUNE""KANDJI""JUMPCLOUD""HEXNODE_UEM""UNKNOWN""RIPPLING""WORKSPACE_ONE""KOLIDE""CUSTOM""INTUNE_GCC_HIGH""CUSTOM_XFA""NINJAONE"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevices!in=query&path=personnelId&t=request)personnelId number

Personnel ID

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

get/devices

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
    `https://public-api.drata.com/public/v2/devices`,
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

`{"data": [{"id": 1,"assetId": 123,"osVersion": "MacOS 10.15.6","serialNumber": "C02T6CDJGTFL","model": "MacBook Pro","macAddress": "65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E","lastCheckedAt": "2025-07-01T16:45:55.246Z","sourceType": "AGENT","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z","appsCount": 20,"isDeviceCompliant": false,"complianceChecks": [ ],"identifiers": [ ],"documents": [ ],"screenLockTime": 30,"screenLockExplanation": {"minutesIdleBeforeSleep": "2","minutesSleepingBeforePasswordIsRequired": "2"},"antivirusEnabled": true,"antivirusExplanation": "No matching app was found","autoUpdateEnabled": true,"autoUpdateExplanation": "No compliances found","passwordManagerEnabled": true,"passwordManagerExplanation": {"passwordManagerApps": ["1password 7"]},"encryptionEnabled": false,"encryptionExplanation": "No encryption provided","firewallEnabled": true,"firewallExplanation": "{}","asset": {"id": 1,"name": "MacBook Pro - Space Black 16-inch","description": "MacBook Pro Space Black - with 16-inch Liquid Retina XDR display","assetType": "PHYSICAL","assetProvider": "AGENT","approvedAt": "2025-07-01T16:45:55.246Z","removedAt": "2025-07-01T16:45:55.246Z","company": "Acme, Inc","notes": "string","assetReferenceType": "PERSONNEL","uniqueId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353"},"userId": 1,"personnelId": 1,"externalId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel)List Devices for Personnel

🔒 Requires **Devices: List Devices** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=path&path=personnelId&t=request)personnelId
required number

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=query&path=externalId&t=request)externalId string<= 191 characters

An externally sourced unique identifier for a device
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=query&path=expand[]&t=request)expand[]Array of strings (DeviceExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"asset""complianceChecks""documents""identifiers"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=query&path=macAddress&t=request)macAddress string

The device MAC address

Example:macAddress=65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=query&path=serialNumber&t=request)serialNumber string

The device serial number

Example:serialNumber=NKRTSPY456
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForPersonnel!in=query&path=sourceType&t=request)sourceType string (MobileDeviceManagementSourceTypeEnum)

Device source type

Enum:"AGENT""JAMF""INTUNE""KANDJI""JUMPCLOUD""HEXNODE_UEM""UNKNOWN""RIPPLING""WORKSPACE_ONE""KOLIDE""CUSTOM""INTUNE_GCC_HIGH""CUSTOM_XFA""NINJAONE"

Responses

200
Successful

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

get/personnel/{personnelId}/devices

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
  const personnelId = 'YOUR_personnelId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/personnel/${personnelId}/devices`,
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

`{"data": [{"id": 1,"assetId": 123,"osVersion": "MacOS 10.15.6","serialNumber": "C02T6CDJGTFL","model": "MacBook Pro","macAddress": "65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E","lastCheckedAt": "2025-07-01T16:45:55.246Z","sourceType": "AGENT","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z","appsCount": 20,"isDeviceCompliant": false,"complianceChecks": [ ],"identifiers": [ ],"documents": [ ],"screenLockTime": 30,"screenLockExplanation": {"minutesIdleBeforeSleep": "2","minutesSleepingBeforePasswordIsRequired": "2"},"antivirusEnabled": true,"antivirusExplanation": "No matching app was found","autoUpdateEnabled": true,"autoUpdateExplanation": "No compliances found","passwordManagerEnabled": true,"passwordManagerExplanation": {"passwordManagerApps": ["1password 7"]},"encryptionEnabled": false,"encryptionExplanation": "No encryption provided","firewallEnabled": true,"firewallExplanation": "{}","asset": {"id": 1,"name": "MacBook Pro - Space Black 16-inch","description": "MacBook Pro Space Black - with 16-inch Liquid Retina XDR display","assetType": "PHYSICAL","assetProvider": "AGENT","approvedAt": "2025-07-01T16:45:55.246Z","removedAt": "2025-07-01T16:45:55.246Z","company": "Acme, Inc","notes": "string","assetReferenceType": "PERSONNEL","uniqueId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353"},"userId": 1,"personnelId": 1,"externalId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevice)Get Device

🔒 Requires **Devices: List Devices** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevice!in=path&path=deviceId&t=request)deviceId
required number

The device ID

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevice!in=query&path=expand[]&t=request)expand[]Array of strings

List of subcollections and sub-objects to expand

Items Enum:"asset""complianceChecks""documents""identifiers""deviceApps"

Example:expand[]=complianceChecks

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

get/devices/{deviceId}

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
  const deviceId = 'YOUR_deviceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/devices/${deviceId}`,
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

`{"id": 1,"assetId": 123,"osVersion": "MacOS 10.15.6","serialNumber": "C02T6CDJGTFL","model": "MacBook Pro","macAddress": "65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E","lastCheckedAt": "2025-07-01T16:45:55.246Z","sourceType": "AGENT","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z","appsCount": 20,"isDeviceCompliant": false,"complianceChecks": [ ],"identifiers": [ ],"documents": [ ],"screenLockTime": 30,"screenLockExplanation": {"minutesIdleBeforeSleep": "2","minutesSleepingBeforePasswordIsRequired": "2"},"antivirusEnabled": true,"antivirusExplanation": "No matching app was found","autoUpdateEnabled": true,"autoUpdateExplanation": "No compliances found","passwordManagerEnabled": true,"passwordManagerExplanation": {"passwordManagerApps": ["1password 7"]},"encryptionEnabled": false,"encryptionExplanation": "No encryption provided","firewallEnabled": true,"firewallExplanation": "{}","asset": {"id": 1,"name": "MacBook Pro - Space Black 16-inch","description": "MacBook Pro Space Black - with 16-inch Liquid Retina XDR display","assetType": "PHYSICAL","assetProvider": "AGENT","approvedAt": "2025-07-01T16:45:55.246Z","removedAt": "2025-07-01T16:45:55.246Z","company": "Acme, Inc","notes": "string","assetReferenceType": "PERSONNEL","uniqueId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353"},"userId": 1,"personnelId": 1,"externalId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd","apps": [{"id": 1,"installedApp": "Adobe Photoshop version 3.3 license 1a2b3c4d"}]}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection)List Devices for Connection 🧪

Get a list of Devices for a given connection

🧪 **Note:** This endpoint is in beta and may change.

🔒 Requires **Devices: List Devices** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=path&path=connectionId&t=request)connectionId
required number

The ID of the connection may be obtained in one of two ways:

*   Calling GET /connections and noting the `id` of the desired custom device connection in the response.
*   In the Drata App:
    *   **New Device Connection**

        1.   Navigate to _Connections_**>**_MDM_.
        2.   Select the _Available Connections_ tab.
        3.   Chose _Custom Device Connection_.

After successfully creating a device connection, the connection ID will be displayed under _Account Information_.

    *   **Existing Device Connection**

        1.   Navigate to _Connections_**>**_MDM_.
        2.   Select the _Active Connections_ tab.
        3.   View a previously created custom device connection.

The connection ID will be displayed under _Account Information_.

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=externalId&t=request)externalId string<= 191 characters

An externally sourced unique identifier for a device
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=expand[]&t=request)expand[]Array of strings (DeviceExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"asset""complianceChecks""documents""identifiers"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=macAddress&t=request)macAddress string

The device MAC address

Example:macAddress=65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=serialNumber&t=request)serialNumber string

The device serial number

Example:serialNumber=NKRTSPY456
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=personnelId&t=request)personnelId number

Personnel Id

Responses

200
Successful

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

get/connections/{connectionId}/devices

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
    `https://public-api.drata.com/public/v2/connections/${connectionId}/devices`,
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

`{"data": [{"id": 1,"assetId": 123,"osVersion": "MacOS 10.15.6","serialNumber": "C02T6CDJGTFL","model": "MacBook Pro","macAddress": "65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E","lastCheckedAt": "2025-07-01T16:45:55.246Z","sourceType": "AGENT","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z","appsCount": 20,"isDeviceCompliant": false,"complianceChecks": [ ],"identifiers": [ ],"documents": [ ],"screenLockTime": 30,"screenLockExplanation": {"minutesIdleBeforeSleep": "2","minutesSleepingBeforePasswordIsRequired": "2"},"antivirusEnabled": true,"antivirusExplanation": "No matching app was found","autoUpdateEnabled": true,"autoUpdateExplanation": "No compliances found","passwordManagerEnabled": true,"passwordManagerExplanation": {"passwordManagerApps": ["1password 7"]},"encryptionEnabled": false,"encryptionExplanation": "No encryption provided","firewallEnabled": true,"firewallExplanation": "{}","asset": {"id": 1,"name": "MacBook Pro - Space Black 16-inch","description": "MacBook Pro Space Black - with 16-inch Liquid Retina XDR display","assetType": "PHYSICAL","assetProvider": "AGENT","approvedAt": "2025-07-01T16:45:55.246Z","removedAt": "2025-07-01T16:45:55.246Z","company": "Acme, Inc","notes": "string","assetReferenceType": "PERSONNEL","uniqueId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353"},"userId": 1,"personnelId": 1,"externalId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection)List Devices for Connection 🧪

Get a list of Devices for a given connection

🧪 **Note:** This endpoint is in beta and may change.

🔒 Requires **Devices: List Devices** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=path&path=connectionId&t=request)connectionId
required number

The ID of the connection may be obtained in one of two ways:

*   Calling GET /connections and noting the `id` of the desired custom device connection in the response.
*   In the Drata App:
    *   **New Device Connection**

        1.   Navigate to _Connections_**>**_MDM_.
        2.   Select the _Available Connections_ tab.
        3.   Chose _Custom Device Connection_.

After successfully creating a device connection, the connection ID will be displayed under _Account Information_.

    *   **Existing Device Connection**

        1.   Navigate to _Connections_**>**_MDM_.
        2.   Select the _Active Connections_ tab.
        3.   View a previously created custom device connection.

The connection ID will be displayed under _Account Information_.

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=externalId&t=request)externalId string<= 191 characters

An externally sourced unique identifier for a device
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=expand[]&t=request)expand[]Array of strings (DeviceExpandEnum)

List of subcollections and sub-objects to expand

Items Enum:"asset""complianceChecks""documents""identifiers"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=macAddress&t=request)macAddress string

The device MAC address

Example:macAddress=65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=serialNumber&t=request)serialNumber string

The device serial number

Example:serialNumber=NKRTSPY456
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDevicesForCustomConnection!in=query&path=personnelId&t=request)personnelId number

Personnel Id

Responses

200
Successful

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

get/connections/{connectionId}/devices

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
    `https://public-api.drata.com/public/v2/connections/${connectionId}/devices`,
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

`{"data": [{"id": 1,"assetId": 123,"osVersion": "MacOS 10.15.6","serialNumber": "C02T6CDJGTFL","model": "MacBook Pro","macAddress": "65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E","lastCheckedAt": "2025-07-01T16:45:55.246Z","sourceType": "AGENT","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z","appsCount": 20,"isDeviceCompliant": false,"complianceChecks": [ ],"identifiers": [ ],"documents": [ ],"screenLockTime": 30,"screenLockExplanation": {"minutesIdleBeforeSleep": "2","minutesSleepingBeforePasswordIsRequired": "2"},"antivirusEnabled": true,"antivirusExplanation": "No matching app was found","autoUpdateEnabled": true,"autoUpdateExplanation": "No compliances found","passwordManagerEnabled": true,"passwordManagerExplanation": {"passwordManagerApps": ["1password 7"]},"encryptionEnabled": false,"encryptionExplanation": "No encryption provided","firewallEnabled": true,"firewallExplanation": "{}","asset": {"id": 1,"name": "MacBook Pro - Space Black 16-inch","description": "MacBook Pro Space Black - with 16-inch Liquid Retina XDR display","assetType": "PHYSICAL","assetProvider": "AGENT","approvedAt": "2025-07-01T16:45:55.246Z","removedAt": "2025-07-01T16:45:55.246Z","company": "Acme, Inc","notes": "string","assetReferenceType": "PERSONNEL","uniqueId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353"},"userId": 1,"personnelId": 1,"externalId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDeviceApps)List Apps for Device

Get a list of Apps associated with a given Device.

🔒 Requires **Devices: List Devices** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDeviceApps!in=path&path=deviceId&t=request)deviceId
required number

The device ID

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDeviceApps!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDeviceApps!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDeviceApps!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDeviceApps!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_getDeviceApps!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false

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

get/devices/{deviceId}/apps

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
  const deviceId = 'YOUR_deviceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/devices/${deviceId}/apps`,
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

`{"data": [{"id": 1,"installedApp": "Adobe Photoshop version 3.3 license 1a2b3c4d"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection)Create or update Device for Custom Connection 🧪

Create a new device or update an existing device for a custom connection.

An existing device is searched for based upon matching one or more of these supplied properties in the request payload:

*   `serialNumber`
*   `macAddress`
*   `externalId`

If an existing device is found, the existing device is updated. Otherwise a new device is created.

🧪 **Note:** This endpoint is in beta and may change.

🔒 Requires **Devices: Create Device** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!in=path&path=connectionId&t=request)connectionId
required number

The ID of the connection may be obtained in one of two ways:

*   Calling GET /connections and noting the `id` of the desired custom device connection in the response.
*   In the Drata App:
    *   **New Device Connection**

        1.   Navigate to _Connections_**>**_MDM_.
        2.   Select the _Available Connections_ tab.
        3.   Chose _Custom Device Connection_.

After successfully creating a device connection, the connection ID will be displayed under _Account Information_.

    *   **Existing Device Connection**

        1.   Navigate to _Connections_**>**_MDM_.
        2.   Select the _Active Connections_ tab.
        3.   View a previously created custom device connection.

The connection ID will be displayed under _Account Information_.

##### Request Body schema: application/json
required

required number or string

Personnel integer ID or user's email address prefixed with `email:`
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=alias&t=request)alias string<= 191 characters

Name of the device.
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=antivirusEnabled&t=request)antivirusEnabled boolean or null

Flag to indicate antivirus software is installed and enabled
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=antivirusExplanation&t=request)antivirusExplanation object

Any additional information to explain the antivirusEnabled value
Array of objects (AppTypeRequestPublicV2Dto)

List of installed applications
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=autoUpdateEnabled&t=request)autoUpdateEnabled boolean or null

Flag to indicate auto-update enabled or disabled
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=autoUpdateExplanation&t=request)autoUpdateExplanation object

Any additional information to explain the autoUpdateEnabled value
Array of objects (AppTypeRequestPublicV2Dto)

List of installed browser extensions
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=externalId&t=request)externalId string

An externally-sourced unique identifier for a device
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=firewallEnabled&t=request)firewallEnabled boolean or null

Flag to indicate the firewall is enabled or disabled
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=firewallExplanation&t=request)firewallExplanation object

Any additional information to explain the firewallEnabled value
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=passwordManagerEnabled&t=request)passwordManagerEnabled boolean or null

Flag to indicate a password manager is in use
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=encryptionEnabled&t=request)encryptionEnabled boolean or null

Flag to indicate hard disk is encrypted
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=encryptionExplanation&t=request)encryptionExplanation object

Any additional information to explain the encryptionEnabled value
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=model&t=request)model string<= 191 characters

Hardware model
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=serialNumber&t=request)serialNumber string

Hardware serial number
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=macAddress&t=request)macAddress string or null

MAC address
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=passwordManagerExplanation&t=request)passwordManagerExplanation object

Any additional information to explain the hasPasswordManager value
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=platformName&t=request)platformName
required string

The Operating System (OS) platform name of the device

Enum:"MACOS""WINDOWS""LINUX""UNIX""ANDROID"
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=platformVersion&t=request)platformVersion
required string<= 191 characters

The Operating System (OS) platform version of the device
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=screenLockEnabled&t=request)screenLockEnabled boolean or null

Flag to indicate hard disk is encrypted
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=screenLockExplanation&t=request)screenLockExplanation object

Any additional information to explain the screenLockTime value
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_createDeviceForCustomConnection!path=screenLockTime&t=request)screenLockTime number

Amount of time before display is turned off
Array of objects (WindowsServiceRequestPublicV2Dto)

List of applicable Windows services

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

post/custom-connections/{connectionId}/devices

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

`{"personnelId": 1,"alias": "danielm-01","antivirusEnabled": true,"antivirusExplanation": {"antivirusApps": ["Crowdstrike windows sensor"]},"appList": [{"name": "Microsoft Defender Antivirus","version": "1.421.12.0","description": "Microsoft Defender Antivirus"}],"autoUpdateEnabled": true,"autoUpdateExplanation": "Disabled","browserExtensions": [{"name": "Microsoft Defender Antivirus","version": "1.421.12.0","description": "Microsoft Defender Antivirus"}],"externalId": "c3d318ab-c51d-45b1-9d05-9fb3c481d788","firewallEnabled": true,"firewallExplanation": "On","passwordManagerEnabled": true,"encryptionEnabled": true,"encryptionExplanation": {"bootPartitionEncryptionDetails": {"partitionFileVault2Percent": 100,"partitionFileVault2State": "ENCRYPTED","partitionName": "Macintosh HD (Boot Partition)"}},"model": "Mac16,1","serialNumber": "BKH8RXT4T9","macAddress": "01-23-45-67-89-AB","passwordManagerExplanation": {"passwordManagerApps": ["1password"]},"platformName": "MACOS","platformVersion": "string","screenLockEnabled": true,"screenLockExplanation": "ScreenLock delay is immediate","screenLockTime": 15,"windowsServices": [{"description": "Security feature that monitors and controls network traffic entering and exiting the device","name": "Windows Firewall","startType": "Automatic","status": "Running"}]}`

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

`{"id": 1,"assetId": 123,"osVersion": "MacOS 10.15.6","serialNumber": "C02T6CDJGTFL","model": "MacBook Pro","macAddress": "65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E","lastCheckedAt": "2025-07-01T16:45:55.246Z","sourceType": "AGENT","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","deletedAt": "2025-07-01T16:45:55.246Z","appsCount": 20,"isDeviceCompliant": false,"complianceChecks": [ ],"identifiers": [ ],"documents": [ ],"screenLockTime": 30,"screenLockExplanation": {"minutesIdleBeforeSleep": "2","minutesSleepingBeforePasswordIsRequired": "2"},"antivirusEnabled": true,"antivirusExplanation": "No matching app was found","autoUpdateEnabled": true,"autoUpdateExplanation": "No compliances found","passwordManagerEnabled": true,"passwordManagerExplanation": {"passwordManagerApps": ["1password 7"]},"encryptionEnabled": false,"encryptionExplanation": "No encryption provided","firewallEnabled": true,"firewallExplanation": "{}","asset": {"id": 1,"name": "MacBook Pro - Space Black 16-inch","description": "MacBook Pro Space Black - with 16-inch Liquid Retina XDR display","assetType": "PHYSICAL","assetProvider": "AGENT","approvedAt": "2025-07-01T16:45:55.246Z","removedAt": "2025-07-01T16:45:55.246Z","company": "Acme, Inc","notes": "string","assetReferenceType": "PERSONNEL","uniqueId": "C02T6CDJGTFL","createdAt": "2025-07-01T16:45:55.246Z","updatedAt": "2025-07-01T16:45:55.246Z","externalId": "i-0c844e3b433e4e3f","externalOwnerId": "account-353"},"userId": 1,"personnelId": 1,"externalId": "aaaaaaaa-bbbb-0000-cccc-dddddddddddd"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_deleteDeviceFromCustomConnection)Delete Device

Delete a Device from a Custom Connection.

🔒 Requires **Devices: Delete Device** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_deleteDeviceFromCustomConnection!in=path&path=connectionId&t=request)connectionId
required number

The ID of the connection may be obtained in one of two ways:

*   Calling GET /connections and noting the `id` of the desired custom device connection in the response.
*   In the Drata App:
    *   **New Device Connection**

        1.   Navigate to _Connections_**>**_MDM_.
        2.   Select the _Available Connections_ tab.
        3.   Chose _Custom Device Connection_.

After successfully creating a device connection, the connection ID will be displayed under _Account Information_.

    *   **Existing Device Connection**

        1.   Navigate to _Connections_**>**_MDM_.
        2.   Select the _Active Connections_ tab.
        3.   View a previously created custom device connection.

The connection ID will be displayed under _Account Information_.
[](https://developers.drata.com/openapi/reference/v2/tag/Devices/#tag/Devices/operation/DevicesPublicV2Controller_deleteDeviceFromCustomConnection!in=path&path=deviceId&t=request)deviceId
required number

The Device ID

Responses

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

delete/custom-connections/{connectionId}/devices/{deviceId}

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
  const deviceId = 'YOUR_deviceId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/custom-connections/${connectionId}/devices/${deviceId}`,
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
*   403
*   404
*   412
*   500

1 more

application/json

`{"name": "string","statusCode": 0,"message": "string","code": 0}`
