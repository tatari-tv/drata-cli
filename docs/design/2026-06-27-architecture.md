# Drata CLI - architecture

- Status: draft (revised post-review-panel)
- Date: 2026-06-27
- Owner: Scott Idler
- Repo: `tatari-tv/drata-cli` (Rust port of the upstream TypeScript `drata-cli`)

## Summary

Port the Drata compliance CLI to Rust in the `pagerduty-cli` house style:
hand-written, typed, safe-by-default resource commands for the common surface,
plus a gated generic `raw` namespace for full coverage of all 167 operations.
`serde_json::Value` is the wire currency (typed structs only where they earn
their keep, exactly as PD does); the committed OpenAPI v2 spec
(`spec/drata-openapi-v2.json`: 110 paths, 167 operations, 35 operation-level
tags, 434 schemas) anchors correctness, `--example` skeletons, and a coverage
test. Writes fail closed: a credential must be explicitly write-enabled, and all
mutating methods confirm.

This supersedes an earlier draft that proposed a generic spec-driven dispatch
with `--data`-only writes. A cross-model review panel (Architect/Gemini,
Staff-Engineer/Codex) converged on rejecting that in favor of curated typed
commands; this revision adopts their recommendation.

## Context

### The upstream TypeScript CLI

Thin REST wrapper: ~3,600 lines of TS, a 133-line `fetch` wrapper, the same
`list/get/create/update/remove` template across 34 resources. Hand-written, no
codegen, untyped (`Record<string, unknown>` / `as any`), no tests, covers ~19 of
110 path templates. Mutations are raw `--data '<json>'` passthrough.

### The asset we have

A complete OpenAPI 3.0.0 spec, extracted from Drata's Redocly docs portal and
committed at `spec/drata-openapi-v2.json` (all three region servers, every
endpoint's params/bodies/response schemas). Narrative guides are under `docs/`.

### Reference implementation: `pagerduty-cli`

`tatari-tv/pagerduty-cli` (~10k src + ~3k test lines) is the template for this
port. Its conventions are harvested below and treated as binding. Files to lift
as starting templates: `client.rs`, `config.rs`, `filter.rs` (reusable nearly
verbatim), `output.rs` + `output/table.rs`, a canonical CRUD resource module
(`resources/service.rs`), plus `.otto.yml`/`build.rs`/`rustfmt.toml`/`clippy.toml`.

## Decision drivers

- The tool's job includes real writes (creates that email vendors, policy acks,
  HRIS upserts), so write-side UX and safety matter as much as reads.
- The author maintains his own tools; large generated modules he cannot read are
  out. But hand-typing 434 response schemas is equally unwanted.
- Drata publishes no sandbox; accidental prod mutation must be hard.
- We want more coverage than the TS subset, ideally all 167 ops.

## Decision

**Option B - curated typed commands + gated `raw` namespace, PD house style.**

1. **Curated, typed resource commands** for the common surface. Each tag is a
   hand-written module with a clap `*Action` enum and typed `Create`/`Update`
   args - the PD pattern (`ServiceAction`, `IncidentTriggerAction`). This gives
   `--help`, completion, and client-side validation of required fields on the
   write path.
2. **`serde_json::Value` as the wire currency.** The client returns `Value`;
   request bodies are built with `json!` and extended conditionally. Typed
   structs only for `--from-file` shapes and GET-mutate-PUT round-trips, exactly
   as PD does. No 434-struct codegen.
3. **A gated generic `raw` namespace** (`drata raw <METHOD> <path> ...`) reaches
   any of the 167 operations for the long tail and power users. Non-GET through
   `raw` is subject to the same write guardrail.
4. **The spec anchors correctness, not structure.** It drives `--example`
   skeletons (from request schemas), and a coverage test asserting every
   operation is reachable (curated or `raw`).

### Rejected alternatives

- **Full OpenAPI codegen (progenitor, ~600 types).** A large unreadable module
  that churns on every spec bump; the CLI touches a fraction of it and would
  still hand-map each method onto clap. Rejected (and rejected again here).
- **Generic spec-driven dispatch with `--data`-only writes (prior draft).** Full
  coverage cheaply, but no write-side validation/discoverability ("typed curl"),
  and "mirrors pagerduty-cli" was simply false - PD hand-writes typed write args.
  Demoted to the `raw` namespace (the escape hatch), not the primary surface.

## Architecture (pagerduty-cli house style, applied)

### Module layout

```
src/
  main.rs            thin: tracing + config + dispatch; crate-level denies live here
  lib.rs             pub mod list + run()/run_auth(); single match on cli.command
  cli.rs             ALL clap derive structs/enums, zero logic
  client.rs          DrataClient (reqwest) -> Value; ApiError; pagination; encode_query
  config.rs          Config/ConfigFile/AuthDiagnostic; own xdg_* helpers; profiles
  filter.rs          3-tier positional matcher (exact -> starts-with -> contains)
  output.rs          print_value(): json/table/auto + optional pager
  output/table.rs    per-resource column renderers + shrink-to-width engine
  resources.rs       `pub mod` list only
  resources/<tag>.rs              one file per tag
  resources/<tag>/{crud,...}.rs   2018-style module split for complex tags
  raw.rs             generic METHOD+path passthrough namespace
```

- `main.rs` carries `#![deny(clippy::unwrap_used)]`, `#![deny(dead_code)]`,
  `#![deny(unused_variables)]`; it only parses CLI, handles pre-config bypasses
  (`--example`, `drata auth`), loads config, sets up tracing, calls `lib::run`.
- `lib::run` is one `match &cli.command`, each arm calling
  `resources::<tag>::handle(action, &client, config).await?`.
- One file per tag; complex tags decompose into a 2018-style module split by
  verb-group/sub-noun (PD's `incident.rs` + `incident/`), never a mega-file.
- Each module exposes `pub async fn handle(action, client, config)` matching its
  `*Action` enum, calling one private `async fn` per verb, converting
  `Option<String>` -> `Option<&str>` with `as_deref()` at the boundary.

### CLI (clap)

- All clap types in `cli.rs`. Two-level tree: `Commands` enum (one variant per
  tag) -> per-tag `*Action` subcommand enum -> verbs (`List/Get/Create/Update/
  Remove`) + nested sub-resource action enums (PD's `ServiceIntegrationAction`).
  This is where the spike's 23 naming collisions dissolve - nesting is explicit.
- Version = `env!("GIT_DESCRIBE")` (build.rs). `after_help` advertises credential
  sources + log path.
- Positional IDs = bare `String`; filter patterns = bare `Vec<String>`. List
  flags = `#[arg(long, num_args = 1..)]` or repeated `value_enum` - **never
  `value_delimiter = ','`** (per the author's CLI rule; PD's one comma flag is a
  known bug, not the pattern). Renamed flags via `#[arg(long = "...")]`. Enum
  flags via `#[derive(clap::ValueEnum)]`.
- Every `Create` gets a bool `--example` that prints a skeleton (generated from
  the spec request schema) and exits before config/auth load. Body-bearing
  verbs also accept `--from-file <path|->` (stdin); CLI flags override file
  fields. The 10 multipart upload ops take `--file <path>`.

### HTTP client

- `DrataClient` { reqwest `Client` (explicit `.timeout`), `base_url`, `api_key`,
  `Option<Cache>` }; `with_base_url` is the wiremock test seam. Everything
  returns `serde_json::Value`. Verb wrappers (`get/post/put/delete/raw`) funnel
  through one `send_inner`, all `#[instrument]`.
- Auth: `Authorization: Bearer <key>`. Three region base URLs (us/eu/apac) from
  the spec `servers`.
- Library error = one `thiserror` `ApiError { status, body, formatted }` so
  callers `downcast_ref` from eyre; `try_get` -> `Ok(None)` on 404.
  `format_api_error` parses Drata's error envelope, names the failing
  `METHOD url`, never collapses to "Unknown error". Retry 429 with `Retry-After`
  (bounded, named consts); 204 -> `Value::Null`. Hand-rolled `QUERY_ENCODE_SET`
  + `encode_query()` at every query param.
- **Pagination (cursor, Drata v2):** loop sending `cursor`, accumulate `data[]`,
  stop when `pagination.cursor` is null. Hardened per review: repeated-cursor
  detection (abort on no-progress), a max-page bound, retry/backoff and 204
  handling inherited from `send_inner`. `--all` drains; for large tenants it
  **streams NDJSON** rather than buffering an unbounded `Vec<Value>`.

### Domain models

- Default to `Value` + `json!`, building bodies incrementally:
  `if let Some(x) = opt { body["k"] = json!(...) }`. Hand-write structs only for
  (a) `--from-file` YAML shapes and (b) GET-mutate-PUT round-trips. API structs
  use `#[serde(rename_all = "snake_case")]` (Drata is camel/snake - confirm per
  endpoint); YAML structs use `kebab-case`; optionals get
  `#[serde(default, skip_serializing_if = "Option::is_none")]`.

### Output

- `print_value(value, format)`: `Json` always JSON; `Table`; `Auto` =
  `!stdout().is_terminal()` (JSON when piped). Table = envelope-key dispatcher in
  `output/table.rs`; each resource is one fn declaring `&["HEADERS"]` + a slice
  of field-extractor closures (`str_field`/`nested_str`/`bool_field`, defaulting
  to `""`). Char-boundary-safe `…` truncation; shrink-widest-unprotected-column
  to fit; pretty-JSON fallback for unknown shapes. `$PAGER` only when TTY and
  output overflows.

### Config / credentials (extends PD; net-new, no PD precedent)

- Define our own `xdg_config_dir`/`xdg_data_dir` (env then `$HOME`), **never**
  `dirs::config_dir()` (the macOS bug PD documents). Cache dir may use
  `dirs::cache_dir()`.
- Credentials at `xdg_config_dir()/drata/credentials.json`, mode **0600**, with
  **profiles** (the TS CLI had these; PD did not). Each profile:
  `{ api_key, region, allow_writes: bool }`. Atomic write + permission
  enforcement + the legacy single-key -> profiles migration, all tested.
- Precedence CLI > env (`DRATA_API_KEY`/`DRATA_REGION`/`DRATA_PROFILE`) > file.
  A `TokenSource`-style enum records which layer won, surfaced by `drata auth`.
- A token-free `AuthDiagnostic` view backs `drata auth` on fresh installs.

### The `raw` namespace

`drata raw <METHOD> <path> [--query k=v ...] [--data <json|@file|->] [--file f]`
hits the active base URL for any of the 167 operations. Non-GET is subject to the
write guardrail below. Optionally validates the path/params against the spec.

### Errors & logging

- `eyre::Result` + `.context()` throughout CLI/handlers; `thiserror` only for
  `ApiError`. Errors propagate to `main`, which returns `Result<()>` so eyre
  prints the chain.
- `tracing` + `tracing-subscriber` to a file under `xdg_data_dir()/drata/logs/`;
  level from a `--log-level` flag (never `RUST_LOG`).
  `#[tracing::instrument(skip(self/body/config/from_file), fields(...))]` on the
  client and every verb; `debug!` on branches, `warn!` on recoverable misses.

## Safety: not munging prod

Drata has no sandbox; every live write hits the real tenant. Hardened per the
review panel (the prior "profile name is the boundary" model was unsound):

- **Hard write guardrail.** Any non-GET request fails closed unless the *resolved
  credential* carries `allow_writes: true` - a property of the key, not the
  profile name, so a stray `DRATA_API_KEY`/`DRATA_PROFILE=readwrite` cannot
  silently enable mutation. Selecting a write-enabled credential is explicit.
- **Confirm on all mutating methods** (POST/PUT/PATCH/DELETE), not just DELETE -
  many POSTs have real blast radius (policy acks, vendor-questionnaire emails,
  HRIS upserts). `--yes` bypasses for scripting.
- **Disposable live verification.** The single live write pass uses
  create -> verify -> delete on loudly-named throwaway objects (`zzz-clitest-`),
  never `PUT`/`DELETE` on existing records. Capture real request/response into
  wiremock fixtures on that pass ("record once, replay forever").
- 16 `DELETE` ops; 47/167 ops are `/workspaces/{workspaceId}`-scoped, so a
  deliberate `--workspace` further bounds blast radius.

## Spec usage & coverage

- **`--example` skeletons** generated from each operation's request schema.
- **Coverage test:** parse the spec (167 ops across 35 operation-level tags -
  source from operation tags, NOT the 33-entry top-level `tags` list, which omits
  `Audit Requests` and `Procurement Connection Mappings`). Assert every op is
  reachable via a curated command or `raw`; report curated-coverage %.
- The spike (below) classifies all ops to plan which tags to curate first.

### Phase-2 spike findings (2026-06-27)

All 167 ops carry a unique `operationId` + path + method + typed params (100%
routable). Naming would collide 23x under a flat verb scheme - moot here, since
curated modules name commands by hand and nest sub-resources explicitly. The only
irregular body set is 10 multipart uploads (`--file`). Max arg depth 3; 72 ops
have >=2 path params (nested sub-resource modules).

## Phased implementation plan

Phases are independently shippable; model annotation in brackets.

- **Phase 1 - Harness + one vertical [opus]**
  Lift/adapt `client.rs` (Value, cursor pagination + hardening, `ApiError`,
  `encode_query`, retry/204), `config.rs` (own xdg, profiles + `allow_writes` +
  0600 + migration), `main.rs`/`lib.rs`/`cli.rs` skeleton, tracing. `filter.rs`
  near-verbatim. Auth commands (`login`/`logout`/`whoami`/`auth`). One full
  curated vertical (`vendors`: list/get/create/update/remove + questionnaires)
  end-to-end as the template. wiremock tests for client + config; tests extracted
  to `<mod>/tests.rs` (Scott's newer rule, not PD's inline `mod tests`).

- **Phase 2 - `raw` namespace + coverage [opus]**
  `raw.rs` generic passthrough (write-gated). Spec parser for `--example` +
  the coverage test. Confirms the curated/raw split covers all 167 ops.

- **Phase 3 - Curate the high-traffic tags [sonnet]**
  PD-style modules for risks, controls, devices, personnel, policies,
  evidence-library, frameworks, assets, company, workspaces. Typed Create/Update,
  `--from-file`, `--example`, table renderers in `output/table.rs`.

- **Phase 4 - Writes, uploads, ergonomics [sonnet]**
  Multipart `--file` uploads (10 ops), confirm-on-mutation + `--yes`, `--expand`,
  `--all` NDJSON streaming. Single disposable live verification pass -> fixtures.

- **Phase 5 - Long tail + CI + polish [sonnet]**
  Curate remaining tags as friction warrants (rest stay `raw`). `.otto.yml`
  (lint/check/test/ci/cov), `clippy.toml`, `rustfmt.toml` (`max_width = 120`),
  `build.rs` `GIT_DESCRIBE`, README with `cli-shakedown`-tested examples.

## Open questions

- **Drata JSON casing.** PD is `snake_case`; Drata v2 looks camelCase. Confirm
  per-endpoint serde casing during Phase 1 (`expand[]` etc. complicate flags).
- **Which tags stay `raw`-only long-term?** Driven by real friction, not guessed
  up front (audit-requests, procurement-connection-mappings, custom-data-records,
  tasks are candidates to defer).
- **`--example` fidelity.** How deeply to materialize nested/`oneOf` schemas in
  skeletons vs. emit a minimal required-fields stub.

## References

- Upstream TS: `yorkeccak/drata-cli`
- Reference impl: `tatari-tv/pagerduty-cli`
- Spec: `spec/drata-openapi-v2.json`; guides under `docs/`
- Drata API v2: <https://developers.drata.com/openapi/reference/v2/overview/>
