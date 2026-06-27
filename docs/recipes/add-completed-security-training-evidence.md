Title: Add Completed Security Training Evidence

URL Source: https://developers.drata.com/developer-portal/v2/recipes/add-completed-security-training-evidence/

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/recipes/add-completed-security-training-evidence/#step-1-get-list-of-personnel)Step 1: Get list of personnel

Get the list of personnel in Drata. By using the `expand[]=complianceChecks` query parameter, you will get a list of all personnel in Drata with compliance checks including the `SECURITY_TRAINING` check. (e.g. `GET /public/v2/personnel?expand[]=complianceChecks`)
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

Target server:

https://public-api.drata.com/public/v2

Security

Bearer Token:

show

**Note**: Your credentials will be saved until the end of the browser session

Parameters

get/personnel

## [](https://developers.drata.com/developer-portal/v2/recipes/add-completed-security-training-evidence/#step-2-attach-file-for-that-personnel)Step 2: Attach file for that personnel

For the user ID you selected above in Step 1, attach a file for evidence of completed security training.

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

import FormData from 'form-data';
import fetch from 'node-fetch';

async function run() {
  const form = new FormData();
  form.append('type','SEC_TRAINING');
  form.append('file','string');
  form.append('base64File[base64String]','data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEAYABg');
  form.append('base64File[filename]','excellent-filename');
  form.append('completionDate','2020-07-06');

  const userId = 'YOUR_userId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/users/${userId}/documents`,
    {
      method: 'POST',
      headers: {
        Authorization: 'Bearer <YOUR_API_KEY_HERE>'
      },
      body: form
    }
  );

  const data = await resp.text();
  console.log(data);
}

run();

Target server:

https://public-api.drata.com/public/v2

Security

Bearer Token:

show

**Note**: Your credentials will be saved until the end of the browser session

Body

Parameters

post/users/{userId}/documents

## [](https://developers.drata.com/developer-portal/v2/recipes/add-completed-security-training-evidence/#step-3-confirm-upload-has-updated-personnels-compliance)Step 3: Confirm upload has updated personnel’s compliance

To confirm the upload has updated the personnel’s compliance from Step 2, get the personnel’s details from Drata using the personnel ID. Use the `expand[]=complianceChecks` query parameter to see the compliance checks for the user (e.g. `GET /public/v2/personnel/1?expand[]=complianceChecks`).

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
    'expand[]': 'customFields'
  }).toString();

  const personnelId = 'YOUR_personnelId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/personnel/${personnelId}?${query}`,
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

Target server:

https://public-api.drata.com/public/v2

Security

Bearer Token:

show

**Note**: Your credentials will be saved until the end of the browser session

Parameters

get/personnel/{personnelId}

## [](https://developers.drata.com/developer-portal/v2/recipes/add-completed-security-training-evidence/#step-4-repeat-for-each-personnel-failing-security-training-compliance)Step 4: Repeat for each personnel failing security training compliance

Repeat Steps 2-3 above with all the personnel failing security training compliance you identified in step 1.
