Title: Fix vendors

URL Source: https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-resolve-gaps/

Markdown Content:
Last updated 2 months ago

Work through vendor gaps from a single menu with live counts. Pick a gap, answer one question, and confirm one previewed batch before anything changes.

| Plugin | Area | Access |
| --- | --- | --- |
| drata-grc-skills | Third-Party & Vendor Risk | Write |

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-resolve-gaps/#purpose)Purpose

*   Offers one flat menu: key information missing, review overdue, on hold, no renewal schedule.
*   Previews every change as a batch and waits for your confirmation.
*   Onboards vendors, imports a CSV, records a decision, or archives on request.

> **Access:** This skill can change your Drata data: create / update / delete vendor. Every change is previewed and confirmed before it is applied.

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-resolve-gaps/#mcp-tools-used)MCP tools used

| MCP tool | Level | OAuth scope |
| --- | --- | --- |
| Get Company | Read | `read:company` |
| List Vendors | Read | `read:vendor` |
| Get Vendor | Read | `read:vendor` |
| Create Vendor | Write | `create:vendor` |
| Update Vendor | Write | `update:vendor` |
| Delete Vendor | Write | `delete:vendor` |
| List Vendor Security Reviews optional | Read | `read:vendor-security-review` |

This skill uses every scope above that is not marked optional; without them it fails partway through. The one marked optional (`read:vendor-security-review`) is not required — the skill degrades cleanly without it. What any tool returns is bounded by your Drata role as well as the scope — see [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-resolve-gaps/#run-it)Run it

Copy

Copied

```
/plugin marketplace add drata/drata-claude-plugin
/plugin install drata-grc-skills@drata
```

Then run the skill by name:

Copy

Copied

`/drata-grc-skills:drata-vendor-resolve-gaps`

Or ask for it in your own words:

*   "fix my vendors"
*   "assign vendor owners"
*   "record the Acme Corp decision"

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-resolve-gaps/#before-you-start)Before you start

*   Connect the Drata MCP server. See [MCP Server setup](https://developers.drata.com/developer-portal/v2/mcp-server/).
*   Your MCP OAuth configuration must grant the scopes behind the tools listed above.

## [](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-resolve-gaps/#related-skills)Related skills

*   [Vendors needing review](https://developers.drata.com/developer-portal/v2/skills/third-party-and-vendor-risk/drata-vendor-identify-gaps/) — Seven checks across your current vendors, grouped by what the fix is, with a decision memo available for any single vendor.
