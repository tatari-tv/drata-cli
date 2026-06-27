Title: MCP Server (Beta)

URL Source: https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/

Markdown Content:
Last updated 3 months ago

**🧪 Beta Feature:** This feature is currently available with early access. If you would like to access the feature, please fill out [this form](https://docs.google.com/forms/d/e/1FAIpQLSeolO8-K3hp6ElIa_mvc9cGTHsDBhPuZPLMAniAoDrAYY6x5g/viewform) and a Drata team member will be in touch with you regarding next steps.

## [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#overview)Overview

The Drata MCP (Model Context Protocol) Server allows AI assistants like Claude, ChatGPT, Cursor, and Microsoft Copilot to securely access your Drata data. This guide walks you through setting up OAuth authentication for MCP clients.

## [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#prerequisites)Prerequisites

*    Administrator access in Drata
*    An MCP-compatible client (Claude, ChatGPT, Cursor, or Microsoft Copilot)

## [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#configuration-steps)Configuration Steps

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#1-access-mcp-oauth-configuration)1. Access MCP OAuth Configuration

1.    Click **Settings** in your Drata account
2.    Click **MCP Configuration**

> **Note:** You must be an administrator in Drata to access this page.

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#2-create-oauth-configuration)2. Create OAuth Configuration

1.    Enter a **name** for the OAuth Configuration
2.    Enter a **description** of the configuration (optional)
3.    Set an **expiration date** on the configuration (optional)
4.    Select the **scopes** you want to configure (see [OAuth Scopes](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#oauth-scopes) below)

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#3-configure-your-mcp-client)3. Configure Your MCP Client

After configuring the scopes, follow the setup instructions for your specific MCP client. Drata provides remote hosted MCP servers at the following endpoints:

*   **US:**`https://mcp.drata.com/mcp/`
*   **EU:**`https://mcp-euc1.drata.com/mcp/`
*   **APAC:**`https://mcp-apse2.drata.com/mcp/`

#### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#client-specific-setup-instructions)Client-Specific Setup Instructions

*   **Claude:**[Getting started with custom connectors using remote MCP](https://support.claude.com/en/articles/11175166-getting-started-with-custom-connectors-using-remote-mcp)
*   **ChatGPT:**[Connect ChatGPT to MCP](https://developers.openai.com/apps-sdk/deploy/connect-chatgpt/)
*   **Cursor:**[MCP Documentation](https://cursor.com/docs/context/mcp)
*   **Microsoft Copilot:**[Add existing server to agent](https://learn.microsoft.com/en-us/microsoft-copilot-studio/mcp-add-existing-server-to-agent)

## [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#oauth-scopes)OAuth Scopes

The following OAuth scopes are available for MCP integration:

| OAuth Scope | Description | Allowed Roles |
| --- | --- | --- |
| `read:risk` | View Risks in Risk Registers | Admin, Risk Manager, Risk Register Owner, Workspace Administrator |
| `read:controls` | View Controls list | Admin, Control Manager, DevOps Engineer, Risk Manager, Risk Register Owner, Information Security Lead, Workspace Administrator |
| `read:control` | View Control details and requirements | Admin, Control Manager, DevOps Engineer, Information Security Lead, Workspace Administrator |
| `read:policy` | View Policies | Admin, Policy Manager, Information Security Lead, Workspace Administrator |
| `read:workspace` | View Workspaces | Admin |
| `read:risk-registers` | View Risk Registers | Admin, Risk Manager |
| `read:assigned-policies` | View User Assigned Policies | Admin, Control Manager, DevOps Engineer, Employee, Internal Auditor, Knowledge Base, People Ops, Policy Manager, Reviewer, Risk Manager, Information Security Lead, Trust Center Manager, Trust Center Reviewer, Workspace Administrator |
| `read:monitor-test` | View Monitoring Tests | Admin, Control Manager, DevOps Engineer, Information Security Lead, Workspace Administrator |

## [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#important-security-considerations)Important Security Considerations

> **Access Control:** End users can only access the intersection of what the OAuth scopes offer and what their roles provide them access to. They cannot access anything beyond what their roles inside the application give them access to while using the MCP.

This means that even if an OAuth scope is granted, users are still limited by their role-based permissions within Drata.

## [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#next-steps)Next Steps

After completing the OAuth configuration:

1.    Test the connection with your MCP client
2.    Verify that the appropriate data is accessible (see [Best Practices](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#best-practices-for-using-the-drata-mcp-server) )
3.    Monitor usage and adjust scopes as needed

## [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#best-practices-for-using-the-drata-mcp-server)Best Practices for Using the Drata MCP Server

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#mention-drata-by-name)Mention Drata by Name

Always include "Drata" in your prompts. This helps the AI model correctly route your request to the Drata MCP tools rather than relying on its built-in knowledge.

*    ✅ "Which controls are missing evidence in Drata?"
*    ❌ "Which controls are missing evidence?"

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#be-specific-with-your-requests)Be Specific with Your Requests

The more specific your prompt, the better the results. Include details like framework names, time ranges, risk categories, or team names when relevant.

*    ✅ "Create a report of risks created in the last 6 months that don't have a treatment plan in Drata"
*    ❌ "Show me risks"

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#use-natural-language)Use Natural Language

You don't need to know Drata's API or data model. Ask questions the way you'd ask a compliance analyst.

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#chatgpt-specific-tips)ChatGPT-Specific Tips

*    Tag **@Drata MCP** in your message to explicitly invoke the connector
*    Use **Developer Mode** for full tool access (read and write)
*    When ChatGPT prompts for confirmation on write actions, review before approving

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#claude-specific-tips)Claude-Specific Tips

*    Toggle the Drata connector on at the start of each conversation using the "+" menu
*    You can combine Drata with other connectors (e.g., Slack, Notion) in the same conversation for cross-tool workflows

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#cursor-specific-tips)Cursor-Specific Tips

*    Switch Cursor to **Agent mode** (Ctrl/Cmd + .) for the best MCP tool integration
*    Reference Drata tools by name when prompting for precision

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#security-reminders)Security Reminders

*    Drata's MCP server uses OAuth authentication — your credentials are never shared with the AI client
*    The AI client can only access data you have permissions for in Drata
*    You can revoke access at any time from your Drata account or from the AI client's connector settings

### [](https://developers.drata.com/developer-portal/v2/recipes/mcp-oauth-setup/#prompt-library)Prompt Library

Here are a few example prompts for interaction with the Drata MCP server:

*    Am I allowed to use Jira on my phone?
*    How often am I required to come into our office to work during the week?
*    How often am I required to do security awareness training?
*    What is the SLA in our policies for fixing critical vulnerabilities?
*    Which controls are missing evidence in Drata?
*    What controls in Drata are related to data retention and storage and who are their owners?
*    Are there any failing tests for versions control systems connected in Drata?
*    Create a report of controls that are not ready and tell me what frameworks they are related to
*    Create a report of risks that need attention in Drata
*    Do we have tests in Drata ensuring data is not publicly accessible?
*    Create a report of all failing tests in Drata and rank them by priority
*    Who is the owner for our access controls in Drata?
*    Create a report of risks created in the last 6 months that don't have a treatment plan
*    What risks in Drata are associated to background checks and security training for personnel?
*    What risks don't have a treatment plan in Drata and who are their owners?
*    I am monitoring a risk regarding our cloud infrastructure. What risks in Drata are currently related to this?
*    My engineering team is handling an incident related to our application. What are the incident response steps that I should be aware of in Drata based on our policies and controls?
*    My team is purchasing a new tool to use internally. What is our vendor management process in Drata based on our policies and controls?
*    I'm reviewing a contract for a new vendor. Do we have any existing risks recorded regarding third-party data handling?
