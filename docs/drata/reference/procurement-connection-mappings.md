Title: Procurement Connection Mappings

URL Source: https://developers.drata.com/openapi/reference/v2/tag/Procurement-Connection-Mappings/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Procurement Connection Mappings control how vendor records from a procurement connection (Ironclad, Tropic, Ramp, or Zip) become Drata Vendors. Each mapping holds a JSONata transform, the Vendor Types to filter on, and a dry run mode.

## [](https://developers.drata.com/openapi/reference/v2/tag/Procurement-Connection-Mappings/#tag/Procurement-Connection-Mappings/operation/ProcurementConnectionMappingsPublicV2Controller_getVendorMapping)Get Vendor Mapping

Get the vendor mapping configuration for a procurement connection (Ironclad or Tropic). Returns the JSONata mapping (merged with AI suggestions if available), vendor type filters, and dry run mode.

🔒 Requires **Connections: Get Procurement Connection Vendor Mapping** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Procurement-Connection-Mappings/#tag/Procurement-Connection-Mappings/operation/ProcurementConnectionMappingsPublicV2Controller_getVendorMapping!in=path&path=connectionId&t=request)connectionId
required number

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

get/connections/{connectionId}/vendor-mapping

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
    `https://public-api.drata.com/public/v2/connections/${connectionId}/vendor-mapping`,
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

`{"connectionId": 42,"clientType": "IRONCLAD_VENDOR","vendorMapping": {"name": "properties.counterpartyName.value","externalId": "id"},"vendorTypes": ["vendor","supplier"],"dryRun": true,"aiMappingSuggested": false,"credentialType": "oauth-application"}`

## [](https://developers.drata.com/openapi/reference/v2/tag/Procurement-Connection-Mappings/#tag/Procurement-Connection-Mappings/operation/ProcurementConnectionMappingsPublicV2Controller_updateVendorMapping)Update Vendor Mapping

Update the vendor mapping configuration for a procurement connection (Ironclad or Tropic). Allows modifying the JSONata mapping, vendor type filters, and dry run mode.

🔒 Requires **Connections: Update Procurement Connection Vendor Mapping** permission.

Security

**bearer** or **oauth2**

Request

##### path Parameters

[](https://developers.drata.com/openapi/reference/v2/tag/Procurement-Connection-Mappings/#tag/Procurement-Connection-Mappings/operation/ProcurementConnectionMappingsPublicV2Controller_updateVendorMapping!in=path&path=connectionId&t=request)connectionId
required number

##### Request Body schema: application/json
required

object or null

JSONata mapping from provider record fields to vendor properties. Keys are vendor field names, values are JSONata expressions.
[](https://developers.drata.com/openapi/reference/v2/tag/Procurement-Connection-Mappings/#tag/Procurement-Connection-Mappings/operation/ProcurementConnectionMappingsPublicV2Controller_updateVendorMapping!path=vendorTypes&t=request)vendorTypes Array of strings or null

Vendor types to filter from the procurement provider
[](https://developers.drata.com/openapi/reference/v2/tag/Procurement-Connection-Mappings/#tag/Procurement-Connection-Mappings/operation/ProcurementConnectionMappingsPublicV2Controller_updateVendorMapping!path=dryRun&t=request)dryRun boolean

Enable dry run mode (no actual vendor creation/updates). Defaults to true if not set.

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

put/connections/{connectionId}/vendor-mapping

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

`{"vendorMapping": {"name": "properties.counterpartyName.value","externalId": "id"},"vendorTypes": ["vendor","supplier"],"dryRun": false}`

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

`{"connectionId": 42,"clientType": "IRONCLAD_VENDOR","vendorMapping": {"name": "properties.counterpartyName.value","externalId": "id"},"vendorTypes": ["vendor","supplier"],"dryRun": true,"aiMappingSuggested": false,"credentialType": "oauth-application"}`
