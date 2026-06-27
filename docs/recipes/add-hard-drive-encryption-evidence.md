Title: Add Hard Drive Encryption Evidence

URL Source: https://developers.drata.com/developer-portal/v2/recipes/add-hard-drive-encryption-evidence/

Published Time: Fri, 26 Jun 2026 18:55:00 GMT

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/recipes/add-hard-drive-encryption-evidence/#step-1-get-list-of-personnel)Step 1: Get list of personnel

Get the list of personnel in Drata. By using the `expand[]=complianceChecks` query parameter, you will get a list of all personnel in Drata with compliance checks including the `HDD_ENCRYPTION` check. (e.g. `GET /public/v2/personnel?expand[]=complianceChecks`)
Choose a personnel that you’d like to upload evidence for and take note of their personnel ID.

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
  const query = new URLSearchParams({
    cursor: 'string',
    size: '50',
    sort: 'createdAt',
    sortDir: 'ASC',
    includeTotalCount: 'false',
    'expand[]': 'customFields',
    'employmentStatus[]': 'CURRENT_EMPLOYEE',
    'complianceStatus[]': 'MISCONFIGURED'
  }).toString();

  const resp = await fetch(
    `https://public-api.drata.com/public/v2/personnel?${query}`,
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

## [](https://developers.drata.com/developer-portal/v2/recipes/add-hard-drive-encryption-evidence/#step-2-get-device-id)Step 2: Get Device ID

Using the personnel ID selected from above, get their associated devices and take note of the device ID you want to upload evidence for.

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
  const query = new URLSearchParams({
    cursor: 'string',
    size: '50',
    sort: 'createdAt',
    sortDir: 'ASC',
    includeTotalCount: 'false',
    externalId: 'string',
    'expand[]': 'asset',
    macAddress: '65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E',
    serialNumber: 'NKRTSPY456',
    sourceType: 'AGENT'
  }).toString();

  const personnelId = 'YOUR_personnelId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/personnel/${personnelId}/devices?${query}`,
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

## [](https://developers.drata.com/developer-portal/v2/recipes/add-hard-drive-encryption-evidence/#step-3-attach-file-for-that-device)Step 3: Attach file for that device

For the device ID you selected above in Step 2, attach a file for evidence of hard drive encryption.

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

## [](https://developers.drata.com/developer-portal/v2/recipes/add-hard-drive-encryption-evidence/#step-4-confirm-upload-has-updated-personnels-compliance)Step 4: Confirm upload has updated personnel’s compliance

To confirm the upload has updated the personnel’s compliance, get the personnel’s device details from Drata.

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
  const query = new URLSearchParams({
    cursor: 'string',
    size: '50',
    sort: 'createdAt',
    sortDir: 'ASC',
    includeTotalCount: 'false',
    externalId: 'string',
    'expand[]': 'asset',
    macAddress: '65-F9-3D-85-7B-6B,99-A9-3E-14-7A-3E',
    serialNumber: 'NKRTSPY456',
    sourceType: 'AGENT'
  }).toString();

  const personnelId = 'YOUR_personnelId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/personnel/${personnelId}/devices?${query}`,
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

## [](https://developers.drata.com/developer-portal/v2/recipes/add-hard-drive-encryption-evidence/#step-5-repeat-for-each-personnel-failing-hard-drive-encryption-compliance)Step 5: Repeat for each personnel failing hard drive encryption compliance

Repeat above with all the personnel failing hard drive encryption compliance you identified in step 1.
