# Design Document: `vendor security-review` command group (Rust drata CLI)

**Author:** Scott Idler
**Date:** 2026-06-28
**Status:** Implemented
**Review Passes Completed:** 5/5 (+ external review panel: Architect/Gemini + Staff Engineer/Codex, findings incorporated 2026-06-28)

## Summary

Add a curated `vendor security-review` command group to the Rust `drata` CLI, nested under
the existing `vendor` command (parallel to `vendor questionnaire`). This **adds the missing
security-review and questionnaire-upload surfaces** that the `drata vendor questionnaire send`
workflow depends on: `send` requires both a `securityReviewId` and a `questionnaireId`, and the
curated CLI today can produce neither without dropping to `raw`. The group covers all 10
security-review operations in the embedded spec (the 5 the TS CLI exposes, plus 5 spec-only
extras), and follows the spec's multipart shape for file uploads rather than the TS CLI's JSON
passthrough.

## Problem Statement

### Background

The Rust `drata` CLI is a port of upstream TS `yorkeccak/drata-cli`, in `tatari-tv/pagerduty-cli`
house style (curated typed commands + a `raw` passthrough driven by an embedded OpenAPI spec).
A TS-vs-Rust output comparison (`docs/ts-vs-rust-comparison.md`) found field fidelity to be
perfect, but surfaced one material **workflow** gap (section 2.5): the entire
`vendor-security-reviews` resource is absent from the curated Rust surface.

### Problem

`drata vendor questionnaire send` (`src/resources/vendor.rs:295-321`) maps to
`POST /vendors/{id}/questionnaires`, whose body (`VendorQuestionnaireCreateRequestPublicV2Dto`)
requires **both** `securityReviewId` and `questionnaireId` (plus `email`, `emailContent`).
Today the curated surface can supply **neither**:
- `securityReviewId` - only obtainable via `raw POST /vendors/{id}/security-reviews`; no curated
  command creates a security review.
- `questionnaireId` ("Vendor Questionnaire ID") - the existing `vendor questionnaire list`
  (GET `/vendors/{id}/questionnaires`) returns these, but it was empty on every tenant vendor;
  questionnaires are created by the `security-questionnaires` upload endpoints, which are also
  uncurated.

So a user following the curated surface hits a dead end mid-workflow, and the fix requires more
than just `create`: it needs the security-review **create** *and* the **upload-questionnaire**
verbs, which is why the full group (not the minimal `create`) is in scope. See the Workflow
note below and Open Q3 for the exact `questionnaireId` origin.

### Goals

- Curated commands for every security-review operation in the embedded spec, so the
  questionnaire-send workflow is completable end-to-end without `raw`.
- Follow the spec's request shapes (multipart for file uploads), not the TS CLI's JSON
  passthrough, which contradicts the spec for the questionnaire-upload endpoints.
- Match existing house style exactly: clap derive surface, write-gating, confirm prompts,
  `--example` skeletons, `--output json|table|auto`, function-level debug logging, and
  `<mod>/tests.rs` unit tests.

### Non-Goals

- No spec changes. All 10 endpoints already resolve via the embedded spec and `raw`.
- No live verification of mutating paths in this work (the available credential is read-only;
  POST/PUT are write-gated and cannot be exercised until a write key exists). Correctness of
  mutating requests is asserted by unit tests on the constructed body/multipart, as the
  existing `vendor` commands are.
- No table renderer is *required* (JSON fallback works); a renderer is an optional polish phase.

## Proposed Solution

### Overview

`vendor.rs` is the module *file*; the `src/resources/vendor/` directory exists only to hold
`vendor/tests.rs` (resolved by `mod tests;` in `vendor.rs:341-342`). It is not a module
directory (`vendor/mod.rs`). Declaring `mod security;` in `vendor.rs` resolves to
`src/resources/vendor/security.rs`, so no file move is needed: just add that file and the
`mod security;` line. Add a
`SecurityReview(VendorSecurityReviewAction)` variant to the existing `VendorAction` enum so the
surface is `drata vendor security-review <verb>`, dispatched from `vendor::handle`. No new
top-level `Commands` variant is needed; this mirrors how `vendor questionnaire` already nests.

### Target workflow (what "closed" looks like)

End-to-end "send a security questionnaire to a vendor", entirely curated:

1. `vendor security-review create <vendor-id> --review-deadline-at .. --status .. --type ..`
   -> returns a `securityReviewId`. **(this group adds this)**
2. `vendor questionnaire send <vendor-id> --security-review-id <sr> --questionnaire-id <q>
   --email .. --email-content ..` -> emails an existing questionnaire **template** to the vendor,
   linked to the review. `<q>` is a template id built in the Drata product (Resolved Q3); it is
   listed by `vendor questionnaire list` once templates exist. **(already exists)**

Separately, `vendor security-review upload-questionnaire` / `upload-questionnaire-to-review`
capture a **completed** questionnaire that came back outside Drata (file upload) - a different
operation from sending a template, and also added by this group.

### Architecture

Components touched (all existing patterns):

- **clap surface** (`src/cli.rs`): new `VendorSecurityReviewAction` enum + two `ValueEnum`s
  for the status/type enums; a new `SecurityReview` arm on `VendorAction` (`:232-314`),
  modeled on the existing `Questionnaire` arm (`:316-350`).
- **dispatch** (`src/resources/vendor.rs`): `handle` (`:52`) gains a `SecurityReview(sub)` match
  arm that calls `security::handle(sub, client, config, confirm).await?`, mirroring the existing
  `Questionnaire` arm. `example_if_requested` (`:45`) gains a `SecurityReview(sub)` arm that
  delegates to `security::example_if_requested`. **`src/lib.rs` needs no change** -
  `Commands::Vendor { action } => resources::vendor::example_if_requested(action)` already routes
  there (`src/lib.rs:31`).
- **handlers** (`src/resources/vendor/security.rs`): one async fn per verb. Per `logging.md`,
  each emits a `debug!` on **entry** (fn name + params) and on **exit** records the outcome
  (status/count) at DEBUG, with errors propagating at WARN/ERROR - not entry-only (the existing
  vendor handlers log entry only, `vendor.rs:181`; the new handlers should do both). Never log
  file contents or the key (previews/lengths only). Then call the HTTP client and `print_value`.
- **HTTP client** (`src/client.rs`, unchanged): `get` (`:438`), `post` (`:457`),
  `put` (`:462`), `post_multipart` (`:426`), `Multipart` builder (`:72-117`). Write-gating in
  `send_inner`/`send_multipart` fails any non-GET closed unless `--allow-writes`.
- **output** (`src/output.rs`, unchanged for JSON; optional `render_security_reviews` in
  `src/output/table.rs` for table mode).

### Data Model

Three `ValueEnum`s, modeled on `RiskTreatmentPlan` (`src/cli.rs:356-363`). The status/type
enums are SCREAMING_SNAKE_CASE per the spec; the action enum is lowercase. All accept
case-insensitive input per `cli.md`:

```rust
#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
enum SecurityReviewStatus { NotYetStarted, InProgress, Completed, NotRequired }

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "SCREAMING_SNAKE_CASE")]
enum SecurityReviewType { Security, SocReport, UploadReport }

#[derive(ValueEnum, Clone, Debug)]
#[clap(rename_all = "lowercase")]
enum SecurityReviewAction { Finalize, Reopen }   // POST /actions body: {"action": ...}
```

Serialized strings confirmed against the spec: status
`NOT_YET_STARTED|IN_PROGRESS|COMPLETED|NOT_REQUIRED`, type `SECURITY|SOC_REPORT|UPLOAD_REPORT`,
action `finalize|reopen`. Pin all three in a unit test so a future spec bump can't drift them.

**Body-key mapping (wiring caveat):** the CLI flags `--status` / `--type` map to body keys
`securityReviewStatus` / `securityReviewType` (the create DTO uses the long names,
`spec` `VendorSecurityReviewCreateRequestPublicV2Dto`). The handler must translate, not pass the
flag names through verbatim.

**ID typing.** Use `vendor_id: String` (every existing `vendor`-tree command does:
`src/cli.rs:298,321,326,333`). For `security_review_id`, use `u64` to match the existing
`vendor questionnaire send` (`src/cli.rs:342`), whose `--security-review-id` this group feeds.
(The existing tree is internally inconsistent - `questionnaire_id` is `String` at `:328` but
`u64` at `:339`; `vendor_id` as `String` and `security_review_id` as `u64` is the
lowest-surprise choice for the new group given those two anchors.)

### API Design

Surface: `drata vendor security-review <verb>`. All paths are under `/vendors/{vendorId}`.

| Verb | Method + path | Body | Notes |
|------|---------------|------|-------|
| `list <vendor-id>` | GET `/security-reviews` | - | spec-only; cursor-paginated, filters `--status`/`--type`, plus `--expand`, `--all` (per house style, `src/cli.rs:235`) |
| `create <vendor-id>` | POST `/security-reviews` | JSON | required: `--review-deadline-at`, `--status`, `--type`; optional: `--title`, `--note`, `--requested-at`, `--requester-user-id`. Body keys: `securityReviewStatus`/`securityReviewType` (see Data Model) |
| `create-with-file <vendor-id> --file <path>` | POST `/security-reviews/with-file` | multipart | required: `--file`, `--title`, `--review-deadline-at`, `--status`, `--type`; optional `--document-type`, `--note`, `--requested-at`, `--requester-user-id`. **Distinct required set from `create`** (`--title` + `--file` required here); see Open Q1 |
| `get <vendor-id> <sr-id>` | GET `/security-reviews/{srId}` | - | supports `--expand` |
| `update <vendor-id> <sr-id>` | PUT `/security-reviews/{srId}` | JSON | spec-only; UpdateDTO has **only** `--title` and `--soc-form` - NOT create's fields |
| `actions <vendor-id> <sr-id>` | GET `/security-reviews/{srId}/actions` | - | spec-only; list actions |
| `run-action <vendor-id> <sr-id> --action finalize\|reopen` | POST `/security-reviews/{srId}/actions` | JSON | spec-only; body `{"action": finalize\|reopen}` (typed `--action` ValueEnum) |
| `questionnaires <vendor-id> <sr-id>` | GET `/security-reviews/{srId}/security-questionnaires` | - | spec-only; list questionnaires on a review |
| `upload-questionnaire <vendor-id> --file <path>...` | POST `/security-questionnaires` | multipart (confirmed) | captures **completed** responses; required `files` array (vendor-level, no sr-id) |
| `upload-questionnaire-to-review <vendor-id> <sr-id> --file <path>...` | POST `/security-reviews/{srId}/security-questionnaires` | multipart (confirmed) | captures **completed** responses; required `files` array |

For JSON-body verbs that have many optional fields (`create`, `update`), follow the existing
pattern: typed flags for the common/required fields plus `--data <json>` for the full body and
`--example` to print a skeleton. Because `vendor::example_if_requested` returns
`Option<&'static str>` (hand-written, infallible - `src/resources/vendor.rs:45-50`), the
security-review skeletons are **hand-written static strings**, matching the vendor pattern
(not spec-derived). `run-action` takes a typed `--action finalize|reopen` (ValueEnum), so it
needs no `--data`/`--example`. Multipart `files` uses repeated `--file` (space-separated/repeated,
`num_args=1..`, never `value_delimiter`, per `cli.md`).

Concrete clap shape (derive), modeled on `VendorQuestionnaireAction` (`src/cli.rs:316-350`):

```rust
#[derive(Subcommand, Clone, Debug)]
enum VendorSecurityReviewAction {
    /// List security reviews for a vendor
    List {
        vendor_id: String,
        #[arg(long, value_enum, ignore_case = true)] status: Option<SecurityReviewStatus>,
        #[arg(long = "type", value_enum, ignore_case = true)] review_type: Option<SecurityReviewType>,
        #[arg(long, num_args = 1..)] expand: Vec<String>,
        #[arg(long)] all: bool,
    },
    /// Create a security review
    Create {
        vendor_id: String,
        #[arg(long)] review_deadline_at: String,
        #[arg(long, value_enum, ignore_case = true)] status: SecurityReviewStatus,
        #[arg(long = "type", value_enum, ignore_case = true)] review_type: SecurityReviewType,
        #[arg(long)] title: Option<String>,
        #[arg(long)] note: Option<String>,
        #[arg(long)] requested_at: Option<String>,
        #[arg(long)] requester_user_id: Option<u64>,
        #[arg(long)] data: Option<String>,
        #[arg(long)] example: bool,
    },
    Get { vendor_id: String, security_review_id: u64, #[arg(long, num_args = 1..)] expand: Vec<String> },
    Update { vendor_id: String, security_review_id: u64, #[arg(long)] title: Option<String>, #[arg(long)] soc_form: Option<String> },
    RunAction { vendor_id: String, security_review_id: u64, #[arg(long, value_enum, ignore_case = true)] action: SecurityReviewAction },
    // Actions, Questionnaires (GETs),
    // CreateWithFile { vendor_id: String, .., #[arg(long, num_args = 1..)] file: Vec<PathBuf> },
    // UploadQuestionnaire / UploadQuestionnaireToReview { .., file: Vec<PathBuf> }
}
```

### Implementation Plan

#### Phase 1: clap surface + dispatch wiring
**Model:** sonnet
- Add `mod security;` to `src/resources/vendor.rs` and create
  `src/resources/vendor/security.rs` (the `vendor/` dir already exists) with a `handle` stub +
  `example_if_requested` stub. No file move.
- Add `VendorSecurityReviewAction` enum and the two `ValueEnum`s to `src/cli.rs`; add the
  `SecurityReview` arm to `VendorAction`.
- Add the `SecurityReview(sub)` arms to `vendor::handle` (dispatch) and
  `vendor::example_if_requested` (delegate). `src/lib.rs` unchanged.
- Acceptance: compiles, `drata vendor security-review --help` and each verb `--help` render.

#### Phase 2: Read + JSON-create handlers
**Model:** sonnet
- Implement `list` (with `--status`/`--type` filters + `--expand`/`--all`), `get` (`--expand`),
  `actions`, `questionnaires` (GETs) and `create`, `update`, `run-action` (JSON POST/PUT) with
  confirm gate + write guardrail reuse (`confirm("POST", &path)?`, `src/resources/vendor.rs:182`).
  `update` builds a body with **only** `title`/`socForm`; `run-action` sends `{"action": ...}`
  from the typed `--action`.
- Unit tests in `src/resources/vendor/security/tests.rs` (declare `mod tests;` in `security.rs`,
  per the `<mod>/tests.rs` convention): body building, enum->string mapping, `set_opt` behavior,
  `--example` gating. Mirror `vendor/tests.rs` (pure unit, no wiremock).

#### Phase 3: Multipart handlers
**Model:** opus
- Implement `upload-questionnaire` and `upload-questionnaire-to-review` (multi-file `files`
  array via repeated `add_file("files", path)`, `src/client.rs:91-97`) and `create-with-file`
  (single `file` part + its **own** required set: `--file`, `--title`, `--review-deadline-at`,
  `--status`, `--type`; modeled on `evidence.rs:143-183`).
- Resolve Open Q1 (with-file content-type) before finalizing; add the actionable `415` error
  path so a multipart-vs-JSON rejection is diagnosable.
- Unit tests asserting multipart field/file part shapes.

#### Phase 4: Output + polish
**Model:** sonnet
- Optional `render_security_reviews` renderer + `pick_renderer` sniff branch
  (`src/output/table.rs:47-138`); module doc comment; `--example` skeletons; final tests.
- Update `docs/ts-vs-rust-comparison.md` section 2.5 / coverage table to reflect the closed gap.

## Alternatives Considered

### Alternative 1: Top-level `drata vendor-security-review` command
- **Description:** New top-level `Commands::VendorSecurityReview` variant (as the research brief
  initially sketched), mirroring TS's top-level `vendor-security-reviews`.
- **Pros:** 1:1 with the TS command name; flat dispatch in `src/lib.rs`.
- **Cons:** Diverges from the established `vendor questionnaire` nesting precedent; security
  reviews are a vendor sub-resource and read more naturally under `vendor`.
- **Why not chosen:** Nesting under `vendor` is more consistent with the existing surface and
  with the chosen module layout (`resources/vendor/`).

### Alternative 2: Copy the TS JSON passthrough for questionnaire uploads
- **Description:** POST `--data <json>` to the two `security-questionnaires` endpoints, as TS does.
- **Pros:** Mirrors the cited TS reference exactly.
- **Cons:** Contradicts the spec, which declares both endpoints `multipart/form-data` with a
  required `files` array; almost certainly fails against the real endpoint (the TS behavior is
  likely a bug).
- **Why not chosen:** The port should be spec-faithful and correct, not bug-compatible.

### Alternative 3: Match only the 5 TS verbs
- **Description:** Implement only create/create-with-file/get/upload-questionnaire/
  upload-questionnaire-to-review.
- **Pros:** Smallest surface.
- **Cons:** Leaves list/get-by-review/update/actions uncurated when they are cheap to add once
  the module exists, and they round out the resource.
- **Why not chosen:** Full spec coverage chosen; the extras are low marginal cost.

## Technical Considerations

### Dependencies
No new crates. Reuses `clap`, the existing `client`, `Multipart`, `confirm`, `output`, `spec`.

### Performance
Mostly thin CLI over single HTTP requests, with **one real memory concern** (flagged by both
reviewers): `send_multipart` (`src/client.rs:329-345`) reads each file **fully into memory** to
support request retry. The questionnaire-upload verbs accept a multi-file `files` array, so
uploading several large SOC2 PDFs at once could OOM a constrained CI runner. Mitigation: this is
existing client behavior shared by every multipart command, not new; the new verbs do not worsen
the per-file cost. If it becomes a problem, the fix belongs in the client (stream + disable retry
for large bodies), not this group. `list` is cursor-paginated and `--all` streams pages like other
Rust lists; security-review counts are small in practice.

### Security
All mutating verbs (create/create-with-file/update/run-action/upload-*) are write operations,
fail-closed under the existing write guardrail unless `--allow-writes`, and prompt via the
confirm gate (bypass with `--yes`). No new credential handling. Never log file contents or the
API key (logging.md: previews/lengths only).

### Testing Strategy
Unit tests in `<mod>/tests.rs` mirroring `vendor/tests.rs` (pure: body/multipart construction,
enum mapping, example gating). End-to-end HTTP behavior (write-gating, multipart wire format)
is already covered by `src/client/tests.rs`. No wiremock added for this module. `otto ci` green
(exit 0 + "All CI checks passed!").

### Rollout Plan
Ships in the next release via the standard gated flow (`bump --no-tag` -> PR -> admin-squash ->
`bump --tag-only` -> push tag by name). No migration; purely additive surface.

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| `create-with-file` rejected as `415` (multipart vs JSON) | Low | Med | Only `with-file` is unconfirmed (upload-questionnaire is confirmed multipart); on `415` emit actionable error pointing at `raw --data <json>` (Open Q1b); least-critical verb; verify live when write key exists |
| Multipart in-memory buffering OOM (large multi-file uploads) | Med | Med | Existing client behavior, not worsened here; bound file count or fix in client (stream + no-retry) if it bites |
| Update body shape mismatch (using create's fields) | Med | High | UpdateDTO is only `title`+`soc_form`; pin in a unit test; clap surface enforces it |
| Mutating paths unverifiable now (read-only key) | High | Low | Unit-test body/multipart shape; matches how existing write verbs ship |
| Tests submodule path confusion (`vendor.rs` + `vendor/`) | Low | Low | No file move; only add `mod security;` + `vendor/security.rs`; `otto ci` catches breakage |
| Enum serialized strings wrong | Low | Med | Pin all three enums against spec values in a unit test |

## Open Questions
- [x] **Q1a: `upload-questionnaire` content-type. RESOLVED.** Drata's official API reference
  (developers.drata.com, Vendor-Security-Reviews tag) confirms both
  `POST /vendors/{vendorId}/security-questionnaires` and
  `POST /vendors/{vendorId}/security-reviews/{securityReviewId}/security-questionnaires` are
  **`multipart/form-data` with a `files` array** (accepted: .pdf .docx .odt .doc .xlsx .ods .pptx
  .odp .csv). The design's multipart choice for these is correct.
- [ ] **Q1b: `create-with-file` content-type (narrowed, low-stakes).** The official reference,
  like the embedded spec, still shows `POST /vendors/{vendorId}/security-reviews/with-file` as
  `application/json` with a single binary `file` field - the same self-inconsistency (the
  reference is auto-generated from the spec, so it doesn't independently resolve it). Design treats
  it as multipart (single `file` part), matching the confirmed-multipart questionnaire endpoints;
  on a `415`, emit an actionable error. This is the *only* remaining content-type unknown and it
  is the least critical verb (it is pure convenience: `create` + a separate upload achieves the
  same result). Settle with a live write-key probe in Phase 3.
- [x] **Q2: POST `/actions` body. RESOLVED.** The checked-in spec defines
  `SecurityReviewActionRequestPublicV2Dto` as `{"action": finalize|reopen}`
  (`SecurityReviewActionEnum`), confirmed by the official reference. Implemented as a typed
  `--action` ValueEnum. (Live *behavior* of finalize/reopen is unverifiable without a write key,
  but the request shape is pinned.)
- [x] **Q3: `questionnaireId` origin. RESOLVED.** Drata's help docs state: *"questionnaires are
  templates. You build and manage them once, then send them to vendors as part of a security
  review."* So `questionnaireId` is an existing **questionnaire-template** id (built in the Drata
  product / account level), NOT something the `upload-questionnaire` endpoints produce - those
  capture *completed responses received outside Drata* (a different purpose). The send workflow is
  therefore fully API-curatable: create a security review (this group) + send an existing template
  (`vendor questionnaire send`). The empty `vendor questionnaire list` on the Tatari tenant means
  **no questionnaire templates have been built yet**, not a CLI gap. Caveat: building a template
  itself is a product action; check whether a template-create API exists if CLI template
  management is later desired (out of scope here).

## References
- `docs/ts-vs-rust-comparison.md` (section 2.5, coverage table)
- `docs/design/2026-06-27-architecture.md` + `-implementation-notes.md`
- TS reference: `~/repos/yorkeccak/drata-cli/dev/src/commands/vendor-security-reviews/index.ts`
- Embedded spec: `spec/drata-openapi-v2.json` (paths matching `security-review|security-questionnaire`)
- Existing analogs: `src/resources/vendor.rs` (questionnaire subgroup), `src/resources/evidence.rs:143-183` (multipart create)
- Drata API reference (content-types, Q1/Q2): https://developers.drata.com/openapi/reference/v2/tag/Vendor-Security-Reviews/
- Drata help (questionnaire-template lifecycle, Q3): https://help.drata.com/en/articles/13557330-create-and-manage-vendor-questionnaires and https://help.drata.com/en/articles/9676307-start-and-manage-security-reviews-for-your-vendors
