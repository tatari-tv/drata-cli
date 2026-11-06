Title: Fix evidence

URL Source: https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-resolve-gaps/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Last updated 2 months ago

Work through evidence gaps from a single menu with live counts. Pick a gap, answer one question, and confirm one previewed batch before anything changes.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Compliance & Audit Readiness | Write |

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-resolve-gaps/#purpose)Purpose

*   Offers one flat menu: no artifact, automated failing, renewal overdue, no control mapped, key information missing.
*   Previews every change as a batch and waits for your confirmation.
*   Creates or retires evidence items on request.

> **Access:** This skill can change your Drata data: create / update / delete evidence. Every change is previewed and confirmed before it is applied.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-resolve-gaps/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| List Evidence | Read | `read:evidence` |
| Search Controls | Read | `read:controls` |
| Create Evidence | Write | `create:evidence` |
| Update Evidence | Write | `update:evidence` |
| Delete Evidence | Write | `delete:evidence` |
| Search Monitoring Tests optional | Read | `read:monitor-test` |

This skill uses every scope above that is not marked optional; without them it fails partway through. The one marked optional (`read:monitor-test`) is not required — the skill degrades cleanly without it. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-resolve-gaps/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-evidence-resolve-gaps`

Or ask for it in your own words:

*   "fix my evidence"
*   "assign evidence owners"
*   "link evidence to DCF-15"

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-resolve-gaps/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-resolve-gaps/#related-skills)Related skills

*   [Evidence needing work](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-evidence-identify-gaps/) — Your evidence library sorted into four workable buckets, so an audit-prep sweep or a renewal plan starts from a list rather than a hunt.
