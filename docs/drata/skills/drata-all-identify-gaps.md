Title: What needs attention today

URL Source: https://developers.drata.com/developer-portal/v2/skills/start-here/drata-all-identify-gaps/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Last updated 2 months ago

A cross-domain briefing of everything needing attention right now — failing controls and tests, high open risks, expired evidence, overdue vendor reviews, non-compliant personnel.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Start Here | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-all-identify-gaps/#purpose)Purpose

*   Counts what is outstanding across every domain in one pass.
*   Sorts it into Now, Next and Watch.
*   Points each item at the skill that handles it in depth.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-all-identify-gaps/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| Search Controls | Read | `read:controls` |
| Search Monitoring Tests | Read | `read:monitor-test` |
| Search Risks | Read | `read:risk` |
| List Vendors | Read | `read:vendor` |
| List Evidence | Read | `read:evidence` |
| Search Personnel Compliance | Read | `read:personnel` |
| List Workspaces | Read | `read:workspace` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-all-identify-gaps/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-all-identify-gaps`

Or ask for it in your own words:

*   "what needs attention today"
*   "what's broken in Drata"

## [](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-all-identify-gaps/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-all-identify-gaps/#related-skills)Related skills

*   [Find the right skill](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-help/) — Lists every skill grouped by domain, explains the naming grammar, and points you at the best fit for what you are trying to do.
