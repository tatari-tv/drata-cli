Title: Evidence freshness

URL Source: https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-report/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Last updated 2 months ago

How current your evidence library is — the share that is valid against what needs an artifact, is expiring soon, or has expired — and why evidence goes stale.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Compliance & Audit Readiness | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-report/#purpose)Purpose

*   Reports valid, needs-artifact, expiring-soon and expired as shares of the library.
*   Shows how much evidence has a renewal cadence that never prompts again.
*   Splits freshness by collection source, automated against manual.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-report/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| List Evidence | Read | `read:evidence` |
| Search Monitoring Tests optional | Read | `read:monitor-test` |

This skill uses every scope above that is not marked optional; without them it fails partway through. The one marked optional (`read:monitor-test`) is not required — the skill degrades cleanly without it. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-report/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-evidence-report`

Or ask for it in your own words:

*   "how fresh is our evidence"
*   "what share is valid or expired"

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-report/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-report/#related-skills)Related skills

*   [Evidence needing work](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-identify-gaps/) — Your evidence library sorted into four workable buckets, so an audit-prep sweep or a renewal plan starts from a list rather than a hunt.
*   [Fix evidence](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-resolve-gaps/) — Work through evidence gaps from a single menu with live counts.
