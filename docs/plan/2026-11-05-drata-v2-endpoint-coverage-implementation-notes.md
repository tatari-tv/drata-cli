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
