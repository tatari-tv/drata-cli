# Design Document: SafeBase REST API in drata-cli

**Author:** Scott Idler
**Date:** 2026-11-05
**Status:** Implemented
**Review Passes Completed:** 5/5

## Summary

Extend `drata` into one CLI for both Drata products: the Drata GRC API it covers today, plus the SafeBase Trust Center REST API. SafeBase commands live under a `drata safebase` namespace, with their own credential, spec, raw passthrough and curated verbs. The Drata surface does not change.

## Problem Statement

### Background

- Drata acquired SafeBase (announced 2025-02-11). SafeBase stays a separate product with its own login, host and API, now branded "SafeBase by Drata" (`docs/safebase/guides/unified-under-drata.md`)
- Tatari moved its trust center from the legacy Drata Trust Center to SafeBase (trust.tatari.tv), done 2026-10-12 (SEC-3043, SEC-2972, SEC-3133)
- Security runs the trust center: approves NDA access requests, publishes the SOC 2 report and pentest letters
- drata-cli covers the Drata public API v2 only: curated verbs plus `drata raw` over the embedded spec (`spec/drata-openapi-v2.json`)
- The SafeBase API was verified 2026-11-05 from `docs.safebase.io` and assembled into `spec/safebase-openapi-v1.json`:
  - base URL `https://app.safebase.io/api/ext/v1/rest`, single region
  - auth: API key in header `x-sb-api-key`, no OAuth
  - 47 ops: 22 GET, 12 POST, 6 PATCH, 6 DELETE, 1 PUT, 12 tags
  - success envelope is `{ok, result}`, or `{ok}` alone on 9 ops (the deletes, `PATCH /document/{id}`, `members/notify`, `POST /webpage/{id}/refresh`)
  - gated to SafeBase Advanced | Enterprise plans (`docs/safebase/guides/api.md`)

### Problem

- Nothing in drata-cli can reach SafeBase. Every seam assumes one backend:
  - one embedded spec, one `OnceLock` index (`src/spec.rs:34`, `:58-69`)
  - Bearer auth and Drata region URLs, in two places: `send_inner` (`src/client.rs:258`) and `send_multipart` (`src/client.rs:375`)
  - cursor pagination over `data[]` + `pagination.cursor` (`src/client.rs:489-618`)
  - `Profile{api_key, region, allow_writes}` with a required `api_key` (`src/config.rs:67-75`)
  - `main` demands a Drata key before dispatch (`src/main.rs:103`), and `run` builds the client up front (`src/lib.rs:81`)
- Trust center work (approve access requests, replace the SOC 2 PDF, add KB entries) happens by hand in the SafeBase web UI

### Goals

- `drata safebase ...` reaches all 47 SafeBase ops through `drata safebase raw` (Scott, 2026-11-05: "expanding drata-cli to cover the safebase REST api as well into one unified cli tool")
- Curated SafeBase verbs for the trust-center workflows Security runs (see API Design)
- Same safety model as Drata: per-credential write flag, fail closed, confirm before mutate
- A SafeBase-only user (no Drata key) can use every `drata safebase` command
- Zero behavior change for existing Drata commands, and an existing `credentials.json` is never rewritten by a read

### Non-Goals

- Drata OAuth2 client credentials (new in the refreshed Drata spec). Parked: revisit when an API key stops being enough
- A second binary or crate (see Alternatives)
- SafeBase webhooks (outbound HTTP from SafeBase, no CLI surface)
- `drata verify` for SafeBase. Phase 0's manual round trip covers the live check
- Typed structs for SafeBase payloads. Same `serde_json::Value` currency as the Drata side (`docs/design/2026-06-27-architecture.md`)
- Curated verbs for organization, products, tags, trust center updates, webpages, questionnaires, account writes. No requester; all reachable through `drata safebase raw`. Parked: curate when someone asks

## Proposed Solution

### Overview

- A `Backend` enum (`Drata | Safebase`) owns everything that differs per API: base URL, auth header, redirect policy, embedded spec
- `DrataClient` keeps its name and gains a backend field (see Resolved Decisions)
- Clients build lazily, per command, from the credential of the backend the command targets
- SafeBase commands nest under `drata safebase`, mirroring atlassian-cli's product namespaces (`atl jira`, `atl confluence`)

### Architecture

```
drata <drata-noun> <verb>          -> Backend::Drata    -> public-api.<region>.drata.com/public/v2   Authorization: Bearer <key>
drata raw <METHOD> <path>          -> Backend::Drata
drata safebase <noun> <verb>       -> Backend::Safebase -> app.safebase.io/api/ext/v1/rest            x-sb-api-key: <key>
drata safebase raw <METHOD> <path> -> Backend::Safebase
```

- `Backend` (new, `src/backend.rs`):
  - `base_url(region: Option<&str>) -> Result<&'static str>`: Drata resolves by region; SafeBase is fixed and rejects a region
  - `auth_header(key) -> (HeaderName, HeaderValue)`: Bearer vs `x-sb-api-key`
  - `spec() -> &'static SpecIndex`: one `OnceLock` per backend
- `client.rs`:
  - `send_inner` AND `send_multipart` take the header pair from the backend. No other place sets an auth header
  - redirect policy per backend. SafeBase: same-origin redirects only, harvested from atlassian-cli `same_origin_redirects` (`src/client.rs:531`). reqwest 0.13.4 strips `Authorization` on a cross-host redirect but not `x-sb-api-key` (`redirect.rs` `remove_sensitive_headers`). Drata: unchanged
  - write guard, 429/Retry-After, 204 -> null, `ApiError` unchanged
  - add `patch()`; `delete()` gains an optional body (SafeBase `DELETE /accounts/{id}/members` requires one)
  - SafeBase envelope check: a 2xx with `ok: false`, or with no boolean `ok`, is an error carrying the body. Callers read `result` only when present
  - `format_api_error` already reads top-level `message` + `errors[]`; it covers both SafeBase error shapes (`{statusCode, error, message}`, `{ok:false, errorCode, errors[]}`)
- Pagination: SafeBase has no cursor. Three list ops page; the rest return the whole set in `result[]`. A small page-number paginator (`src/client/pages.rs`) driven by a per-op descriptor. atlassian-cli's descriptor has no page-number increment, so only its validation and stop rules are harvested:

| Op | Request | Items | Stop |
|---|---|---|---|
| `GET /accounts` | `pageNumber` (from 1), `pageSize` (max 1000) | `result[]` | `pageNumber >= pagination.totalPages` |
| `GET /documents` | `page` (from 0), `limit` (max 100) | `result.documents[]` | collected `>= result.count` |
| `GET /kb/search` | `pageNumber` (from 1), `pageSize` (max 1000) | `result.items[]` | `pageNumber >= pagination.totalPages` |

Amended after Phase 0 Observed: `/documents` is zero-based, and `/kb/search` `result.count` counts one page only.

  - list commands fetch every page by default; `--all` switches output to NDJSON streaming. Same contract as the Drata lists (`src/resources/vendor.rs:147-151`)
  - errors, never empty success or silent truncation:
    - a page missing its stop field or items field, or either has the wrong type
    - an empty page before the stop condition
    - a page whose items equal the previous page's (server ignoring the page param)
    - collected > `count`
    - `/accounts` past `pageNumber` 500 (spec bound): error naming the cap. Exactly 500 pages succeeds
    - a hard page cap on `/documents` and `/kb/search`, same constant as Drata's `MAX_PAGES` (`src/client.rs:171`)
  - `totalPages = 0` or `count = 0` on page 1 is an empty success
  - a failure mid-stream under `--all` exits non-zero and names the page number; lines already written stay written
  - page sizes come from Phase 0's observed maximums, not guesses
  - Drata's cursor path stays as is
- Document upload (`drata safebase document upload`), three steps:
  1. `POST /documents/upload` with `{fileName, mimeType, md5b64, fileSize}` -> `{fileId, signedUrl, headers}`
  2. send the bytes to `signedUrl` with exactly the returned `headers`, through a separate `reqwest::Client` that carries no credential and follows no redirects. Never `send_inner`
  3. `PUT /documents/upload` with `{name, fileId, mimeType, externalId, ...}`, plus `replacementDocumentId` on `--replace`
  - local checks before step 1, from the spec: `fileSize <= 78643200` bytes, `mimeType` (from `mime_guess`) in the spec's enum. Either failure exits non-zero with no request sent
  - `md5b64` computed locally (new direct deps `md-5`, `base64`, via `cargo add`)
  - idempotency: the CLI sets `externalId` to a fresh UUID v4, or `--external-id`, which must parse as a UUID (the lookup path param is `format: uuid`); a non-UUID exits non-zero before any request. `GET /document/{id}?type=external` looks a document up by external ID (`type` defaults to `document`)
  - outcomes:
    - step 1 or 2 fails: definite, no document. Exit non-zero, print the `fileId` (SafeBase has no op to delete an upload)
    - step 3 returns an HTTP error: definite, no document. Exit non-zero
    - step 3 times out or the connection drops: the CLI runs `GET /document/<externalId>?type=external` once
      - found, and its `size` and checksum match the local file: print it, exit 0 with a warning. SafeBase does not enforce `externalId` uniqueness, so a mismatch is treated as not found
      - not found: exit non-zero with `outcome unknown` (one GET cannot prove a timed-out PUT never lands), printing `fileId` and `externalId`
      - never retries the PUT on its own
  - step 2 uses the API client's timeout and no retry. 429 on the presigned host is an error like any other
  - `--replace`: Phase 0 observes which fields (tags, products, expiry, type, usedForAi) survive a replacement; the verb's help text states the observed behavior
  - the signed URL takes PUT (Phase 0 Observed); the spec text says POST and is wrong

### Data Model

`~/.config/drata/credentials.json`, profiles become one block per backend:

```json
{
  "profiles": {
    "default": {
      "drata":    { "api-key": "...", "region": "us", "allow-writes": false },
      "safebase": { "api-key": "...", "allow-writes": false }
    }
  }
}
```

- Both blocks optional; a profile needs at least one
- Reading:
  - a flat profile (`{api-key, region, allow-writes}`) parses as a `drata` block, profile by profile, in memory
  - a profile with both flat fields and a nested block is a parse error naming the profile
  - a read never rewrites the file. The nested-shape migration does not use the `migrated` save path in `Credentials::load` (`src/config.rs:153-156`); that path stays for the legacy single-key file only
- Writing (login/logout only):
  - the first save that changes the file's shape copies the original to `credentials.json.pre-safebase` (mode 0600). Rolling back to a binary that only knows the flat shape is `mv credentials.json.pre-safebase credentials.json`
  - `drata login` replaces only the `drata` block; `drata safebase login` replaces only the `safebase` block
  - `drata logout` removes only the `drata` block; `drata safebase logout` removes only the `safebase` block; a profile left with no block is removed
  - mode 0600, atomic write, unchanged
- Env vars: `SAFEBASE_API_KEY` (one per backend, same as `DRATA_API_KEY`; keep holds `safebase-readonly-api-key.age` and `safebase-readwrite-api-key.age`, see Phase 0 Observed). `DRATA_API_KEY`, `DRATA_REGION`, `DRATA_PROFILE` unchanged. `DRATA_PROFILE` selects the profile for both backends
- Key precedence per backend: `--api-key` > env var > profile > error naming the backend and profile
- A profile with no block for the command's backend is that same error, never a fallback to the other backend's key
- Write flag per backend, same rule as today (`src/config.rs:281-288`): `--allow-writes` OR the profile block's `allow-writes`, and the profile flag counts only when the key came from that profile. A key from `--api-key` or `SAFEBASE_API_KEY` is read-only unless `--allow-writes` is passed

### API Design

Global flags, scoped to the command's backend:

- `--api-key`: the key for the backend the command calls
- `--allow-writes`: as in Data Model
- `--region`: Drata only. Passing the flag to a `safebase` command is an error. `DRATA_REGION` in the environment is ignored by `safebase` commands (shells export it for Drata)
- On `drata auth` / `drata whoami`, which report both backends: `--api-key` and `--allow-writes` apply to the Drata lines only (today's meaning). The SafeBase lines resolve from `SAFEBASE_API_KEY` and the profile

New commands:

```
drata safebase login --api-key <key> [--allow-writes]
drata safebase logout
drata safebase raw <METHOD> <path> [--query k=v ...] [--data <json|@file|->] [--file <path> --file-field <name> --field k=v ...]
drata safebase raw --example <METHOD> <path>          # skeleton from the SafeBase spec, no key needed

drata safebase account list [--all] | get <id> | url <id>
drata safebase member list [--email <e>] [--sfdc-account-id <id>]
drata safebase member add <account-id> --email <e> --first-name <f> --last-name <l> [--message <text> | --notify]
drata safebase member remove <account-id> --email <e>...
drata safebase document list [--all] [--query <q>] | get <id> [--external] | types
drata safebase document upload <file> --name <n> [--type <t>] [--replace <document-id>] [--external-id <id>]
drata safebase request list [--status pending|approved|declined]
drata safebase request approve <request-id> --account-id <uuid>
drata safebase request decline <request-id> [--message <text> | --notify]
drata safebase kb search [<query>] [--all] | add --data <json> | update <id> --data <json>
```

- Curated set traces to the workflows Security runs: NDA access requests (approve needs an account id, which `account list` supplies), documents (SOC 2 report, pentest letters), account members, KB entries. Everything else is raw-only (Non-Goals)
- Email side effects are opt-in. `--message <text>` sends SafeBase's email with the text; `--notify` sends it with no text; neither omits `message`, so no email (spec: `AccountMemberPayloadArray.message`, decline body `message`)
- `request approve` that creates a new account in the same call (`account` payload) is raw-only
- `member add` takes one member per call with names, because `AccountMemberPayload` requires `firstName` + `lastName` (or the deprecated `name`) beside `email`. Bulk adds go through raw
- `drata auth` / `drata whoami` add SafeBase lines below the existing Drata lines, which keep their current wording
- `drata safebase raw` has the same flags as `drata raw` (no `--all`, same as today) and accepts any method, including PATCH, through the same confirm prompt and write guard
- `WriteGuardError` and the no-key error name the backend: `write blocked: the active SafeBase credential is not write-enabled.` (today the text hard-codes "Drata", `src/client.rs:48-51`)

### Implementation Plan

#### Phase 0: Prove the SafeBase API against Tatari's tenant
**Model:** sonnet
- Operator step (Scott): mint a SafeBase API key, store as `keep/.env/safebase-api-key.age`
- Zero code. curl with `x-sb-api-key`, record status, envelope and any rate-limit headers for each:
  - `GET /organizations`
  - pagination: `GET /accounts` pages 1 and 2 at `pageSize=1` (items differ); `GET /documents` and `GET /kb/search` likewise with `limit` / `pageSize` 1
  - largest honored page size: `GET /documents?limit=N` and `GET /kb/search?pageSize=N` for N in 100, 500, 1000, comparing items returned to `count`
  - a redirect check: `curl -s -o /dev/null -w '%{http_code} %{redirect_url}'` on `/organizations`
  - upload round trip on a `zzz-clitest-` PDF with an `externalId`: POST upload -> signed URL send -> PUT upload
  - `GET /document/<externalId>?type=external` returns the document; record which response field carries a checksum comparable to the uploaded `md5b64`
  - replace: set tags/expiry on that document, upload a second file with `replacementDocumentId`, record which fields survive
  - cleanup: `DELETE /document/{id}`, then `GET /document/{id}`
  - whether keys can be minted read-only (scopes)
- Paste every response shape under an `Observed` block in this doc
- **Success criteria:**
  - `curl -s -o /dev/null -w '%{http_code}' -H "x-sb-api-key: $SAFEBASE_API_KEY" https://app.safebase.io/api/ext/v1/rest/organizations` prints `200`
  - `Observed` holds: signed-URL method and headers, max honored page sizes, fields kept on replace
  - `GET /document/{id}` after the DELETE returns 404

##### Observed (2026-11-07, Tatari tenant, plan `business`)

Keys: Scott minted two, `keep/.env/safebase-readonly-api-key.age` and `keep/.env/safebase-readwrite-api-key.age`. SafeBase enforces scopes server side.

- `GET /organizations` with the read-only key: `200`, `{"ok":true,"result":{"id":"dccaf15d-...","name":"Tatari","primaryDomain":"tatari.tv","spCustomDomain":"trust.tatari.tv","plan":"business",...}}`
- Read-only key on `PATCH /document/{id}` and `POST /documents/upload`: `403 {"statusCode":403,"cause":"forbidden","name":"ApiError","message":"Missing required scope(s): documents:edit.","error":"forbidden"}`
- Bad key: `400 {"statusCode":400,"cause":"bad_request","name":"ApiError","message":"Invalid API key.","error":"bad_request"}` (400, not 401)
- Validation error: `400 {"statusCode":400,...,"message":"Invalid request: [ValidationError: \"limit\" must be less than or equal to 100]","error":"bad_request"}`. Path ids are Joi GUIDs: the nil UUID gets `"id" must be a valid GUID`
- Not found: `404 {"statusCode":404,"cause":"not_found","name":"ApiError","message":"Not found","error":"not_found"}`
- Rate-limit headers: none on any response (no `x-ratelimit-*`, `ratelimit-*`, `retry-after`)
- Redirect check on `/organizations`: `200 []`, no redirect
- Pagination:
  - `GET /accounts?pageNumber=N&pageSize=1`: pages 1 and 2 differ. Shape `{"ok":true,"result":[...],"pagination":{"pageNumber":1,"pageSize":1,"totalCount":3,"totalPages":3,"maxPages":500,"maxSize":1000}}`. `pageSize` max 1000 (1001 -> 400). `pageNumber` min 1 (0 -> 400). Without `pageNumber`, `totalPages` is `null`
  - `GET /documents?page=N&limit=1`: pages differ. `page` is ZERO-based: `page=0&limit=10` -> 10 items, `page=1` -> 10, `page=2` -> 3, `page=3` -> 0, `count` 23 throughout. `limit` max 100 (500 -> 400), default 20. No `pagination` object: `{"ok":true,"result":{"documents":[...],"count":23}}`. `checksum` is the file's md5b64
  - `GET /kb/search?pageNumber=N&pageSize=1`: pages differ. `result.count` is the item count of THIS page: `pageSize=1` -> `"count":1` with `pagination.totalCount` 7. Shape `{"ok":true,"result":{"items":[...],"count":1},"pagination":{"pageNumber":1,"pageSize":1,"totalCount":7,"totalPages":7,"maxPages":500,"maxSize":1000}}`. `pageSize` max 1000, `pageNumber` min 1, past the end (`pageNumber=8`) -> 0 items
- Largest honored page size: `/accounts` 1000, `/documents` 100, `/kb/search` 1000
- Upload round trip (`zzz-clitest-1.pdf`, 552 bytes, `aiOptOut: true`):
  - step 1 `POST /documents/upload` -> `{"ok":true,"result":{"fileId":"...","signedUrl":"https://temp-org-storage.safebase.io/<org>/documents/<fileId>?X-Goog-Algorithm=...&X-Goog-Expires=900&X-Goog-SignedHeaders=content-md5;content-type;host;x-goog-content-length-range;x-goog-meta-original-name&...","headers":{"content-type":"application/pdf","content-md5":"<md5b64>","x-goog-content-length-range":"0,552","x-goog-meta-original-name":"zzz-clitest-1.pdf"}}}`
  - step 2: the signed URL takes **PUT** with exactly the returned headers -> `200`, empty body. GCS V4 signed URL, valid 900 s. The spec text's POST is wrong
  - step 3 `PUT /documents/upload` -> `200 {"ok":true,"result":{"id":"...","externalId":"<uuid>","checksum":"<md5b64>","size":552,"uploadStatus":"pending",...}}`; `uploadStatus` reads `complete` seconds later
- `GET /document/<externalId>?type=external` -> `200`, the same document. `result.checksum` equals the uploaded `md5b64`; `result.size` equals the byte count. A non-UUID external id (`drata_policy:64`, which exists) returns 404
- Replace (`PATCH` sets `type`, `expiresAt`, `productIds:["default"]`, `aiOptOut`, then upload `zzz-clitest-2.pdf` with `replacementDocumentId`): the document keeps its `id`. `name`, `externalId`, `checksum`, `size` take the new values. `type`, `expiresAt`, `aiOptOut`, `products` survive. Tags untested: the tenant has none (`GET /tags` -> `{"items":[],"count":0}`)
- Cleanup: `DELETE /document/{id}` -> `200 {"ok":true}`; `GET /document/{id}` -> `404`; the old external id -> `404`; `/documents` back to 23 with no `zzz` names
- Read-only keys: yes, SafeBase mints keys without write scopes (the 403 above)
- `PATCH /document/{id}` success body is `{"ok":true}`, matching the spec

Folded into later phases:
- `/kb/search` stops on `pageNumber >= pagination.totalPages`, not on `result.count`
- `/documents` sends `page` from 0 and `limit` 100
- `/accounts` always sends `pageNumber`, with `pageSize` 1000
- Upload step 2 uses PUT
- `SAFEBASE_API_KEY` stays the CLI's env var. The keep files are `safebase-readonly-api-key` and `safebase-readwrite-api-key`, so the name no longer matches a keep file

#### Phase 1: Backend seam
**Model:** opus
- `src/backend.rs`: `Backend`, base URL, auth header, redirect policy, per-backend spec index
- `client.rs`: backend field, with a doc comment on `DrataClient` stating it serves both the Drata GRC API and SafeBase, `Backend` picking which; `send_inner` and `send_multipart` auth from the backend; SafeBase same-origin redirects; envelope check; `patch()`; `delete()` with optional body
- `spec.rs`: second `include_str!`, `operations(backend)`, `find_by_method_path(backend, ..)`; `build_skeleton` handles OpenAPI 3.1 `type: [T, "null"]`
- No CLI change yet
- **Success criteria:**
  - `cargo test` passes; existing tests change only for the new signatures (constructor, `backend` argument, `delete` body)
  - wiremock tests pass: `safebase_client_sends_sb_key_and_no_authorization` (JSON) and `safebase_multipart_sends_sb_key_and_no_authorization`; `safebase_cross_origin_redirect_is_refused` (second server, zero requests reach it)
  - `spec::operations(Backend::Safebase)` has 47 ops, `spec::operations(Backend::Drata)` 308

#### Phase 2: Credentials, lazy clients, `drata safebase` skeleton
**Model:** opus
- Nested profile blocks, in-memory flat-shape read, no rewrite on read, `.pre-safebase` backup on first shape-changing save
- Per-block login/logout; `drata safebase login | logout` with the minimal `safebase` subcommand tree
- `SAFEBASE_API_KEY`; flag scoping per API Design
- `WriteGuardError`, the no-key message and `AuthDiagnostic` name the backend
- Lazy client construction: drop the up-front Drata key requirement in `main.rs:103` / `lib.rs:81`
- `auth` / `whoami` report both
- **Success criteria:**
  - test `flat_credentials_load_unchanged_and_file_untouched`: a pre-change flat file loads with every Drata field identical and its bytes and mtime unchanged
  - test `drata_login_preserves_safebase_block` and `safebase_logout_preserves_drata_block`
  - test `safebase_write_guard_sends_no_request`: SafeBase PATCH on a read-only key returns `WriteGuardError` and wiremock records zero requests

#### Phase 3: `drata safebase raw`
**Model:** sonnet
- raw routed to `Backend::Safebase`, `--example` from the SafeBase spec
- `--file` + `--file-field` + `--field` multipart (`POST /questionnaires`: file field `qnr`, scalar fields `accountId`, `productId`, `dueDate`, `notes`)
- Coverage test per backend: `EXPECTED_OPERATIONS` 308 | 47; `raw_reachable` includes PATCH
- **Success criteria:**
  - `cargo test --test coverage` passes and asserts all 47 SafeBase ops reachable
  - `env -u DRATA_API_KEY -u SAFEBASE_API_KEY drata safebase raw --example PATCH '/document/{id}'` prints a JSON object, exit 0
  - wiremock test `safebase_raw_patch_sends_body`: the server receives `PATCH /document/1` with the given JSON body and the SafeBase auth header

#### Phase 4: Page-number pagination
**Model:** opus
- `src/client/pages.rs` with the three descriptors and every error rule above; list default fetch-all, `--all` NDJSON
- **Success criteria:**
  - wiremock: `/accounts` with `totalPages=3` returns every item in exactly 3 requests; `totalPages=0` returns empty success in 1 request
  - wiremock: each error rule (missing stop field, repeated page, collected > count, page 501) exits non-zero with a message naming the rule
  - wiremock: `/accounts` with exactly 500 pages succeeds

#### Phase 5: Curated SafeBase reads
**Model:** sonnet
- `account list|get|url`, `member list`, `document list|get|types`, `request list`, `kb search`
- Table renderers for the `result` envelope; `{ok}`-only responses print `ok`
- **Success criteria:**
  - one wiremock test per verb asserting method, path, query params, and the rendered rows
  - a SafeBase `CURATED` list in `tests/coverage.rs` has every entry matching a spec op

#### Phase 6: Curated SafeBase writes
**Model:** opus
- `document upload` (three steps, `--replace`, `externalId`, outcome rules), `request approve|decline`, `member add|remove`, `kb add|update`
- **Success criteria:**
  - wiremock with two servers: the signed-URL request carries no `x-sb-api-key` and no `Authorization`; the step-3 body carries `fileId`, `externalId`, and `replacementDocumentId` on `--replace`
  - wiremock: a step-3 timeout triggers exactly one `GET /document/<externalId>?type=external` and no second PUT; a found document with a different `size` exits non-zero with `outcome unknown`
  - one wiremock test per write verb asserting the exact JSON body (`approve` has `accountId`; `member add` has `members[0]` with names and no `message` unless `--message`/`--notify`); a 409 on approve exits non-zero and prints `errors[]`
  - `md5b64` of a fixture equals `openssl dgst -md5 -binary <f> | base64`

#### Phase 7: Docs and help
**Model:** sonnet
- README SafeBase section; replace every `drata-cli <command>` example with `drata <command>` (the binary is `drata`)
- clap `about` / `after_help`, `drata-cli.yml` sample, crate description
- **Success criteria:**
  - `rg -c '^## SafeBase' README.md` prints `1`
  - `rg -c 'drata-cli [a-z]' README.md` exits 1 (no matches)

## Acceptance Criteria

Observed on branch `refresh-drata-spec-and-safebase-docs` (main + the spec refresh), 2026-11-05, `target/debug/drata`.

- [ ] `drata --help` lists a `safebase` subcommand
  - Observed: `drata --help | rg -i -c safebase` exits 1, no match
- [ ] `env -u DRATA_API_KEY drata safebase raw --example PATCH '/document/{id}'` prints a JSON object and exits 0
  - Observed: `error: unrecognized subcommand 'safebase'`, exit 2
- [ ] `cargo test safebase_write_guard_sends_no_request` runs 1 test and passes
  - Observed: `0 passed; ... 161 filtered out` (test does not exist)
- [ ] `cargo test --test coverage` passes and asserts every op of both specs reachable (308 Drata, 47 SafeBase)
  - Observed: `test result: ok. 4 passed`; asserts Drata only
- [ ] `cargo test safebase_client_sends_sb_key_and_no_authorization` runs 1 test and passes
  - Observed: `0 passed; ... 161 filtered out` (test does not exist)

## Resolved Decisions

- 2026-11-05, Scott: one unified CLI, not a separate tool
- 2026-11-05, author + both seats: namespace `drata safebase ...`. Drata stays top-level (no breaking change); SafeBase `document` / `questionnaire` would collide in meaning with Drata evidence and vendor questionnaires at top level
- 2026-11-05, author: binary stays `drata`. SafeBase is a Drata brand now
- 2026-11-05, author + both seats: nested per-backend credential blocks, with no rewrite on read and a `.pre-safebase` backup (panel round 1, must-fix 6)
- 2026-11-05, author + both seats: separate SafeBase page-number paginator; Drata cursor path untouched
- 2026-11-05, author: `allow-writes` stays a client-side flag per credential even if SafeBase keys carry scopes. Defense in depth, same as Drata
- 2026-11-05, author: the SafeBase spec is current. `v1.20.0` is the docs.safebase.io portal label (the live reference page shows it); `1.4.1` is the spec's own `info.version`
- 2026-11-05, author: curated reads trimmed to accounts, members, documents, requests, KB (architect, round 1); the rest has no requester
- 2026-11-05, author: `DrataClient` keeps its name. SafeBase is a Drata product and the binary is `drata`; the client is the Drata-company client
- 2026-11-05, author: no auto-delete of an orphaned upload. SafeBase has no op that deletes an upload `fileId` (only `DELETE /document/{id}`)

## Alternatives Considered

### Alternative 1: separate `safebase` binary / repo
- **Pros:** no seam work in drata-cli
- **Cons:** duplicate config, write guard, confirm, raw, output; sibling drift
- **Why not chosen:** Scott asked for one CLI. atlassian-cli rejected two binaries for the same reason (`tatari-tv/atlassian-cli` `docs/design/2026-10-07-atlassian-cli.md`)

### Alternative 2: SafeBase nouns at top level
- **Pros:** shorter commands
- **Cons:** `drata document` vs Drata evidence, `drata questionnaire` vs vendor questionnaires; reader cannot tell which API a command hits
- **Why not chosen:** names must tell the truth about the backend

### Alternative 3: flat profile fields (`safebase-api-key`, `safebase-allow-writes`)
- **Pros:** no migration
- **Cons:** `api-key` keeps meaning "Drata key" next to a prefixed SafeBase key; `api_key` is required today, so a SafeBase-only profile needs it made optional anyway
- **Why not chosen:** asymmetric names for sibling credentials

### Alternative 4: route `drata raw` by path (paths never overlap today)
- **Why not chosen:** implicit, and breaks the day either API adds a colliding path

### Alternative 5: port atlassian-cli's full `Pagination` descriptor onto both backends
- **Why not chosen:** it has no page-number increment (`atlassian-cli` `src/spec/pagination.rs:60`), and moving Drata's working cursor path buys nothing

## Technical Considerations

### Dependencies
- New direct: `md-5`, `base64` (via `cargo add`)
- Cross-repo blast radius: one consumer, `tatari-skills` `plugins/sec/skills/vendor-eval/SKILL.md`. It runs `drata vendor list | create` and `drata raw`, and says `drata whoami` reports whether Drata writes are enabled. The Drata lines of `whoami` keep their wording and SafeBase lines are added below, so nothing it relies on changes. No ship order forced

### Performance
- SafeBase documents no rate limits; Phase 0 records headers. Existing 429/Retry-After handling applies to both backends

### Security
- SafeBase key never leaves `app.safebase.io`: same-origin redirects only on the API client; the presigned upload client carries no credential and follows no redirects. Both tested
- Per-credential write flag, fail closed, confirm on non-GET, for both backends
- Keys never logged; `auth` prints source, not value (existing behavior)

### Testing Strategy
- wiremock per backend via `with_base_url`, matching on the backend's auth header
- Break-to-prove: each new guard test is run once against a deliberately broken implementation
- Coverage tripwire per spec

### Rollout Plan
- Prerequisite: land the open spec-refresh branch (`refresh-drata-spec-and-safebase-docs`) first, then `docs/plan/2026-11-05-drata-v2-endpoint-coverage.md`, whose Phase 2 adds `src/client/presigned.rs` (`put_presigned(method, url, headers, bytes)`); this plan's Phase 6 upload step 2 reuses it
- One commit per phase on a feature branch, `otto ci` green each
- `bump release` on the PR; install; shakedown against Tatari's SafeBase with `zzz-clitest-` objects
- Rollback: install the prior release; if `credentials.json` was rewritten, restore `credentials.json.pre-safebase`

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Tatari's SafeBase plan has no API | Med | High | Phase 0 gates everything |
| Signed-URL method/headers differ from spec text | Med | Med | Phase 0 observes; Phase 6 uses only returned headers |
| SafeBase spec changes under us | Med | Low | coverage tripwire per backend |
| Credential rewrite breaks the prior binary | Low | High | no rewrite on read; `.pre-safebase` backup on first shape change |
| Upload step 3 outcome unknown | Low | Med | `externalId` lookup, never an automatic second PUT |

## Open Questions

- (none)

## Review Log

- Pass 1, draft: shape from the design-research brief (backend seam, namespace, phases)
- Pass 2, correctness: `drata raw` has no `--all`, dropped it from `safebase raw`; `kb search` query and `member list` filters are optional per spec; `WriteGuardError` hard-codes "Drata", added backend naming; found the `tatari-skills` vendor-eval consumer and stated the blast radius
- Pass 3, clarity: member and request flags spelled out with spec enums; acceptance criterion 3 pinned to the exact stderr text
- Pass 4, edge cases: `DRATA_REGION` vs `safebase`; profile missing a backend block; `/accounts` 50k cap; upload size and mimeType checked locally; orphaned `fileId` on a failed upload; per-profile migration
- Pass 5, excellence: voice lint (no em-dashes, no "real"); phase criteria checked for criteria measured before their own change
- Panel round 1 (`~/.cache/review-panel/runs/fonjWXNa/synthesis.md`), all folded in:
  - must-fix 1: `request approve` takes `--account-id`; account creation raw-only
  - must-fix 2: `member add` takes names; one member per call; email opt-in via `--message` / `--notify`
  - must-fix 3: `update list` had no backing op; topic/update/product/tag verbs dropped with the curated-read trim
  - must-fix 4: `send_multipart` auth from the backend; multipart auth test in Phase 1
  - must-fix 5: SafeBase same-origin redirects; redirect test in Phase 1
  - must-fix 6: no rewrite on read, `.pre-safebase` backup, per-block login/logout, mixed-shape profile is an error
  - must-fix 7: upload outcome model with `externalId`; replace behavior observed in Phase 0; architect's auto-delete pushed back (no op exists)
  - must-fix 8: `{ok}`-only successes; envelope check
  - cheap-wins 9-15: write-flag inheritance stated; list fetch-all default; pagination unhappy paths and Phase 0 page-size probe; `auth`/`whoami` flag meaning; Phase 2/3 boundary moved, Phase 1 and Phase 7 criteria fixed; write-verb payload criteria; acceptance criterion 3 now a zero-request mock test
  - defer: `DrataClient` rename (staff) declined, see Resolved Decisions; 429 on presigned host folded into upload rules; vendor-eval sentence reworded
- Panel round 2, disputes only: both seats accepted the `DrataClient` pushback and the no-auto-delete pushback. Folded in: lookup needs `?type=external`; `--external-id` must be a UUID; size/checksum match before trusting a found document; `outcome unknown` wording; `DrataClient` doc comment. Nothing open, nothing disputed

## References

- `spec/safebase-openapi-v1.json`, `docs/safebase/endpoint-index.md`, `docs/safebase/guides/*.md`
- `docs/design/2026-06-27-architecture.md`
- `tatari-tv/atlassian-cli` `src/client.rs` (`same_origin_redirects`), `src/spec/pagination.rs`, `src/cli.rs`
- `tatari-tv/pagerduty-cli` `src/client.rs` (second base URL with different auth)
