Title: Monitoring coverage

URL Source: https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-monitoring-report/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Last updated 2 months ago

Where your monitoring stands — how many tests are enabled, how many pass, fail or error, how long anything has been failing, and which check types account for the failures.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Compliance & Audit Readiness | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-monitoring-report/#purpose)Purpose

*   Reports test status, enabled against disabled.
*   Reports results as passed, failed and errored, scoped to Production so the numbers match the Drata UI.
*   Shows how long tests have been failing, and failures grouped by check type.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-monitoring-report/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| Search Monitoring Tests | Read | `read:monitor-test` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-monitoring-report/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-monitoring-report`

Or ask for it in your own words:

*   "monitoring coverage"
*   "how many tests are failing"

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-monitoring-report/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-monitoring-report/#related-skills)Related skills

*   [Controls needing work](https://developers.drata.com/developer-portal/v2/skills/compliance-and-audit-readiness/drata-control-identify-gaps/) — Two worklists over your not-ready controls — the ones with nothing mapped, and the ones whose manual evidence is out of date — plus any control missing an owner or a description.
