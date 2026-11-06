Title: Vendor Types

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/

Markdown Content:
Vendor Types are user-defined classifications used to categorize and organize vendors.

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_listVendorTypes)List Vendor Types

List all Vendor Types configured for your account.

🔒 Requires **Vendors: List Vendor Types** permission.

💎 Requires your account have the **TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### query Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_listVendorTypes!in=query&path=cursor&t=request)cursor string

This parameter is used to paginate through results. No value is needed for the first request. If there are additional results, the response will contain a `pagination.cursor` value that can be used in the subsequent request to retrieve the next page of results
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_listVendorTypes!in=query&path=size&t=request)size number[ 1 .. 500 ]

Default:50

Number of results to return
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_listVendorTypes!in=query&path=sort&t=request)sort string (SortTypeLimitedEnum)

Which field to sort by

Enum:"createdAt""updatedAt"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_listVendorTypes!in=query&path=sortDir&t=request)sortDir string (SortDirectionEnum)

The direction to sort the data

Enum:"ASC""DESC"
[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_listVendorTypes!in=query&path=includeTotalCount&t=request)includeTotalCount boolean

Include total count of all matching records in response. Only honored on first page (when cursor is null).

Example:includeTotalCount=false

Responses

200

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

get/vendor-types

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
    `https://public-api.drata.com/public/v2/vendor-types`,
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

`{"data": [{"id": 1,"name": "Cloud Provider"}],"pagination": {"cursor": "string","totalCount": 0}}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_createVendorType)Create Vendor Type

Create a new vendor type.

🔒 Requires **Vendors: Create Vendor Type** permission.

💎 Requires your account have the **TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_createVendorType!path=name&t=request)name
required string<= 191 characters

The name of the Vendor Type

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

409
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

post/vendor-types

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

`{"name": "Cloud Provider"}`

Response samples

*   201
*   400
*   401
*   402
*   403
*   409
*   412
*   500

3 more

application/json

`{"id": 1,"name": "Cloud Provider"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_updateVendorType)Update Vendor Type

Update an existing Vendor Type.

🔒 Requires **Vendors: Update Vendor Type** permission.

💎 Requires your account have the **TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_updateVendorType!in=path&path=vendorTypeId&t=request)vendorTypeId
required number

##### Request Body schema: application/json
required

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_updateVendorType!path=name&t=request)name
required string<= 191 characters

The name of the Vendor Type

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
There is a conflict in the business rules with this request

412
You must accept the Drata terms and conditions to use the API

500
Internal server error

put/vendor-types/{vendorTypeId}

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

`{"name": "Cloud Service Provider"}`

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

`{"id": 1,"name": "Cloud Provider"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_deleteVendorType)Delete Vendor Type

Soft delete a Vendor Type.

🔒 Requires **Vendors: Delete Vendor Type** permission.

💎 Requires your account have the **TPRM Standard, Third Party Risk Management Pro, TPRM Pro Max** feature. Contact your Account Manager for help upgrading.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Vendor-Types/#tag/Vendor-Types/operation/VendorTypesPublicV2Controller_deleteVendorType!in=path&path=vendorTypeId&t=request)vendorTypeId
required number

Responses

204
No Content

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

delete/vendor-types/{vendorTypeId}

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
  const vendorTypeId = 'YOUR_vendorTypeId_PARAMETER';
  const resp = await fetch(
    `https://public-api.drata.com/public/v2/vendor-types/${vendorTypeId}`,
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
