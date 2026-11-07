Title: Fix risks

URL Source: https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-resolve-gaps/

Markdown Content:
Last updated 2 months ago

Work the register from a single menu with live counts. Pick a gap, answer one question, and confirm one previewed batch before anything changes.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Risk Management | Write |

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-resolve-gaps/#purpose)Purpose

*   Offers one flat menu: untreated, no owner, not scored, residual missing or wrong, treatment date passed or missing.
*   Previews every change as a batch and waits for your confirmation.
*   Logs, closes or deletes risks on request.

> **Access:** This skill can change your Drata data: create / update / delete risk. Every change is previewed and confirmed before it is applied.

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-resolve-gaps/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| List Risk Registers | Read | `read:risk-registers` |
| Search Risks | Read | `read:risk` |
| Create Risk | Write | `create:risk` |
| Update Risk | Write | `update:risk` |
| Delete Risk | Write | `delete:risk` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-resolve-gaps/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-risk-resolve-gaps`

Or ask for it in your own words:

*   "work the register"
*   "treat this risk"
*   "mark risk accepted"

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-resolve-gaps/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-resolve-gaps/#related-skills)Related skills

*   [Risks needing work](https://developers.drata.com/developer-portal/v2/skills/risk-management/drata-risk-identify-gaps/) — Only the risks where the record is incomplete or contradictory and someone owes an edit.
