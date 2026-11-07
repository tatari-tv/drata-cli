Title: Vendor portfolio

URL Source: https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-report/

Markdown Content:
Last updated 2 months ago

Your third-party portfolio at a glance — composition, assessment coverage, inherent against residual exposure, and what is waiting at intake.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Third-Party & Vendor Risk | Read-only |

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-report/#purpose)Purpose

*   Shows the portfolio split across current, prospective and archived vendors.
*   Reports assessment coverage and an inherent by residual grid.
*   Breaks exposure down by category, with a single intake bar for prospective vendors.

> **Access:** This skill cannot change anything in Drata.

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-report/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| List Vendors | Read | `read:vendor` |

This skill uses every scope listed above; without them it fails partway through. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-report/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-vendor-report`

Or ask for it in your own words:

*   "vendor risk overview"
*   "unassessed vendors"

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-report/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-report/#related-skills)Related skills

*   [Vendors needing review](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-identify-gaps/) — Seven checks across your current vendors, grouped by what the fix is, with a decision memo available for any single vendor.
*   [Fix vendors](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-resolve-gaps/) — Work through vendor gaps from a single menu with live counts.
