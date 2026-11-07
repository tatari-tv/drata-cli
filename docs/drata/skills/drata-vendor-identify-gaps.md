Title: Vendors needing review

URL Source: https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-identify-gaps/

Markdown Content:
Last updated 2 months ago

Seven checks across your current vendors, grouped by what the fix is, with a decision memo available for any single vendor.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Third-Party & Vendor Risk | Read-only, with one optional write |

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-identify-gaps/#purpose)Purpose

*   Finds security reviews never started, past deadline, or last completed over a year ago.
*   Flags overdue next reviews, missing business units, missing security owners and on-hold status.
*   Lists prospective vendors whose review has not started.

> **Access:** Read-only, except that it can create a risk, and only when you explicitly confirm it.

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-identify-gaps/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| List Vendors | Read | `read:vendor` |
| Get Vendor | Read | `read:vendor` |
| List Vendor Security Reviews | Read | `read:vendor-security-review` |
| List Vendor Documents | Read | `read:vendor-document` |
| Search Risks | Read | `read:risk` |
| Create Risk optional | Write | `create:risk` |

This skill uses every scope above that is not marked optional; without them it fails partway through. The one marked optional (`create:risk`) is not required — the skill degrades cleanly without it. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-identify-gaps/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-vendor-identify-gaps`

Or ask for it in your own words:

*   "which vendors need review"
*   "overdue security reviews"

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-identify-gaps/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-identify-gaps/#related-skills)Related skills

*   [Fix vendors](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-resolve-gaps/) — Work through vendor gaps from a single menu with live counts.
*   [Vendor portfolio](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-report/) — Your third-party portfolio at a glance — composition, assessment coverage, inherent against residual exposure, and what is waiting at intake.
