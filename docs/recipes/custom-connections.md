Title: Work with Custom Connections

URL Source: https://developers.drata.com/developer-portal/v2/recipes/custom-connections/

Markdown Content:
Last updated 1 month ago

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#1-general)1. General

Custom Connections let you push your own data into Drata so it can be used for compliance evidence. The behavior depends on the provider type you choose when creating the connection:

| Provider type | Schema required? | Record endpoint |
| --- | --- | --- |
| `CUSTOM` | Yes — you define the schema via `schema` or `sampleData` + `displayNameKey` | `/custom-connections/{connectionId}/resources/{resourceId}/records` |
| `MDM` | No — Drata uses a fixed common device model | `/custom-connections/{connectionId}/devices` |
| `HRIS` | No — Drata uses a fixed common HRIS model | `/custom-connections/{connectionId}/hris-user-identities` |

> **Note:** Passing `schema`, `sampleData`, `displayNameKey`, or `workspaceIds` for an `MDM` or `HRIS` connection will return a `400 Bad Request`.

Sections 3 and 4 of this recipe (record and session management) apply to **`CUSTOM` connections only**. For `MDM` and `HRIS` connections, refer to the [Devices](https://developers.drata.com/docs/openapi/reference/v2/tag/Devices/) and [HRIS User Identities](https://developers.drata.com/docs/openapi/reference/v2/tag/HRIS-User-Identities/) endpoint references respectively.

For `CUSTOM` connections, records can be uploaded two ways:

*   **Direct upload** — immediately live, good for incremental updates to individual records
*   **Session-based upload** — stage a full batch before committing it; when you complete the session, Drata atomically replaces the active dataset with the new batch

The session-based approach is recommended for bulk syncs where you want an all-or-nothing replacement of your data.

* * *

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#2-setup)2. Setup

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#21-create-a-custom-connection)2.1 Create a Custom Connection

Copy

Copied

`POST https://public-api.drata.com/public/v2/custom-connections`

#### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#custom-connection-user-defined-schema)CUSTOM connection (user-defined schema)

You define the schema for your records. Use `schema` to provide a JSON Schema directly, or `sampleData` to have Drata infer the schema from a representative record. `displayNameKey` is required and must be a top-level key in the schema.

**Option A: Provide a JSON Schema directly**

Copy

Copied

```
{
  "name": "Software Inventory",
  "providerTypes": ["CUSTOM"],
  "workspaceIds": [1],
  "displayNameKey": "packageName",
  "description": "Software packages installed across the fleet.",
  "schema": {
    "type": "object",
    "properties": {
      "packageName": { "type": "string" },
      "version": { "type": "string" },
      "vendor": { "type": "string" },
      "installedAt": { "type": "string" },
      "isApproved": { "type": "boolean" }
    },
    "additionalProperties": true
  }
}
```

**Option B: Provide sample data and let Drata infer the schema**

Copy

Copied

```
{
  "name": "Software Inventory",
  "providerTypes": ["CUSTOM"],
  "workspaceIds": [1],
  "displayNameKey": "packageName",
  "description": "Software packages installed across the fleet.",
  "sampleData": {
    "packageName": "1Password",
    "version": "8.10.36",
    "vendor": "AgileBits",
    "installedAt": "2024-01-15T00:00:00Z",
    "isApproved": true
  }
}
```

| Field | Required | Description |
| --- | --- | --- |
| `name` | Yes | Display name for the connection |
| `providerTypes` | Yes | `["CUSTOM"]` |
| `workspaceIds` | Yes | At least one workspace ID |
| `schema` | No* | JSON Schema for your records. Provide either `schema` or `sampleData`, not both. *Required if `sampleData` is not provided. |
| `sampleData` | No* | A representative record from which Drata infers the schema. Provide either `sampleData` or `schema`, not both. *Required if `schema` is not provided. |
| `displayNameKey` | Yes | Top-level schema field used as the human-readable label for each record in the Drata UI (e.g. `"packageName"`). Must exist as a property in the provided or inferred schema. |
| `description` | No | Optional description |

* * *

#### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#mdm-connection-fixed-device-model)MDM connection (fixed device model)

MDM connections use Drata's built-in device data model. Do **not** provide `schema`, `sampleData`, `displayNameKey`, or `workspaceIds` — MDM connections are global.

Copy

Copied

```
{
  "name": "Fleet MDM",
  "providerTypes": ["MDM"],
  "description": "Device inventory synced from our MDM solution."
}
```

After creating, use the [Devices endpoints](https://developers.drata.com/docs/openapi/reference/v2/tag/Devices/) to upload device records.

* * *

#### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#hris-connection-fixed-hris-model)HRIS connection (fixed HRIS model)

HRIS connections use Drata's built-in HRIS data model. Do **not** provide `schema`, `sampleData`, `displayNameKey`, or `workspaceIds` — HRIS connections are global.

Copy

Copied

```
{
  "name": "Employee Directory",
  "providerTypes": ["HRIS"],
  "description": "Employee roster synced from our internal HRIS."
}
```

After creating, use the [HRIS User Identities endpoints](https://developers.drata.com/docs/openapi/reference/v2/tag/HRIS-User-Identities/) to upload employee records.

* * *

**Common fields for all connection types:**

| Field | Required | Description |
| --- | --- | --- |
| `name` | Yes | Display name for the connection |
| `providerTypes` | Yes | One of: `CUSTOM`, `MDM`, `HRIS` |
| `description` | No | Optional description |

**Reference:**[Create Custom Connection API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomConnectionsPublicV2Controller_createCustomConnection/)

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#22-retrieve-the-resource-id-custom-connections-only)2.2 Retrieve the Resource ID (CUSTOM connections only)

All `CUSTOM` record and session operations require a `resourceId`. This is created automatically when you create the connection and is tied to the schema you provided.

Fetch it by expanding `customResources` on the connection:

Copy

Copied

`GET https://public-api.drata.com/public/v2/custom-connections/{connectionId}?expand[]=customResources`

**Response (relevant fields):**

Copy

Copied

```
{
  "id": 42,
  "customResources": [
    {
      "id": 7,
      "name": "Software Inventory",
      "schema": { ... },
      "createdAt": "2025-01-01T00:00:00Z",
      "updatedAt": "2025-01-01T00:00:00Z"
    }
  ]
}
```

Use `customResources[0].id` as the `resourceId` in all subsequent record and session requests.

**Reference:**[Get Custom Connection API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomConnectionsPublicV2Controller_getCustomConnection/)

* * *

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#3-managing-records-directly)3. Managing Records Directly

> **`CUSTOM` connections only.** For `MDM` and `HRIS` connections, use the [Devices](https://developers.drata.com/docs/openapi/reference/v2/tag/Devices/) and [HRIS User Identities](https://developers.drata.com/docs/openapi/reference/v2/tag/HRIS-User-Identities/) endpoints instead.

Use the direct upload approach for incremental updates — adding, updating, or removing individual records without replacing the full dataset.

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#31-upsert-records)3.1 Upsert Records

Records are matched by `id`. If a record with that ID already exists it is updated; otherwise a new record is created. You can send a single object or an array.

Copy

Copied

`POST https://public-api.drata.com/public/v2/custom-connections/{connectionId}/resources/{resourceId}/records`

**Request Payload (single record):**

Copy

Copied

```
{
  "data": {
    "id": "PKG-001",
    "packageName": "1Password",
    "version": "8.10.36",
    "isApproved": true
  }
}
```

**Request Payload (multiple records):**

Copy

Copied

```
{
  "data": [
    {
      "id": "PKG-001",
      "packageName": "1Password",
      "version": "8.10.36",
      "isApproved": true
    },
    {
      "id": "PKG-002",
      "packageName": "Zoom",
      "version": "6.2.0",
      "isApproved": true
    }
  ]
}
```

> **Note:** If an active session exists for this connection/resource, directly uploaded records are automatically grouped under it.

**Reference:**[Upsert Records API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomDataRecordsPublicV2Controller_createCustomData/)

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#32-update-a-record)3.2 Update a Record

Update a specific record by its `recordId`.

Copy

Copied

`PUT https://public-api.drata.com/public/v2/custom-connections/{connectionId}/resources/{resourceId}/records/{recordId}`

**Request Payload:**

Copy

Copied

```
{
  "data": {
    "version": "8.10.40",
    "isApproved": false
  }
}
```

**Reference:**[Update Record API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomDataRecordsPublicV2Controller_updateCustomData/)

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#33-delete-a-record)3.3 Delete a Record

Copy

Copied

`DELETE https://public-api.drata.com/public/v2/custom-connections/{connectionId}/resources/{resourceId}/records/{recordId}`

Returns `204 No Content` on success.

**Reference:**[Delete Record API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomDataRecordsPublicV2Controller_deleteCustomData/)

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#34-list-records)3.4 List Records

Copy

Copied

`GET https://public-api.drata.com/public/v2/custom-connections/{connectionId}/resources/{resourceId}/records`

Optionally filter by session using the `sessionId` query parameter:

Copy

Copied

`GET https://public-api.drata.com/public/v2/custom-connections/{connectionId}/resources/{resourceId}/records?sessionId=upload-2025-04-01`

**Reference:**[List Records API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomDataRecordsPublicV2Controller_listCustomDataRecords/)

* * *

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#4-session-based-record-management)4. Session-Based Record Management

> **`CUSTOM` connections only.** For `MDM` and `HRIS` connections, use the [Devices](https://developers.drata.com/docs/openapi/reference/v2/tag/Devices/) and [HRIS User Identities](https://developers.drata.com/docs/openapi/reference/v2/tag/HRIS-User-Identities/) endpoints instead.

Sessions are designed for bulk syncs where you want to stage an entire dataset and commit it atomically. When you complete a session, it becomes the authoritative dataset — everything not in that session is permanently deleted.

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#41-session-states)4.1 Session States

| State | Description |
| --- | --- |
| `IN_PROGRESS` | Session is open. Records are staged but not yet live. |
| `ACTIVE` | Session has been completed. Its records are the live dataset. |
| `CANCELLED` | Session was aborted. All staged records are discarded. |
| `ARCHIVED` | System-managed. A previously ACTIVE session that was replaced by a newer one. Its records have been deleted. |

Only one session can be `IN_PROGRESS` at a time per connection/resource.

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#42-upload-records-to-a-session)4.2 Upload Records to a Session

Choose any string as your `sessionId` — if the session doesn't exist yet it will be created in `IN_PROGRESS` state. You can call this endpoint multiple times with the same `sessionId` to upload in batches.

Copy

Copied

`POST https://public-api.drata.com/public/v2/custom-connections/{connectionId}/resources/{resourceId}/sessions/{sessionId}`

**Session ID rules:** 3–64 characters, letters/numbers/hyphens/underscores only (e.g. `upload-2025-04-01`, `batch_sync_v2`).

**Request Payload:**

Copy

Copied

```
{
  "data": [
    {
      "id": "PKG-001",
      "packageName": "1Password",
      "version": "8.10.36",
      "isApproved": true
    },
    {
      "id": "PKG-002",
      "packageName": "Zoom",
      "version": "6.2.0",
      "isApproved": true
    }
  ]
}
```

Records uploaded to a session are staged and not visible as live data until the session is completed.

**Reference:**[Upsert Records with Session API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomDataRecordsPublicV2Controller_manageSession/)

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#43-complete-a-session)4.3 Complete a Session

Once all records are uploaded, complete the session to make them live.

Copy

Copied

`POST https://public-api.drata.com/public/v2/custom-connections/{connectionId}/resources/{resourceId}/sessions/{sessionId}/actions`

**Request Payload:**

Copy

Copied

```
{
  "action": "complete"
}
```

**What happens on completion:**

1.    The session transitions from `IN_PROGRESS` → `ACTIVE`
2.    All records in the session become the live dataset
3.    Any previously ACTIVE session is archived and its records are **permanently deleted**
4.    Any records that were uploaded directly (not via a session) are also **permanently deleted**

> **Warning:** Completing a session is destructive and irreversible. Every record not part of the completing session — including any records uploaded via the direct endpoint — will be hard deleted.

**Response:**

Copy

Copied

```
{
  "sessionId": "upload-2025-04-01",
  "status": "ACTIVE",
  "action": "complete",
  "connectionId": 42,
  "resourceId": 7
}
```

**Reference:**[Perform Session Action API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomDataRecordsPublicV2Controller_performSessionAction/)

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#44-cancel-a-session)4.4 Cancel a Session

To discard a session and all its staged records without committing them:

Copy

Copied

`POST https://public-api.drata.com/public/v2/custom-connections/{connectionId}/resources/{resourceId}/sessions/{sessionId}/actions`

**Request Payload:**

Copy

Copied

```
{
  "action": "cancel"
}
```

The session transitions to `CANCELLED` and all staged records are deleted. The previously active dataset (if any) is unaffected.

**Reference:**[Perform Session Action API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomDataRecordsPublicV2Controller_performSessionAction/)

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#45-list-sessions)4.5 List Sessions

Copy

Copied

`GET https://public-api.drata.com/public/v2/custom-connections/{connectionId}/resources/{resourceId}/sessions`

Filter by status using the `status` query parameter:

Copy

Copied

`GET https://public-api.drata.com/public/v2/custom-connections/{connectionId}/resources/{resourceId}/sessions?status=IN_PROGRESS`

**Reference:**[List Sessions API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomDataRecordsPublicV2Controller_listSessions/)

* * *

## [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#5-managing-custom-connections)5. Managing Custom Connections

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#51-list-custom-connections)5.1 List Custom Connections

Copy

Copied

`GET https://public-api.drata.com/public/v2/custom-connections`

Use `expand[]` to include sub-objects in the response:

| Value | Description |
| --- | --- |
| `customResources` | Includes the resource(s) and their schemas |
| `createdByUser` | Includes the user or API key that created the connection |

Copy

Copied

`GET https://public-api.drata.com/public/v2/custom-connections?expand[]=customResources`

**Reference:**[List Custom Connections API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomConnectionsPublicV2Controller_listCustomConnections/)

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#52-get-custom-connection)5.2 Get Custom Connection

Copy

Copied

`GET https://public-api.drata.com/public/v2/custom-connections/{connectionId}`

Supports the same `expand[]` values as List.

**Reference:**[Get Custom Connection API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomConnectionsPublicV2Controller_getCustomConnection/)

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#53-update-custom-connection)5.3 Update Custom Connection

Only `clientAlias` and `description` can be updated after creation.

Copy

Copied

`PUT https://public-api.drata.com/public/v2/custom-connections/{connectionId}`

**Request Payload:**

Copy

Copied

```
{
  "clientAlias": "Fleet Software Tracker",
  "description": "Software packages synced nightly from our asset management tool"
}
```

**Reference:**[Update Custom Connection API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomConnectionsPublicV2Controller_updateCustomConnection/)

* * *

### [](https://developers.drata.com/developer-portal/v2/recipes/custom-connections/#54-delete-custom-connection)5.4 Delete Custom Connection

Copy

Copied

`DELETE https://public-api.drata.com/public/v2/custom-connections/{connectionId}`

Returns `204 No Content` on success. Deletes the connection and all associated records.

**Reference:**[Delete Custom Connection API](https://developers.drata.com/docs/openapi/reference/v2/operation/CustomConnectionsPublicV2Controller_deleteCustomConnection/)
