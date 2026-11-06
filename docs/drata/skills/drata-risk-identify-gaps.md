Title: Risks needing work

URL Source: https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-identify-gaps/

Markdown Content:
Last updated 2 months ago

Only the risks where the record is incomplete or contradictory and someone owes an edit. Risks already assessed and treated are counted, not listed.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Risk Management | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-identify-gaps/#purpose)Purpose

*   Lists risks that are untreated, unowned or unscored.
*   Flags records with no residual score, no reduction, or a residual above inherent.
*   Lists treatments that are overdue or have no date.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-identify-gaps/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| List Risk Registers | Read | `read:risk-registers` |
| Search Risks | Read | `read:risk` |
| Search Controls | Read | `read:controls` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-identify-gaps/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-risk-identify-gaps`

Or ask for it in your own words:

*   "which risks need attention"
*   "overdue treatments"

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-identify-gaps/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-identify-gaps/#related-skills)Related skills

*   [Fix risks](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-resolve-gaps/) — Work the register from a single menu with live counts.
*   [Risk posture](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-report/) — The visual risk dashboard — headline numbers, a likelihood-by-impact heat map, treatment status per register, and where risk concentrates.
