# drata-cli

One CLI for both Drata products: the Drata GRC API and the SafeBase Trust Center
API. Curated commands cover the common surface, and a generic `raw` namespace
reaches every operation of both APIs. Drata commands are top-level; SafeBase
commands live under `drata safebase` (see [SafeBase](#safebase)).

## Installation

```
cargo install --path .
```

The crate is `drata-cli`; the binary is named `drata`. After install it is on `$PATH` via `~/.cargo/bin`.

## Authentication

Credentials are stored in `~/.config/drata/credentials.json` as named profiles.
Each profile holds one block per backend (`drata`, `safebase`), each with an API
key and an explicit write-enable flag. This section covers the Drata block;
SafeBase is in [SafeBase](#safebase).

### Save a credential profile

```
# Read-only profile (default)
drata login --api-key YOUR_API_KEY --region us

# Write-enabled profile (required for any POST/PUT/DELETE)
drata login --api-key YOUR_API_KEY --region us --allow-writes --profile write

# EU tenant
drata login --api-key YOUR_API_KEY --region eu
```

### Verify your credential

```
drata whoami
drata auth     # shows diagnostics even without a configured key
```

### Credential precedence

1. `--api-key` CLI flag
2. `DRATA_API_KEY` environment variable
3. Profile from `credentials.json` (selected by `--profile` or `DRATA_PROFILE`, defaulting to `default`)

Regions: `us` (default), `eu`, `apac`.

### Write gate

Non-GET requests fail unless the resolved credential has `allow-writes: true`.
This prevents accidental mutation from a stray `DRATA_API_KEY` environment variable.
All mutating operations (POST/PUT/DELETE) also prompt for confirmation unless `--yes` is set.

```
# Writes require an allow-writes credential
drata --profile write vendor create --name "Acme Corp"

# Or pass --allow-writes at invocation time
drata --api-key $DRATA_API_KEY --allow-writes vendor create --name "Acme Corp"

# Skip confirmation for scripting
drata --profile write --yes vendor remove 42
```

## Logs

Logs are written to `~/.local/share/drata/logs/drata.log`. Control the level with
`-l / --log-level` (trace, debug, info, warn, error).

## Output format

Use `--output json`, `--output table`, or `--output auto` (default: table when TTY,
JSON when piped). Pass `--output json` to pipe results to `jq`.

## Curated resource commands

Each resource supports `--help` on both the resource and the verb.

### Vendors

```
drata vendor list
drata vendor list acme corp          # filter by name pattern
drata vendor get 123
drata vendor create --example        # print a JSON skeleton and exit
drata vendor create --name "Acme" --category SECURITY --risk LOW
drata vendor update 123 --risk HIGH
drata vendor remove 123
drata vendor upload 123 --file ./contract.pdf
drata vendor questionnaire list 123
drata vendor questionnaire send 123 --email sec@vendor.com --questionnaire-id 5 --security-review-id 7 --email-content "Please complete"
drata vendor questionnaire get 123 5
drata vendor questionnaire questions 123 5
drata vendor search                  # all pages as rows
drata vendor search --facet <name>   # one page, whole {data, pagination, facets} envelope
drata vendor trust-overview 123
```

#### Vendor security reviews

```
drata vendor security-review list 123
drata vendor security-review create 123 --example
drata vendor security-review get 123 <review_id>
drata vendor security-review update 123 <review_id> --example
drata vendor security-review actions 123 <review_id>
drata vendor security-review run-action 123 <review_id> --action <action>
drata vendor security-review create-with-file 123 --file ./soc2.pdf --title "SOC 2" --review-deadline-at 2027-01-01 --status IN_PROGRESS --type SOC_REPORT --document-type SOC_DOCUMENT
drata vendor security-review upload-questionnaire 123 --file ./answers.xlsx
drata vendor security-review questionnaires 123 <review_id>
drata vendor security-review note list 123 <review_id>              # also get | create | update | remove
drata vendor security-review observation list 123 <review_id>       # also get | create | update | remove
drata vendor security-review assessment list 123 <review_id>        # also get | stats
```

The agent workflow verbs start background jobs and print the 202 body; nothing
polls. Read progress with `workflow` and `summary`:

```
drata vendor security-review start-agent 123 <review_id>
drata vendor security-review confirm-criteria 123 <review_id> --example
drata vendor security-review start-assessment 123 <review_id>
drata vendor security-review stop-assessment 123 <review_id>
drata vendor security-review collect-public-documents 123 <review_id>
drata vendor security-review access-request-requirements 123 <review_id>
drata vendor security-review request-trust-center-access 123 <review_id> --example
drata vendor security-review provide-trust-center-access-link 123 <review_id> --data @link.json
drata vendor security-review skip-trust-center-access-request 123 <review_id>
drata vendor security-review add-documents 123 <review_id> --file ./a.pdf ./b.pdf
drata vendor security-review upload-questionnaire-to-review 123 <review_id> --file ./answers.xlsx
drata vendor security-review generate-follow-up-questionnaire 123 <review_id>
drata vendor security-review follow-up-questionnaire 123 <review_id>
drata vendor security-review send-follow-up-questionnaire 123 <review_id> --example
drata vendor security-review residual-risk 123 <review_id>
drata vendor security-review finalize 123 <review_id> --example
drata vendor security-review summary 123 <review_id>
drata vendor security-review workflow 123 <review_id>
```

#### Vendor criteria, templates and risk levels

```
drata vendor-criteria list
drata vendor-criteria-creation list <workspace_id>
drata vendor-questionnaire-template list
drata vendor-questionnaire-template questions <questionnaire_id>
drata vendor-risk-level list
drata vendor-security-review list      # security reviews across all vendors
```

### Risk Registers

Required to discover `riskRegisterId` before using `risk` commands.

```
drata register list
drata register get 1
drata register create --example
drata register create --name "Main Register" --description "Primary risk register"
drata register update 1 --name "Updated Name"
drata register remove 1
```

### Risks

```
drata risk list <register_id>
drata risk get <register_id> <risk_id>
drata risk create <register_id> --example
drata risk create <register_id> --title "Password Policies" --description "..." --treatment-plan MITIGATE --impact 4 --likelihood 3
drata risk update <register_id> <risk_id> --status CLOSED
drata risk insights <register_id>
drata risk upload <register_id> <risk_id> --file ./evidence.pdf            # one or more --file
drata risk upload <register_id> <risk_id> --file ./a.pdf ./b.pdf
drata risk remove <register_id> <risk_id>
drata risk search                                  # all pages as rows; --facet <name> for facet counts
drata risk search --risk-register-id 3 9           # only these registers (--register-id <id> searches one)
drata risk ticket list <register_id> <risk_id>
drata risk ticket link <register_id> <risk_id> --ticket-url https://example.atlassian.net/browse/SEC-1
drata risk ticket unlink <register_id> <risk_id> <ticket_id>
drata risk ticket create <register_id> <risk_id> --example
```

### Controls

```
drata control list <workspace_id>
drata control get <workspace_id> <control_id>
drata control create <workspace_id> --example
drata control create <workspace_id> --name "Access Control" --description "..." --code "AC-1"
drata control update <workspace_id> <control_id> --name "Updated"
drata control requirements <workspace_id> <control_id>
drata control compare <workspace_id> --control-ids 1 2 3
drata control ticket list <workspace_id> <control_id>
drata control ticket link <workspace_id> <control_id> --ticket-url https://example.atlassian.net/browse/SEC-1
drata control ticket unlink <workspace_id> <control_id> <ticket_id>
```

Controls have no `ticket create`: the API has no `action-create` op for them.

### Devices

```
drata device list
drata device get <device_id>
drata device for-personnel <personnel_id>
drata device apps <device_id>
drata device upload <device_id> --file ./cert.pdf --type ANTIVIRUS_EVIDENCE
```

### Personnel

```
drata personnel list
drata personnel get <personnel_id>
drata personnel update <personnel_id> --employment-status CURRENT_EMPLOYEE
drata personnel search                          # all pages as rows; --facet <name> for facet counts
drata personnel group list <workspace_id>       # scoped personnel groups
drata personnel group counts <workspace_id>
drata personnel group add <workspace_id> <group_id>
drata personnel group replace <workspace_id> --group-id 1 2
drata personnel group remove <workspace_id> <group_id>
drata group list
```

### Policies

```
drata policy list
drata policy get <policy_id>
drata policy create --example
drata policy create --name "Security Policy" --owner-id 42 --source-type UPLOADED --description "..." --renewal-date 2027-01-01 --file ./policy.pdf
drata policy update <policy_id> --name "Updated Policy"
drata policy actions <policy_id>
drata policy versions <policy_id>
drata policy version <policy_id> <version_id>
drata policy version create <policy_id> --example
drata policy version create <policy_id> --source-type UPLOADED --owner-id 42 --material-change false --renewal-date 2027-01-01 --file ./policy-v2.pdf
drata policy version replace-file <policy_id> <version_id> --file ./policy-v2.pdf --language fr-FR
drata policy-language settings
drata policy-language list <policy_id> <version_id> --status published draft
drata policy-language versions --policy-id 7 --expand policy
drata policy-language get <id> --expand downloadPdfUrl
```

### Evidence Library

```
drata evidence list <workspace_id>
drata evidence get <workspace_id> <evidence_id>
drata evidence create <workspace_id> --example
drata evidence create <workspace_id> --name "SOC 2 Report" --renewal-schedule-type ONE_YEAR --filed-at 2026-11-01 --owner-id 42 --file ./report.pdf
drata evidence update <workspace_id> <evidence_id> --renewal-schedule-type SIX_MONTHS
drata evidence remove <workspace_id> <evidence_id>
drata evidence get-version <workspace_id> <evidence_id> <version_id>
```

### Evidence Items

The spec's Evidence tag manages Evidence Items, a different resource from the
Evidence Library above. `evidence-item remove` deletes the item and all its artifacts.

```
drata evidence-item list <workspace_id>
drata evidence-item get <workspace_id> <evidence_id>
drata evidence-item create <workspace_id> --example
drata evidence-item update <workspace_id> <evidence_id> --example
drata evidence-item remove <workspace_id> <evidence_id>
drata evidence-item upload <workspace_id> --file ./report.pdf      # prints the fileKey to pass to --file-key
drata evidence-item artifact list <workspace_id> <evidence_id>
drata evidence-item artifact update <workspace_id> <evidence_id> <artifact_id> --example
drata evidence-item artifact remove <workspace_id> <evidence_id> <artifact_id>
drata evidence-item artifact run-action <workspace_id> <evidence_id> <artifact_id> --example
```

### Upload URLs

```
drata upload-url create --purpose <purpose> [--workspace <id>]
drata upload-url create --purpose <purpose> --file ./report.pdf    # also PUTs the file; prints the objectKey
```

### Access Reviews

Only one review period may be active on the account at a time. `app users set`
replaces the application's full user list: users omitted from it are removed.

```
drata access-review app list
drata access-review app get <application_id>
drata access-review app create --name "Okta" --reviewer-id 42
drata access-review app update <application_id> --reviewer-id 42 43
drata access-review app remove <application_id>
drata access-review app users list <application_id>
drata access-review app users set <application_id> --user IDENTITY:USERNAME:EMAIL --role IDENTITY:ROLE
drata access-review period list --status active
drata access-review period get <period_id>
drata access-review period create --starting-date 2026-11-01T00:00:00Z --ending-date 2026-12-01T00:00:00Z --application APP_ID:REVIEWER_ID
drata access-review period run-action <period_id> --action complete
drata access-review period app get <period_id> <review_app_id>
drata access-review period app update <period_id> <review_app_id> --reviewer-id 42
drata access-review period app run-action <period_id> <review_app_id> --action complete
drata access-review period app users <period_id> <review_app_id>
drata access-review period evidence-package <period_id>            # prints the signed URL, never downloads
```

### Standard Connections

`connection aws | azure | gcp | gitlab` share one verb set. Bodies carry cloud
credentials, so `create` and `update` take `--data @file` or `--data -` only and
reject inline JSON before any request. `--example` prints the provider's body skeleton.
`<connection_id>` is the numeric id, or the provider's natural key: `roleArn:<role ARN>`
(AWS), `subscriptionId:<subscription ID>` (Azure), `projectId:<project ID>` (GCP).
GitLab takes the numeric id only. Pass an ARN unencoded; the CLI encodes its `/`.

```
drata connection aws list
drata connection aws get <connection_id>
drata connection aws get roleArn:arn:aws:iam::123456789012:role/DrataAuditRole
drata connection aws create --example
drata connection aws create --data @aws-connection.json
drata connection aws update <connection_id> --data @aws-connection.json
drata connection aws remove <connection_id>
```

### Custom Connections

```
drata custom-connection resource-attribute <workspace_id> <connection_id> <resource_id>
drata custom-field list
drata custom-field get <custom_field_id>
drata custom-field create --example
```

### Identities

`infrastructure-identity` and `version-control-identity` share one verb set.

```
drata infrastructure-identity list <workspace_id>
drata infrastructure-identity get <workspace_id> <identity_id>
drata infrastructure-identity actions <workspace_id> <identity_id>
drata infrastructure-identity run-action <workspace_id> <identity_id> --example
drata version-control-identity list <workspace_id>      # same verbs: get | actions | run-action
```

### Compliance Check Exclusions

Per-item results print every row; the command exits 1 if any row failed.

```
drata compliance-check-exclusion list <workspace_id>
drata compliance-check-exclusion create <workspace_id> --example
drata compliance-check-exclusion remove <workspace_id> --target-id 1 2 --compliance-check <check>
```

### Frameworks

```
drata framework list <workspace_id>
drata framework create <workspace_id> --example
drata framework create <workspace_id> --name "SOC 2" --short-name "SOC2" --description "..."
drata framework update <workspace_id> <framework_id> --name "Updated"
drata framework requirements <workspace_id> <framework_id>
drata framework requirement controls <workspace_id> <framework_id> <requirement_id>
```

### Assets

```
drata asset list
drata asset list --limit 50     # stop after 50 assets
drata asset get <asset_id>
drata asset create --example
drata asset create --name "Server" --description "..." --asset-type PHYSICAL --asset-class-types HARDWARE COMPUTE --owner-id 42
drata asset update <asset_id> --asset-type VIRTUAL
drata asset remove <asset_id>
```

### Company

```
drata company get
```

### Workspaces

```
drata workspace list
```

### Users and Roles

```
drata user list
drata user get <user_id>
drata user roles
drata user role <role_id>
drata user role-users <role_id>
```

### Monitoring Tests

```
drata monitor list <workspace_id>
drata monitor get <workspace_id> <test_id>
drata monitor update <workspace_id> <test_id> --enabled false
drata monitor exclusions <workspace_id> <test_id>
drata monitor failures <workspace_id> <test_id>
drata monitor passes <workspace_id> <test_id>
drata monitor create <workspace_id> --example
drata monitor validate-recipe <workspace_id> --example     # exits 1 when the recipe is invalid
drata monitor recipe <workspace_id> <test_id> --example
drata monitor retest <workspace_id> <test_id>              # 409 and 429 print the server's message, never retried
drata monitor exclusion create <workspace_id> <test_id> --example
drata monitor exclusion remove <workspace_id> <test_id> --exclusion-id 10 11
drata monitor codebase-exclusion list <workspace_id> <test_id>
drata monitor codebase-exclusion create <workspace_id> <test_id> --example
drata monitor codebase-exclusion remove <workspace_id> <test_id> --exclusion-id 10
drata monitor ticket list <workspace_id> <test_id>
drata monitor ticket link <workspace_id> <test_id> --ticket-url https://example.atlassian.net/browse/SEC-1
drata monitor ticket unlink <workspace_id> <test_id> <ticket_id>
drata monitor ticket create <workspace_id> <test_id> --example
```

Exclusion `create` and `remove` print every per-item result row and exit 1 if any row failed.

### Audits

```
drata audit list <workspace_id>
drata audit get <workspace_id> <audit_id>
drata audit get <workspace_id> <audit_id> --expand dcfSuggestionsSummary
```

#### Audit request DCF suggestions (beta)

AI-suggested controls (DCFs) for audit requests. `generate` answers 202 and is not
polled: read progress with `audit get --expand dcfSuggestionsSummary`. `generate`
and `accept` print every per-request row and exit 1 if any row failed.

```
drata audit request dcf-suggestion generate <workspace_id> <audit_id> --request-id 1 2 3
drata audit request dcf-suggestion accept <workspace_id> <audit_id> --request-id 1 2 --confidence-level high
drata audit request dcf-suggestion accept <workspace_id> <audit_id> --all-requests
drata audit request dcf-suggestion list <workspace_id> <audit_id> <request_id> --status pending
drata audit request dcf-suggestion get <workspace_id> <audit_id> <request_id> <suggestion_id>
drata audit request dcf-suggestion run-action <workspace_id> <audit_id> <request_id> <suggestion_id> --action accept
```

### Events

```
drata event list
drata event get <event_id>
drata event list --all       # stream all pages as NDJSON
drata event list --limit 100 # stop after 100 events
```

## Request body skeletons (--example)

Every `create` command supports `--example` to print a spec-derived JSON skeleton
and exit without making any API call. Useful for understanding required fields:

```
drata vendor create --example
drata risk create 1 --example
drata register create --example
drata asset create --example
```

Example output for `vendor create --example`:

```json
{
  "name": "Example Vendor",
  "category": "SECURITY",
  "risk": "LOW",
  "status": "ACTIVE",
  "url": "https://vendor.example.com",
  "notes": "Free-form notes about this vendor"
}
```

## From-file input (--from-file)

Body-bearing verbs accept `--from-file <path|->` to read the request body from a
JSON or YAML file (or stdin with `-`). CLI flags override file fields.

## Multipart uploads (--file)

Operations that upload documents accept `--file <path>`. Supported on vendor upload,
risk upload (one or more files), device upload (requires `--type`), policy create,
evidence create/update. Each endpoint sends the spec's required scalar fields
alongside the file (e.g. device documents send `type`, risk documents send the
files under the `files` part).

Via `raw`, multipart is available for both POST and PUT operations:

```bash
# device document upload: file part `file` plus a `type` scalar field
drata raw POST /devices/123/documents --file ./cert.pdf --field type=ANTIVIRUS_EVIDENCE --allow-writes --yes
# evidence update is PUT multipart; --file-field overrides the part name when needed
drata raw PUT /workspaces/1/evidence-library/9 --file ./report.pdf --allow-writes --yes
# array-valued file part (e.g. risk documents use `files`)
drata raw POST /risk-registers/1/risks/2/documents --file-field files --file ./a.pdf ./b.pdf --allow-writes --yes
```

## Expanding sub-collections (--expand)

Many list and get commands accept `--expand` to fetch sub-resources inline:

```
drata vendor list --expand questionnaires contacts
drata control get <workspace_id> <control_id> --expand requirements
```

## Streaming all pages (--all)

For large result sets use `--all` to stream pages as NDJSON (one JSON object per line).
Suitable for piping to `jq` or further processing:

```
drata vendor list --all | jq '.name'
drata personnel list --all > personnel.ndjson
drata event list --all | grep '"type":"USER_LOGIN"'
```

A list without `--all` or `--limit` still fetches every page, which on a large
tenant means thousands of requests. `asset list` and `event list` take
`--limit N` to stop once N items are in hand. Page requests are paced under
Drata's per-IP rate limit, and a 429 waits out the server's `Retry-After`
(up to `MAX_RETRY_DELAY_SECS` in `src/client.rs`) with a notice on stderr.

## The raw namespace

`raw` provides generic access to every API operation not yet curated, or for
advanced use cases:

```
drata raw GET /vendors
drata raw GET /vendors --query page=1 size=20
drata raw GET /workspaces/{workspaceId}/audits/{auditId}/requests
drata raw POST /vendors --data '{"name":"Acme"}' --allow-writes
drata raw POST /vendors --data @body.json --allow-writes --yes
drata raw POST /vendors --data - --allow-writes --yes  < body.json
drata raw --example POST /risk-registers  # print skeleton from spec
```

Non-GET `raw` calls are subject to the same write guardrail and confirmation prompt
as curated commands.

## SafeBase

Everything under `drata safebase` calls the SafeBase Trust Center REST API
(`app.safebase.io`, single region), with its own credential. The Drata commands
above are unchanged.

- Needs a SafeBase plan with API access and an API key
- A SafeBase-only user (no Drata key) can run every `drata safebase` command
- `drata auth` and `drata whoami` print the Drata lines first, then an indented `safebase:` block

### Credential

```
drata safebase login --api-key YOUR_SAFEBASE_KEY                  # read-only
drata safebase login --api-key YOUR_SAFEBASE_KEY --allow-writes   # write-enabled
drata safebase logout
```

- `login` replaces only the profile's `safebase` block; `drata login` replaces only the `drata` block
- `logout` removes only its own block; a profile left with no block is removed
- Key precedence: `--api-key`, then `SAFEBASE_API_KEY`, then the profile's `safebase` block, then an error naming the backend and profile
- A profile with no `safebase` block never falls back to the Drata key
- `DRATA_PROFILE` / `--profile` select the profile for both backends

### Write gate

- Per backend: a non-GET request needs `--allow-writes` or the profile block's `allow-writes: true`
- The profile flag counts only when the key came from that profile. A key from `--api-key` or `SAFEBASE_API_KEY` is read-only unless `--allow-writes` is passed
- Non-GET calls also prompt for confirmation unless `--yes` is set
- SafeBase can mint keys without write scopes; it enforces them server side (a 403 naming the missing scope)
- `--region` is Drata-only: passing it to a `safebase` command is an error, and `DRATA_REGION` is ignored
- On `auth` / `whoami`, `--api-key` and `--allow-writes` apply to the Drata lines only

### credentials.json shape

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

- Both blocks are optional; a profile needs at least one
- An older flat profile (`api-key`, `region`, `allow-writes`) reads as a `drata` block, in memory. A profile mixing flat fields and a nested block is an error naming the profile
- A read never rewrites the file. The first `login` or `logout` that changes the file's shape copies the original to `credentials.json.pre-safebase` (mode 0600)
- Rollback to a binary that only knows the flat shape: `mv ~/.config/drata/credentials.json.pre-safebase ~/.config/drata/credentials.json`

### Raw

Any of the SafeBase operations is reachable with `raw`, same flags as `drata raw`
(including PATCH, through the same write guard and confirm prompt):

```
drata safebase raw GET /organizations
drata safebase raw PATCH /document/<id> --data '{"name":"SOC 2 report"}' --allow-writes
drata safebase raw --example POST /documents/upload     # skeleton from the SafeBase spec, no key needed
drata safebase raw POST /questionnaires --file ./qnr.xlsx --file-field qnr --field accountId=<uuid> --field productId=default --allow-writes --yes
```

### Curated reads

```
drata safebase account list [--all]
drata safebase account get <id>
drata safebase account url <id>
drata safebase member list --email <e> [--sfdc-account-id <id>]
drata safebase document list [--all] [--query <q>]
drata safebase document get <id> [--external]
drata safebase document types
drata safebase request list [--status pending|approved|declined]
drata safebase kb search [<query>] [--all]
```

- Lists fetch every page and print rows; `--all` streams NDJSON instead
- A page that is malformed, empty before the end, repeated, or past the count is an error, never a silent truncation
- A failure mid-stream under `--all` exits non-zero and names the page; lines already written stay written
- `account url` prints the bare URL, for `$(...)`

### Curated writes

```
drata safebase request approve <request-id> --account-id <uuid>
drata safebase request decline <request-id> [--message <text> | --notify]
drata safebase member add <account-id> --email <e> --first-name <f> --last-name <l> [--message <text> | --notify]
drata safebase member remove <account-id> --email <e>...
drata safebase kb add --data <json>
drata safebase kb update <id> --data <json>
drata safebase document upload <file> --name <n> [--type <t>] [--replace <document-id>] [--external-id <uuid>]
```

- Email is opt-in: `--message <text>` sends SafeBase's email with the text, `--notify` sends it with none, neither sends no email
- `member add` takes one member per call; bulk adds go through `raw`
- Approving a request that also creates an account in the same call is raw-only
- Everything without a curated verb (organization, products, tags, webpages, questionnaires, account writes) is reachable through `raw`

### Document upload

`document upload` runs three steps: request an upload, PUT the bytes to the returned signed URL, then create (or replace) the document.

- Checked locally before any request: file size within the spec's limit, and a MIME type in the spec's enum
- The signed-URL request carries no SafeBase key, no `Authorization`, and follows no redirects
- `--external-id` must be a UUID; default is a fresh v4. `document get <id> --external` looks a document up by it
- `--replace` keeps the document's id; name, external id, checksum and size take the new file's values. Type, expiry, `aiOptOut` and products survive; tags are untested
- Outcomes:
  - upload request or byte send fails: no document exists; non-zero exit, `fileId` printed (SafeBase has no op to delete an upload)
  - the create step returns an HTTP error: no document exists; non-zero exit
  - the create step times out or the connection drops: the CLI looks the document up once by external id. If found with the same size and checksum, it prints the document, warns, and exits 0
  - not found, or found with a different size or checksum: non-zero exit with `outcome unknown`, `fileId` and `externalId` printed
- The create step is never retried automatically

## Tags deferred to raw-only

The following tags are intentionally left as raw-only (reachable via `drata raw`):

- **Audit Requests** - only the beta DCF suggestion ops are curated (`audit request dcf-suggestion`); the other audit request ops stay raw
- **Procurement Connection Mappings** - narrow vendor-procurement integration
- **Custom Data Records** - complex session-based upload workflow
- **Tasks** - workspace-scoped task management; sufficient via raw for scripting
- **HRIS User Identities** - connections-based, specialized use case
- **Control Notes / Owners** - sub-resource management, raw covers these
- **Risk Documents / Notes / Library** - sub-resource management, raw covers these
- **Device Documents** - document sub-resources, raw covers get/list/delete
- **Background Checks** - single POST endpoint
- **Vendor Types** - vendor sub-resources via raw
- **User Documents / Assigned Policies** - user sub-resources via raw
- **Custom Connections** - connection management via raw; only `custom-connection resource-attribute` is curated
- **Framework requirement create/update** - plus legacy requirement endpoints

Every operation is reachable (curated + raw); coverage is verified by the CI
test suite in `tests/coverage.rs`.

## Live verification

The `drata verify` subcommand implements a disposable create-verify-delete pass
against a live tenant using `zzz-clitest-<uuid>` object names. This is a manual
step intended to record wiremock fixtures; it requires a write-enabled credential
and must NOT be run in CI.

## Spec coverage

The committed specs, `spec/drata-openapi-v2.json` and `spec/safebase-openapi-v1.json`,
anchor `--example` skeletons and the coverage test. Every operation is reachable via raw; `cargo test --test coverage -- --nocapture`
prints the current curated coverage.

## Development

```
otto ci       # full CI: lint + bloat + clippy + fmt + test
otto test     # tests only
otto cov      # test with llvm-cov coverage
otto build    # release binary
```

CI requires a green `otto ci` before any commit. The `tests/coverage.rs` integration
test asserts every operation is reachable and enforces a curated-coverage floor,
counted from the command table in `tests/commands/table.rs`: one row per curated
operation, each dispatched against wiremock by `tests/commands.rs`. A new curated
command needs a row, or `tests/commands.rs` fails.
