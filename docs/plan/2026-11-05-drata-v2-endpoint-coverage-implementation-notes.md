## Phase 0: Prove the unverified server behaviors
### Design decisions
- Keys came from the `DRATA_READONLY_API_KEY` and `DRATA_READWRITE_API_KEY` env vars (no `~/.config/drata/credentials.json` on this host) | scratchpad probe scripts | the read key for reads, the write key only for `POST /upload-urls`. The scripts passed the key to `curl` from the environment, so no key was echoed or written to a file.
- Searched for a `zzz-clitest` monitor by paging the whole monitoring-test list, not only `?name=` | scratchpad | the server-side name filter is unverified, so a client-side scan rules out a false "none exists".

### Deviations
- None.

### Tradeoffs
- Skipped the retest probe rather than retesting a production monitor | no `zzz-clitest` test exists and the doc forbids any other target. The 409/429 behavior stays spec-documented, not live-verified.

### Open questions
- Create a `zzz-clitest` custom monitoring test (needs Phase 6's `monitor create`) so the retest cooldown gets probed during the Phase 6 shakedown?

## Phase 1: Command table, truth-up, conform the existing curated surface
### Design decisions
- Command table lives in `tests/commands/table.rs`, pulled into both `tests/commands.rs` and `tests/coverage.rs` with `#[path]` | a file in a `tests/` subdirectory is not auto-built as its own test crate, and `#[path]` avoids a `mod.rs`. `coverage.rs` reads only `method` and `path`, so its `mod table` carries `#[allow(dead_code)]`.
- One row per op, enforced by `rows_are_one_per_operation` | a second row for the same op would let a deleted row hide behind its twin and keep the count at 83.
- Rows carry a `Response` enum (`Page`, `Object`, `Empty`) rather than a raw JSON value | three shapes cover every curated op; `Page` returns a null cursor so `get_all` stops after exactly one request.
- `src/lib.rs` `run` now builds the client and calls a new `pub async fn dispatch(cli, client, config, confirm)` | the command table needs to dispatch over a client pointed at wiremock; `run` keeps its signature.
- The dispatch test uses `default_confirm(cli.yes)` with `--yes` appended to every argv | it exercises the real confirm gate instead of an injected always-yes.
- The leaf walk (`every_curated_leaf_has_a_row`) maps each row's argv to its subcommand path by descending `Cli::command()` while tokens name subcommands | no row needs a hand-written command path that could drift from its argv.
- `src/cli.rs` vendor section moved to `src/cli/vendor.rs`, re-exported with `pub use vendor::*` | `cli.rs` was 1407 lines and the Phase 1 flags would have crossed the 1500-line bloat limit; `crate::cli::VendorAction` and friends keep their paths.
- New `src/query.rs`: `QueryPath` (push / push_opt / push_each) and `wire_name` for clap `ValueEnum`s | vendor list, security-review list, control list and monitor list all build query strings; the security-review list's hand-rolled closure was replaced by it. `wire_name` reads clap's possible-value name, which equals the spec value because every filter enum is `SCREAMING_SNAKE_CASE`.
- New filter enums `VendorStatus`, `VendorCategory`, `VendorType`, `ReviewDeadlineStatus`, `RequiredApprovals`, `SecurityReviewDocumentType` (all `ignore_case = true`) | the spec enumerates each, and Data Model says a curated flag that takes a Drata enum gets a `ValueEnum`.
- `--sub-processor`, `--action-required`, `--draft` are `Option<bool>` taking `true|false` | matches the existing `monitor update --enabled`, and lets a caller filter on `false`.
- `run-action` body is built by the pure `build_run_action_body`, which rejects `--decision`/`--note` with `reopen` and `--reset-decision` with `finalize` before confirm | the spec marks each "only valid when action is ..."; fail closed client-side rather than send a body the server rejects.
- `control create` validates that `--external-evidence-metadata` and `--base64-files` each parse as a JSON array, then sends the flag text verbatim as the string field (`build_create_body`) | the spec types both as `string` holding a JSON array; a malformed value fails before the confirm prompt.
- `evidence create` builders split into pure `build_create_form` (multipart, `--file`) and `build_create_body` (JSON, optional `fileKey`) over a `CreateFields` struct | the two shapes share four scalar fields; the struct keeps `create` under the clippy argument threshold.
- Wire-level tests in `tests/commands.rs` (`vendor_update_sends_vendor_type_id`, `risk_create_and_update_send_vendor_id`, `audit_get_sends_expand`, `control_create_sends_evidence_defaults`, `security_review_update_sends_updated_at`) | those handlers build their bodies inline; dispatching through wiremock proves the flag reaches the wire without refactoring each handler.
- `reports_curated_coverage_percentage` prints the count before asserting the floor | a run with a deleted row now prints `curated coverage: 82/308` instead of only the panic message.

### Deviations
- `evidence create --file` also requires `--filed-at`, `--owner-id` and `--renewal-schedule-type` (clap `requires`), not only `--filed-at` | the spec's `file` field says "When provided, `renewalScheduleType`, `filedAt`, and `ownerId` are required", the same rule as `fileKey`. `--file` and `--file-key` conflict (one artifact source). README's `evidence create --file` example gained `--filed-at` and `--owner-id` so it still parses.
- `--document-type` on `create-with-file` is a required `ValueEnum` (8 spec values), not a required free-text string | same effect (clap exit 2 when missing), and the spec enumerates the values.
- `README.md` "Development" paragraph now names the command table | not in the plan bullet; without it the README described the old hand-written floor.
- Existing tests inverted by name: `create_with_file_omits_unset_optionals` -> `create_with_file_always_sends_document_type_and_omits_unset_optionals` (asserts `documentType` is always sent); `update_body_only_has_title_and_soc_form` and `run_action_body_uses_action_string` built their own JSON and exercised no production code, replaced by `update_body_has_only_update_fields` and the `run_action_*` builder tests.

### Tradeoffs
- Splitting `cli.rs` by noun (vendor only, now) vs a full per-noun split | only the vendor section moved: it is the largest and the one Phase 1 grows most. Later phases that add nouns can follow the same `src/cli/<noun>.rs` pattern.
- Client-side rejection of finalize/reopen flag mixes vs passing them through for the server to judge | the spec states the rule plainly, and a rejected mutation is better caught before the confirm prompt.
- `wire_name` uses `expect` on clap's possible value vs a hand-written match per enum | six new enums would each need a match that can drift from the clap names; no filter enum skips a variant, so the `expect` is an invariant, not a runtime path.

### Open questions
- `vendor security-review create-with-file` is still sent as multipart; the refreshed spec lists only `application/json` (with a binary `file` field) for it. The existing 415 fallback message still applies. Worth a live check during the shakedown?

## Phase 2: Transport and upload URLs
### Design decisions
- `put_presigned` is a free function in `src/client/presigned.rs` that builds its own `reqwest::Client` per call (`redirect::Policy::none()`, 300s timeout) | it shares nothing with `DrataClient`, so no code path can attach the Bearer token or base URL. A per-call client costs nothing at one upload per invocation.
- Signature `put_presigned(method: Method, url: &str, headers: &[(String, String)], bytes: Vec<u8>) -> Result<()>` | the caller supplies every header; SafeBase's Phase 6 will convert its response headers into the same slice.
- Redaction is `redact_url` (split at `?`/`#`), applied to the error (`PresignedError`), the `instrument` span and every `debug!`. Transport errors go through `reqwest::Error::without_url` | reqwest's own error Display embeds the full URL, signature included; `transport_error_hides_the_query` pins it.
- No retry and no redirect on the presigned path; a 3xx is an error | a redirect would replay a signed request to a host the caller never named.
- `Retry429` lives in `src/client.rs` beside `send_inner`. Under `Never` a 429 falls through to the normal non-2xx branch, so it surfaces as an `ApiError` with the server's body | Phase 6 can match on `ApiError.status` for 409 vs 429 without a new error type. No public `Never` verb yet: Phase 6 adds the call site.
- `src/data.rs`: `read_data(data, DataSource)`; `raw` and both curated `--data` flags (`vendor security-review create | update`) use `DataSource::Any`. The inline rejection message never echoes the rejected body | `FileOrStdinOnly` exists for Standard Connections' credential bodies.
- `upload-url` resource lives in `src/resources/upload.rs` and its clap types in `src/cli/upload.rs` | `upload_url.rs` would be a compound filename; the noun's single word is `upload`.
- `--purpose` is a `ValueEnum` `UploadPurpose` (spec `UploadPurposeEnum`, one value `evidence`) with `workspace_scoped()`; `build_request_body` fails before confirm when a workspace-scoped purpose has no `--workspace` | the spec says `workspaceId` is required for `evidence`; fail closed client-side.
- With `--file`, the file is read before the `POST /upload-urls` (an unreadable file fails before the mutation); the response's `presignedUrl` and `objectKey` must both be strings before the PUT; on success only the bare `objectKey` is printed with `println!` | the value feeds `--file-key` directly (`$(drata upload-url create ...)`), and the presigned URL is never printed in this mode.
- `mime_guess` was already a direct dependency (`Cargo.toml`, used by `send_multipart`) | no crate added, consistent with "No new crates".
- Coverage floor raised to 84 in `tests/coverage.rs`; its comment no longer names a count | the count lives in the constant only.

### Deviations
- None.

### Tradeoffs
- No `--data`/`--example` on `upload-url create` vs the Overview's generic "flags plus `--data`" | API Design Phase 2 names exactly `--purpose`, `--workspace`, `--file`; the browser-flow fields (`includeWebUploadUrl`, `existingArtifactId`, `multiArtifact`, ...) stay reachable through `drata raw POST /upload-urls`.
- No table renderer for the `/upload-urls` response vs adding `render_upload_url` | the no-`--file` response is a single object whose point is the URL; the JSON fallback prints it whole, and no existing sniff matches its keys.
- Curated `--data` parsing still happens after the confirm prompt (as before) vs moving it ahead | moving it would read stdin (`--data -`) before the prompt reads the TTY; behavior unchanged from Phase 1.

### Open questions
- None.

## Phase 3: Tickets, risk remove and search, framework requirement controls
### Design decisions
- One ticket implementation in `src/resources/ticket.rs`: `control_target`, `risk_target` and `monitor_target` fold each parent's ids into its `.../tickets` path and yield a `TicketOp` (`List | Link | Unlink | Create`); `ticket::handle` runs it | the three parents differ only by path, so the verbs exist once. `ControlTicketAction` has no `Create` variant, so `control ticket create` is a clap exit 2 by construction.
- Three clap enums (`ControlTicketAction`, `RiskTicketAction`, `MonitorTicketAction`) in `src/cli/ticket.rs`, with `create`'s flags in one flattened `TicketCreateArgs` | positional names differ per parent (`<control_id>`, `<risk_id>`, `<test_id>`) and help should say so; one shared enum would print a generic "parent id".
- `ticket create` flags: `--connection-id`, `--project-id`, `--issue-type-id`, `--list-id`, `--team-id`, `--field <field> <type> <value>` (repeatable, `num_args = 3`), `--use-rich-content`, `--data`, `--example`. `--data` conflicts with every field flag. The field `type` is checked against the spec's `TicketFieldTypePublicV2Enum` before the confirm prompt | a triple avoids a delimiter that a ticket value could contain.
- `src/resources/search.rs` is the reusable search verb: `search::run(client, config, QueryPath, &[facet])`. No facets: `get_all`, rows. Facets: one `client.get` with `facets[]`, the whole envelope printed as JSON regardless of `--output` | the table renderer shows only `data[]` and would silently drop the facet counts. Phase 4 and Phase 7 pass their own `QueryPath`.
- `risk search` is one verb over both ops: `--register-id` switches `/risks-search` to `/risk-registers/{id}/risks-search`. `--facet riskRegisterId` with `--register-id` fails before any request, since `RiskSearchInRegisterFacetEnum` omits it. `--facet` and `--expand` are camelCase `ValueEnum`s (`RiskSearchFacet`, `RiskSearchExpand` in `src/cli/risk.rs`) read through `wire_name`.
- `risk remove` is `DELETE /risk-registers/{id}/risks/{riskId}` through confirm. `framework requirement controls <ws> <fw> <req>` is a new `requirement` noun under `framework`, via `get_all`.
- `render_tickets` in `src/output/table.rs`, sniffed on `externalTicketId` and placed first | a unique key in first position cannot collide with a looser sniff.
- `monitor::example_if_requested` and its arm in `lib.rs::example_if_requested` added | monitor had no `--example` before, and `monitor ticket create --example` must run without credentials.
- Coverage floor raised to 99; 15 rows added to `tests/commands/table.rs`.

### Deviations
- None from the doc's command names or ops. Same effect, different seam: the ticket-create `--example` skeletons come from `spec::example_for_operation` rather than hand-written text, since the generator already tracks the spec.

### Tradeoffs
- No flags for `risk search`'s date-range params (`anticipatedCompletionDate*`, `completedDate*`), `sort`, `sortDir`, `customFields`, `customRangeFields` | the doc names no flag list and the structured custom-field filters have no obvious flag shape. They stay reachable through `drata raw GET /risks-search?...`.
- No `--sort`/`--sort-dir` on the ticket lists | nothing in the plan needs ordering; `get_all` returns the server default.
- `risk search` as one verb with an optional `--register-id` vs two verbs | the doc names one `risk search`; the facet rule that differs between the ops is a pre-request check.
- Ticket `create` builds its body before the confirm prompt (Phase 1/2 handlers build after) | flag errors and unreadable `--data` files should fail before asking.

### Open questions
- None.

## Phase 4: Vendor-adjacent reads, VSR notes, observations, assessments
### Design decisions
- Clap types in `src/cli/vendor/{search,reference,finding}.rs`, re-exported from `src/cli/vendor.rs` | `vendor.rs` was 463 lines and the new types would have nearly doubled it; the submodule dir follows the no-`mod.rs` rule.
- Five new top-level nouns `vendor-criteria`, `vendor-criteria-creation`, `vendor-questionnaire-template`, `vendor-risk-level`, `vendor-security-review` (the cross-vendor list), implemented together in `src/resources/vendor/reference.rs` | the spec's resource names slugged per the Overview naming rule; each is a handful of `get_all` lists, so one file with one pure path builder per op.
- Notes and observations share one implementation, `src/resources/vendor/security/finding.rs`, parameterized by `Kind` (`notes|observations`, body field `note|observation`); `note_target` / `observation_target` fold each clap enum into a common `Target` | the two collections differ only by path segment and field name. Two clap enums stay separate so help and flags say `--note` / `--observation`. Assessments are `security/assessment.rs`.
- Security-review, note, observation and assessment ids are `u64` positionals, vendor id stays `String` | matches `vendor security-review get`, and a non-numeric id is a clap exit 2 instead of a path-injected request.
- `create`/`update` ids are `required_unless_present = "example"` and `Option` in the enum, so `--example` runs with no ids (the existing `security-review create --example` needs dummy ids). The handler path errors if they are missing; unreachable through clap.
- `--example` skeletons for note/observation create and update come from `spec::example_for_operation`; `vendor::example_if_requested` and `security::example_if_requested` now return `Option<Result<String>>` to carry that (the static ones are wrapped) and `lib.rs` lost its `.map(Ok)`.
- `update` has `--clear-source` (sends `source: null`, the spec's documented clear) conflicting with `--source`; an update with no field is an error before the confirm prompt.
- `search::fetch` split out of `search::run`: it returns the value to print (`{data: rows}` for the all-pages path, the whole envelope with facets) and `run` only prints | `vendor_search_facet_returns_envelope` can assert on the printed value, not just the request. Behavior unchanged.
- `vendor search` flags: `--q`, `--status`, `--category`, `--vendor-type-id`, `--vendor-user-id`, `--risk`, `--impact-level`, `--sub-processor`, `--action-required`, `--renewal-from`, `--renewal-to`, `--review-deadline-status`, `--security-review-status`, `--security-review-decision`, `--expand`, `--facet`. Reuses `VendorStatus`, `VendorCategory`, `ReviewDeadlineStatus`, `SecurityReviewStatus`, `SecurityReviewDecision`. New enums: `VendorRisk`, `ImpactLevel`, `Severity`, `CriterionType`, `CriteriaExpand`, `CriteriaCreationStatus`, `TemplateStatus`, `AssessmentStatus` (`N_A` pinned with `#[value(name)]`), `VendorSearchExpand`, `VendorSearchFacet`.
- Renderers (`src/output/table.rs`), all placed above the `recipientEmail` and vendor sniffs: `render_findings` (`securityReviewId` + `note|observation`), `render_assessments` (`criteriaName` + `currentStatus`), `render_assessment_stats` (`notMet` + `partiallyMet`), `render_criteria` (`isDraft` + `severity`), `render_templates` (`isDefault` + `title`), `render_questions` (`question` + `allowMultipleSelection`, so a control's `code` + `question` row is untouched), `render_risk_levels` (`position` + `isSystem`), `render_criteria_creations` (exactly the four keys `id, workspaceId, status, createdAt`, the only unique signal that shape has). `vendor search` rows are vendor-shaped and use `render_vendors`; the cross-vendor review list uses the existing `render_security_reviews`.
- Renderer test feeds each op's spec response `example` values (read from the embedded spec, not hand-copied) through `pick_renderer`, plus a guard that a control, vendor and security review still pick their old renderers.
- 22 rows added to `tests/commands/table.rs`; floor raised 99 -> 121.

### Deviations
- None from the doc's command names or ops. Same effect, different seam: the facet test asserts on `search::fetch`'s returned value (what `run` prints) because stdout is not capturable in the in-process command table.

### Tradeoffs
- No `--sort`/`--sort-dir`, `--size`, `--include-total-count` on any new list; no `renewalScheduleType`, `scheduledQuestionnaireStatus`, `customFields`, `customRangeFields` on `vendor search` | the doc names no flag list and the structured custom-field filters have no obvious flag shape; reachable through `drata raw`.
- `vendor-security-review list` has no `--all` NDJSON stream (the per-vendor list does) | the doc names none; `get_all` rows serve the cross-vendor read.
- Reused `VendorStatus` (includes `CURRENT`, which `/vendors-search` `status[]` omits) and `VendorCategory` (includes `NONE`, which the template `categories[]` omits) | a value the server does not accept comes back as its 400 instead of adding two near-duplicate enums.
- No renderer for `vendor trust-overview` (`{"overview": "..."}`) | a one-key sniff would be the loosest in the file; the JSON fallback prints it whole.
- `render_criteria_creations` sniffs on an exact key set | a new server field makes it fall back to JSON, which is harmless; a looser sniff could steal other rows.

### Open questions
- None.

### Phase 4 follow-up (supersedes the enum-reuse tradeoff above)
#### Design decisions
- Every enum flag added in Phase 4 now accepts exactly the spec's values for its op's param. `vendor search --status` takes `VendorSearchStatus` (no `CURRENT`); `vendor-questionnaire-template list --category` takes `TemplateCategory` (no `NONE`). `vendor list` keeps `VendorStatus` with `CURRENT`, `vendor search --category` keeps `VendorCategory` with `NONE`. A value the spec omits is a clap usage error (exit 2), not a server 400.
- The audit found two more free-string flags whose params are spec enums: `expand` on `vendor-security-review list` (now `ReviewListExpand`: user, requesterUser, vendor) and on note/observation list and get (now `FindingExpand`: user). Both were `Vec<String>`.
- `tests/enums.rs` compares each Phase 4 enum's wire names with the `enum` of its op's param in the embedded spec (set equality, both directions), so a spec refresh that adds or drops a value fails the test. Parse tests pin the two rejections and that the sibling commands still accept those values.
- Every other Phase 4 enum already matched its op's spec enum exactly.

#### Deviations
- None.

#### Tradeoffs
- Two near-duplicate enums (`VendorSearchStatus`, `TemplateCategory`) instead of reuse | the spec's value sets differ per op, and a flag must never accept a value the spec does not list.

#### Open questions
- None.

## Phase 5: VSR agent workflow
### Design decisions
- 17 verbs, flat under `drata vendor security-review`. Names are the spec's `AgentReviewActionEnum` values slugged wherever the op has one, so the `availableActions` of a 422 names the verb to run next. The rest use the path's resource name. Verb map: `start-agent` (POST `agent`), `confirm-criteria` (POST `criteria-confirmations`), `start-assessment` (POST `assessment-runs`, which also re-runs; the enum's `re_run_assessment` hits the same op), `stop-assessment` (DELETE `assessment-runs/current`), `collect-public-documents` (POST `public-document-collections`), `access-request-requirements` (GET), `request-trust-center-access` (POST `access-requests`), `provide-trust-center-access-link` (POST `access-links`), `skip-trust-center-access-request` (POST `access-request-skips`), `add-documents` (multipart POST `documents`), `generate-follow-up-questionnaire` (POST `follow-up-questionnaires`), `follow-up-questionnaire` (GET), `send-follow-up-questionnaire` (POST `follow-up-questionnaire-sends`), `residual-risk` (GET), `finalize` (POST `finalizations`), `summary` (GET `summary/status`), `workflow` (GET). `workflow` and `summary` are the names the doc's Non-Goals already use.
- Clap types in `src/cli/vendor/agent.rs` as `SecurityReviewAgentAction`, flattened into `VendorSecurityReviewAction` with `#[command(flatten)] Agent(...)` | the verbs sit directly under `security-review` while their types stay out of `vendor.rs`. Shared positionals are `ReviewIds` (`vendor_id: String`, `security_review_id: u64`, matching Phase 4) and `ExampleReviewIds` (both optional, `required_unless_present = "example"`) for the five body verbs with `--example`.
- Implementation in `src/resources/vendor/security/agent.rs`: `plan(action) -> Call` resolves every verb to its request with no I/O beyond reading `--data`/`--fields`, and `handle` confirms, sends and prints | path and body tests run on `plan` and the pure builders. `Call` is an enum (`Get`, `Post`, `PostForm`, `Delete`) so a method can only carry the body shape it takes; no runtime method/body mismatch branch exists.
- Every 2xx body prints through `print_value`, the 202 acceptance body included; nothing polls. The command table's new `Response::Accepted` answers the nine 202 ops with a real 202 and `{securityReviewId, workflowId, action}`, so each row proves a 202 dispatches clean.
- The empty-bodied action ops send `{}`: the spec marks `AgentReviewActionRequestPublicV2Dto` as a required body. `stop-assessment` is a DELETE with `{}` for the same reason, through a new `DrataClient::delete_with_body` (`src/client.rs`).
- Body verbs: `confirm-criteria --criterion-id <id>...` (omitted sends `{}`, the server's default set); `request-trust-center-access --fields <json object|@file|->` (wrapped as `{"fields": ...}`, a non-object is an error); `provide-trust-center-access-link --access-link`; `send-follow-up-questionnaire --email... --cc... --bcc... --email-subject --email-content`; `finalize --decision --note`. Each also takes `--data` (through `read_data`, conflicting with the field flags) and `--example` (from `spec::example_for_operation`). `add-documents --file <path>... [--type]` builds a `Multipart` with every file under `files`.
- Client-side limits from the spec fail before confirm: 1 to 10 `add-documents` files, 1 to 5 `--email`, at most 5 `--cc`/`--bcc`, `--decision` required on `finalize` (the spec: no default, unlike `run-action`).
- Enum flags reuse existing types whose values match the op exactly: `finalize --decision` is `SecurityReviewDecision` (= `SecurityReviewFinalizeDecisionEnum`), `add-documents --type` is `SecurityReviewDocumentType` (= the multipart `type` enum). `tests/enums.rs` gains `body_values` (reads a request-body field's enum, JSON or multipart) and `agent_body_enums_match_the_spec`.
- A 422 still fails the command through `ApiError` and `format_api_error`. `format_api_error` now appends `agent_guidance`: one `Blocker <reason>: <message> [<resolution type>: <action>]` line per blocker, `Available actions: ...` and `Current step: ...`, only when the body carries those keys | the agent's 422 (`AgentActionNotAvailableResponsePublicV2Dto`) says what to do next beside `message`, and printing only `message` would hide it.
- No new renderers. `agent_workflow_responses_print_whole_as_json` feeds every Phase 5 success response's spec shape through `pick_renderer` and asserts none claims it | none of the 17 shapes is a list of rows; a sniff stealing one would print a partial table of another resource. `spec_response_row` grew a general `spec_response_object(method, path, status, every_key)`; with `every_key` a property with no `example` is present as `null` (`summary/status` has no examples).
- Bodies are never logged in `agent::handle`: the access link and the access-request form are secrets.
- 17 rows added to `tests/commands/table.rs`; floor raised 121 -> 138.

### Deviations
- None from the doc's ops or rules. Same effect, different seam: the "a 422 from `finalizations` exits non-zero" criterion is proven in-process (`finalizations_422_fails_and_prints_the_servers_message` asserts `dispatch` returns `Err` with the server's message and one request); `main` returns that `Err`, which exits 1. The binary has no base-URL seam to point at wiremock, the same constraint Phase 2 and Phase 4 recorded.
- `format_api_error` gained the blocker/actions lines (above). The plan says only "printed through `format_api_error`"; the extension stays inside that function and changes nothing for a body without those keys (`plain_errors_carry_no_agent_guidance`).

### Tradeoffs
- Flat verbs under `security-review` vs an `agent` subgroup | the doc names `vendor security-review workflow` / `summary` directly, and the enum-derived names already mark the agent verbs. The cost is a long `security-review --help` (31 entries).
- One `start-assessment` verb for start and re-run vs two verbs | both actions are the same op; the help says it re-runs.
- `--fields` takes the whole form as one JSON object vs a repeatable `--field <name> <value>` | form values are a string, a boolean or a list of strings, and a two-token flag cannot say which without a type token. The form's names come from `access-request-requirements`.
- `--access-link` is a plain flag vs `@file`/stdin only | the doc's file-or-stdin rule covers Standard Connections credentials only. `--data @file` or `--data -` keeps the link out of shell history today; see Open questions.
- Recipient and file-count limits checked client-side vs leaving them to the server's 400 | same as Phase 1's run-action rule: a mutation the spec forbids fails before the confirm prompt.

### Open questions
- Should `provide-trust-center-access-link` accept the link only from a file or stdin, like the Standard Connections credentials? The spec says to treat it as a secret ("anyone with it can read those documents"), and as a flag it lands in shell history and the process table.

### Phase 5 follow-up: access link from a file or stdin only (supersedes the `--access-link` tradeoff and closes the open question above)
#### Design decisions
- `provide-trust-center-access-link` takes its body only as `--data <@file|->`, read through `read_data` with `DataSource::FileOrStdinOnly`; `--access-link <url>` is gone (`--access-link` is now a clap usage error, exit 2). Of the two shapes offered (`--access-link <@file|->` holding one URL, or `--data <@file|->` holding the JSON body) I chose `--data`. It reuses the one reader and rule the Standard Connections credentials get, and `--example` prints exactly the file to fill in (`{"accessLink": "..."}`) with no second format to explain.
- Inline JSON fails in `plan`, before confirm and before any request, with the shared `read_data` message naming `@file` and `-` (stdin). `--data` is `required_unless_present = "example"`.
- `check_access_link_body` (`src/resources/vendor/security/agent.rs`) requires an object with a non-empty string `accessLink` and its error names only the JSON kind, never the value. The confirm prompt shows method and path; `agent::handle` logs method and path only; `read_data` logs the file path, never the body.
- The command-table row now passes `--data @tests/fixtures/access-link.json` (`ACCESS_LINK_DATA` in `tests/commands/table.rs`). The fixture is a test input under `tests/`, not `docs/`.
- Tests: `access_link_inline_data_is_rejected_before_any_request` (wiremock sees zero requests, the error never contains the link), `access_link_file_data_sends_the_access_link_body`, and unit tests `access_link_rejects_inline_data_without_echoing_it`, `access_link_reads_its_body_from_a_file`, `access_link_body_needs_a_non_empty_link_and_never_quotes_it`, `access_link_has_no_inline_flag`. Each was run against its reverted change and failed.

#### Deviations
- None.

#### Tradeoffs
- `--data` holding JSON vs `--access-link` holding a bare URL | one reader and one rule for every secret body, at the cost of wrapping one URL in JSON; `--example` prints that wrapper.

#### Open questions
- None. The Phase 5 open question is closed: yes, file or stdin only.
