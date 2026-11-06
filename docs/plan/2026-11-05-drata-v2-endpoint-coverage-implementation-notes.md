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
