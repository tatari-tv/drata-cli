Title: Control readiness

URL Source: https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-report/

Markdown Content:
Last updated 2 months ago

How many of your controls are ready, and why the rest are not — in scope versus out, the ready share, and the not-ready ones broken down by cause.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Compliance & Audit Readiness | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-report/#purpose)Purpose

*   Splits the control library into in-scope and out-of-scope.
*   Reports what share of in-scope controls are ready.
*   Breaks the not-ready ones down by cause: test failing, evidence not satisfying readiness, policy not satisfying readiness, or nothing mapped.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-report/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| Search Controls | Read | `read:controls` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-report/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-control-report`

Or ask for it in your own words:

*   "how many controls are ready"
*   "why are controls not ready"

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-report/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-report/#related-skills)Related skills

*   [Controls needing work](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-identify-gaps/) — Two worklists over your not-ready controls — the ones with nothing mapped, and the ones whose manual evidence is out of date — plus any control missing an owner or a description.
*   [Fix controls](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-resolve-gaps/) — Work through control gaps from a single menu with live counts.
