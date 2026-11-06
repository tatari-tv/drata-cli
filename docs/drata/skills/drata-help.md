Title: Find the right skill

URL Source: https://developers.drata.com/developer-portal/v2/skills/start-here/drata-help/

Markdown Content:
Last updated 2 months ago

Lists every skill grouped by domain, explains the naming grammar, and points you at the best fit for what you are trying to do.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Start Here | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-help/#purpose)Purpose

*   Explains the three skill types: **report** to see where you stand, **identify-gaps** to get a worklist, **resolve-gaps** to make changes.
*   Lists all 18 skills by domain, marking the ones that can write.
*   Recommends a skill for a goal you describe in your own words — and tells you when none is needed.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-help/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-help/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-help`

Or ask for it in your own words:

*   "what can this plugin do"
*   "which skill should I use"

## [](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-help/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-help/#related-skills)Related skills

*   [What needs attention today](https://developers.drata.com/developer-portal/v2/skills/start-here/drata-all-identify-gaps/) — A cross-domain briefing of everything needing attention right now — failing controls and tests, high open risks, expired evidence, overdue vendor reviews, non-compliant personnel.
