Title: Fix controls

URL Source: https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-resolve-gaps/

Markdown Content:
Last updated 2 months ago

Work through control gaps from a single menu with live counts. Pick a gap, answer one question, and confirm one previewed batch before anything changes.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Compliance & Audit Readiness | Write |

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-resolve-gaps/#purpose)Purpose

*   Offers one flat menu: nothing mapped, missing a policy, evidence not current, tests failing, key information missing.
*   Previews every change as a batch and waits for your confirmation.
*   Creates controls and maps requirements, policies and tests on request.

> **Access:** This skill can change your Drata data: create / update control, and create / update evidence when the gap you pick is an evidence gap. Every change is previewed and confirmed before it is applied.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-resolve-gaps/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| Search Controls | Read | `read:controls` |
| List Requirements | Read | `read:framework` |
| List Policies | Read | `read:policy` |
| Create Control | Write | `create:control` |
| Update Control | Write | `update:control` |
| List Evidence optional | Read | `read:evidence` |
| Create Evidence optional | Write | `create:evidence` |
| Update Evidence optional | Write | `update:evidence` |
| Search Monitoring Tests optional | Read | `read:monitor-test` |

This skill uses every scope above that is not marked optional; without them it fails partway through. The 4 marked optional (`read:evidence`, `create:evidence`, `update:evidence`, `read:monitor-test`) are not required — the skill degrades cleanly without them. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-resolve-gaps/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-control-resolve-gaps`

Or ask for it in your own words:

*   "fix my controls"
*   "assign an owner"
*   "map this control to CC6.1"

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-resolve-gaps/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-resolve-gaps/#related-skills)Related skills

*   [Controls needing work](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-identify-gaps/) — Two worklists over your not-ready controls — the ones with nothing mapped, and the ones whose manual evidence is out of date — plus any control missing an owner or a description.
