Title: Executive briefing

URL Source: https://developers.drata.com/developer-portal/v2/skills/reporting-and-stakeholder-communications/drata-executive-report/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Last updated 2 months ago

A board, executive, audit-committee or QBR view of live posture — headline metrics, framework readiness and top risks, for one workspace or rolled up across the organization.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Reporting & Stakeholder Communications | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/reporting-and-stakeholder-communications/drata-executive-report/#purpose)Purpose

*   Reports headline metrics, framework readiness and top risks.
*   Rolls up across workspaces with a per-workspace scorecard.
*   Detects issues repeating across workspaces.

> **Access:** This skill cannot change anything in Drata.
>
>
> **Note:** Point-in-time only. Drata exposes no history, so this reports where things stand today — never a trend or a change since last period.

## [](https://developers.drata.com/developer-portal/v2/skills/reporting-and-stakeholder-communications/drata-executive-report/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| List Workspaces | Read | `read:workspace` |
| Search Controls | Read | `read:controls` |
| List Requirements | Read | `read:framework` |
| Search Monitoring Tests | Read | `read:monitor-test` |
| Search Risks | Read | `read:risk` |
| List Vendors | Read | `read:vendor` |
| Search Personnel Compliance | Read | `read:personnel` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/reporting-and-stakeholder-communications/drata-executive-report/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-executive-report`

Or ask for it in your own words:

*   "board deck numbers"
*   "exec summary of our compliance posture"

## [](https://developers.drata.com/developer-portal/v2/skills/reporting-and-stakeholder-communications/drata-executive-report/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/reporting-and-stakeholder-communications/drata-executive-report/#related-skills)Related skills

*   [What needs attention today](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-all-identify-gaps/) — A cross-domain briefing of everything needing attention right now — failing controls and tests, high open risks, expired evidence, overdue vendor reviews, non-compliant personnel.
