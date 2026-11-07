# drata-cli

Drata compliance management CLI. Hand-typed, typed-safe Rust port of the upstream
TypeScript CLI, with curated commands for the common surface and a generic `raw`
namespace for full coverage of every API operation.

## Installation

```
cargo install --path .
```

The binary is named `drata-cli`. After install it is on `$PATH` via `~/.cargo/bin`.

## Authentication

Credentials are stored in `~/.config/drata/credentials.json` as named profiles.
Each profile holds an API key, region, and an explicit write-enable flag.

### Save a credential profile

```
# Read-only profile (default)
drata-cli login --api-key YOUR_API_KEY --region us

# Write-enabled profile (required for any POST/PUT/DELETE)
drata-cli login --api-key YOUR_API_KEY --region us --allow-writes --profile write

# EU tenant
drata-cli login --api-key YOUR_API_KEY --region eu
```

### Verify your credential

```
drata-cli whoami
drata-cli auth     # shows diagnostics even without a configured key
```

### Credential precedence

1. `--api-key` CLI flag
2. `DRATA_API_KEY` environment variable
3. Profile from `credentials.json` (selected by `--profile` or `DRATA_PROFILE`, defaulting to `default`)

Regions: `us` (default), `eu`, `apac`.

### Write gate

Non-GET requests fail unless the resolved credential has `allow_writes: true`.
This prevents accidental mutation from a stray `DRATA_API_KEY` environment variable.
All mutating operations (POST/PUT/DELETE) also prompt for confirmation unless `--yes` is set.

```
# Writes require an allow-writes credential
drata-cli --profile write vendor create --name "Acme Corp"

# Or pass --allow-writes at invocation time
drata-cli --api-key $DRATA_API_KEY --allow-writes vendor create --name "Acme Corp"

# Skip confirmation for scripting
drata-cli --profile write --yes vendor remove 42
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
drata-cli vendor list
drata-cli vendor list acme corp          # filter by name pattern
drata-cli vendor get 123
drata-cli vendor create --example        # print a JSON skeleton and exit
drata-cli vendor create --name "Acme" --category SECURITY --risk LOW
drata-cli vendor update 123 --risk HIGH
drata-cli vendor remove 123
drata-cli vendor upload 123 --file ./contract.pdf
drata-cli vendor questionnaire list 123
drata-cli vendor questionnaire send 123 --email sec@vendor.com --questionnaire-id 5 --security-review-id 7 --email-content "Please complete"
drata-cli vendor questionnaire get 123 5
drata-cli vendor questionnaire questions 123 5
drata-cli vendor search                  # all pages as rows
drata-cli vendor search --facet <name>   # one page, whole {data, pagination, facets} envelope
drata-cli vendor trust-overview 123
```

#### Vendor security reviews

```
drata-cli vendor security-review list 123
drata-cli vendor security-review create 123 --example
drata-cli vendor security-review get 123 <review_id>
drata-cli vendor security-review update 123 <review_id> --example
drata-cli vendor security-review actions 123 <review_id>
drata-cli vendor security-review run-action 123 <review_id> --action <action>
drata-cli vendor security-review create-with-file 123 --file ./soc2.pdf --title "SOC 2" --review-deadline-at 2027-01-01 --status IN_PROGRESS --type SOC_REPORT --document-type SOC_DOCUMENT
drata-cli vendor security-review upload-questionnaire 123 --file ./answers.xlsx
drata-cli vendor security-review questionnaires 123 <review_id>
drata-cli vendor security-review note list 123 <review_id>              # also get | create | update | remove
drata-cli vendor security-review observation list 123 <review_id>       # also get | create | update | remove
drata-cli vendor security-review assessment list 123 <review_id>        # also get | stats
```

The agent workflow verbs start background jobs and print the 202 body; nothing
polls. Read progress with `workflow` and `summary`:

```
drata-cli vendor security-review start-agent 123 <review_id>
drata-cli vendor security-review confirm-criteria 123 <review_id> --example
drata-cli vendor security-review start-assessment 123 <review_id>
drata-cli vendor security-review stop-assessment 123 <review_id>
drata-cli vendor security-review collect-public-documents 123 <review_id>
drata-cli vendor security-review access-request-requirements 123 <review_id>
drata-cli vendor security-review request-trust-center-access 123 <review_id> --example
drata-cli vendor security-review provide-trust-center-access-link 123 <review_id> --data @link.json
drata-cli vendor security-review skip-trust-center-access-request 123 <review_id>
drata-cli vendor security-review add-documents 123 <review_id> --file ./a.pdf ./b.pdf
drata-cli vendor security-review upload-questionnaire-to-review 123 <review_id> --file ./answers.xlsx
drata-cli vendor security-review generate-follow-up-questionnaire 123 <review_id>
drata-cli vendor security-review follow-up-questionnaire 123 <review_id>
drata-cli vendor security-review send-follow-up-questionnaire 123 <review_id> --example
drata-cli vendor security-review residual-risk 123 <review_id>
drata-cli vendor security-review finalize 123 <review_id> --example
drata-cli vendor security-review summary 123 <review_id>
drata-cli vendor security-review workflow 123 <review_id>
```

#### Vendor criteria, templates and risk levels

```
drata-cli vendor-criteria list
drata-cli vendor-criteria-creation list <workspace_id>
drata-cli vendor-questionnaire-template list
drata-cli vendor-questionnaire-template questions <questionnaire_id>
drata-cli vendor-risk-level list
drata-cli vendor-security-review list      # security reviews across all vendors
```

### Risk Registers

Required to discover `riskRegisterId` before using `risk` commands.

```
drata-cli register list
drata-cli register get 1
drata-cli register create --example
drata-cli register create --name "Main Register" --description "Primary risk register"
drata-cli register update 1 --name "Updated Name"
drata-cli register remove 1
```

### Risks

```
drata-cli risk list <register_id>
drata-cli risk get <register_id> <risk_id>
drata-cli risk create <register_id> --example
drata-cli risk create <register_id> --title "Password Policies" --description "..." --treatment-plan MITIGATE --impact 4 --likelihood 3
drata-cli risk update <register_id> <risk_id> --status CLOSED
drata-cli risk insights <register_id>
drata-cli risk upload <register_id> <risk_id> --file ./evidence.pdf            # one or more --file
drata-cli risk upload <register_id> <risk_id> --file ./a.pdf ./b.pdf
drata-cli risk remove <register_id> <risk_id>
drata-cli risk search                                  # all pages as rows; --facet <name> for facet counts
drata-cli risk search --risk-register-id 3 9           # only these registers (--register-id <id> searches one)
drata-cli risk ticket list <register_id> <risk_id>
drata-cli risk ticket link <register_id> <risk_id> --ticket-url https://example.atlassian.net/browse/SEC-1
drata-cli risk ticket unlink <register_id> <risk_id> <ticket_id>
drata-cli risk ticket create <register_id> <risk_id> --example
```

### Controls

```
drata-cli control list <workspace_id>
drata-cli control get <workspace_id> <control_id>
drata-cli control create <workspace_id> --example
drata-cli control create <workspace_id> --name "Access Control" --description "..." --code "AC-1"
drata-cli control update <workspace_id> <control_id> --name "Updated"
drata-cli control requirements <workspace_id> <control_id>
drata-cli control compare <workspace_id> --control-ids 1 2 3
drata-cli control ticket list <workspace_id> <control_id>
drata-cli control ticket link <workspace_id> <control_id> --ticket-url https://example.atlassian.net/browse/SEC-1
drata-cli control ticket unlink <workspace_id> <control_id> <ticket_id>
```

Controls have no `ticket create`: the API has no `action-create` op for them.

### Devices

```
drata-cli device list
drata-cli device get <device_id>
drata-cli device for-personnel <personnel_id>
drata-cli device apps <device_id>
drata-cli device upload <device_id> --file ./cert.pdf --type ANTIVIRUS_EVIDENCE
```

### Personnel

```
drata-cli personnel list
drata-cli personnel get <personnel_id>
drata-cli personnel update <personnel_id> --employment-status CURRENT_EMPLOYEE
drata-cli personnel search                          # all pages as rows; --facet <name> for facet counts
drata-cli personnel group list <workspace_id>       # scoped personnel groups
drata-cli personnel group counts <workspace_id>
drata-cli personnel group add <workspace_id> <group_id>
drata-cli personnel group replace <workspace_id> --group-id 1 2
drata-cli personnel group remove <workspace_id> <group_id>
drata-cli group list
```

### Policies

```
drata-cli policy list
drata-cli policy get <policy_id>
drata-cli policy create --example
drata-cli policy create --name "Security Policy" --owner-id 42 --source-type UPLOADED --description "..." --renewal-date 2027-01-01 --file ./policy.pdf
drata-cli policy update <policy_id> --name "Updated Policy"
drata-cli policy actions <policy_id>
drata-cli policy versions <policy_id>
drata-cli policy version <policy_id> <version_id>
drata-cli policy version create <policy_id> --example
drata-cli policy version create <policy_id> --source-type UPLOADED --owner-id 42 --material-change false --renewal-date 2027-01-01 --file ./policy-v2.pdf
drata-cli policy version replace-file <policy_id> <version_id> --file ./policy-v2.pdf --language fr-FR
drata-cli policy-language settings
drata-cli policy-language list <policy_id> <version_id> --status published draft
drata-cli policy-language versions --policy-id 7 --expand policy
drata-cli policy-language get <id> --expand downloadPdfUrl
```

### Evidence Library

```
drata-cli evidence list <workspace_id>
drata-cli evidence get <workspace_id> <evidence_id>
drata-cli evidence create <workspace_id> --example
drata-cli evidence create <workspace_id> --name "SOC 2 Report" --renewal-schedule-type ONE_YEAR --filed-at 2026-11-01 --owner-id 42 --file ./report.pdf
drata-cli evidence update <workspace_id> <evidence_id> --renewal-schedule-type SIX_MONTHS
drata-cli evidence remove <workspace_id> <evidence_id>
drata-cli evidence get-version <workspace_id> <evidence_id> <version_id>
```

### Evidence Items

The spec's Evidence tag manages Evidence Items, a different resource from the
Evidence Library above. `evidence-item remove` deletes the item and all its artifacts.

```
drata-cli evidence-item list <workspace_id>
drata-cli evidence-item get <workspace_id> <evidence_id>
drata-cli evidence-item create <workspace_id> --example
drata-cli evidence-item update <workspace_id> <evidence_id> --example
drata-cli evidence-item remove <workspace_id> <evidence_id>
drata-cli evidence-item upload <workspace_id> --file ./report.pdf      # prints the fileKey to pass to --file-key
drata-cli evidence-item artifact list <workspace_id> <evidence_id>
drata-cli evidence-item artifact update <workspace_id> <evidence_id> <artifact_id> --example
drata-cli evidence-item artifact remove <workspace_id> <evidence_id> <artifact_id>
drata-cli evidence-item artifact run-action <workspace_id> <evidence_id> <artifact_id> --example
```

### Upload URLs

```
drata-cli upload-url create --purpose <purpose> [--workspace <id>]
drata-cli upload-url create --purpose <purpose> --file ./report.pdf    # also PUTs the file; prints the objectKey
```

### Access Reviews

Only one review period may be active on the account at a time. `app users set`
replaces the application's full user list: users omitted from it are removed.

```
drata-cli access-review app list
drata-cli access-review app get <application_id>
drata-cli access-review app create --name "Okta" --reviewer-id 42
drata-cli access-review app update <application_id> --reviewer-id 42 43
drata-cli access-review app remove <application_id>
drata-cli access-review app users list <application_id>
drata-cli access-review app users set <application_id> --user IDENTITY:USERNAME:EMAIL --role IDENTITY:ROLE
drata-cli access-review period list --status active
drata-cli access-review period get <period_id>
drata-cli access-review period create --starting-date 2026-11-01T00:00:00Z --ending-date 2026-12-01T00:00:00Z --application APP_ID:REVIEWER_ID
drata-cli access-review period run-action <period_id> --action complete
drata-cli access-review period app get <period_id> <review_app_id>
drata-cli access-review period app update <period_id> <review_app_id> --reviewer-id 42
drata-cli access-review period app run-action <period_id> <review_app_id> --action complete
drata-cli access-review period app users <period_id> <review_app_id>
drata-cli access-review period evidence-package <period_id>            # prints the signed URL, never downloads
```

### Standard Connections

`connection aws | azure | gcp | gitlab` share one verb set. Bodies carry cloud
credentials, so `create` and `update` take `--data @file` or `--data -` only and
reject inline JSON before any request. `--example` prints the provider's body skeleton.
`<connection_id>` is the numeric id, or the provider's natural key: `roleArn:<role ARN>`
(AWS), `subscriptionId:<subscription ID>` (Azure), `projectId:<project ID>` (GCP).
GitLab takes the numeric id only. Pass an ARN unencoded; the CLI encodes its `/`.

```
drata-cli connection aws list
drata-cli connection aws get <connection_id>
drata-cli connection aws get roleArn:arn:aws:iam::123456789012:role/DrataAuditRole
drata-cli connection aws create --example
drata-cli connection aws create --data @aws-connection.json
drata-cli connection aws update <connection_id> --data @aws-connection.json
drata-cli connection aws remove <connection_id>
```

### Custom Connections

```
drata-cli custom-connection resource-attribute <workspace_id> <connection_id> <resource_id>
drata-cli custom-field list
drata-cli custom-field get <custom_field_id>
drata-cli custom-field create --example
```

### Identities

`infrastructure-identity` and `version-control-identity` share one verb set.

```
drata-cli infrastructure-identity list <workspace_id>
drata-cli infrastructure-identity get <workspace_id> <identity_id>
drata-cli infrastructure-identity actions <workspace_id> <identity_id>
drata-cli infrastructure-identity run-action <workspace_id> <identity_id> --example
drata-cli version-control-identity list <workspace_id>      # same verbs: get | actions | run-action
```

### Compliance Check Exclusions

Per-item results print every row; the command exits 1 if any row failed.

```
drata-cli compliance-check-exclusion list <workspace_id>
drata-cli compliance-check-exclusion create <workspace_id> --example
drata-cli compliance-check-exclusion remove <workspace_id> --target-id 1 2 --compliance-check <check>
```

### Frameworks

```
drata-cli framework list <workspace_id>
drata-cli framework create <workspace_id> --example
drata-cli framework create <workspace_id> --name "SOC 2" --short-name "SOC2" --description "..."
drata-cli framework update <workspace_id> <framework_id> --name "Updated"
drata-cli framework requirements <workspace_id> <framework_id>
drata-cli framework requirement controls <workspace_id> <framework_id> <requirement_id>
```

### Assets

```
drata-cli asset list
drata-cli asset list --limit 50     # stop after 50 assets
drata-cli asset get <asset_id>
drata-cli asset create --example
drata-cli asset create --name "Server" --description "..." --asset-type PHYSICAL --asset-class-types HARDWARE COMPUTE --owner-id 42
drata-cli asset update <asset_id> --asset-type VIRTUAL
drata-cli asset remove <asset_id>
```

### Company

```
drata-cli company get
```

### Workspaces

```
drata-cli workspace list
```

### Users and Roles

```
drata-cli user list
drata-cli user get <user_id>
drata-cli user roles
drata-cli user role <role_id>
drata-cli user role-users <role_id>
```

### Monitoring Tests

```
drata-cli monitor list <workspace_id>
drata-cli monitor get <workspace_id> <test_id>
drata-cli monitor update <workspace_id> <test_id> --enabled false
drata-cli monitor exclusions <workspace_id> <test_id>
drata-cli monitor failures <workspace_id> <test_id>
drata-cli monitor passes <workspace_id> <test_id>
drata-cli monitor create <workspace_id> --example
drata-cli monitor validate-recipe <workspace_id> --example     # exits 1 when the recipe is invalid
drata-cli monitor recipe <workspace_id> <test_id> --example
drata-cli monitor retest <workspace_id> <test_id>              # 409 and 429 print the server's message, never retried
drata-cli monitor exclusion create <workspace_id> <test_id> --example
drata-cli monitor exclusion remove <workspace_id> <test_id> --exclusion-id 10 11
drata-cli monitor codebase-exclusion list <workspace_id> <test_id>
drata-cli monitor codebase-exclusion create <workspace_id> <test_id> --example
drata-cli monitor codebase-exclusion remove <workspace_id> <test_id> --exclusion-id 10
drata-cli monitor ticket list <workspace_id> <test_id>
drata-cli monitor ticket link <workspace_id> <test_id> --ticket-url https://example.atlassian.net/browse/SEC-1
drata-cli monitor ticket unlink <workspace_id> <test_id> <ticket_id>
drata-cli monitor ticket create <workspace_id> <test_id> --example
```

Exclusion `create` and `remove` print every per-item result row and exit 1 if any row failed.

### Audits

```
drata-cli audit list <workspace_id>
drata-cli audit get <workspace_id> <audit_id>
drata-cli audit get <workspace_id> <audit_id> --expand dcfSuggestionsSummary
```

#### Audit request DCF suggestions (beta)

AI-suggested controls (DCFs) for audit requests. `generate` answers 202 and is not
polled: read progress with `audit get --expand dcfSuggestionsSummary`. `generate`
and `accept` print every per-request row and exit 1 if any row failed.

```
drata-cli audit request dcf-suggestion generate <workspace_id> <audit_id> --request-id 1 2 3
drata-cli audit request dcf-suggestion accept <workspace_id> <audit_id> --request-id 1 2 --confidence-level high
drata-cli audit request dcf-suggestion accept <workspace_id> <audit_id> --all-requests
drata-cli audit request dcf-suggestion list <workspace_id> <audit_id> <request_id> --status pending
drata-cli audit request dcf-suggestion get <workspace_id> <audit_id> <request_id> <suggestion_id>
drata-cli audit request dcf-suggestion run-action <workspace_id> <audit_id> <request_id> <suggestion_id> --action accept
```

### Events

```
drata-cli event list
drata-cli event get <event_id>
drata-cli event list --all       # stream all pages as NDJSON
drata-cli event list --limit 100 # stop after 100 events
```

## Request body skeletons (--example)

Every `create` command supports `--example` to print a spec-derived JSON skeleton
and exit without making any API call. Useful for understanding required fields:

```
drata-cli vendor create --example
drata-cli risk create 1 --example
drata-cli register create --example
drata-cli asset create --example
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
drata-cli raw POST /devices/123/documents --file ./cert.pdf --field type=ANTIVIRUS_EVIDENCE --allow-writes --yes
# evidence update is PUT multipart; --file-field overrides the part name when needed
drata-cli raw PUT /workspaces/1/evidence-library/9 --file ./report.pdf --allow-writes --yes
# array-valued file part (e.g. risk documents use `files`)
drata-cli raw POST /risk-registers/1/risks/2/documents --file-field files --file ./a.pdf ./b.pdf --allow-writes --yes
```

## Expanding sub-collections (--expand)

Many list and get commands accept `--expand` to fetch sub-resources inline:

```
drata-cli vendor list --expand questionnaires contacts
drata-cli control get <workspace_id> <control_id> --expand requirements
```

## Streaming all pages (--all)

For large result sets use `--all` to stream pages as NDJSON (one JSON object per line).
Suitable for piping to `jq` or further processing:

```
drata-cli vendor list --all | jq '.name'
drata-cli personnel list --all > personnel.ndjson
drata-cli event list --all | grep '"type":"USER_LOGIN"'
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
drata-cli raw GET /vendors
drata-cli raw GET /vendors --query page=1 size=20
drata-cli raw GET /workspaces/{workspaceId}/audits/{auditId}/requests
drata-cli raw POST /vendors --data '{"name":"Acme"}' --allow-writes
drata-cli raw POST /vendors --data @body.json --allow-writes --yes
drata-cli raw POST /vendors --data - --allow-writes --yes  < body.json
drata-cli raw --example POST /risk-registers  # print skeleton from spec
```

Non-GET `raw` calls are subject to the same write guardrail and confirmation prompt
as curated commands.

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
against a real tenant using `zzz-clitest-<uuid>` object names. This is a manual
step intended to record wiremock fixtures; it requires a write-enabled credential
and must NOT be run in CI.

## Spec coverage

The committed spec at `spec/drata-openapi-v2.json` anchors `--example` skeletons
and the coverage test. Every operation is reachable via raw; `cargo test --test coverage -- --nocapture`
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
