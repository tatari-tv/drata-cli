# CLI Shakedown Report: drata v0.2.0

Binary: `~/.cargo/bin/drata` (`drata v0.2.0`). Source: `tatari-tv/drata-cli` @ `36b4fcd` (tag `v0.2.0`).
Run 2026-11-07 against Tatari's live tenant (US region). Reads used the read-only API key, writes used the
read-write key, both passed as `--api-key "$VAR"` from the environment. No `credentials.json` was written and
no key value appears here.

Scope is the design doc's Rollout Plan (`docs/plan/2026-11-05-drata-v2-endpoint-coverage.md`): every read verb,
plus only the writes it allows. Excluded and never run: `control create`, `custom-field create`, every ticket
`create`, VSR access requests, follow-up questionnaire sends and agent runs, `connection` writes,
`access-review period create`, `monitor retest`, `vendor security-review create-with-file` and `verify`.

## Summary

| Metric | Count |
|--------|-------|
| Commands discovered | 233 (227 curated ops from the command table, plus `login`, `logout`, `whoami`, `auth`, `raw`, `verify`) |
| Read ops (GET) in the command table | 119 |
| Read ops exercised live with data | 82 |
| Read ops blocked by plan tier (402) | 9 |
| Read ops blocked by tenant setting or key scope (403/400) | 6 |
| Read ops answering 404/400 on an existing ID (server side) | 4 |
| Read ops unverified (no data on the tenant; only the 404 path ran) | 18 |
| Writes attempted | 5 (3 succeeded and cleaned up, 2 blocked server side) |
| Writes skipped | 5 (VSR `note`/`observation` `create`/`remove`, `risk ticket unlink`, `evidence-item remove`; reasons below) |
| CLI defects found | 3 bugs, 3 cosmetic, 1 suggestion |
| Pipelines tested | 8 |
| Edge cases tested | 12 categories, plus 20 nonexistent-ID lookups |

Curated coverage stays 227/308 per `cargo test --test coverage`; `drata coverage` does not exist.

## Command Results

`--output json` is used below for `jq`. With no `--output` and a piped stdout the CLI emits JSON (auto mode),
confirmed on `workspace list`.

### Exercised (82 rows)

Chained: list IDs fed into get commands (vendor 38, register 1 / risk 230, device 912, personnel 555 and 232,
policy 65 / version 63, evidence 173, framework 1 / requirement 61, monitor test 90000, audit `775273a5-...`,
VSR vendor 25 / review 18, identities 281 and 236, access-review app 138, user 564, roles 36 and 37,
event `5b38a22c-...`, asset 495488).

| Command | Exit | Sample |
|---|---|---|
| `drata workspace list --output table` | 0 | `1   Tatari  true  2024-04-03T18:20:04.381Z` |
| `drata company get --output json \| jq .entitlements` | 0 | 15 entitlements incl. `ACCESS_REVIEW`, `TPRM_STANDARD`, `CUSTOM_FIELDS_AND_FORMULAS` |
| `drata vendor list --output json` | 0 | 33 vendors; first `{"id":38,"name":"Datadog","status":"ACTIVE"}` |
| `drata vendor list Data` | 0 | `["Datadog","Databricks"]` |
| `drata vendor list --status ACTIVE --category ENGINEERING` | 0 | 11 rows, all matching both filters |
| `drata vendor get 38` | 0 | `{"id":38,"name":"Datadog","risk":"HIGH"}` |
| `drata vendor search` / `--facet status` | 0 | 32 rows; envelope keys `data, facets, pagination` |
| `drata vendor-security-review list` | 0 | 16 reviews |
| `drata vendor security-review list 25 --created-from 2026-01-01` | 0 | `[18]` |
| `drata vendor security-review get 25 18` | 0 | `"April 2026 SOC 2 Type 2 report review", COMPLETED` |
| `drata vendor security-review actions 25 18` | 0 | `{"data":[{"action":"reopen"}]}` |
| `drata vendor security-review summary 25 18` | 0 | document with `signedUrl` (query string redacted here) |
| `drata vendor security-review questionnaires \| note list \| observation list 25 18` | 0 | empty `data[]` |
| `drata vendor questionnaire list 24` | 0 | empty `data[]` |
| `drata vendor-questionnaire-template list` / `questions 3` | 0 | 3 templates / 12 questions |
| `drata vendor-risk-level list` | 0 | 6 levels |
| `drata risk list 1` / `risk get 1 230` | 0 | 49 risks / `CR-11` |
| `drata risk search` / `--register-id 1` / `--risk-register-id 1 --facet status` | 0 | 49 rows; facet `ACTIVE: 49` |
| `drata register list` / `register get 1` | 0 | `Risk Management` |
| `drata control list 1` | 0 | 259 controls (65 archived) |
| `drata control requirements 1 1` | 0 | empty `data[]` |
| `drata control compare 1 --control-ids 258 259` | 0 | 2 comparison rows |
| `drata device list` / `get 912` / `for-personnel 232` / `apps 912` | 0 | 526 devices / match / `[912,232]` / empty |
| `drata personnel list` / `get 555` | 0 | 553 personnel |
| `drata personnel search` / `--workspace-id 1` / `--facet employmentStatus` | 0 | 553 rows; facet `CURRENT_EMPLOYEE: 382` |
| `drata personnel group list 1` / `counts 1` | 0 | empty / `{"scopedPersonnelCount":553,"totalPersonnelCount":553}` |
| `drata group list` | 0 | 645 groups |
| `drata compliance-check-exclusion list 1` | 0 | 913 rows |
| `drata custom-field list` | 0 | empty `data[]` |
| `drata policy list` / `get 65` / `versions 65` / `version 65 63` | 0 | 27 policies; version 63 `PUBLISHED` |
| `drata policy actions 65` | 1 / 0 | read-only key: 403 `Missing required permissions: policies-submit-for-approval, ...`; read-write key: `{"data":[]}` |
| `drata policy-language list 65 63` / `versions` | 0 | empty |
| `drata evidence list 1` / `get 1 173` | 0 | 148 items |
| `drata framework list 1` / `requirements 1 1` / `requirement controls 1 1 61` | 0 | HIPAA, SOC 2 / 61 / `[96,21]` |
| `drata asset get 495488` | 0 | `MacBook Air, PHYSICAL, KANDJI` |
| `drata asset list --all \| head -n 1` | 1 (EPIPE) | first asset; see bug 1 and cosmetic 5 |
| `drata user list` / `get 564` / `roles` / `role 37` / `role-users 36` | 0 | 563 users / 35 roles |
| `drata monitor list 1` / `get 1 90000` | 0 | 192 tests |
| `drata monitor list 1 --name "High Vulnerabilities Addressed (Socket)"` | 0 | 1 row (exact match; `--name Socket` returns 0) |
| `drata monitor exclusions \| failures \| passes \| ticket list 1 90000` | 0 | 0 / 0 / 4 / empty |
| `drata audit list 1` / `get 1 <id> --expand dcfSuggestionsSummary` | 0 | `SOC_2_TYPE_2 PREPARING`; summary all zero |
| `drata event list --all \| head -n 2` / `event get <id>` | 1 (EPIPE) / 0 | two NDJSON events; get matches |
| `drata infrastructure-identity list 1` / `get 1 281` / `actions 1 281` | 0 | 281 rows; actions `linkUser`, `markServiceAccount` |
| `drata version-control-identity list 1` / `get 1 236` / `actions 1 236` | 0 | 236 rows; 4 actions |
| `drata access-review app list` / `get 138` / `users list 138` | 0 | 101 apps; `Pendo` |
| `drata access-review period list` (also `--status completed`) | 0 | empty (no periods on the tenant) |
| `drata connection aws \| azure \| gcp \| gitlab list` | 0 | all empty |

### Blocked by plan tier: 402 `You must upgrade your plan to use this feature` (9 rows)

`vendor-criteria list`, `vendor-criteria-creation list 1`, `vendor security-review assessment list | get | stats 25 18`,
`vendor security-review workflow 25 18`, `vendor security-review residual-risk 25 18`, `risk insights 1`,
`risk ticket list 1 230`. Each exits 1 with the server message.

### Blocked by tenant setting or key scope (6 rows)

| Command | Status | Server message |
|---|---|---|
| `evidence-item list 1`, `get 1 <id>`, `artifact list 1 <id>` | 403 | `In order to access evidence with multiple artifacts, tenants must opt into the new Drata Experience` (both keys) |
| `vendor security-review follow-up-questionnaire 25 18`, `access-request-requirements 25 18` | 403 | `The AI agent is not available for this account or Security Review.` |
| `policy-language settings` | 400 | `Policy languages are not enabled for this organization` |

### Server 404/400 on an existing ID (4 rows, not CLI defects)

- `drata control get 1 259` -> 404 for every ID tried (1, 258, 259, `code:DCF-192`), with both keys, and through
  `drata raw GET /workspaces/1/controls/259`. `control list`, `compare` and `requirements` on the same IDs answer
  200, and the path matches the spec, so this is Drata's side. `control ticket list 1 1` also 404s.
- `drata vendor trust-overview 25` -> 404 (vendor has no Trust Center).
- `drata monitor codebase-exclusion list 1 90000` -> 400 `Codebase Exclusions exist only for codebase Monitoring Tests.`

### Unverified: no data on the tenant, only the 404 path ran (18 rows)

`vendor questionnaire get | questions`, `vendor security-review note get | observation get`, `custom-field get`,
`policy-language get`, `evidence get-version`, `access-review period get | app get | app users | evidence-package`,
`connection aws | azure | gcp | gitlab get` (AWS via `roleArn:...role/zzz-clitest`, sent with `/` as `%2F`; GCP via
`projectId:zzz-clitest`), `custom-connection resource-attribute`, `audit request dcf-suggestion list | get`.
Each returned the server's 404 message and exit 1.

### Writes

| Command | Exit | Result | Cleanup |
|---|---|---|---|
| `drata --allow-writes --yes upload-url create --purpose evidence --workspace 1 --file zzz-clitest.txt` | 0 | printed only the objectKey `<account-uuid>/evidence-library/1/fd5fd590-b492-4349-ab89-3ffc47d75b3e/zzz-clitest.txt`; no presigned URL on stdout or in the log | unattached S3 object; no API delete exists (the Rollout Plan accepts this) |
| `drata --allow-writes --yes evidence-item create 1 --name zzz-clitest-shakedown-v020 --filed-at 2026-11-07 --file-key <objectKey>` | 1 | 403 `...must opt into the new Drata Experience` | nothing created |
| `drata --allow-writes --yes risk ticket link 1 230 --ticket-url https://example.atlassian.net/browse/ZZZCLITEST-1` | 1 | 402 | nothing linked |
| `drata --allow-writes --yes vendor create --name zzz-clitest-shakedown-v020` | 0 | vendor **40** | removed |
| `drata --allow-writes --yes vendor remove 40` | 0 | printed `null`; `vendor get 40` then 404, `vendor list zzz` empty | done |

Skipped writes:
- `evidence-item remove`: no item could be created (403 above).
- `risk ticket unlink`: no ticket could be linked (402 above).
- VSR `note create | remove`, `observation create | remove`: the new `zzz-clitest-` vendor 40 has no security review
  (`vendor security-review list 40` -> `{"data":[]}`). Creating one is a write outside the allowed list with no DELETE
  op, the same reason `create-with-file` needs Scott's approval. I removed the vendor without creating one.

Left behind: only the unattached `zzz-clitest.txt` S3 object, which the Rollout Plan allows.

## Output Format Matrix

| Command | auto (piped) | `--output json` | `--output table` |
|---|---|---|---|
| `workspace list` | JSON `{data:[...]}` | valid (`jq .`) | aligned |
| `register list` | JSON | valid | aligned |
| `vendor list` | JSON | valid | aligned, 6 columns |
| `vendor get 38` | JSON | valid | one-row table |
| `risk list 1` | JSON | valid | aligned; long titles padded, no overflow |
| `monitor list 1` | JSON | valid | aligned, but the ID column is wrong (bug 3) |
| `vendor-security-review list` | JSON | valid | aligned, 6 columns |
| `vendor search --facet`, `risk search --facet`, `personnel search --facet` | JSON | full `{data, facets, pagination}` envelope | n/a |
| `personnel list --all`, `event list --all`, `asset list --all` | NDJSON | n/a | n/a |

`--output xml` is rejected by clap (exit 2) with the valid values listed.

## Failures & Bugs

### 1. [bug] Default `list` drains every page; `asset list` and `event list` cannot finish on this tenant and lock the key out
- Commands: `drata asset list --output json`, `drata event list --output json`.
- The tenant has about 495k assets (first page IDs 495488..494989) and, per the comment on `PAGINATION_SIZE` in
  `src/client.rs`, about 945k events. `get_all` fetches every page at size 500 before printing: about 990 requests for
  assets, 1,890 for events. Measured with `-l debug`: `asset list` sent 543 GETs in 48 s before I killed it. Drata's
  limit is 500 requests/minute.
- When both ran next to other reads, Cloudflare answered 429 with `retry-after: 162` and every concurrent `drata`
  command failed with `Rate limited after 3 attempts`, a plain `custom-field list` included, for about 10 minutes.
- Expected: a bounded default (one page, or a `--limit`), or pacing under 500 req/min. Today only `--all | head` works.

### 2. [bug] 429 retry caps `Retry-After` at 60 s, so all three retries fail inside the block, with no terminal output
- Command (run while blocked): `drata --allow-writes --yes evidence-item create 1 --name zzz-clitest-x`.
- After about 3 minutes with nothing on stdout or stderr: `Error: Command failed / Caused by: Rate limited after 3 attempts / Location: src/client.rs:293:21`, exit 1.
- Cause: `MAX_RETRY_DELAY_SECS = 60` caps the server's `retry-after: 162`, so the retries land inside the block. The
  `rate limited, retrying` warnings go only to `~/.local/share/drata/logs/drata.log`.
- Expected: honor the server's value (or fail fast and print it), and say on stderr that the CLI is waiting.

### 3. [bug] `monitor list` table shows `id` in the ID column; every monitor verb takes `testId`
- `drata monitor list 1 --output table` shows `264  S3 Data Lake Versioning Enabled`; the JSON row has `"testId": 90000`.
- `drata monitor get 1 264` -> `API error 404 Not Found on GET .../monitoring-tests/264`; `drata monitor get 1 90000` works.
- Cause: `render_monitors` in `src/output/table.rs` reads `scalar_field(r, "id")`. It should read `testId` (header `TEST_ID`).
- A table user cannot chain into `get`, `exclusions`, `ticket` or any other monitor verb.

### 4. [cosmetic] `vendor remove` prints `null` on success
- `drata --allow-writes --yes vendor remove 40` printed `null` (the 204 maps to `Value::Null`), exit 0.

### 5. [cosmetic] `--all | head` reports the closed pipe as an error
- `drata event list --all | head -n 2` prints two lines, then `Error: Command failed / 0: Failed to write NDJSON line / 1: Broken pipe (os error 32)`, exit 1.
  A closed stdout is the normal end of `| head`; exit 0 quietly.

### 6. [cosmetic, docs] README examples that do not work
- `drata vendor list --expand questionnaires contacts` -> 400 `expand must be one of the following values: customFields,documents,integrations,lastQuestionnaire,latestSecurityReviews,reviews,...`. `--expand` is not validated client side.
- README calls the binary `drata-cli` (`drata-cli login ...`); the installed binary is `drata`.

### 7. [suggestion] `monitor list --name` is an exact match
- `--name Socket` returns 0 rows; the full name returns 1. The help says only "Filter by test name".

## Pipeline Recipes

```bash
drata vendor list --output json | jq -r '.data[].category' | sort | uniq -c | sort -rn | head -5
#  14 ENGINEERING / 4 null / 4 INFORMATION_TECHNOLOGY / 3 SECURITY / 3 NONE

drata monitor list 1 --output json | jq -r '.data[].checkResultStatus' | sort | uniq -c | sort -rn
#  101 READY / 52 FAILED / 39 PASSED

drata personnel list --all | jq -r '.employmentStatus' | sort | uniq -c | sort -rn
#  382 CURRENT_EMPLOYEE / 127 FORMER_EMPLOYEE / 36 SERVICE_ACCOUNT / 8 OUT_OF_SCOPE

drata risk list 1 --output json | jq -r '.data[] | [.riskId, .treatmentPlan, .impact, .likelihood] | @tsv' | head -3
#  CR-11  ACCEPT  5  1

drata vendor-security-review list --output json | jq -c '[.data[] | select(.status=="IN_PROGRESS") | {id}]'
#  [{"id":16}]

# chain the cross-vendor list into the per-vendor get
drata vendor-security-review list --output json \
  | jq -r '.data[] | select(.status=="IN_PROGRESS") | "\(.vendorId) \(.id)"' | head -1 \
  | xargs -n2 drata vendor security-review get --output json | jq -c '{id,vendorId,status}'
#  {"id":16,"vendorId":24,"status":"IN_PROGRESS"}

# the only way to read events or assets on this tenant (bug 1)
drata event list --all | head -n 2 | jq -c '{id,type,category,createdAt}'
drata asset list --all | head -n 1 | jq -c '{id,name,assetType,assetProvider}'
```

## Edge Cases

| Input | Result |
|---|---|
| `drata vendor get` (missing ID) | clap usage, exit 2 |
| `drata frobnicate` | `unrecognized subcommand`, exit 2 |
| `drata --output xml workspace list` | invalid value, possible values listed, exit 2 |
| `drata --region mars ...` | `Unknown region "mars". Valid values: us, eu, apac`, exit 1, no request |
| `drata vendor get abc` | server 400 `Must be positive integer`, exit 1 |
| 20 nonexistent IDs (`vendor get 99999999`, `user get`, `evidence get`, VSR note/observation get, connections, access-review period, dcf-suggestion, ...) | server 404 with its message, exit 1, nothing on stdout (`2>/dev/null \| wc -c` = 0) |
| read-only key, `--yes evidence-item remove 1 99999999` | `write blocked: ... not write-enabled`, exit 1, no request |
| `--allow-writes evidence-item remove ...` without `--yes`, non-TTY | `requires confirmation but stdin/stdout is not a TTY`, names the op as permanently deleting the item and its artifacts, exit 1 |
| `evidence-item create 1 --name x --url https://example.com` | clap: `--filed-at` required, exit 2 |
| `upload-url create --file /nonexistent/zzz.txt` | `Failed to read upload file`, exit 1, no request |
| `raw --example GET /vendors` | `has no JSON request body`, exit 1 |
| `--all \| head` | data printed, then the broken-pipe error, exit 1 (cosmetic 5) |

## Release Validation

- Tag `v0.2.0`: exists, annotated (`git cat-file -t` = `tag`, message `Release v0.2.0`), points to `36b4fcd7528d5f5ddefec6f60cf20ef6998bb4df`. Correct.
- Release `v0.2.0`: published 2026-11-07T15:38:19Z, not draft, not prerelease.
- Assets: `drata-v0.2.0-linux-amd64.tar.gz` and `drata-v0.2.0-linux-arm64.tar.gz`, each with a `.sha256`. No macOS (darwin) builds.
- Binary test: downloaded `linux-amd64` (host `Linux x86_64`), `sha256sum -c` OK, the tarball holds `drata`, which prints `drata v0.2.0`, matching the installed binary.

## Observations

- Validation, write guard and confirm all fail before any network I/O, with clear messages. Errors go to stderr; stdout stays clean for pipes.
- The presigned upload never printed or logged the presigned URL at the default log level. `vendor security-review summary` prints the API's `signedUrl`, as designed.
- `policy actions` needs write-scoped permissions although it is a GET: the read-only key gets 403.
- Gated surface on this tenant: VSR assessments, workflow and residual risk, vendor criteria, risk insights and risk tickets answer 402. Evidence Items need the "new Drata Experience" opt-in (403, not 402). The VSR AI-agent reads answer 403. Phase 0 probed list endpoints only, so it did not catch these.
- `vendor list` returns 33 vendors, `vendor search` 32.
- Never run list commands in parallel with `asset list` or `event list` on this tenant (bug 1).
