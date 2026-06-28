# drata-cli: TS vs Rust output comparison (the "gamut")

Date: 2026-06-28. Compares the Rust port (`drata`, this repo, v0.1.1) against upstream
TS `yorkeccak/drata-cli` to find real discrepancies: missing/renamed fields, wrong
representation, coverage gaps, and error-handling differences.

## Verdict

**Field fidelity is perfect.** Across every resource that could be read with the available
key, the Rust CLI returns byte-for-byte the same record data as the TS CLI (after unwrapping
the list envelope). No renamed fields, no missing fields, no type/representation differences
were found in any record.

The only differences are **structural and coverage-level**, not data-level:
1. List envelope / pagination shape (a deliberate, known design choice - excluded by request).
2. Curated-command coverage gaps (TS has curated commands for things Rust only exposes via `raw`).
3. One TS error-handling bug (`vendors stats`).
4. Entitlement-gated endpoints return 402 on both CLIs (not a CLI difference).
5. A workflow gap: Rust can't create a vendor security review, so its `questionnaire send` is
   not usable end-to-end without `raw`.

## Method

- Read-only key (`~/.config/drata/credentials.json`, region `us`); TS fed the same key via
  `DRATA_API_KEY` env (TS uses a different on-disk schema and won't read the Rust file).
- For each resource: discover an id from a TS `list` (one page), then `get <id>` on **both**
  CLIs, unwrap `.data`, sort keys, and `diff`. Rust auto-paginates lists, so single-record
  `get` was used for fidelity to keep calls bounded.
- The Drata API rate-limits aggressively and intermittently 502s; all calls were paced
  (~10s spacing) with retries.
- **Guard:** an early pass produced two *false* "IDENTICAL" results where both `get` calls had
  been rate-limited to empty output (diff of two empty files passes). Those were caught by a
  byte-size check and re-run against gettable ids. All "identical" results below are from
  non-empty, real captures.

## 1. Field fidelity - all identical

| Resource        | Compared via            | id          | Result            |
|-----------------|-------------------------|-------------|-------------------|
| company         | `company get`           | -           | byte-identical (2727 B) |
| workspaces      | `workspace list` records| 1           | identical         |
| users           | `user get`              | 518         | identical         |
| personnel       | `personnel get`         | 511         | identical         |
| assets          | `asset get`             | 396479      | identical         |
| devices         | `device get`            | 831         | identical         |
| policies        | `policy get`            | 62          | identical         |
| vendors         | `vendor get`            | 31          | identical         |
| controls        | `control get`           | 100         | identical (818 B) |
| evidence        | `evidence get`          | 171         | identical (233 B) |
| monitoring-tests| `monitor get`           | 251         | identical (443 B) |
| events          | `event get`             | 7f79b8c5-…  | identical (371 B) |
| frameworks      | `framework list` record | 2           | identical         |
| risk-registers  | -                       | -           | blocked: 402 (see 2.4) |
| risks           | -                       | -           | blocked: depends on risk-registers |

## 2. Edges & different shapes found

### 2.1 List envelope & pagination (intended design delta - excluded)
- Rust lists **auto-paginate** and return `{ "data": [ ...all pages... ] }` (no pagination key).
- TS lists return **one page**: `{ "data": [ ...page... ], "pagination": { "cursor": "..." } }`.
- Single-record `get` on **both** returns the bare object (no `data` wrapper) - confirmed identical.
- This is the deliberate pagerduty-cli-house-style decision and is not a defect.

### 2.2 Coverage gaps (commands one CLI has, the other lacks)

Everything missing from Rust is still reachable via `drata raw <METHOD> <path>` - these are
**curated-command** gaps, not capability gaps. Most are write ops the read-only key can't
exercise anyway; the read gaps (a curated GET a user would expect) are flagged.

**Rust is a superset (has more) on:**
- `policy` adds create/update/actions; `framework` adds create/update; `monitor` adds update/passes;
  `risk` adds upload.
- **`audit` is Rust-only** - TS has no audits command at all.

**Whole resources TS has but Rust does not (curated):**

| Missing in Rust          | TS verbs                                              | read gap? |
|--------------------------|------------------------------------------------------|-----------|
| control-notes            | list/get/create/update/delete                        | yes (list/get) |
| control-owners           | list/add/modify/remove                               | yes (list) |
| user-documents           | list/get/upload/delete                               | yes (list/get) |
| user-policies            | get/acknowledge                                       | yes (get) |
| risk-library             | list/get/copy                                        | yes (list/get) |
| risk-notes               | list/get/create/update/delete                        | yes (list/get) |
| vendor-types             | list/create/update/delete                            | yes (list) |
| vendor-security-reviews  | list/create/create-with-file/get/update/actions/run-action/questionnaires/upload-questionnaire/upload-questionnaire-to-review | **curated** (see 2.5) |
| custom-connections       | list/get/create/update/delete                        | yes (list/get) |
| hris-identities          | list/get/batch-upsert/update/delete                  | yes (list/get) |
| background-checks        | create                                               | no (write) |

**Document sub-resources** - Rust offers `upload` only; TS offers full management:
- `device-documents`, `risk-documents`, `vendor-documents`: TS has list/get/(upload)/delete;
  Rust folds only the upload into `device|risk|vendor upload` and has no list/get/delete.

**Individual verbs missing from Rust:**
- `controls reset-mappings` (Rust has `compare` but not reset)
- `personnel reset-sync`
- `events download` / `download-status` (PDF download job)
- `devices list-for-connection`, `devices create`, `devices remove` (custom-connection devices)
- `vendors stats` (see 2.3)

### 2.3 TS error-handling bug: `vendors stats`
`vendors stats` (TS, `GET /vendors/stats`-ish) returns:
```json
{"error":"400 Bad Request: [object Object]"}
```
The TS error serializer stringifies an object to `[object Object]`, losing the real message.
Rust has no `stats` command, so there is nothing to compare - but this is a concrete TS defect
worth noting. (Rust's error path, by contrast, surfaces the full upstream message + path, e.g.
`API error 402 Payment Required on GET .../risk-registers: You must upgrade your plan...`.)

### 2.4 Entitlement-gated endpoints return 402 on both CLIs
- `risk-registers` / `risks` return **402 Payment Required - "You must upgrade your plan to use
  this feature."** on **both** CLIs. This is a tenant-licensing limit, not a CLI difference; it
  just means these two resources couldn't be field-compared.

### 2.5 vendor-security-reviews - gap closed (curated as of design doc 2026-06-28)
- **Resolved.** The Rust CLI now exposes a full `drata vendor security-review` command group
  covering all 10 operations in the embedded spec (5 the TS CLI had, plus 5 spec-only extras):
  `list`, `create`, `create-with-file`, `get`, `update`, `actions`, `run-action`,
  `questionnaires`, `upload-questionnaire`, `upload-questionnaire-to-review`.
- The `vendor questionnaire send` workflow is now completable end-to-end without `raw`:
  `vendor security-review create <vendor-id> ...` yields a `securityReviewId`, which feeds
  `vendor questionnaire send <vendor-id> --security-review-id <sr> --questionnaire-id <q> ...`.
- Rust is now a **superset** of the TS CLI for this resource (10 curated verbs vs. TS's 5)
  and follows the spec's multipart shape for file uploads rather than the TS JSON passthrough.
- Design doc: `docs/design/2026-06-28-vendor-security-reviews.md`.

### 2.6 List-but-not-gettable records (404) - identical on both CLIs
- Some controls (e.g. 259 `DCF-192`, 258, 1) and monitoring-tests (256-261) appear in `list`
  but **404 on individual `get`** - they are Drata template/archived records not fetchable by
  the `/workspaces/{ws}/.../{id}` path. **Both CLIs 404 identically**, so this is upstream API
  behavior, not a CLI defect. (Gettable ids - control 100, monitor 251 - compared identical.)

## 3. Vendor questionnaires (focused question)
- Feature **is enabled**: company `entitlements` includes `SECURITY_QUESTIONNAIRE_AUTOMATION`
  ("Security Questionnaire Automation"), and the questionnaire endpoints return HTTP 200
  (`{"data":[]}`), not the 402 a disabled feature returns.
- **No questionnaires are currently sent**: all 24 vendors on the first page returned
  `questionnaires=0`. (A second vendor page exists - not swept.)
- Capability: Rust `vendor questionnaire list/get/send`; TS adds the `vendor-security-reviews`
  group (see 2.5).

## 4. Not verified / blocked
- **Write/mutation parity** (create/update/delete, `verify`, questionnaire `send`, security-review
  create): needs a **write-enabled key** (current key is read-only).
- **risk-registers / risks** field fidelity: 402-gated on this tenant.
- **Remaining vendor pages** for questionnaires: only the first page (25 vendors) was swept.

## 5. Recommendations
1. ~~**Add a `vendor-security-reviews` command group to Rust**~~ **Done** - `drata vendor
   security-review` is now fully curated (10 verbs). `questionnaire send` is usable end-to-end
   without `raw` (see 2.5).
2. Consider curated **read** commands for the high-traffic gaps that today force `raw`:
   `control-notes list`, `control-owners list`, `user-policies get`, `risk-library list`,
   `vendor-types list`, `custom-connections list`, `hris-identities list`, and document
   `list/get` for device/risk/vendor documents.
3. Lower priority write verbs to consider: `controls reset-mappings`, `personnel reset-sync`,
   `events download`/`download-status`, `devices create/remove/list-for-connection`,
   `vendor-types`, `background-checks create`.
4. No action needed on field fidelity, the envelope/pagination delta, the 402 entitlement
   responses, or the 404 template-record behavior - those are correct/intended.
5. (Upstream, optional) the TS `vendors stats` `[object Object]` error is a real TS bug; Rust
   already handles errors better.

## Appendix: ids used
workspace 1 (Tatari) · user 518 · personnel 511 · asset 396479 · device 831 · policy 62 ·
vendor 31 · control 100 (259/258/1 = 404 template) · evidence 171 · monitor 251 (256-261 = 404
template) · event 7f79b8c5-9ef4-48a3-a00f-1cd825f4de27 · framework 2.
