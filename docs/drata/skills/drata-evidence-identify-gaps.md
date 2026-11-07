Title: Evidence needing work

URL Source: https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-identify-gaps/

Markdown Content:
Last updated 2 months ago

Your evidence library sorted into four workable buckets, so an audit-prep sweep or a renewal plan starts from a list rather than a hunt.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Compliance & Audit Readiness | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-identify-gaps/#purpose)Purpose

*   Groups evidence into needs-artifact, automated-but-failing, renewal overdue and renewal due soon.
*   Calls out evidence mapped to no control.
*   Flags items missing an owner, renewal schedule, artifact or description.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-identify-gaps/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| List Evidence | Read | `read:evidence` |
| Search Controls | Read | `read:controls` |
| Search Monitoring Tests | Read | `read:monitor-test` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-identify-gaps/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-evidence-identify-gaps`

Or ask for it in your own words:

*   "what's broken in our evidence library"
*   "renewal planning"

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-identify-gaps/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-identify-gaps/#related-skills)Related skills

*   [Fix evidence](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-resolve-gaps/) — Work through evidence gaps from a single menu with live counts.
*   [Evidence freshness](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-report/) — How current your evidence library is — the share that is valid against what needs an artifact, is expiring soon, or has expired — and why evidence goes stale.
