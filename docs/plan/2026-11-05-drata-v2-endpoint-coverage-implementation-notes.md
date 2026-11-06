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
