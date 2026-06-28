# drata-cli

Drata compliance management CLI. Hand-typed, typed-safe Rust port of the upstream
TypeScript CLI, with curated commands for the common surface and a generic `raw`
namespace for full coverage of all 167 API operations.

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
```

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
```

### Evidence Library

```
drata-cli evidence list <workspace_id>
drata-cli evidence get <workspace_id> <evidence_id>
drata-cli evidence create <workspace_id> --example
drata-cli evidence create <workspace_id> --name "SOC 2 Report" --renewal-schedule-type ONE_YEAR --file ./report.pdf
drata-cli evidence update <workspace_id> <evidence_id> --renewal-schedule-type SIX_MONTHS
drata-cli evidence remove <workspace_id> <evidence_id>
drata-cli evidence get-version <workspace_id> <evidence_id> <version_id>
```

### Frameworks

```
drata-cli framework list <workspace_id>
drata-cli framework create <workspace_id> --example
drata-cli framework create <workspace_id> --name "SOC 2" --short-name "SOC2" --description "..."
drata-cli framework update <workspace_id> <framework_id> --name "Updated"
drata-cli framework requirements <workspace_id> <framework_id>
```

### Assets

```
drata-cli asset list
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
```

### Audits

```
drata-cli audit list <workspace_id>
drata-cli audit get <workspace_id> <audit_id>
```

### Events

```
drata-cli event list
drata-cli event get <event_id>
drata-cli event list --all       # stream all pages as NDJSON
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

## The raw namespace

`raw` provides generic access to all 167 API operations not yet curated, or for
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

- **Audit Requests** - requires knowing both workspaceId and auditId in advance
- **Procurement Connection Mappings** - narrow vendor-procurement integration
- **Custom Data Records** - complex session-based upload workflow
- **Tasks** - workspace-scoped task management; sufficient via raw for scripting
- **HRIS User Identities** - connections-based, specialized use case
- **Control Notes / Owners / Library** - sub-resource management, raw covers these
- **Risk Documents / Notes / Library** - sub-resource management, raw covers these
- **Device Documents** - document sub-resources, raw covers get/list/delete
- **Background Checks** - single POST endpoint
- **Vendor Documents / Security Reviews / Types** - vendor sub-resources via raw
- **User Documents / Assigned Policies** - user sub-resources via raw
- **Custom Connections** - connection management via raw
- **Framework requirement create/update** - plus legacy requirement endpoints

All 167 operations are reachable (curated + raw); coverage is verified by the CI
test suite in `tests/coverage.rs`.

## Live verification

The `drata verify` subcommand implements a disposable create-verify-delete pass
against a real tenant using `zzz-clitest-<uuid>` object names. This is a manual
step intended to record wiremock fixtures; it requires a write-enabled credential
and must NOT be run in CI.

## Spec coverage

The committed spec at `spec/drata-openapi-v2.json` (110 paths, 167 operations,
35 operation-level tags) anchors `--example` skeletons and the coverage test.
Current curated coverage: ~42% (70/167 operations); all 167 reachable via raw.

## Development

```
otto ci       # full CI: lint + bloat + clippy + fmt + test
otto test     # tests only
otto cov      # test with llvm-cov coverage
otto build    # release binary
```

CI requires a green `otto ci` before any commit. The `tests/coverage.rs` integration
test asserts all 167 operations are reachable and enforces a curated-coverage floor.
