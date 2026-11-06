Title: MCP Server

URL Source: https://developers.drata.com/developer-portal/v2/mcp-server/

Markdown Content:
Last updated 2 months ago

## [](https://developers.drata.com/developer-portal/v2/mcp-server/#overview)Overview

The Drata MCP (Model Context Protocol) Server allows AI assistants like Claude, ChatGPT, Cursor, and Microsoft Copilot to securely access your Drata data. This guide walks you through setting up OAuth authentication for MCP clients.

## [](https://developers.drata.com/developer-portal/v2/mcp-server/#prerequisites)Prerequisites

*    Administrator access in Drata
*    An MCP-compatible client (Claude, ChatGPT, Cursor, or Microsoft Copilot)

## [](https://developers.drata.com/developer-portal/v2/mcp-server/#configuration-steps)Configuration Steps

### [](https://developers.drata.com/developer-portal/v2/mcp-server/#1-configure-the-drata-mcp-server)1. Configure the Drata MCP Server

1.    Click **Settings** in your Drata account
2.    Click **MCP Configuration**

> **Note:** You must be an administrator in Drata to access this page.

### [](https://developers.drata.com/developer-portal/v2/mcp-server/#2-configure-your-mcp-client)2. Configure Your MCP Client

Follow the setup instructions for your specific MCP client. Drata provides remote hosted MCP servers at the following endpoints:

*   **US:**`https://mcp.drata.com/mcp/`
*   **EU:**`https://mcp-euc1.drata.com/mcp/`
*   **APAC:**`https://mcp-apse2.drata.com/mcp/`

#### [](https://developers.drata.com/developer-portal/v2/mcp-server/#client-specific-setup-instructions)Client-Specific Setup Instructions

*   **Claude:**[Getting started with custom connectors using remote MCP](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp)
*   **ChatGPT:**[Connect ChatGPT to MCP](https://developers.openai.com/apps-sdk/deploy/connect-chatgpt/)
*   **Cursor:**[MCP Documentation](https://cursor.com/docs/context/mcp)
*   **Microsoft Copilot:**[Add existing server to agent](https://learn.microsoft.com/en-us/microsoft-copilot-studio/mcp-add-existing-server-to-agent)

## [](https://developers.drata.com/developer-portal/v2/mcp-server/#mcp-tools)MCP Tools

The Drata MCP Server exposes the following tools. Each tool maps to a Drata Public API v2 operation and requires the OAuth scope shown below. Access is always limited to the intersection of the scopes you grant and the permissions your Drata role already provides.

| Tool | Description | OAuth Scope | Allowed Roles |
| --- | --- | --- | --- |
| Get Company | Get the account's company information. | `read:company` | Admin |
| Search Controls | Search or list controls for a workspace. | `read:controls` | Admin, Control Manager, DevOps Engineer, Information Security Lead, Restricted Control Manager, Restricted Risk Manager, Risk Manager, Risk Register Owner, Workspace Administrator |
| Create Control | Create a new control in a workspace. | `create:control` | Admin, Control Manager, DevOps Engineer, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| Update Control | Update an existing control in a workspace. | `update:control` | Admin, Control Manager, DevOps Engineer, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| List Devices | List a person's devices with each device's per-monitor compliance checks. | `read:device` | Admin, Information Security Lead, Personnel Compliance Manager, Workspace Administrator |
| Get Evidence Bucket | Get one evidence bucket, including its renewal date, renewal cadence, and status. | `read:evidence` | Admin, Control Manager, DevOps Engineer, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| List Evidence Artifacts | List the individual files inside one evidence bucket. | `read:evidence` | Admin, Control Manager, DevOps Engineer, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| List Evidence Buckets | List or search Evidence Library items via Public API v2, with optional client-side filtering. | `read:evidence` | Admin, Control Manager, DevOps Engineer, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| Create Evidence Bucket | Register (POST) a new evidence bucket with exactly one artifact via Public API v2. | `create:evidence` | Admin, Control Manager, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| Delete Evidence Artifact | Permanently delete one file from an evidence bucket. | `delete:evidence` | Admin, Control Manager, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| Delete Evidence Bucket | Delete an existing evidence library item. | `delete:evidence` | Admin, Control Manager, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| Update Evidence Artifacts | Add, replace, archive, or restore the files inside one evidence bucket. | `update:evidence` | Admin, Control Manager, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| Update Evidence Bucket | Update an existing Evidence Library item. | `update:evidence` | Admin, Control Manager, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| List Requirements | List framework requirements, with optional client-side code/name filter. | `read:framework` | Admin, Control Manager, DevOps Engineer, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| List Personnel Groups | List the personnel groups (teams / departments) in the account. | `read:personnel` | Admin, Information Security Lead, Personnel Compliance Manager, Workspace Administrator |
| Search Monitoring Tests | Search or list monitoring tests for a workspace. | `read:monitor-test` | Admin, Control Manager, DevOps Engineer, Information Security Lead, Restricted Control Manager, Workspace Administrator |
| List Personnel | List/search personnel (identity, employment, overall compliance) via OpenSearch. | `read:personnel` | Admin, Information Security Lead, Personnel Compliance Manager, Workspace Administrator |
| Search Device Compliance | Find personnel by a specific DEVICE compliance reason (agent, encryption, antivirus, etc.). | `read:personnel` | Admin, Information Security Lead, Personnel Compliance Manager, Workspace Administrator |
| Search Personnel Compliance | Find personnel by a specific PERSON compliance reason (policies, BG check, MFA, training, offboarding). | `read:personnel` | Admin, Information Security Lead, Personnel Compliance Manager, Workspace Administrator |
| Update Personnel | Update a person's employment record, identified by personnel_id, email, or resolved name. | `update:personnel` | Admin, Information Security Lead, Personnel Compliance Manager, Workspace Administrator |
| List Policies | List account policies, or a user's assigned policies when assigned _to_ user is set. | `read:assigned-policies` `read:policy` | Admin, Control Manager, DevOps Engineer, Employee, Information Security Lead, Internal Auditor, Knowledge Base, Personnel Compliance Manager, Policy Manager, Restricted Control Manager, Restricted Risk Manager, Reviewer, Risk Manager, Trust Center Manager, Trust Center Reviewer, Trust User, Workspace Administrator |
| Search My Assigned Policies | Search policies assigned to the authenticated user. | `read:assigned-policies` | Admin, Control Manager, DevOps Engineer, Employee, Information Security Lead, Internal Auditor, Knowledge Base, Personnel Compliance Manager, Policy Manager, Restricted Control Manager, Restricted Risk Manager, Reviewer, Risk Manager, Trust Center Manager, Trust Center Reviewer, Trust User, Workspace Administrator |
| List Risk Registers | List all risk registers for the account. | `read:risk-registers` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager |
| Search Risks | Search or list risks from the risk register. | `read:risk` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Risk Register Owner, Workspace Administrator |
| Create Risk | Create a new risk in a risk register. | `create:risk` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Risk Register Owner, Workspace Administrator |
| Delete Risk | Delete an existing risk from a risk register. | `delete:risk` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Risk Register Owner, Workspace Administrator |
| Update Risk | Update an existing risk. | `update:risk` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Risk Register Owner, Workspace Administrator |
| Look Up User Identity | Look up a Drata account (incl. roles / SSO identities) via Public API v2. | `read:user` `read:users` | Admin, Control Manager, DevOps Engineer, Employee, Information Security Lead, Internal Auditor, Knowledge Base, Personnel Compliance Manager, Policy Manager, Restricted Control Manager, Restricted Risk Manager, Reviewer, Risk Manager, Risk Register Owner, Trust Center Manager, Trust Center Reviewer, Trust User, Workspace Administrator |
| Get Vendor | Get a single vendor by ID. | `read:vendor` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Workspace Administrator |
| List Vendor Documents | List documents for a vendor, with optional filtering. | `read:vendor-document` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Workspace Administrator |
| List Vendor Security Reviews | List security reviews for a vendor, with optional filtering. | `read:vendor-security-review` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Workspace Administrator |
| List Vendors | List third-party vendors for the account, with optional filtering. | `read:vendor` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Workspace Administrator |
| Create Vendor | Create a new vendor in the account. | `create:vendor` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Workspace Administrator |
| Delete Vendor | Permanently delete a vendor from the account. | `delete:vendor` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Workspace Administrator |
| Update Vendor | Update an existing vendor. | `update:vendor` | Admin, Information Security Lead, Restricted Risk Manager, Risk Manager, Workspace Administrator |
| List Workspaces | List all workspaces. | `read:workspace` | Admin, Control Manager, DevOps Engineer, Information Security Lead, Personnel Compliance Manager, Policy Manager, Restricted Control Manager, Restricted Risk Manager, Risk Manager, Risk Register Owner |

**Notes on roles:**

*    For read ( `read:*` ) scopes, the read-only variant of any listed role (e.g. Read-Only Admin, Read-Only Control Manager) has the same view access; read-only roles cannot use `create` , `update` , or `delete` scopes.
*    The Service User (integration) role also has equivalent access to every scope above.
*    A user always receives only the intersection of the granted scope and their role's permissions in Drata.

## [](https://developers.drata.com/developer-portal/v2/mcp-server/#important-security-considerations)Important Security Considerations

> **Access Control:** End users can only access the intersection of what the OAuth scopes offer and what their roles provide them access to. They cannot access anything beyond what their roles inside the application give them access to while using the MCP.

This means that even if an OAuth scope is granted, users are still limited by their role-based permissions within Drata.

## [](https://developers.drata.com/developer-portal/v2/mcp-server/#next-steps)Next Steps

After completing the OAuth configuration:

1.    Test the connection with your MCP client
2.    Verify that the appropriate data is accessible (see [Best Practices](https://developers.drata.com/developer-portal/v2/mcp-server/#best-practices-for-using-the-drata-mcp-server) )
3.    Monitor usage and adjust scopes as needed

## [](https://developers.drata.com/developer-portal/v2/mcp-server/#best-practices-for-using-the-drata-mcp-server)Best Practices for Using the Drata MCP Server

### [](https://developers.drata.com/developer-portal/v2/mcp-server/#chatgpt-specific-tips)ChatGPT-Specific Tips

*    Tag **@Drata MCP** in your message to explicitly invoke the connector
*    Use **Developer Mode** for full tool access (read and write)
*    When ChatGPT prompts for confirmation on write actions, review before approving

### [](https://developers.drata.com/developer-portal/v2/mcp-server/#claude-specific-tips)Claude-Specific Tips

*    Toggle the Drata connector on at the start of each conversation using the "+" menu
*    You can combine Drata with other connectors (e.g., Slack, Notion) in the same conversation for cross-tool workflows

### [](https://developers.drata.com/developer-portal/v2/mcp-server/#cursor-specific-tips)Cursor-Specific Tips

*    Switch Cursor to **Agent mode** (Ctrl/Cmd + .) for the best MCP tool integration
*    Reference Drata tools by name when prompting for precision

### [](https://developers.drata.com/developer-portal/v2/mcp-server/#security-reminders)Security Reminders

*    Drata's MCP server uses OAuth authentication — your credentials are never shared with the AI client
*    The AI client can only access data you have permissions for in Drata
*    You can revoke access at any time from your Drata account or from the AI client's connector settings

### [](https://developers.drata.com/developer-portal/v2/mcp-server/#prompt-library)Prompt Library

See the [Drata MCP Setup & Usage Guide](https://help.drata.com/en/articles/13379899-drata-mcp-setup-usage-guide) for a full prompt library and additional usage tips.

## [](https://developers.drata.com/developer-portal/v2/mcp-server/#privacy)Privacy

The Drata MCP server operates on your own Drata data, in your own session, through your own OAuth grant — the connection stores no credentials of its own, and what any request can reach is bounded by the scopes you grant and your Drata role. Your data is handled under Drata's [Privacy Notice](https://drata.com/privacy). For how Drata secures its platform, see the [Drata Trust Center](https://trust.drata.com/).

## [](https://developers.drata.com/developer-portal/v2/mcp-server/#support)Support

*   **Documentation and help:**[help.drata.com](https://help.drata.com/)
*   **Security questions or reports:**[security@drata.com](mailto:security@drata.com)
