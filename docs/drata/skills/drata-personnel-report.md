Title: Workforce compliance

URL Source: https://developers.drata.com/developer-portal/v2/skills/personnel-and-access-compliance/drata-personnel-report/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Last updated 2 months ago

How your workforce is tracking against each compliance check — pass, fail and excluded, split by employee against contractor and by department.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Personnel & Access Compliance | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/personnel-and-access-compliance/drata-personnel-report/#purpose)Purpose

*   Shows pass, fail and excluded per check as stacked bars.
*   Splits results by employee against contractor, and by department.
*   Reports onboarding time-to-compliance and offboarding ageing.

> **Access:** This skill cannot change anything in Drata.
>
>
> **Note:** Population-level only. This report never names an individual.

## [](https://developers.drata.com/developer-portal/v2/skills/personnel-and-access-compliance/drata-personnel-report/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| Search Personnel Compliance | Read | `read:personnel` |
| Search Device Compliance | Read | `read:device` |
| List Personnel | Read | `read:personnel` |
| List Devices | Read | `read:device` |
| List Personnel Groups | Read | `read:personnel` |
| Lookup User Identity | Read | `read:users`+`read:user` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/personnel-and-access-compliance/drata-personnel-report/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-personnel-report`

Or ask for it in your own words:

*   "how compliant is our workforce"
*   "training and policy coverage"

## [](https://developers.drata.com/developer-portal/v2/skills/personnel-and-access-compliance/drata-personnel-report/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.
