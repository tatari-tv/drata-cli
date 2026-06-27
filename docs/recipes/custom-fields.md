Title: Work with Custom Fields

URL Source: https://developers.drata.com/developer-portal/v2/recipes/custom-fields/

Markdown Content:
Last updated 2 months ago

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#1-general)1. General

Custom Fields support is integrated with existing administrative (CRUD) API endpoints: GET, POST, PUT, and DELETE methods rather than implemented in separate, dedicated custom field endpoints.

For example, to set control custom field values for an existing control, use:

Copy

Copied

`PUT https://public-api.drata.com/public/v2/workspaces/{workspaceId}/controls`

Similarly to retrieve control custom field values for an existing control, use:

Copy

Copied

`GET https://public-api.drata.com/public/v2/workspaces/{workspaceId}/controls/{controlId}`

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#2-assets)2. Assets

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#21-list-assets)2.1 List Assets

By default, custom field values are not returned in the response. By including the query parameter `expand[]=customFields`, custom fields are included in the response.

Copy

Copied

`GET https://public-api.drata.com/public/v2/assets?expand[]=customFields`

**Reference:**[List Assets API](https://developers.drata.com/docs/openapi/reference/v2/operation/AssetsPublicV2Controller_listAssets/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#22-get-asset)2.2 Get Asset

By default, custom field values are not returned in the response. By including the query parameter `expand[]=customFields`, custom fields are included in the response.

Copy

Copied

`GET https://public-api.drata.com/public/v2/assets/{assetId}?expand[]=customFields`

**Reference:**[Get Asset API](https://developers.drata.com/docs/openapi/reference/v2/operation/AssetsPublicV2Controller_getAsset/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#23-update-asset)2.3 Update Asset

When updating an asset, custom fields may also be included or only be included as a part of the request payload.

Copy

Copied

`PUT https://public-api.drata.com/public/v2/assets/{assetId}`

**Request Payload:**

Copy

Copied

```
{
  "description": "MacBook Pro 16-inch with 14-Core CPU 32-Core GPU 36GB Unified Memory 1TB SSD Storage",
  "notes": "This is a MacBook Pro",
  "customFields": [
    {
      "name": "Stakeholders",
      "value": "Security & IT"
    },
    {
      "name": "Amount",
      "value": 56.89
    }
  ]
}
```

**Reference:**[Update Asset API](https://developers.drata.com/docs/openapi/reference/v2/operation/AssetsPublicV2Controller_updateAsset/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#24-create-asset)2.4 Create Asset

When creating an asset, custom fields may also be included as a part of the request payload.

> **Note:** If any required custom fields exist, they must be passed as a part of the required payload, otherwise, a 400 error will be returned.

Copy

Copied

`POST https://public-api.drata.com/public/v2/assets`

**Request Payload:**

Copy

Copied

```
{
  "name": "MacBook Pro 16-inch",
  "description": "MacBook Pro 16-inch with 14-Core CPU 32-Core GPU 36GB Unified Memory 1TB SSD Storage",
  "notes": "This is a MacBook Pro",
  "assetClassTypes": ["HARDWARE", "PERSONNEL"],
  "assetType": "PHYSICAL",
  "ownerId": 1,
  "customFields": [
    {
      "name": "Stakeholders",
      "value": "Customer"
    },
    {
      "name": "Amount",
      "value": 12.34
    }
  ]
}
```

**Reference:**[Create Asset API](https://developers.drata.com/docs/openapi/reference/v2/operation/AssetsPublicV2Controller_createAsset/)

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#3-controls)3. Controls

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#31-list-controls)3.1 List Controls

By default, custom field values are not returned in the response. By including the query parameter `expand[]=customFields`, custom fields are included in the response.

Copy

Copied

`GET https://public-api.drata.com/public/v2/workspaces/{workspaceId}/controls?expand[]=customFields`

**Reference:**[List Controls API](https://developers.drata.com/docs/openapi/reference/v2/operation/ControlsPublicV2Controller_getControls/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#32-get-control)3.2 Get Control

By default, custom field values are not returned in the response. By including the query parameter `expand[]=customFields`, custom fields are included in the response.

Copy

Copied

`GET https://public-api.drata.com/public/v2/workspaces/{workspaceId}/controls/{controlId}?expand[]=customFields`

**Reference:**[Get Control API](https://developers.drata.com/docs/openapi/reference/v2/operation/ControlsPublicV2Controller_getControlById/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#33-modify-control)3.3 Modify Control

When updating a control, custom fields may also be included or only be included as a part of the request payload.

Copy

Copied

`PUT https://public-api.drata.com/public/v2/workspaces/{workspaceId}/controls/{controlId}`

**Request Payload:**

Copy

Copied

```
{
  "name": "Updated quarterly User Access Reviews",
  "description": "Updated access to critical systems is reviewed on a quarterly basis to ensure that only authorized users retain access.",
  "customFields": [
    {
      "name": "Stakeholders",
      "value": "Security & IT"
    },
    {
      "name": "Amount",
      "value": 56.89
    }
  ]
}
```

**Reference:**[Modify Control API](https://developers.drata.com/docs/openapi/reference/v2/operation/ControlsPublicV2Controller_modifyControl/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#34-create-control)3.4 Create Control

When creating a control, custom fields may also be included as a part of the request payload.

> **Note:** If any required custom fields exist, they must be passed as a part of the required payload, otherwise, a 400 error will be returned.

Copy

Copied

`POST https://public-api.drata.com/public/v2/workspaces/{workspaceId}/controls`

**Request Payload:**

Copy

Copied

```
{
  "name": "Quarterly User Access Reviews",
  "description": "Access to critical systems is reviewed on a quarterly basis to ensure that only authorized users retain access.",
  "question": "A very good question",
  "code": "DRA-69",
  "activity": "On a quarterly schedule, generate access reports from identity providers.",
  "customFields": [
    {
      "name": "Stakeholders",
      "value": "Security & IT"
    },
    {
      "name": "Amount",
      "value": 56.89
    }
  ]
}
```

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#4-personnel)4. Personnel

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#41-list-personnel)4.1 List Personnel

By default, custom field values are not returned in the response. By including the query parameter `expand[]=customFields`, custom fields are included in the response.

Copy

Copied

`GET https://public-api.drata.com/public/v2/personnel?expand[]=customFields`

**Reference:**[List Personnel API](https://developers.drata.com/docs/openapi/reference/v2/operation/PersonnelPublicV2Controller_listPersonnel/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#42-get-personnel)4.2 Get Personnel

By default, custom field values are not returned in the response. By including the query parameter `expand[]=customFields`, custom fields are included in the response.

Copy

Copied

`GET https://public-api.drata.com/public/v2/personnel/{personnelId}?expand[]=customFields`

**Reference:**[Get Personnel API](https://developers.drata.com/docs/openapi/reference/v2/operation/PersonnelPublicV2Controller_getPerson/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#43-update-personnel)4.3 Update Personnel

When updating a personnel record, custom fields may also be included or only be included as a part of the request payload.

Copy

Copied

`PUT https://public-api.drata.com/public/v2/personnel/{personnelId}`

**Request Payload:**

Copy

Copied

```
{
  "startedAt": "2020-07-06",
  "separatedAt": "2020-07-06",
  "employmentStatus": "CURRENT_CONTRACTOR",
  "notHumanReason": "This is not a real personnel, but a placeholder for anyone in charge of X",
  "customFields": [
    {
      "name": "Stakeholders",
      "value": "Security & IT"
    },
    {
      "name": "Amount",
      "value": 56.89
    }
  ]
}
```

**Reference:**[Update Personnel API](https://developers.drata.com/docs/openapi/reference/v2/operation/PersonnelPublicV2Controller_modifyPerson/)

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#5-vendors)5. Vendors

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#51-list-vendors)5.1 List Vendors

By default, custom field values are not returned in the response. By including the query parameter `expand[]=customFields`, custom fields are included in the response.

Copy

Copied

`GET https://public-api.drata.com/public/v2/vendors?expand[]=customFields`

**Reference:**[List Vendors API](https://developers.drata.com/docs/openapi/reference/v2/operation/VendorsPublicV2Controller_listVendors/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#52-get-vendor)5.2 Get Vendor

By default, custom field values are not returned in the response. By including the query parameter `expand[]=customFields`, custom fields are included in the response.

Copy

Copied

`GET https://public-api.drata.com/public/v2/vendors/{vendorId}?expand[]=customFields`

**Reference:**[Get Vendor API](https://developers.drata.com/docs/openapi/reference/v2/operation/VendorsPublicV2Controller_getVendor/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#53-update-vendor)5.3 Update Vendor

When updating a vendor, custom fields may also be included or only be included as a part of the request payload.

Copy

Copied

`PUT https://public-api.drata.com/public/v2/vendors/{vendorId}`

**Request Payload:**

Copy

Copied

```
{
  "name": "Acme Vendor",
  "hasPii": true,
  "customFields": [
    {
      "id": 1,
      "name": "Compliance Status",
      "value": "Security & IT"
    }
  ]
}
```

**Reference:**[Update Vendor API](https://developers.drata.com/docs/openapi/reference/v2/operation/VendorsPublicV2Controller_updateVendor/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#54-create-vendor)5.4 Create Vendor

When creating a vendor, custom fields may also be included as a part of the request payload.

> **Note:** If any required custom fields exist, they must be passed as a part of the required payload, otherwise, a 400 error will be returned.

Copy

Copied

`POST https://public-api.drata.com/public/v2/vendors`

**Request Payload:**

Copy

Copied

```
{
  "name": "Acme Vendor",
  "hasPii": true,
  "passwordMfaEnabled": true,
  "customFields": [
    {
      "id": 1,
      "name": "Compliance Status",
      "value": "Security & IT"
    }
  ]
}
```

**Reference:**[Create Vendor API](https://developers.drata.com/docs/openapi/reference/v2/operation/VendorsPublicV2Controller_createVendor/)

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#6-framework-requirements)6. Framework Requirements

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#61-list-framework-requirements)6.1 List Framework Requirements

By default, custom field values are not returned in the response. By including the query parameter `expand[]=customFields`, custom fields are included in the response.

Copy

Copied

`GET https://public-api.drata.com/public/v2/workspaces/{workspaceId}/framework-requirements?expand[]=customFields`

> **Note:** You can combine multiple expand parameters. For example, `expand[]=controls&expand[]=customFields` will include both controls and custom field values in the response.

**Reference:**[List Framework Requirements API](https://developers.drata.com/docs/openapi/reference/v2/operation/FrameworksPublicV2Controller_listFrameworkRequirements/)

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-fields/#62-update-framework-requirement)6.2 Update Framework Requirement

When updating a framework requirement, custom fields may be included as a part of the request payload.

Copy

Copied

`PUT https://public-api.drata.com/public/v2/workspaces/{workspaceId}/framework-requirements/{frameworkRequirementId}`

**Request Payload:**

Copy

Copied

```
{
  "customFields": [
    {
      "name": "Risk Level",
      "value": "High"
    },
    {
      "name": "Review Priority",
      "value": 5
    }
  ]
}
```

> **Note:** This endpoint requires the Custom Fields & Formulas entitlement. Custom fields can be referenced by `id` or `name`. To clear a custom field value, pass `null` as the value.

**Reference:**[Update Framework Requirement API](https://developers.drata.com/docs/openapi/reference/v2/operation/FrameworksPublicV2Controller_updateFrameworkRequirement/)
