Title: Risk posture

URL Source: https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-report/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Last updated 2 months ago

The visual risk dashboard — headline numbers, a likelihood-by-impact heat map, treatment status per register, and where risk concentrates.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Risk Management | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-report/#purpose)Purpose

*   Renders a 5×5 likelihood by impact heat map.
*   Shows treatment status as a stacked bar per register.
*   Breaks risk down by category, owner concentration and vendor exposure.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-report/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| Search Risks | Read | `read:risk` |
| List Risk Registers | Read | `read:risk-registers` |
| Search Controls | Read | `read:controls` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-report/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-risk-report`

Or ask for it in your own words:

*   "risk heat map"
*   "risk by category"

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-report/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-report/#related-skills)Related skills

*   [Risks needing work](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-identify-gaps/) — Only the risks where the record is incomplete or contradictory and someone owes an edit.
*   [Fix risks](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-resolve-gaps/) — Work the register from a single menu with live counts.
