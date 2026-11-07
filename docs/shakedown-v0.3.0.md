# CLI Shakedown Report: drata v0.3.0

Binary: `~/.cargo/bin/drata` (`drata v0.3.0`). Source: `tatari-tv/drata-cli` @ `256d7f0` (tag `v0.3.0`).
Run 2026-11-07 against Tatari's live SafeBase tenant (plan `business`) and the live Drata tenant (US region).

Verdict: the SafeBase namespace works. Upload, replace and delete round-tripped clean, the credential split holds in
both directions, and the write guard stops every write locally. Two bugs: `member list` needs `--email` despite the
docs, and `document get --external` does not lowercase the id the way `upload` does.

Focus is the new `drata safebase` namespace (`docs/plan/2026-11-05-safebase-api.md`). Every SafeBase command ran
with `XDG_CONFIG_HOME` and `XDG_DATA_HOME` pointed at a scratch dir, so the scratch `credentials.json` and log were
the only ones touched. Keys came from keep (`safebase-readonly-api-key`, `safebase-readwrite-api-key`,
`drata-readonly-api-key`) and reached the CLI through `drata safebase login --api-key` into the scratch file, or as
an env var for a single call. No key value appears here. The scratch `credentials.json` files were removed with
`shred -u` at the end.

Customer names and emails from the trust center are redacted below as `<customer>` / `<email>`.

## Summary

| Metric | Count |
|--------|-------|
| SafeBase commands discovered | 18 (`login`, `logout`, `raw`, 3 `account`, 3 `member`, 4 `document`, 3 `request`, 3 `kb`) |
| SafeBase commands tested live | 13 |
| SafeBase commands passed | 12 (`member list` fails without `--email`, bug 1) |
| SafeBase commands skipped (mutating) | 6 (`member add/remove`, `request approve/decline`, `kb add/update`): argument validation only |
| Live writes | 3 (upload, replace, delete of `zzz-clitest-shakedown-1`), all succeeded, tenant back to 23 documents |
| Drata regression commands | 6 (`whoami`, `auth`, `vendor list`, `vendor list --all`, `vendor get`, `raw GET /workspaces`), all passed |
| Defects found | 2 bugs, 4 cosmetic, 1 suggestion |
| Pipelines tested | 7 |
| Edge cases tested | 34 |
| Hangs | 0 (every call returned in well under a second) |

## Credential isolation

- `src/config.rs` `xdg_config_dir()` honors `$XDG_CONFIG_HOME` when it is absolute, then falls back to
  `$HOME/.config`; `credentials_path()` is `<that>/drata/credentials.json`. `xdg_data_dir()` does the same for the
  log under `$XDG_DATA_HOME`
- `~/.config/drata/` does not exist on this machine, before or after the run (`ls: cannot access`). There was no
  file to hash, so the before/after check is "absent before, absent after"
- `drata whoami` / `auth` run against the default location (no `XDG_CONFIG_HOME`) read nothing and created nothing
- No key value appears in the scratch log, the scratch outputs, or `~/.local/share/drata/logs/drata.log`
  (checked with `grep -F` against each decrypted key, file names only; the only hits were the scratch
  `credentials.json` files, since shredded)

## Command Results

### Credential commands (scratch config)

| Command | Exit | Result |
|---|---|---|
| `drata safebase account list` (no credential) | 1 | `No SafeBase API key found for profile "default".` plus login and env hints |
| `drata auth` / `drata whoami` (empty config) | 0 | Drata lines first, then indented `safebase:` block, both `not found` |
| `drata safebase login --api-key <ro>` | 0 | `Saved SafeBase credential to profile "default" (writes disabled).` File mode `0600`, nested shape, no `.pre-safebase` (file was new) |
| `drata login --profile drataonly --api-key <drata-ro> --region us` | 0 | `Saved profile "drataonly" (region us, writes disabled).` |
| `drata safebase login --profile rw --api-key <rw> --allow-writes` | 0 | `Saved SafeBase credential to profile "rw" (writes enabled).` |
| `drata whoami` (SafeBase-only profile) | 0 | `key: not found` for Drata, `safebase: key: found / source: profile "default" / writes: disabled` |
| `drata safebase logout --profile drataonly` | 0 | `No SafeBase credential in profile "drataonly" to remove.` (file not rewritten) |
| `drata safebase logout --profile rw` | 0 | `Removed the SafeBase credential and profile "rw".` |
| `drata safebase logout --profile nosuch` | 0 | `No SafeBase credential in profile "nosuch" to remove.` |
| `drata whoami` on a flat profile (dummy key) | 0 | reads as a Drata block; file hash unchanged (a read never rewrites) |
| `drata safebase login` over that flat file (dummy key) | 0 | writes the nested shape, copies the original to `credentials.json.pre-safebase` (0600), byte-identical to the flat file |
| any command on a profile mixing flat fields and a `safebase` block | 1 | `profile "mixed" in credentials.json mixes flat fields (api-key, region, allow-writes) with a nested "drata"/"safebase" block. Move the flat fields into its "drata" block.` |

### Raw

| Command | Exit | Result |
|---|---|---|
| `drata safebase raw GET /organizations` | 0 | `{"ok":true,"result":{"name":"Tatari","plan":"business","spCustomDomain":"trust.tatari.tv",...}}` |
| `drata safebase raw GET /accounts --query pageNumber=2 pageSize=1` | 0 | 1 item, `pagination.totalPages: 3`, matches Phase 0 Observed |
| `drata safebase raw get /documents --query page=1 limit=10` | 0 | 10 items, `count: 23` (lower-case method accepted) |
| `drata safebase raw GET /tags` | 0 | `{"ok":true,"result":{"count":0,"items":[]}}`; the tenant still has no tags |
| `drata safebase raw GET /documents --query limit=500` | 1 | `400 ... "limit" must be less than or equal to 100` |
| `drata safebase raw GET /organizations/members/me` | 1 | `404 ... Organization Member not found` (server side: an API key is not an org member) |
| `drata safebase raw GET /vendors` (not in the SafeBase spec) | 1 | sent anyway, `404` with a full HTML page in the error (cosmetic 4) |
| `drata safebase raw DELETE /document/<id> --profile rw --yes` | 0 | `{"ok": true}` |
| `drata safebase raw --example POST /documents/upload` | 0 | `{"fileName":"string","fileSize":0,"md5b64":"string","mimeType":"text/csv"}`, no key needed |
| `drata safebase raw --example POST /accounts` | 0 | populated `input` skeleton from the spec examples |
| `drata safebase raw --example POST '/accounts/{id}/members'` | 0 | `members[]` + `message` skeleton |
| `drata safebase raw --example POST /kb/entries` | 0 | `{"question":"string"}` |
| `drata safebase raw --example PATCH '/document/{id}'` | 0 | `{}` (every field optional; disclosed in the implementation notes) |
| `drata safebase raw --example DELETE '/document/{id}'` | 1 | `has no JSON request body` |
| `drata safebase raw --example GET /vendors` | 1 | `no operation 'GET /vendors' in the SafeBase spec; check the method and path template (e.g. /vendors/{id})` |

### Curated reads (read-only key, default profile)

| Command | Exit | Result |
|---|---|---|
| `drata safebase account list --output table` | 0 | 3 rows: `ID NAME DOMAINS MEMBERS NDA LAST SEEN` |
| `drata safebase account list --output json` | 0 | `{"data":[...]}`, 3 items, 15 keys each |
| `drata safebase account list --all` | 0 | 3 NDJSON lines, same ids as the JSON list |
| `drata safebase account get 58183613-e62c-45ee-a6a6-bee93318df9d` | 0 | `Test  example.com`, MEMBERS/NDA/LAST SEEN blank (cosmetic 3) |
| `drata safebase account url 58183613-...` | 0 | `https://app.safebase.io/accounts/58183613-.../share`, bare text in JSON mode too (disclosed) |
| `drata safebase member list` | 1 | `400 ... "email" is required` (bug 1) |
| `drata safebase member list --sfdc-account-id 001000000000000AAA` | 1 | `400 ... "email" is required` (bug 1) |
| `drata safebase member list --email <email>` | 0 | 1 row: `ACCOUNT NAME EMAIL LAST SEEN` |
| `drata safebase member list --email nobody@example.com` | 1 | `404 ... Not found` (suggestion 7) |
| `drata safebase document list --output json` | 0 | 23 documents, 23 unique ids, all `application/pdf` |
| `drata safebase document list --all -l debug` | 0 | 23 NDJSON lines, identical to the JSON list; log: `stream_all_pages complete total_items=23 pages=1` |
| `drata safebase document list --query SOC` | 0 | 3 SOC 2 documents |
| `drata safebase document get 59461068-e2d3-438f-a2f4-567c811a7fa4` | 0 | `AI Data Governance Policy.pdf  43144  complete`; JSON has `checksum` (md5b64) |
| `drata safebase document get drata_policy:65 --external` | 1 | `404` (Phase 0 Observed: non-UUID external ids do not resolve) |
| `drata safebase document types` | 0 | 119 types, `{"data":[{"id":"acceptable_use_policy",...}]}` |
| `drata safebase request list` | 0 | 4 requests (3 approved, 1 declined) |
| `drata safebase request list --status pending / approved / declined` | 0 | 0 / 3 (all approved) / 1 (declined) |
| `drata safebase kb search` | 0 | 7 entries |
| `drata safebase kb search --all` | 0 | 7 NDJSON lines, identical to the JSON list |
| `drata safebase kb search incident` | 0 | 1 entry: `How do you test your incident response procedures?` |
| `drata safebase kb search zzzzqqqxx-nothing` | 0 | `{"data": []}` |

Paging note: every list on this tenant fits in one page (3 accounts, 23 documents, 7 KB entries against page sizes
1000 / 100 / 1000), so the multi-page loop did not run live through a curated verb. The debug log shows one request
and a clean stop on `count`. The page parameter semantics (zero-based `/documents`, one-based `/accounts`) were
re-confirmed through `raw`.

### Curated writes (read-write key, `rw` profile)

The confirm prompt reads stdin and stdout; when either is not a TTY and `--yes` is absent it fails closed
(`src/confirm.rs`). This run is non-interactive, so every live write passed `--yes`. Without it:
`mutation DELETE /document/<id> requires confirmation but stdin/stdout is not a TTY. Pass --yes to bypass, or run interactively.`, exit 1, nothing sent.

Test files: two hand-built one-page PDFs with a timestamp in the text stream, so each has its own md5.

| File | Bytes | md5b64 |
|---|---|---|
| `zzz-clitest-shakedown-1.pdf` | 435 | `NG4SaaTYuXK4+2NiyANicA==` |
| `zzz-clitest-shakedown-2.pdf` | 448 | `XUNKB858eaBQgCVool2EwA==` |

| Step | Command | Exit | Result |
|---|---|---|---|
| Upload | `drata safebase document upload zzz-clitest-shakedown-1.pdf --name zzz-clitest-shakedown-1 --profile rw --yes --output json -l debug` | 0 | id `a1ff99f2-9bcd-471a-be7f-bdb58e5b513d`, externalId `24a4dc75-17ae-480e-a791-6eee52a4a734` (fresh v4), checksum `NG4SaaTYuXK4+2NiyANicA==`, size 435, `uploadStatus: pending` |
| Lookup | `drata safebase document get 24a4dc75-17ae-480e-a791-6eee52a4a734 --external` | 0 | same id, checksum and size, `uploadStatus: complete` |
| Replace | `drata safebase document upload zzz-clitest-shakedown-2.pdf --name zzz-clitest-shakedown-1 --replace a1ff99f2-... --external-id 7E3B1C2A-9D4F-4A6B-8C1D-2E3F4A5B6C7D --profile rw --yes` | 0 | id kept `a1ff99f2-...`; checksum now `XUNKB858eaBQgCVool2EwA==`, size 448; externalId stored lowercase `7e3b1c2a-...` |
| Lookup after replace | `document get 7e3b1c2a-... --external` | 0 | found, new checksum; the old external id `24a4dc75-...` now 404s |
| Lookup, uppercase | `document get 7E3B1C2A-9D4F-4A6B-8C1D-2E3F4A5B6C7D --external` | 1 | 404 (bug 2) |
| Delete | `drata safebase raw DELETE /document/a1ff99f2-9bcd-471a-be7f-bdb58e5b513d --profile rw --yes` | 0 | `{"ok": true}` |
| Confirm gone | `drata safebase document get a1ff99f2-9bcd-471a-be7f-bdb58e5b513d` | 1 | `API error 404 Not Found ... Not found` |
| Tenant check | `drata safebase document list` | 0 | 23 documents, no `zzz` names |

Debug log for the upload: one confirm covering all three steps, step 1 `200`, the presigned PUT to
`temp-org-storage.safebase.io/<org>/documents/<fileId>` logged without its query string and with 4 headers (the
ones SafeBase returned, no key), step 3 `200`.

### Skipped (mutating), argument validation only

Never sent: `request approve | decline`, `member add | remove`, `kb add | update`. They email customers or change
the public trust center. A debug log over the whole block recorded zero requests to `app.safebase.io`.

| Command | Exit | Result |
|---|---|---|
| `request approve <id>` | 2 | `--account-id <ACCOUNT_ID>` required |
| `request approve --account-id <id>` | 2 | `<REQUEST_ID>` required |
| `request decline <id> --message hi --notify` | 2 | `'--message <MESSAGE>' cannot be used with '--notify'` |
| `member add <id> --email a@example.com` | 2 | `--first-name`, `--last-name` required |
| `member add <id> --email .. --first-name A --last-name B --message hi --notify` | 2 | `--message` conflicts with `--notify` |
| `member remove <id>` | 2 | `--email <EMAIL>...` required |
| `kb add` | 2 | `--data <DATA>` required |
| `kb add --data {notjson` | 1 | `--data is not valid JSON` |
| `kb update --data {}` | 2 | `<ID>` required |
| `kb update x --data {notjson` | 1 | `--data is not valid JSON` |
| `document upload --name zzz` | 2 | `<FILE>` required |

### Drata regression (default config location, Drata read-only key from env)

`~/.config/drata/` does not exist on this machine, so the own-profile check ran with the default
`XDG_CONFIG_HOME` and the Drata read-only key passed as `DRATA_API_KEY` for each call. `drata login | logout` were
not run against the default location.

| Command | Exit | Result |
|---|---|---|
| `drata whoami` (no key) | 0 | `key: not found` ... then the `safebase:` block |
| `DRATA_API_KEY=.. drata whoami` | 0 | `key: found / source: DRATA_API_KEY env var / profile: default / region: (default us) / writes: disabled`, then `safebase:` |
| `DRATA_API_KEY=.. drata auth` | 0 | same Drata lines, then `safebase:` and the SafeBase setup hint only |
| `DRATA_API_KEY=.. drata vendor list --output json` | 0 | 33 vendors, first `{"id":38,"name":"Datadog","status":"ACTIVE"}` (same as v0.2.0) |
| `DRATA_API_KEY=.. drata vendor list --all` | 0 | 33 NDJSON lines |
| `DRATA_API_KEY=.. drata vendor get 38` | 0 | `{"id":38,"name":"Datadog"}` |
| `DRATA_API_KEY=.. drata raw GET /workspaces` | 0 | `["Tatari"]` |
| `drata workspace list --profile drataonly` (scratch) | 0 | `["Tatari"]` |

The Drata lines lead and keep v0.2.0's labels and order (`key`, `source`, `profile`, `region`, `writes`, `creds`,
`profiles`, checked against `git show v0.2.0:src/resources/auth.rs`). The only unindented `writes:` line is still
Drata's.

## Output Format Matrix

| Command | auto (piped) | `--output json` | `--output table` | `--all` |
|---|---|---|---|---|
| `account list` | JSON object | `{"data":[3]}` | table | NDJSON, 3 lines |
| `account get` | JSON object | object | table (3 blank columns) | n/a |
| `account url` | plain URL | plain URL (by design) | plain URL | n/a |
| `member list --email` | JSON object | `{"data":[1]}` | table | n/a |
| `document list` | JSON object | `{"data":[23]}` | table | NDJSON, 23 lines |
| `document get` | JSON object | object | table | n/a |
| `document types` | JSON object | `{"data":[119]}` | table | n/a |
| `request list` | JSON object | `{"data":[4]}` | table | n/a |
| `kb search` | JSON object | `{"data":[7]}` | table | NDJSON, 7 lines |
| `whoami` / `auth` | text | text (ignores `--output json`, unchanged from v0.2.0) | text | n/a |

Every JSON and NDJSON output went through `jq -e`. No table output parsed as JSON, so no silent fallback.
`--all --output table` prints NDJSON, which is the documented `--all` contract.

## Failures & Bugs

### 1. [bug] `member list` without `--email` always fails
- Commands: `drata safebase member list`, `drata safebase member list --sfdc-account-id 001000000000000AAA`.
- Both exit 1: `API error 400 Bad Request on GET .../accounts/members: Invalid request: [ValidationError: "email" is required]`.
- The help ("List account members, optionally filtered"), the README synopsis
  (`member list [--email <e>] [--sfdc-account-id <id>]`) and the spec (both params optional) all say a bare call
  lists members. The live API requires `email`. Phase 0 did not probe this op.
- Expected: make `--email` required in clap and say `--sfdc-account-id` narrows it. The error is clean, so nothing
  breaks silently, but the documented call never works.

### 2. [bug] `document get --external` does not canonicalize the id the way `upload --external-id` does
- `upload --external-id 7E3B1C2A-9D4F-4A6B-8C1D-2E3F4A5B6C7D` accepted the uppercase UUID and stored
  `7e3b1c2a-9d4f-4a6b-8c1d-2e3f4a5b6c7d` (help: "Sent in lowercase hyphenated form; `document get --external` finds it").
- `document get 7E3B1C2A-9D4F-4A6B-8C1D-2E3F4A5B6C7D --external` sends the string verbatim and gets 404.
  The lowercase form finds it.
- Expected: when `--external` is set and the id parses as a UUID, send the same lowercase hyphenated form upload
  stores. Non-UUID external ids (`drata_policy:65`) 404 server side either way.

### 3. [cosmetic] `account get` table has three columns that are always blank
- `drata safebase account get <id> --output table` shows `MEMBERS`, `NDA`, `LAST SEEN` empty for every account.
- `GET /accounts/{id}` does not return `numMembers`, `isNdaDone` or `lastSeen` (only `GET /accounts` does), and
  `get` reuses the list renderer. `account list` shows `1  true  2026-11-06T...` for the same account.
- Expected: a `get` table with only the fields `GET /accounts/{id}` returns.

### 4. [cosmetic] Non-JSON error bodies are printed whole
- `drata safebase raw GET /vendors` prints SafeBase's full Next.js 404 page (about 20 KB of inline scripts and
  config) inside the error. `drata safebase document get ../organizations` (encoded to `..%2Forganizations`, no
  traversal) prints a full Cloudflare challenge page on a 403.
- Shared with the Drata path (`format_api_error` in `src/client.rs` falls back to the raw body).
- Expected: when the body is not JSON, print the status, content type and the first line or so.

### 5. [cosmetic] Narrow tables truncate id columns into uselessness
- Piped or under 120 columns, `render_table` shrinks the widest column first. KB ids are `<orgId>~<entryId>`
  (73 chars), so `kb search` shows `dccaf15d-095a-46b1-bd53-dd5411a7870a…` on every row, identical, and no row's id
  can be copied into `kb update`. `request list` ids lose their tail the same way, and one `document types` id
  (`information_security_management_system_isms_and_privacy_in…`) can no longer be passed to `--type`.
- At 250 columns every id prints in full. Same mechanism as the Drata tables.
- Expected: never shrink an ID column, or shrink KB ids from the left.

### 6. [cosmetic] `raw --example` hint names a Drata path under `safebase`
- `drata safebase raw --example GET /vendors`: `... check the method and path template (e.g. /vendors/{id})`.
  `/vendors` is a Drata path; under `safebase` the example should be a SafeBase one (`/document/{id}`).

### 7. [suggestion] `member list --email <unknown>` is a 404 error, not an empty list
- `drata safebase member list --email nobody@example.com` exits 1 with `404 ... Not found`. That is SafeBase's
  answer, but for a list verb an empty `{"data": []}` with exit 0 reads better in scripts. Worth a decision either way.

## Pipeline Recipes

```bash
# Trust center URL for an account, by name
drata safebase account list --all | jq -r 'select(.name=="Test")|.id' | xargs -I{} drata safebase account url {}
# https://app.safebase.io/accounts/58183613-e62c-45ee-a6a6-bee93318df9d/share

# Which account an approved requester landed in
email=$(drata safebase request list --status approved --output json | jq -r '.data[]|select(.companyName=="<customer>")|.email')
acct=$(drata safebase member list --email "$email" --output json | jq -r '.data[0].accountId')
drata safebase account get "$acct" --output json | jq -r .name
# <customer>

# Document MIME mix
drata safebase document list --output json | jq -r '.data[].mimeType' | sort | uniq -c | sort -rn
#      23 application/pdf

# First document by id
drata safebase document list --output json | jq -r '.data[0].id' | xargs drata safebase document get --output json | jq -r .name
# AI Data Governance Policy.pdf

# Companies with approved access
drata safebase request list --status approved --output json | jq -r '.data[].companyName' | sort -u

# KB entries by access level
drata safebase kb search --all | jq -r .accessLevel | sort | uniq -c
#       1 internal
#       6 public

# SOC 2 documents
drata safebase document list --query SOC --output json | jq -r '.data[].name'
# Tatari SOC 2 Readiness In Progress Report Memo.pdf
# Tatari SOC 2 Attestation In Progress Report Memo.pdf
# 2026 SOC 2 Type 2 Report.pdf
```

## Edge Cases

| Case | Result |
|---|---|
| `drata safebase account list --region us` | exit 1, `--region applies to Drata commands only; SafeBase has a single region` |
| `drata --region eu safebase document types` (global flag before the namespace) | exit 1, same error |
| `drata safebase raw --example POST /documents/upload --region eu` | exit 1, same error, no key needed |
| `drata safebase login --api-key dummy --region eu` | exit 1, same error; scratch `credentials.json` mtime unchanged |
| `DRATA_REGION=eu drata safebase document types` | exit 0, 119 types (env ignored, as designed) |
| `account get 11111111-2222-4333-8444-555555555555` | exit 1, `404 ... Not found` |
| `account get not-a-uuid` | exit 1, `400 ... "accountId" must be a valid GUID` |
| `account url 00000000-0000-0000-0000-000000000000` | exit 1, `400 ... must be a valid GUID` |
| `document get <unknown uuid>` / with `--external` | exit 1, 404 / 404 |
| `document get a/b` | exit 1, path encoded to `a%2Fb`, `400 ... "id" must be a valid GUID` |
| `document get ../organizations` | exit 1, encoded to `..%2Forganizations`, Cloudflare 403 (cosmetic 4) |
| `request list --status bogus` | exit 2, clap lists `pending, approved, declined` |
| SafeBase command on `drataonly` profile | exit 1, `No SafeBase API key found for profile "drataonly".`, no request sent |
| same with `DRATA_PROFILE=drataonly` | exit 1, same error |
| same with `DRATA_API_KEY` set to a working Drata key | exit 1, same error; no fallback to the Drata key |
| Drata command on a SafeBase-only profile | exit 1, `No Drata API key found for profile "default".`; no fallback the other way |
| read-only profile: `raw DELETE /document/<id> --yes` | exit 1, `write blocked: the active SafeBase credential is not write-enabled.` |
| read-only profile: `document upload ... --yes` | exit 1, `upload step 1 (POST /documents/upload) failed, no document was created: write blocked ...` |
| read-only profile: `request decline <id> --yes` | exit 1, write blocked |
| read-only profile: `kb add --data '{"question":"zzz"}' --yes` | exit 1, write blocked |
| (the four above, debug log) | zero requests to any `safebase.io` host |
| read-write key via `SAFEBASE_API_KEY`, no `--allow-writes` | exit 1, write blocked (env keys are read-only by default) |
| read-write key via env, `--allow-writes`, no `--yes`, stdin `/dev/null` | exit 1, confirm fails closed, nothing sent |
| read-only key with `--allow-writes --yes` on upload | exit 1, server `403 ... Missing required scope(s): documents:edit.` (key scope enforced server side) |
| upload an 80 MiB file (sparse) | exit 1, `big.pdf is 83886080 bytes; SafeBase accepts at most 78643200 bytes per upload`, before any request |
| upload `odd.xyz` | exit 1, `not a type SafeBase accepts (chemical/x-xyz); allowed: ...` |
| upload a file with no extension | exit 1, `no type from its extension` |
| upload `--external-id not-a-uuid` | exit 1, `--external-id 'not-a-uuid' is not a UUID` |
| upload a missing file | exit 1, `Failed to read upload file 'missing.pdf'` |
| `raw GET /documents --query limit=500` | exit 1, server 400 naming the limit |
| `raw GET /vendors` (off-spec path) | sent anyway with a log-only warning, 404 HTML (cosmetic 4) |
| `raw --example` on a GET or DELETE op | exit 1, `has no JSON request body` |
| flat + nested profile mix | exit 1, names the profile and the fix |
| no credential at all | exit 1, names the backend, profile, login command and env var |

## Release Validation

- Tag: `git cat-file -t v0.3.0` prints `tag` (annotated, tagger Scott A. Idler, 2026-11-07 14:47:04 -0800,
  "Release v0.3.0"). `v0.3.0^{commit}`, `origin/main` and `HEAD` are all `256d7f062079a5af8aa4d6a98b712feb2897e163`.
  `Cargo.toml` says `0.3.0`.
- Release: https://github.com/tatari-tv/drata-cli/releases/tag/v0.3.0 exists, published 2026-11-07T22:50:47Z by
  `github-actions[bot]`, not a draft or prerelease, marked Latest.
- Assets: `drata-v0.3.0-linux-amd64.tar.gz`, `drata-v0.3.0-linux-arm64.tar.gz`, each with a `.sha256`. No macOS
  assets (same as earlier releases).
- Binary test: downloaded the linux amd64 tarball, `sha256sum -c` OK
  (`a468ae064acd8d67660fdf7434d4dab056d12f809a7eaf4b2aead62c492bb67a`), extracted one `drata` (x86-64 ELF).
  `--version` prints `drata v0.3.0`, matching `~/.cargo/bin/drata`. The release binary also ran
  `drata safebase document types` against the scratch config: 119 types.

## Observations

- The upload path matches the design: local size and MIME checks before any request, md5b64 computed locally and
  equal to SafeBase's `checksum`, the signed URL logged without its query, no key on the storage request, one
  confirm for all three steps. `--replace` keeps the id and takes the new name, external id, checksum and size, as
  Phase 0 recorded. Tags across `--replace` are still untested: the tenant has no tags.
- The write guard runs before the confirm prompt and before any network call, on every write verb tried. A
  read-only key with `--allow-writes` reaches SafeBase and gets the server's 403, so the two layers stack.
- Every existing document carries a Drata-sync external id (`drata_policy:<n>`, `drata_evidence:<n>:<m>`), which
  `document get --external` cannot reach (server 404 on non-UUIDs). Only documents uploaded with a UUID external
  id are reachable that way.
- Errors still end with a `Location: src/...:line` trailer (eyre). Same as v0.2.0.
- `whoami` and `auth` ignore `--output json`. Same as v0.2.0.
- Each successful upload consumes a SafeBase `fileId`, and SafeBase has no op to delete an upload; the two from this
  run were attached to the deleted document.
- No 429s and no rate-limit headers seen, matching Phase 0.
