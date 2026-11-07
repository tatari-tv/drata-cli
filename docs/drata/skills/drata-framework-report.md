Title: Framework readiness

URL Source: https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-framework-report/

Markdown Content:
Last updated 2 months ago

How ready a workspace is for a given framework — readiness percentage, pass rates per area, and the requirements that are not yet met.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Compliance & Audit Readiness | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-framework-report/#purpose)Purpose

*   Asks which workspace and which framework, selecting automatically when there is only one.
*   Reports readiness with per-area pass rates.
*   Lists the not-ready requirements, and gives directional coverage for a framework you are considering.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-framework-report/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| List Workspaces | Read | `read:workspace` |
| List Requirements | Read | `read:framework` |
| Search Controls | Read | `read:controls` |
| Search Monitoring Tests | Read | `read:monitor-test` |
| List Evidence | Read | `read:evidence` |
| Search Risks | Read | `read:risk` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-framework-report/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-framework-report`

Or ask for it in your own words:

*   "SOC 2 readiness"
*   "ISO 27001 readiness"
*   "audit prep"

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-framework-report/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-framework-report/#related-skills)Related skills

*   [Controls needing work](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-identify-gaps/) — Two worklists over your not-ready controls — the ones with nothing mapped, and the ones whose manual evidence is out of date — plus any control missing an owner or a description.
