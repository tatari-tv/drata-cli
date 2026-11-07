Title: Controls needing work

URL Source: https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-identify-gaps/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Last updated 2 months ago

Two worklists over your not-ready controls — the ones with nothing mapped, and the ones whose manual evidence is out of date — plus any control missing an owner or a description.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Compliance & Audit Readiness | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-identify-gaps/#purpose)Purpose

*   Lists controls with no evidence, policy or monitor mapped.
*   Lists controls whose manual evidence is no longer current.
*   Flags in-scope controls with no owner or no description.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-identify-gaps/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| Search Controls | Read | `read:controls` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-identify-gaps/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-control-identify-gaps`

Or ask for it in your own words:

*   "which controls have nothing mapped"
*   "controls with no owner"

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-identify-gaps/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-identify-gaps/#related-skills)Related skills

*   [Fix controls](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-resolve-gaps/) — Work through control gaps from a single menu with live counts.
*   [Control readiness](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-report/) — How many of your controls are ready, and why the rest are not — in scope versus out, the ready share, and the not-ready ones broken down by cause.
