# CLI Shakedown Report: drata v0.1.0

Binary: `~/.cargo/bin/drata` (`drata v0.1.0`). Source: `tatari-tv/drata-cli` @ `b369b21` (squash-merge of PR #1).

**Scope note:** No Drata credential is configured (`~/.config/drata/credentials.json` absent), so this is an
**offline shakedown**: every code path that runs *before* a network call was exercised; live GET/list/get,
real mutations, the `verify` create->verify->delete cycle, and the TS-vs-Rust comparison are **blocked on a
credential** (see Skipped). Guardrail/validation paths were driven with a throwaway `DRATA_API_KEY` env value;
each one short-circuits before any HTTP request, so no real Drata call was made.

## Summary
| Metric | Count |
|--------|-------|
| Top-level commands discovered | 22 (+`help`) |
| Offline behaviors tested | 20 |
| Passed | 19 |
| Failed | 1 (release pipeline) |
| Skipped (need credential) | live reads, mutations, `verify`, TS-vs-Rust diff |

## What passed (offline)
| Check | Command | Result |
|-------|---------|--------|
| Version | `drata --version` | `drata v0.1.0`, binary is `drata` (not `drata-cli`) |
| Top-level help | `drata --help` | all 22 commands + `verify` listed; `--log-level` shows enum values |
| Auth diag (no key) | `drata auth` / `drata whoami` | graceful "key: not found", exit 0 |
| Example skeletons | `drata {vendor,asset,policy,framework,evidence,register,control,risk} create --example` | all valid JSON, exit 0 |
| **--example w/o positional** | `drata control create --example`, `drata risk create --example` | skeleton printed without `<workspace_id>`/`<register_id>` (fix verified) |
| **LogLevel enum** | `drata -l bogus auth` | clap rejects: `[possible values: trace, debug, info, warn, error, off]`, exit 2 |
| **Required positional** | `drata control create` (no id, no `--example`) | clap: `required arguments not provided: <WORKSPACE_ID>`, exit 2 |
| **Required-field validation** | `drata control create w1 --yes` | `requires --name (or use --example)`, exit 1, no network |
| **control compare controlIds[]** | `drata control compare w1` | `requires at least one --control-ids value`, exit 1 |
| **Write guardrail** | `vendor create --name X --yes` (no `--allow-writes`) | `write blocked: ... not write-enabled`, exit 1 |
| **Confirm fail-closed** | `vendor create --name X` (non-TTY, no `--yes`) | `requires confirmation but stdin/stdout is not a TTY`, exit 1 |
| raw example | `raw --example POST /vendors` | skeleton; `raw --example GET /vendors` -> "no JSON request body" |
| **raw PUT multipart** | `raw PUT /workspaces/1/evidence-library/9 --file ... --yes` | reaches write-guard (PUT accepted; POST-only rejection fixed) |
| raw bad-method multipart | `raw DELETE /x --file ... --yes` | `only supported for POST and PUT requests (got DELETE)` |
| Unknown command | `drata frobnicate` | clap: `unrecognized subcommand`, exit 2 |

Every correctness-review and shakedown fix is confirmed live in the installed binary.

## Failures & bugs
### 1. [CRITICAL, FIXED] Release pipeline packaged a non-existent binary name
- **Run:** `release.yml` for tag `v0.1.0` (run 28315425588) — all four build jobs failed at "Package binary":
  `cp: target/<target>/release/drata-cli: No such file or directory`; `create-release` skipped.
- **Cause:** the binary was renamed to `drata` (`[[bin]] name = "drata"`), but `release.yml` still referenced
  `drata-cli` in the `cp`, `tar`, and asset-name steps. Regression from the rename.
- **Effect:** `v0.1.0` has **no published GitHub release / binaries**. (Install from source works: `cargo install --path .`.)
- **Fix:** `release.yml` updated (`drata-cli` -> `drata` for the binary path, tar member, and asset names).
- **Caveat:** `v0.1.0`'s assets cannot be regenerated without moving the tag, which is forbidden. The fix
  takes effect on the next tag; a clean published release needs a follow-up version (e.g. `v0.1.1`).

## Skipped — require a Drata credential (user/org-admin)
- All live reads (`vendor list`, `control get`, `company get`, ...) — would hit the real API.
- All real mutations (create/update/delete/upload) — guardrail + confirm verified offline only.
- `drata verify` (live create->verify->delete) — never run against a tenant.
- **TS-vs-Rust comparison** (requested): needs an API key + region to run both `~/repos/yorkeccak/drata-cli`
  and `drata` against the same endpoints and diff output.

## Release validation
- Tag `v0.1.0`: **annotated** (`git cat-file -t` = `tag`), points to `b369b21` (merged tip). Correct.
- GitHub release: **missing** (build failed — see bug #1).
- Binary download test: N/A (no assets published).

## Observations
- `--help`, `--example`, and all clap-level validation are solid and fail with clear, actionable messages.
- Guardrail ordering is correct: validation -> confirm -> write-guard, each failing before any network I/O.
- Log file is now created on all paths (tracing initialized before config load).
