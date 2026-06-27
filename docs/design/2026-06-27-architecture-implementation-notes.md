# Drata CLI - implementation notes

Append-only. One section per phase.

## Phase 1: Harness + one vertical

### Design decisions

- **Region selection in the client, not stored as a URL.** `client::base_url_for_region`
  maps `us`/`eu`/`apac` to the three spec `servers` URLs; `DrataClient::new` takes a
  region string and resolves it. Unknown regions fall back to US (fail-safe to the
  primary region rather than erroring). Rationale: the design doc says base URLs come
  from the spec `servers` block; keeping the mapping in one function makes adding a
  region a one-line change.
- **Write guardrail lives in `client::send_inner`.** Every verb (`get/post/put/delete/raw`)
  funnels through `send_inner`, so the `method != GET && !allow_writes` check there covers
  all current and future mutating paths, including the Phase 2 `raw` namespace. The flag is
  a constructor argument (`DrataClient::new(.., allow_writes)`) sourced from the resolved
  credential, never from the profile name - per the review-panel hardening. A dedicated
  `client::WriteGuardError` is returned so callers/tests can `downcast_ref` it.
- **`allow_writes` resolution in `config::Config::load`.** Writes are enabled only when the
  CLI `--allow-writes` flag is set OR the resolved credential is a write-enabled *profile*.
  A key resolved from `--api-key`/`DRATA_API_KEY` (CliFlag/EnvVar `TokenSource`) does NOT
  inherit a profile's write flag - so a stray `DRATA_API_KEY` cannot silently mutate. This
  is the "property of the key, not the profile name" rule made concrete.
- **Credentials format: `{ "profiles": { name: {api-key, region, allow-writes} } }`.**
  `config::Credentials` is a `BTreeMap<String, Profile>` (deterministic key order on
  serialize). serde `rename_all = "kebab-case"` translates `api-key`/`allow-writes` on disk
  to snake_case in Rust, matching the project's config-key convention.
- **Legacy migration via two-shot parse.** `config::Credentials::parse` first tries the
  profiles shape; if that yields no profiles, it tries the flat legacy
  `{api-key, region}` shape and wraps it in a `default` profile, returning a `migrated`
  flag. `Credentials::load` persists the upgraded shape atomically when `migrated` is true
  ("record once").
- **Atomic 0600 write.** `config::atomic_write_0600` writes to `.credentials.<pid>.tmp` in
  the credentials directory, fsyncs, sets mode 0600 on the temp, renames over the target,
  then re-enforces 0600 on the final path. Per the filesystem-safety rule (no torn writes;
  temp in the target's own dir for a same-filesystem rename).
- **Generic table dispatch by field-sniffing.** Drata wraps every list in the same
  `{ "data": [...] }` envelope, so `output::table::render` cannot dispatch on an envelope
  key like pagerduty-cli does. `output::table::pick_renderer` instead inspects the first
  row's fields (vendor: has `category` + `risk`; questionnaire: has `recipientEmail` /
  `isCompleted`). Unknown shapes return `None` -> JSON fallback. Empty `data` renders
  `(no results)` rather than `[]`.
- **`scalar_field` for numeric IDs.** Drata IDs are JSON numbers, not strings, so the table
  renderer needs `output::table::scalar_field` (handles number/string/bool) in addition to
  pagerduty-cli's string-only `str_field`.
- **Vendor bodies built incrementally.** `resources::vendor::set_opt` inserts a key only
  when the CLI flag was provided, so `update` sends a sparse body and does not clobber unset
  fields with nulls.

### Deviations

- **No `colored`/`env_logger`/`log`/`serde_yaml` dependencies.** The scaffold shipped these;
  Phase 1 replaces logging with `tracing` + `tracing-subscriber` (per the design doc's
  Errors & logging section) and removes the now-unused `colored` and `serde_yaml`. The old
  toy `Config { name, age, debug }` and toy `run()` were replaced wholesale, as the task
  instructed.
- **reqwest TLS feature is `rustls`, not the scaffold/PD default-tls.** reqwest 0.13 renamed
  features; `rustls` (with `http2`, `json`, `charset`) avoids a system OpenSSL dependency.
  Pure build-system choice, no behavioral impact on the API surface.
- **Dropped `#[cfg(not(unix))]` permission stubs.** The first lint pass flagged the `_f` /
  `_path` parameters in the non-unix stubs of `set_mode_0600`/`enforce_mode_0600` (the
  `_varname` rule). Since the tool targets Linux/macOS only, the unix implementations are now
  unconditional rather than carrying dead non-unix stubs. A Windows port would need them back.
- **Sample `drata-cli.yml` rewritten.** The scaffold's sample had fake `name/age/debug`
  fields the code never reads. It now documents the credentials.json profile shape and
  precedence instead (the `--config` flag remains reserved; credentials live in JSON).

### Tradeoffs

- **Field-sniffing table dispatch vs. an explicit per-call resource hint.** Sniffing keeps
  the renderer decoupled from the call site (the resource module just emits `{ "data": [...] }`),
  but two resources with overlapping field sets could be misrouted. Acceptable now (vendors
  vs. questionnaires are disjoint); if Phase 3 hits a collision, callers can pass an explicit
  resource tag into `print_value`.
- **`get_all` requests `size=50` per page.** Larger pages mean fewer round-trips but a bigger
  buffered `Vec<Value>`. The design doc defers `--all` NDJSON streaming to Phase 4, so Phase 1
  buffers; 50 is a middle-ground page size. `MAX_PAGES = 10_000` caps a pathological tenant at
  ~500k buffered items before bailing with a warning.
- **Repeated-cursor guard uses a `HashSet<String>` of seen cursors.** O(pages) memory, but
  pages are bounded by `MAX_PAGES` and cursors are short; simpler and stricter than only
  comparing against the immediately preceding cursor (catches an A->B->A cycle, not just
  A->A).
- **rustls over native-tls** trades a slightly larger dependency tree for no system OpenSSL
  requirement and reproducible builds across the team's machines.

### Open questions

- **Drata JSON casing is camelCase (confirmed).** The design doc's open question - camelCase
  vs snake_case - resolves to **camelCase** for the vendor and questionnaire endpoints I
  inspected in `spec/drata-openapi-v2.json`: list response fields are `renewalDate`,
  `recipientEmail`, `isCompleted`, `dateSent`; the questionnaire-send body uses
  `questionnaireId`, `securityReviewId`, `emailContent`, `emailSubject`. The vendors create
  body, however, uses `contactEmail` (singular) while the vendor *response* uses
  `contactsEmail` (plural) - a real inconsistency in the spec to watch when Phase 3 wires up
  more vendor write fields. Phase 3+ modules should NOT assume a global serde
  `rename_all = "camelCase"` works for every endpoint; confirm per-endpoint.
- **Vendor `category`/`risk`/`status` are spec enums** (e.g. risk: NONE/LOW/MODERATE/HIGH;
  status: PROSPECTIVE/ACTIVE/ARCHIVED/...). Phase 1 accepts them as free strings on the CLI
  for flexibility and to avoid the API rejecting a value we failed to model. Should these
  become `clap::ValueEnum` flags (case-insensitive, validated client-side) in Phase 3? That
  buys `--help` discoverability and tab-completion at the cost of churn when Drata adds an
  enum value.
- **Questionnaire-send requires `securityReviewId` and `questionnaireId`** (both numeric, both
  required by the spec). There is no curated command yet to *list* available questionnaire
  templates or security reviews, so a user must already know those IDs. Worth a convenience
  lookup (or curating the Vendor Security Reviews tag) in a later phase - confirm priority.
