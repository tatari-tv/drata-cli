# Handoff: raw spec matching and pagination fixes

Branch: main

HEAD at time of writing: d3518e9 (v0.1.4). Findings from a read-only review on 2026-10-07; nothing has been changed yet.

## Next action

Fix `spec::find_by_method_path` so a concrete path matches its spec template, then make the `raw` miss a hard error behind a `--force` flag instead of a warning.

## Read first

1. `src/spec.rs:136-146` (`find_by_method_path`)
2. `src/raw.rs:78-88` (the warn-and-send-anyway block)
3. `src/client.rs:489-618` (`get_all` and `stream_all`)
4. `tests/coverage.rs` (the spec-op coverage assertion; extend it for template matching)

## Defects, each with the probe that proves it

- **Concrete paths never match the spec.** `find_by_method_path` compares `op.path == path` as plain strings (`src/spec.rs:142`). `raw GET /vendors/123` never equals `/vendors/{vendorId}`, so every real call logs "method+path not found in spec; sending anyway" (`src/raw.rs:83`) and a typo'd path is accepted the same way. Probe: `drata raw GET /vendors/1 --example` or `RUST_LOG=debug drata raw GET /vendors/1` and watch for the warn line. Fix shape: split both on `/`, treat `{...}` segments as wildcards; add a test with `/vendors/123` vs `/vendors/{vendorId}`.
- **No request validation against the spec.** Query params are percent-encoded but never checked (`src/raw.rs:130-148`); bodies are checked only for valid JSON (`src/raw.rs:154-172`). The spec is already parsed into a `Value` in `src/spec.rs:58-69`, so required-param and unknown-param checks are a lookup away.
- **`raw` does not paginate.** It makes one request and prints it (`src/raw.rs:123-124`). Drata list endpoints return `pagination.cursor`; `raw` has no `--all`.
- **`get_all` and `stream_all` are near-copies.** `src/client.rs:489` and `src/client.rs:558` share the cursor loop, `MAX_PAGES` guard and repeated-cursor abort. Collapse to one page iterator that both consume.
- **The `multipart` flag is detected per op (`src/spec.rs:103-108`) but `raw` ignores it.** The user has to pass `--file` by hand.
- **Stray dirs.** `main/` (holds only `.claude/`) and `spec/.claude/` are untracked scratch. Probe: `git status --porcelain --ignored | grep -e '^?? main' -e 'spec/.claude'`. Both are the known sandbox phantom-file class, so verify with a sandbox-off `git status` before touching either.

## Not defects, noted for the Jira/Confluence CLI

- No `plugin/` and no MCP server. `fleet-plugins.md` does not list drata-cli, so this is consistent, not a miss.
- `drata-cli.yml` is comments only; nothing in `src/` reads it. Config is `~/.config/drata/credentials.json`.

## Suggested skills

- `how-to-execute-a-plan` if this becomes a design doc; otherwise direct edits plus `otto ci`.
- `cli-shakedown` after the fix, against `raw GET /vendors/<id>`.
- `shipit` to release (`bump --gates` to confirm the path).
