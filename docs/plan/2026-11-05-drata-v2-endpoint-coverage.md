# Design Document: Curate the Drata v2 endpoints added since June

**Author:** Scott Idler
**Date:** 2026-11-05
**Status:** In Review
**Review Passes Completed:** 5/5

## Summary

Drata's public API v2 grew from 167 to 308 operations between the June spec drata-cli was built on and the spec refreshed on 2026-11-05 (commit d788b46). This plan curates all 144 added operations as typed `drata` commands. It also brings the 83 operations drata-cli already curates up to the new schemas, and mirrors the 3 removed Control Library operations.

## Problem Statement

### Background

- drata-cli embeds `spec/drata-openapi-v2.json`, now at 308 ops. Every op is reachable through `drata raw`, and curated commands cover the common surface
- Delta from the June spec (`git show 165c6cf:spec/drata-openapi-v2.json`) to d788b46:
  - 144 ops added across 26 tags, 13 of them new tags. 40 are marked 🧪 beta: Vendor Security Reviews 30, Audit Requests 5, scoped personnel groups 5
  - 3 ops removed, all under the now-gone Control Library tag: `GET /control-library`, `GET /control-library/{templateId}`, `POST /control-library/action-import`
  - 164 ops in both. Every change to them is additive: no param, body field, response field or enum value was removed or renamed. 161 gained an `oauth2` security entry next to `bearer`
  - schemas 434 -> 747
- Ops marked 💎 (feature-gated) return 402 without the matching Drata plan feature: all 31 Vendor Security Reviews ops, all 16 Access Reviews ops, Risk Tickets, Custom Field Definitions, some vendor-adjacent and monitoring-test ops

### Problem

- 144 new ops are reachable only through `drata raw`: JSON hand-built, no curated `--example`, no table output
- Two curated commands no longer match the spec:
  - `control create`: `ControlCreateRequestPublicV2Dto.required` grew from `[name, description, code]` to add `externalEvidenceMetadata` and `base64Files` (JSON-string fields). The handler sends neither (`src/resources/control.rs:160-167`)
  - `vendor security-review create-with-file`: `documentType` is now required; the CLI flag is optional (`src/cli.rs:530`, `src/resources/vendor/security.rs:553`)
- `evidence create` (Evidence Library) does not meet the spec: `filedAt` is required for any artifact source, and a `fileKey` source also requires `renewalScheduleType` and `ownerId`. `EvidenceAction::Create` has no `--filed-at` or `--owner-id` (`src/cli.rs` `EvidenceAction`)
- Curated commands lack flags for new optional inputs:
  - `GET /vendors`: `statuses[]`, `categories[]`, `types[]`, `vendorTypeIds[]`, `vendorUserIds[]`, `isSubProcessor`, `actionRequired`, `nextReviewDeadlineStatus[]`
  - `GET /vendors/{vendorId}/security-reviews`: `createdAtFrom`, `createdAtTo`, `updatedAtFrom`, `updatedAtTo`
  - `GET /workspaces/{workspaceId}/monitoring-tests`: `draft`, `name`. `GET /workspaces/{workspaceId}/controls`: `requiredApprovals`
  - bodies: `PUT /vendors/{id}` `vendorTypeId`; risk create/update `vendorId`; security-review update `updatedAt`; security-review run-action `decision`, `note`, `resetDecision` (the CLI sends only `{"action"}`, `src/resources/vendor/security.rs:467`); evidence-library create `fileKey`
  - `audit get` takes no `--expand`, so `dcfSuggestionsSummary` is unreachable (`src/resources/audit.rs:46`)
- The curated count is wrong and unchecked. `tests/coverage.rs` CURATED is a hand-written list of 70 ops matched on method + path only; the code reaches 83. The 10 security-review ops (`src/resources/vendor/security.rs:330-704`) and 3 document uploads (`vendor.rs:252`, `device.rs:106`, `risk.rs:244`) are missing, and nothing proves a listed op has a command. `README.md` still lists "Vendor Documents / Security Reviews" as raw-only
- Client gaps for some new ops:
  - the client retries every 429 up to 3 times, with or without `Retry-After` (`src/client.rs:163`, `:266-281`). `action-retest` returns 429 for a post-run cooldown and 409 while a run is in flight; neither should be retried
  - `POST /upload-urls` returns `{presignedUrl, objectKey, expiresAt, webUploadUrl}`. Every client request carries the Drata Bearer token and base URL, so nothing can PUT a file to the presigned S3 URL
  - `--data @file | -` parsing lives privately in `src/raw.rs` (`read_data`); curated `--data` flags parse inline JSON only (`src/resources/vendor/security.rs:336`)
  - `client.get_all` returns only `data[]` (`src/client.rs:489`). The new `*-search` ops also return `facets`

### Goals

- Every op added since June has a curated command (Scott, 2026-11-05: "add all of the new endpoints and update all of the schemas to the latest spec. anything that is removed should be mirrored on the cli side")
- Every curated command matches the new schemas: required fields enforced, new optional params and body fields as flags
- Removed ops leave no trace in code, tests, README or help
- The curated list is derived from commands that a test parses, dispatches and observes on the wire

### Non-Goals

- Curating the 81 ops that existed in June and are still raw-only (Tasks, Control Notes / Owners, Risk Notes / Library, Custom Data Records and others). Not in the ask. Parked: curate on request
- Drata OAuth2 client credentials. Parked, same as `docs/plan/2026-11-05-safebase-api.md`
- Polling 202 background jobs (VSR agent runs, DCF suggestions). Commands print the 202 body; status is read with `vendor security-review workflow` / `summary` and `audit get --expand dcfSuggestionsSummary`
- The `text/html` representation of `GET /policy-language-versions/{id}`. The client and `raw` send `Accept: application/json` (`src/client.rs:257`); the JSON representation is curated
- Typed structs for payloads. `serde_json::Value` stays the currency (`docs/design/2026-06-27-architecture.md`)
- Renaming any existing noun

## Proposed Solution

### Overview

- One phase per cluster of related tags, each its own commit with `otto ci` green. Appendix A assigns all 144 added ops to a phase
- Each new command follows the existing vertical (vendor is the template):
  1. `src/cli.rs`: `Commands` variant + `<Noun>Action` subcommand enum; nested groups get their own enum. Value enums for bodies get names that do not end in `Action`
  2. `src/resources/<noun>.rs` (children in `src/resources/<noun>/<child>.rs`): `handle` + `example_if_requested`
  3. `src/lib.rs`: dispatch arm in `run`, plus `example_if_requested` arm
  4. `src/output/table.rs`: a `pick_renderer` branch + `render_<noun>`, placed above any looser sniff it could collide with
  5. tests: unit tests in `src/resources/<noun>/tests.rs`; a renderer test feeding the op's spec response example through `pick_renderer`; one row per op in the command table (Architecture)
- Naming: a new noun is the spec's resource name slugged (`Access Reviews` -> `access-review`; the Evidence tag's resource is "Evidence Item" -> `evidence-item`); child resources nest under their parent (`control ticket`, `risk ticket`, `monitor ticket`); `/actions` endpoints use the existing `actions` / `run-action` verbs; `*-search` ops become a `search` verb
- Lists call `client.get_all` only when the op has a `cursor` param; GETs without one use `client.get`
- Beta ops are curated like the rest (Scott: "all"); their help text ends in `(beta)`
- Bodies: flags for the common fields, plus `--data <json|@file|->` for the full body through the shared reader. Standard Connections bodies carry cloud credentials, so those commands accept only `--data @file` or `--data -` and reject inline JSON before any request
- Per-item results inside a 2xx (DCF generate rows, exclusion actions): print every row; exit 1 if any row reports a failure status

### Architecture

- **Command table** (Phase 1): `tests/commands.rs` holds one row per curated op: `(method, path, argv, wiremock response)`. A test parses each `argv` with clap, dispatches it against wiremock with a write-enabled test credential and `--yes`, and asserts exactly one request with that method and a path matching the template. `CURATED` in `tests/coverage.rs` is derived from this table, so the coverage count is the number of commands proven to reach their op. A second test walks the clap tree (`Cli::command()`) and fails if any leaf subcommand, other than `raw`, `login`, `logout`, `whoami`, `auth` and `verify`, has no row, so a command cannot ship without proving its op
- **Transport** (Phase 2), consumed by later phases:
  - `src/client/presigned.rs`: `put_presigned(method, url, headers, bytes)` on a separate `reqwest::Client` with no credential and no redirects. The caller supplies every header: Drata passes only `Content-Type`, equal to the `contentType` sent to `/upload-urls`; SafeBase passes the headers its upload response returns. Errors print the raw body (S3 answers XML) and the URL with its query string removed, since the query carries the signature
  - retry policy: `send_inner` takes a `Retry429` value (`Default | Never`); existing call sites pass `Default`. `monitor retest` passes `Never` and treats 409 and 429 as distinct, unretried errors that print the server's message
  - `src/data.rs`: `read_data` moves out of `src/raw.rs`; `raw` and every curated `--data` flag use it. A `DataSource::FileOrStdinOnly` mode rejects inline JSON
- **Shared helpers**:
  - `src/resources/ticket.rs`, parameterized by parent path: `list | link | unlink` for control, risk and monitoring-test tickets; `create` (the `action-create` op) for risk and monitoring-test only. Controls have no `action-create` op
  - one identity implementation for Infrastructure and Version Control User Identities
  - one connection implementation for AWS, Azure, GCP, GitLab, with a per-provider `--example`
- **Search verbs**: by default fetch every page and print rows like a list. `--facet <name>...` fetches a single page with `facets[]` set and prints the whole `{data, pagination, facets}` envelope

### Data Model

No credential or config change. New Drata enums become clap `ValueEnum`s only where a curated flag takes them; free-text flags pass new values through unchanged.

### API Design

The curated surface, by phase. Exact ops per phase are in Appendix A.

- Phase 1, existing commands:
  - `drata vendor list` gains `--status`, `--category`, `--type`, `--vendor-type-id`, `--vendor-user-id`, `--sub-processor`, `--action-required`, `--review-deadline-status` (multi-value, space-separated)
  - `drata vendor security-review list` gains `--created-from`, `--created-to`, `--updated-from`, `--updated-to`
  - `drata monitor list` gains `--draft`, `--name`; `drata control list` gains `--required-approvals`
  - `drata vendor update --vendor-type-id`; `drata risk create|update --vendor-id`; `drata vendor security-review update --updated-at`
  - `drata vendor security-review run-action` gains `--decision`, `--note` (finalize) and `--reset-decision` (reopen)
  - `--document-type` becomes required on `create-with-file`
  - `drata evidence create` gains `--filed-at` (required with `--file` or `--file-key`), `--owner-id` and `--file-key`; clap `requires` enforces `--file-key` -> `--owner-id` + `--renewal-schedule-type` + `--filed-at`
  - `drata control create` sends `externalEvidenceMetadata` and `base64Files`. Flags `--external-evidence-metadata <json>` and `--base64-files <json>` default to `"[]"` (no evidence); `--example` shows both
  - `drata audit get --expand`
- Phase 2: `drata upload-url create --purpose <p> [--workspace <id>] [--file <path>]`. Without `--file`, prints the response. With `--file`, requests the URL with the file's name and `contentType` (from `mime_guess`), PUTs the bytes, and prints `objectKey` as the value to pass to `--file-key`
- Phase 3: `control ticket list | link | unlink`; `risk ticket` and `monitor ticket` add `create`; `risk remove`; `risk search`; `framework requirement controls`
- Phase 4: `vendor search`, `vendor trust-overview`, `vendor questionnaire questions`, `vendor-criteria list`, `vendor-criteria-creation list`, `vendor-questionnaire-template list | questions`, `vendor-risk-level list`, `vendor-security-review list` (cross-vendor), `vendor security-review note | observation | assessment`
- Phase 5: `vendor security-review` agent workflow verbs (Appendix A Phase 5)
- Phase 6: `monitor create | validate-recipe | recipe | retest`, `monitor exclusion`, `monitor codebase-exclusion`
- Phase 7: `personnel search`, `personnel group` (scoped personnel groups), `group list`, `compliance-check-exclusion`, `custom-field`, `custom-connection resource-attribute`
- Phase 8: `infrastructure-identity`, `version-control-identity`
- Phase 9: `policy version create` (multipart), `policy version replace-file`, `policy-language`
- Phase 10: `evidence-item` (list, get, create, update, remove, artifacts, `evidence-files` upload)
- Phase 11: `access-review app | period` with `users`, `actions`, `evidence-package`. `app users set` replaces the full user list (the spec: "users omitted from it are removed"); its confirm prompt says so
- Phase 12: `connection aws | azure | gcp | gitlab` (`list | get | create | update | remove`)
- Phase 13: `audit request dcf-suggestion` verbs

### Implementation Plan

Coverage numbers below are `curated/308` from `cargo test --test coverage -- --nocapture`, counted from the command table.

#### Phase 0: Prove the unverified server behaviors
**Model:** sonnet
- Zero code. Reads use the read-only key; writes are marked and use the write key
- Retest cooldown, with `curl` and the key (not `drata raw`, whose 429 retry hides the body), on a `zzz-clitest` custom test only. Retesting a production monitor is a side effect, so if no such test exists, skip; Phase 6 builds from the spec's documented 409/429:
  - wait until the test's `checkStatus` is not `TESTING`, then `POST .../action-retest` (write) twice; record the first response and the second (expected 429 cooldown) body and headers
- Presigned upload (write): `POST /upload-urls` with `purpose: evidence`, the workspace, a `zzz-clitest.txt` name and `contentType: text/plain`; then `curl -X PUT -H 'Content-Type: text/plain' --data-binary @zzz-clitest.txt '<presignedUrl>'`; record status codes and the returned `objectKey`
- Entitlements (read): one list GET per gated area: `/access-review-periods`, `/vendor-security-reviews`, `/custom-field-definitions`, `/risks-search`. Record 200 as available, 402 as not on the plan, 401/403/404 as inconclusive
- No control-create probe: controls have no DELETE op, so any 2xx would leave a permanent control. Phase 1 follows the spec
- Paste results under an `Observed` block in this doc
- **Success criteria:**
  - `Observed` records the retest responses (or the skip), the presigned PUT status and `objectKey`, and a status per gated area

#### Phase 1: Command table, truth-up, conform the existing curated surface
**Model:** opus
- `tests/commands.rs` command table with rows for all 83 ops the code reaches today; `CURATED` derived from it; floor raised to 83
- README: drop "Vendor Documents / Security Reviews" from the raw-only list
- Every Phase 1 flag in API Design; hand-written `--example` skeletons checked against the new schemas (`SECURITY_REVIEW_UPDATE_EXAMPLE` gains `updatedAt`)
- **Success criteria:**
  - `cargo test --test coverage -- --nocapture` prints `curated coverage: 83/308`, and deleting any one command-table row drops it to 82
  - tests `vendor_list_encodes_status_filters`, `run_action_finalize_body_has_decision_and_note`, `control_create_body_has_evidence_fields`, `evidence_create_file_key_requires_owner_and_filed_at` pass, and each fails with its change reverted
  - `env -u DRATA_API_KEY drata vendor security-review create-with-file 1 --file /dev/null --title t --review-deadline-at 2026-12-31 --status NOT_YET_STARTED --type SECURITY` exits 2 (clap usage error, missing `--document-type`)
    - Observed today: exit 1, `Error: Failed to load configuration`

#### Phase 2: Transport and upload URLs
**Model:** opus
- `src/client/presigned.rs`, `Retry429`, `src/data.rs`; `drata upload-url create`
- **Success criteria:**
  - coverage `>= 84/308`
  - wiremock with two servers: the presigned PUT carries no `Authorization` header and the requested Content-Type; a 403 XML body from the presigned server prints with the URL's query string removed
  - test `retry_never_sends_once`: a 429 with `Retry429::Never` is requested exactly once

#### Phase 3: Tickets, risk remove and search, framework requirement controls
**Model:** sonnet
- **Success criteria:**
  - coverage `>= 99/308`
  - `drata control ticket create --help` exits 2 (no such verb); `drata risk ticket create --help` exits 0
    - Observed on 371c13f: both exit 2 (no `ticket` subcommand yet)

#### Phase 4: Vendor-adjacent reads, VSR notes, observations, assessments
**Model:** sonnet
- **Success criteria:**
  - coverage `>= 121/308`
  - test `vendor_search_facet_returns_envelope`: `--facet` prints `facets` and requests one page

#### Phase 5: VSR agent workflow
**Model:** opus
- 17 ops; 9 return 202 and print the 202 body. Out-of-order calls return 422, printed through `format_api_error`
- Multipart `documents` uses the existing `Multipart` builder
- **Success criteria:**
  - coverage `>= 138/308`
  - wiremock: a 422 from `finalizations` exits non-zero and prints the server's message

#### Phase 6: Monitoring tests
**Model:** opus
- Custom test create, recipe validate/update, retest (`Retry429::Never`), exclusions, codebase exclusions
- **Success criteria:**
  - coverage `>= 147/308`
  - wiremock: retest answered 409 and 429 is each requested exactly once, exits non-zero, and prints the server's message

#### Phase 7: Personnel, groups, compliance-check exclusions, custom fields, custom-connection attributes
**Model:** sonnet
- **Success criteria:**
  - coverage `>= 162/308`
  - wiremock: an exclusion action with one failed item in a 2xx body exits 1 after printing every row

#### Phase 8: Infrastructure and Version Control identities
**Model:** sonnet
- **Success criteria:**
  - coverage `>= 170/308`
  - test `identity_paths_for_each_kind` covers both kinds through the one implementation

#### Phase 9: Policies and policy languages
**Model:** sonnet
- Multipart policy-version create and file replace through the existing `post_multipart` / `put_multipart` (`src/client.rs:429`, `:436`), Policy Languages
- **Success criteria:**
  - coverage `>= 176/308`
  - wiremock: replace-file is a `PUT` multipart with the file field the spec names

#### Phase 10: Evidence items
**Model:** opus
- `evidence-item` noun, artifacts, `evidence-files` multipart; its renderer sits above the Evidence Library sniff (`src/output/table.rs`, `evidenceTemplateCode`)
- **Success criteria:**
  - coverage `>= 186/308`
  - renderer test: an Evidence Item example row renders with the `evidence-item` table, an Evidence Library row still with the library table

#### Phase 11: Access reviews
**Model:** opus
- 16 ops, all 💎. One active period per account; two `/actions` endpoints; the evidence package is a signed URL in JSON, printed, not downloaded
- **Success criteria:**
  - coverage `>= 202/308`
  - test per `/actions` body builder, failing when reverted; `app users set` body carries every user passed and nothing else

#### Phase 12: Standard connections
**Model:** sonnet
- **Success criteria:**
  - coverage `>= 222/308`
  - wiremock: `drata connection aws create --data '{"k":"v"}' --yes` exits non-zero and the server records zero requests

#### Phase 13: Audit DCF suggestions
**Model:** sonnet
- 5 beta ops; generate returns 202 with per-row status
- README: raw-only list and command reference updated for every phase's nouns
- **Success criteria:**
  - coverage `= 227/308`
  - wiremock: a generate response with one `FAILED` row exits 1 after printing every row

## Acceptance Criteria

Observed on branch `drata-v2-endpoint-coverage` (HEAD 371c13f), 2026-11-05, `target/debug/drata`.

- [ ] `cargo test --test coverage -- --nocapture` prints `curated coverage: 227/308`, counted from the command table
  - Observed: `curated coverage: 70/308 operations (22.7%); raw covers all 308` (hand list, no command table)
- [ ] `rg -n -i 'control-library|control library' src tests README.md` exits 1
  - Observed: exit 1 (no matches; d788b46 already removed the README mention)
- [ ] `drata evidence-item --help` and `drata access-review --help` each exit 0
  - Observed: both exit 2 (unrecognized subcommand)
- [ ] `otto ci` exits 0
  - Observed: exit 0 on d788b46
- [ ] After `bump release`, `drata --version` prints the released version and the Rollout shakedown has run with its results recorded in this doc
  - Observed: cannot run until the work ships

## Resolved Decisions

- 2026-11-05, Scott: curate all added ops, beta included; mirror removals
- 2026-11-05, author: the 81 June-era raw-only ops stay raw-only; not in the ask
- 2026-11-05, author + staff (panel round 1): the Evidence tag's resource is the spec's "Evidence Item" -> new noun `evidence-item`. `drata evidence` stays the Evidence Library. Rejected: renaming `drata evidence` to `evidence-library`, because `drata evidence remove <ws> <id>` would keep parsing and delete a different resource (an Evidence Item and all its artifacts; both ids are numeric)
- 2026-11-05, author: mirroring the removal needs no code. No command, test or README line references Control Library; `drata raw` on a removed path warns and still sends (`src/raw.rs:78-83`)
- 2026-11-05, author + both seats (panel round 1): `control create` follows the spec with `"[]"` defaults. No live probe: controls have no DELETE op
- 2026-11-05, author + both seats (panel round 1): ship order. This plan lands first; SafeBase is blocked on a key. The presigned transport lands early here (Phase 2), and the SafeBase plan's Phase 6 (document upload) reuses `put_presigned`, passing the method and headers SafeBase returns

## Alternatives Considered

### Alternative 1: leave new ops raw-only
- **Why not chosen:** Scott asked for all of them curated

### Alternative 2: codegen the new commands from the spec
- **Why not chosen:** rejected in June for the whole CLI (`docs/design/2026-06-27-architecture.md`); the curated + raw split stays

### Alternative 3: rename `drata evidence` -> `evidence-library`, new tag takes `drata evidence`
- **Pros:** noun matches the spec tag name
- **Cons:** silently retargets `list`, `get` and `remove`; `remove` deletes a different resource
- **Why not chosen:** a destructive verb must never change targets under an unchanged command line

### Alternative 4: keep the hand-written CURATED list
- **Why not chosen:** it passes with no command written; the count must come from commands proven on the wire

## Technical Considerations

### Dependencies
- No new crates
- Cross-repo blast radius: `tatari-skills` `plugins/sec/skills/vendor-eval/SKILL.md` runs `drata vendor list | create`, `drata whoami`, `drata raw`. `vendor list` gains flags only; `raw` keeps its interface (its `--data` reader moves module, same behavior). No ship order forced

### Performance
- Rate limit 500 req/min per IP; 429 retry unchanged except `Retry429::Never` call sites

### Security
- Write guard and confirm on every new mutating verb (existing `send_inner` and `confirm`)
- Cloud credentials for Standard Connections only through `--data @file` or stdin
- Presigned PUT carries no Drata credential; its URL query is never printed or logged

### Testing Strategy
- Command table: every curated op parsed, dispatched and observed on the wire
- Unit tests per body builder and path encoder, each run once against a reverted change to prove it bites
- wiremock for multipart, presigned PUT, 409, 422 and 429 paths
- Each live-verified op is recorded as exercised | blocked (402) | unverified in the shakedown section

### Rollout Plan
- One commit per phase on `drata-v2-endpoint-coverage`, `otto ci` green each
- PR stacked on `refresh-drata-spec-and-safebase-docs`; `bump release`; install
- Shakedown on Tatari's tenant:
  - every read verb
  - writes only where the object is deletable by the CLI and nothing leaves Drata: `evidence-item create | remove`, `upload-url create --file`, `custom-field create | remove`, `vendor security-review note | observation create | remove` on a `zzz-clitest-` vendor, `risk ticket link | unlink` with a dummy URL
  - excluded: `control create` (no DELETE), ticket `create` (reaches the connected ticketing system), VSR `access-requests`, `follow-up-questionnaire-sends` and agent runs (email or contact vendors), `connection` writes (live integrations), `access-review period` create (one active period per account), `monitor retest` on anything but a `zzz-clitest` test

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Beta ops change shape | Med | Low | `(beta)` in help; `raw` still reaches them; the command table catches path changes on spec refresh |
| 💎 tags not on Tatari's plan | Med | Med | Phase 0 probes; wiremock covers the rest; shakedown records blocked ops |
| `control create` defaults rejected live | Low | Med | flags override the `"[]"` defaults; first live use reports the server's error |
| Presigned URL leaks into logs | Low | Med | query stripped in errors and logs; tested |

## Open Questions

- (none)

## Review Log

- Pass 1, draft: shape from the design-research brief; Appendix A generated from the two spec versions
- Pass 2, correctness: `create-with-file` takes `--file`, not a positional; Phase 1 criterion rewritten and observed (exit 1 today). `drata raw` on a removed path confirmed warn-and-send (`src/raw.rs:78-83`)
- Pass 3, clarity: per-command checklist and naming rule stated once in Overview instead of per phase
- Pass 4, edge cases: Phase 0 retest restricted to a `zzz-clitest` custom test; renderer collisions get a per-noun pick test; `get_all` vs `get` rule for non-cursor GETs
- Pass 5, excellence: voice lint; phase criteria use `>=` coverage so reordering cannot break them
- Panel round 1 (`~/.cache/review-panel/runs/XBH1ppIg/synthesis.md`), all folded in:
  - must-fix 1: control-create probe dropped; spec-following defaults
  - must-fix 2-3: presigned helper takes method + headers; `objectKey`, not `fileKey`; SafeBase reuse is its Phase 6
  - must-fix 4: `--filed-at`, `--owner-id` and clap `requires` on evidence create
  - must-fix 5: rename dropped; new noun `evidence-item`
  - must-fix 6: no `control ticket create`
  - must-fix 7: retest probe waits out `TESTING`, uses curl; 409 and 429 both unretried
  - must-fix 8: search `--facet` returns the envelope
  - must-fix 9: command table; CURATED derived from it
  - cheap-wins: shared `read_data` + file-or-stdin mode; probe paths and 401/403/404 inconclusive; `audit get --expand`; per-row failure exit rule; presigned URL redaction; Phase 6 split (identities -> Phase 8), Phase 8 split (transport -> Phase 2), policies to sonnet; Phase 11/12/13 criteria; shipped-bar acceptance criterion; shakedown write list; `Retry429` mechanism named
  - defer 18 folded: `upload-url create` without `--file` prints the response
- Panel round 2: both seats accepted all five items (evidence-item noun, ship order, command table, both architect pushbacks). Panel lead's defer folded in: a clap-tree walk fails on any command without a command-table row. Nothing open, nothing disputed
  - architect pushbacks (round 1): 429 retry fires without `Retry-After` (`unwrap_or(DEFAULT_RETRY_DELAY_SECS)`), so the opt-out stays; no legacy-`evidence` warning shim (no rename now)

## References

- `spec/drata-openapi-v2.json` (d788b46); June spec at `165c6cf`
- `docs/design/2026-06-27-architecture.md`, `docs/design/2026-06-28-vendor-security-reviews.md`
- `docs/drata/endpoint-index.md`, `docs/drata/reference/*.md`

## Appendix A: Added ops by phase

Generated from `git show 165c6cf:spec/drata-openapi-v2.json` vs `spec/drata-openapi-v2.json` at d788b46.

### Phase 2 (1 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Uploads | `POST` | `/upload-urls` |  | Request Upload URL |

### Phase 3 (15 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Control Tickets | `GET` | `/workspaces/{workspaceId}/controls/{controlId}/tickets` |  | List Control Tickets |
| Control Tickets | `POST` | `/workspaces/{workspaceId}/controls/{controlId}/tickets` |  | Link Control Ticket |
| Control Tickets | `DELETE` | `/workspaces/{workspaceId}/controls/{controlId}/tickets/{ticketId}` |  | Unlink Control Ticket |
| Frameworks | `GET` | `/workspaces/{workspaceId}/frameworks/{frameworkId}/requirements/{requirementId}/controls` |  | List Controls for a Framework Requirement |
| Monitoring Test Tickets | `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets` |  | List Monitoring Test Tickets |
| Monitoring Test Tickets | `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets` |  | Link Monitoring Test Ticket |
| Monitoring Test Tickets | `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets/action-create` |  | Create Monitoring Test Ticket |
| Monitoring Test Tickets | `DELETE` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/tickets/{ticketId}` |  | Unlink Monitoring Test Ticket |
| Risk Tickets | `GET` | `/risk-registers/{riskRegisterId}/risks/{riskId}/tickets` |  | List Risk Tickets |
| Risk Tickets | `POST` | `/risk-registers/{riskRegisterId}/risks/{riskId}/tickets` |  | Link Risk Ticket |
| Risk Tickets | `POST` | `/risk-registers/{riskRegisterId}/risks/{riskId}/tickets/action-create` |  | Create Risk Ticket |
| Risk Tickets | `DELETE` | `/risk-registers/{riskRegisterId}/risks/{riskId}/tickets/{ticketId}` |  | Unlink Risk Ticket |
| Risks | `GET` | `/risk-registers/{riskRegisterId}/risks-search` |  | Search Risks by Risk Register |
| Risks | `DELETE` | `/risk-registers/{riskRegisterId}/risks/{riskId}` |  | Delete Risk |
| Risks | `GET` | `/risks-search` |  | Search Risks |

### Phase 4 (22 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Vendor Criteria | `GET` | `/vendor-criteria` |  | List Vendor Criteria |
| Vendor Criteria Creations | `GET` | `/workspaces/{workspaceId}/vendor-criteria-creations` |  | List Vendor Criteria Creations |
| Vendor Questionnaire Templates | `GET` | `/vendor-questionnaire-templates` |  | List Vendor Questionnaire Templates |
| Vendor Questionnaire Templates | `GET` | `/vendor-questionnaire-templates/{questionnaireId}/questions` |  | List Vendor Questionnaire Template Questions |
| Vendor Risk Levels | `GET` | `/vendor-risk-levels` |  | List Vendor Risk Levels |
| Vendor Security Reviews | `GET` | `/vendor-security-reviews` |  | List Vendor Security Reviews Across Vendors |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments` | yes | List Assessments |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments-stats` | yes | Get Assessments Stats |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/assessments/{assessmentId}` | yes | Get Assessment |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/notes` | yes | List Notes |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/notes` | yes | Create Note |
| Vendor Security Reviews | `DELETE` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}` | yes | Delete Note |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}` | yes | Get Note |
| Vendor Security Reviews | `PUT` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/notes/{securityReviewNoteId}` | yes | Update Note |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/observations` | yes | List Observations |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/observations` | yes | Create Observation |
| Vendor Security Reviews | `DELETE` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}` | yes | Delete Observation |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}` | yes | Get Observation |
| Vendor Security Reviews | `PUT` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/observations/{observationId}` | yes | Update Observation |
| Vendors | `GET` | `/vendors-search` |  | Search Vendors |
| Vendors | `GET` | `/vendors/{vendorId}/questionnaires/{questionnaireId}/questions` |  | List Vendor Questionnaire Questions |
| Vendors | `GET` | `/vendors/{vendorId}/trust-overview` |  | Get Vendor Trust Center Overview |

### Phase 5 (17 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/access-links` | yes | Provide Agent Review Trust Center Access Link |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/access-request-requirements` | yes | Get Agent Review Trust Center Access Request Requirements |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/access-request-skips` | yes | Skip Agent Review Trust Center Access Request |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/access-requests` | yes | Request Agent Review Trust Center Access |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/agent` | yes | Start Agent Review on Security Review |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/assessment-runs` | yes | Start Agent Review Assessment |
| Vendor Security Reviews | `DELETE` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/assessment-runs/current` | yes | Stop Agent Review Assessment |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/criteria-confirmations` | yes | Confirm Agent Review Criteria |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/documents` | yes | Add Agent Review Documents |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/finalizations` | yes | Finalize Agent Review |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/follow-up-questionnaire` | yes | Get Agent Review Follow-up Questionnaire |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/follow-up-questionnaire-sends` | yes | Send Agent Review Follow-up Questionnaire |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/follow-up-questionnaires` | yes | Generate Agent Review Follow-up Questionnaire |
| Vendor Security Reviews | `POST` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/public-document-collections` | yes | Collect Agent Review Public Trust Center Documents |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/residual-risk` | yes | Get Security Review Residual Risk |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/summary/status` | yes | Get Vendor Security Review Summary |
| Vendor Security Reviews | `GET` | `/vendors/{vendorId}/security-reviews/{securityReviewId}/workflow` | yes | Get Vendor Security Review Workflow |

### Phase 6 (9 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Monitoring Tests | `POST` | `/workspaces/{workspaceId}/monitoring-tests` |  | Create Custom Monitoring Test |
| Monitoring Tests | `POST` | `/workspaces/{workspaceId}/monitoring-tests/action-validate-recipe` |  | Validate Custom Monitoring Test Recipe |
| Monitoring Tests | `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/action-retest` |  | Run Monitoring Test |
| Monitoring Tests | `GET` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions` |  | List Monitoring Test Codebase Exclusions |
| Monitoring Tests | `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions` |  | Create Monitoring Test Codebase Exclusions |
| Monitoring Tests | `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/codebase-exclusions/action-delete` |  | Delete Monitoring Test Codebase Exclusions |
| Monitoring Tests | `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions` |  | Create Monitoring Test Exclusions |
| Monitoring Tests | `POST` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/exclusions/action-delete` |  | Delete Monitoring Test Exclusions |
| Monitoring Tests | `PUT` | `/workspaces/{workspaceId}/monitoring-tests/{testId}/recipe` |  | Update Custom Monitoring Test Recipe |

### Phase 7 (15 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Compliance Check Exclusions | `GET` | `/workspaces/{workspaceId}/compliance-check-exclusions` |  | List Compliance Check Exclusions |
| Compliance Check Exclusions | `POST` | `/workspaces/{workspaceId}/compliance-check-exclusions` |  | Exclude Personnel Compliance Checks |
| Compliance Check Exclusions | `POST` | `/workspaces/{workspaceId}/compliance-check-exclusions/action-delete` |  | Delete Personnel Compliance Check Exclusions |
| Custom Connections | `GET` | `/workspaces/{workspaceId}/custom-connections/{connectionId}/resources/{resourceId}/attributes` |  | Get Custom Connection Resource Attributes |
| Custom Field Definitions | `GET` | `/custom-field-definitions` |  | List Custom Field Definitions |
| Custom Field Definitions | `POST` | `/custom-field-definitions` |  | Create Custom Field Definition |
| Custom Field Definitions | `GET` | `/custom-field-definitions/{customFieldId}` |  | Get Custom Field Definition |
| Groups | `GET` | `/groups` |  | List Groups |
| Personnel | `GET` | `/personnel-search` |  | Search Personnel across all workspaces |
| Personnel | `GET` | `/workspaces/{workspaceId}/personnel-search` |  | Search Personnel within a workspace |
| Personnel | `GET` | `/workspaces/{workspaceId}/scoped-personnel-counts` | yes | Get Workspace Scoped Personnel Counts |
| Personnel | `GET` | `/workspaces/{workspaceId}/scoped-personnel-groups` | yes | List Workspace Scoped Personnel Groups |
| Personnel | `POST` | `/workspaces/{workspaceId}/scoped-personnel-groups` | yes | Add Personnel Group To Workspace Scope |
| Personnel | `PUT` | `/workspaces/{workspaceId}/scoped-personnel-groups` | yes | Replace Workspace Scoped Personnel Groups |
| Personnel | `DELETE` | `/workspaces/{workspaceId}/scoped-personnel-groups/{groupId}` | yes | Remove Personnel Group From Workspace Scope |

### Phase 8 (8 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Infrastructure User Identities | `GET` | `/workspaces/{workspaceId}/infrastructure-user-identities` |  | List Infrastructure User Identities |
| Infrastructure User Identities | `GET` | `/workspaces/{workspaceId}/infrastructure-user-identities/{identityId}` |  | Get Infrastructure User Identity |
| Infrastructure User Identities | `GET` | `/workspaces/{workspaceId}/infrastructure-user-identities/{identityId}/actions` |  | List Infrastructure User Identity Actions |
| Infrastructure User Identities | `POST` | `/workspaces/{workspaceId}/infrastructure-user-identities/{identityId}/actions` |  | Perform Infrastructure User Identity Action |
| Version Control User Identities | `GET` | `/workspaces/{workspaceId}/version-control-user-identities` |  | List Version Control User Identities |
| Version Control User Identities | `GET` | `/workspaces/{workspaceId}/version-control-user-identities/{identityId}` |  | Get Version Control User Identity |
| Version Control User Identities | `GET` | `/workspaces/{workspaceId}/version-control-user-identities/{identityId}/actions` |  | List Version Control User Identity Actions |
| Version Control User Identities | `POST` | `/workspaces/{workspaceId}/version-control-user-identities/{identityId}/actions` |  | Perform Version Control User Identity Action |

### Phase 9 (6 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Policies | `POST` | `/policies/{policyId}/policy-versions` |  | Create Policy Version |
| Policies | `PUT` | `/policies/{policyId}/policy-versions/{policyVersionId}/file` |  | Replace Policy Version File |
| Policy Languages | `GET` | `/policies/{policyId}/policy-versions/{policyVersionId}/languages` |  | List Policy Version Languages |
| Policy Languages | `GET` | `/policy-language-settings` |  | List Policy Language Settings |
| Policy Languages | `GET` | `/policy-language-versions` |  | List Policy Language Versions |
| Policy Languages | `GET` | `/policy-language-versions/{policyLanguageVersionId}` |  | Get Policy Language Version |

### Phase 10 (10 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Evidence | `GET` | `/workspaces/{workspaceId}/evidence` |  | List Evidence Items |
| Evidence | `POST` | `/workspaces/{workspaceId}/evidence` |  | Create Evidence |
| Evidence | `POST` | `/workspaces/{workspaceId}/evidence-files` |  | Upload Evidence Artifact File |
| Evidence | `DELETE` | `/workspaces/{workspaceId}/evidence/{evidenceId}` |  | Delete Evidence |
| Evidence | `GET` | `/workspaces/{workspaceId}/evidence/{evidenceId}` |  | Get Evidence Item |
| Evidence | `PUT` | `/workspaces/{workspaceId}/evidence/{evidenceId}` |  | Update Evidence |
| Evidence | `GET` | `/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts` |  | List Evidence Artifacts |
| Evidence | `DELETE` | `/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}` |  | Delete Evidence Artifact |
| Evidence | `PUT` | `/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}` |  | Update Evidence Artifact |
| Evidence | `POST` | `/workspaces/{workspaceId}/evidence/{evidenceId}/artifacts/{artifactId}/actions` |  | Perform Evidence Artifact Action |

### Phase 11 (16 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Access Reviews | `GET` | `/access-review-applications` |  | List Access Review Applications |
| Access Reviews | `POST` | `/access-review-applications` |  | Create Access Review Application |
| Access Reviews | `DELETE` | `/access-review-applications/{applicationId}` |  | Delete Access Review Application |
| Access Reviews | `GET` | `/access-review-applications/{applicationId}` |  | Get Access Review Application |
| Access Reviews | `PUT` | `/access-review-applications/{applicationId}` |  | Update Access Review Application |
| Access Reviews | `GET` | `/access-review-applications/{applicationId}/users` |  | List Access Review Application Users |
| Access Reviews | `PUT` | `/access-review-applications/{applicationId}/users` |  | Create or Update Access Review Application Users |
| Access Reviews | `GET` | `/access-review-periods` |  | List Access Reviews |
| Access Reviews | `POST` | `/access-review-periods` |  | Conduct Access Review |
| Access Reviews | `GET` | `/access-review-periods/{periodId}` |  | Get Access Review |
| Access Reviews | `POST` | `/access-review-periods/{periodId}/actions` |  | Perform Access Review Action |
| Access Reviews | `GET` | `/access-review-periods/{periodId}/applications/{reviewAppId}` |  | Get Application in Access Review |
| Access Reviews | `PUT` | `/access-review-periods/{periodId}/applications/{reviewAppId}` |  | Update Application Reviewers in Access Review |
| Access Reviews | `POST` | `/access-review-periods/{periodId}/applications/{reviewAppId}/actions` |  | Perform Access Review Application Action |
| Access Reviews | `GET` | `/access-review-periods/{periodId}/applications/{reviewAppId}/users` |  | List Application Users in Access Review |
| Access Reviews | `GET` | `/access-review-periods/{periodId}/evidence-package` |  | Download Access Review Evidence Package |

### Phase 12 (20 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Standard Connections | `GET` | `/aws-connections` |  | List AWS Connections |
| Standard Connections | `POST` | `/aws-connections` |  | Create AWS Connection |
| Standard Connections | `DELETE` | `/aws-connections/{connectionId}` |  | Delete AWS Connection |
| Standard Connections | `GET` | `/aws-connections/{connectionId}` |  | Get AWS Connection |
| Standard Connections | `PUT` | `/aws-connections/{connectionId}` |  | Update AWS Connection |
| Standard Connections | `GET` | `/azure-connections` |  | List Azure Connections |
| Standard Connections | `POST` | `/azure-connections` |  | Create Azure Connection |
| Standard Connections | `DELETE` | `/azure-connections/{connectionId}` |  | Delete Azure Connection |
| Standard Connections | `GET` | `/azure-connections/{connectionId}` |  | Get Azure Connection |
| Standard Connections | `PUT` | `/azure-connections/{connectionId}` |  | Update Azure Connection |
| Standard Connections | `GET` | `/gcp-connections` |  | List GCP Connections |
| Standard Connections | `POST` | `/gcp-connections` |  | Create GCP Connection |
| Standard Connections | `DELETE` | `/gcp-connections/{connectionId}` |  | Delete GCP Connection |
| Standard Connections | `GET` | `/gcp-connections/{connectionId}` |  | Get GCP Connection |
| Standard Connections | `PUT` | `/gcp-connections/{connectionId}` |  | Update GCP Connection |
| Standard Connections | `GET` | `/gitlab-connections` |  | List GitLab Connections |
| Standard Connections | `POST` | `/gitlab-connections` |  | Create GitLab Connection |
| Standard Connections | `DELETE` | `/gitlab-connections/{connectionId}` |  | Delete GitLab Connection |
| Standard Connections | `GET` | `/gitlab-connections/{connectionId}` |  | Get GitLab Connection |
| Standard Connections | `PUT` | `/gitlab-connections/{connectionId}` |  | Update GitLab Connection |

### Phase 13 (5 ops)

| Tag | Method | Path | Beta | Summary |
|---|---|---|---|---|
| Audit Requests | `POST` | `/workspaces/{workspaceId}/audits/{auditId}/requests/action-accept-dcf-suggestions` | yes | Accept Audit Request DCF Suggestions |
| Audit Requests | `POST` | `/workspaces/{workspaceId}/audits/{auditId}/requests/action-generate-dcf-suggestions` | yes | Generate Audit Request DCF Suggestions |
| Audit Requests | `GET` | `/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}/dcf-suggestions` | yes | List Audit Request DCF Suggestions |
| Audit Requests | `GET` | `/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}/dcf-suggestions/{suggestionId}` | yes | Get Audit Request DCF Suggestion |
| Audit Requests | `POST` | `/workspaces/{workspaceId}/audits/{auditId}/requests/{requestId}/dcf-suggestions/{suggestionId}/actions` | yes | Decide an Audit Request DCF Suggestion |
