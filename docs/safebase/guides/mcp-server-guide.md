Title: SafeBase MCP Server Guide

URL Source: https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide

Markdown Content:
Copyright (c) 2023, Intercom, Inc. (legal@intercom.io) with Reserved Font Name "Inter". This Font Software is licensed under the SIL Open Font License, Version 1.1.Copyright (c) 2023, Intercom, Inc. (legal@intercom.io) with Reserved Font Name "Inter". This Font Software is licensed under the SIL Open Font License, Version 1.1.[Skip to main content](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#main-content)

[![Image 1: SafeBase Help Center](https://downloads.intercomcdn.com/i/o/am69aeco/936714/d04dfa8869410748abaf552c2b4d/2166c043d05a8b3df6f8ecc87e07abb5.png)](https://help.safebase.io/en/)

[Resource Center](https://drata.com/resources)[Login](https://app.safebase.io/)

[Resource Center](https://drata.com/resources)[Login](https://app.safebase.io/)

Search for articles...

1.   [All Collections](https://help.safebase.io/en/)
2.   [General Resources](https://help.safebase.io/en/collections/19708328-general-resources)
3.   SafeBase MCP Server Guide

# SafeBase MCP Server Guide

SafeBase MCP Server Guide

Updated over 3 weeks ago

Table of contents

[Overview](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_faf6e111db)[What is it?](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_2564efb2be)[Who can use it?](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_c6e0da9afb)[Disclaimer](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_cbc051243b)[Setup](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_f64762a6e4)[Prerequisites](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_1ddb47cb40)[Client-specific setup](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_8262d643dd)[ChatGPT (Plus / Pro / Business / Enterprise / Edu)](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_de03bccc8c)[Claude Code](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_744c1cb66b)[Cursor](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_759b1d9403)[Tools Supported](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_3a5bc13888)[Ask AI](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_4509c16f06)[Accounts](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_ac5d132a44)[Account Members](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_fb40874afe)[Trust Center access requests](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_fba8d08b42)[Trust Center Updates](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_16d63c6dc9)[Trust Center content](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_84a1b3e351)[Content Collections](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_620f29be0c)[Documents](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_7157339c1e)[Knowledge Base](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_454e6bfedd)[Questionnaires](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_fbca9beb22)[Org / admin](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_cd960cf286)[Trust Library reference](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_a17639611a)[Trust Library Search](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_af0100003a)[Permission Profiles](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_4dabdac0f6)[Translations](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_6776b6296a)

# Overview

Bring AI tools to the Trust Center. Search and manage Knowledge Base entries, approve access requests, and pull approved Trust Library content into your AI workflows from Claude, Cursor, ChatGPT, or any MCP-compatible client.

## What is it?

*   An MCP (Model Context Protocol) server that connects SafeBase to AI clients
*   The MCP server lets you use natural language to manage your Trust Library content without leaving your AI tool of choice.
*   Works with any MCP-compatible client

## Who can use it?

*   You must be an active organization member in SafeBase

## Disclaimer

*   Organizations can enable the MCP server with read-only or read + write permissions.
*   With read + write enabled, you accept the risk of unintended changes to your Trust Library content. Some tools overwrite or remove data: `questionnaires_retry_autofill_rows` queues eligible rows for a new AI answer, replacing what is there now, `questionnaires_edit_answer_and_comment` clears a row's AI answer sources and confidence when you change it, `tags_delete` and `translations_apply` in delete mode remove data, and the Trust Center content tools publish live with no draft step.

# Setup

## Prerequisites

*   An admin must enable either read-only or read + write in **Settings > Generative AI > MCP** for your organization.

**SafeBase MCP server URL:**`https://app.safebase.io/api/mcp`

## Client-specific setup

**Claude ([claude.ai](http://claude.ai/)) and Claude Desktop**
​

**Team/Enterprise:** An owner or admin must first add the connector via **Organization settings → Connectors → Add → Custom → Web**. Once added, members can connect from the steps below.

1.   Open **Customize → Connectors**
2.   Click the **+** button → **Add custom connector**
3.   Enter the SafeBase server URL above
4.   Sign in to SafeBase when prompted
5.   To use it in a chat: click the **+** button (lower left) → **Connectors** → enable SafeBase

## ChatGPT (Plus / Pro / Business / Enterprise / Edu)

Custom MCP connectors require **Developer Mode**. For Team/Enterprise/Edu, a workspace admin must first enable it under **Admin Workspace Settings → Permissions & Roles → Connected Data → Developer mode**.

1.   Open **Settings → Connectors**
2.   Scroll to **Advanced** and toggle **Developer mode** on
3.   Click **Create**
4.   Enter the SafeBase server URL above
5.   Sign in to SafeBase when prompted

## Claude Code

Run:

claude mcp add --transport http safebase <https://app.safebase.io/api/mcp>

Then inside Claude Code, run `/mcp` to complete the browser sign-in.

## Cursor

Add the SafeBase server to `~/.cursor/mcp.json` (or `.cursor/mcp.json` in your project):

{   "mcpServers": {     "safebase": {       "url": "<https://app.safebase.io/api/mcp>"     }   } }

Restart Cursor and sign in to SafeBase when prompted.

# Tools Supported

## Ask AI

*   `ai_ask` — Ask a natural-language question grounded in your approved Trust Library content

## Accounts

*   `accounts_list` — List accounts
*   `accounts_get` — Get an account by ID
*   `accounts_create` — Create an account
*   `accounts_edit` — Edit an account

## Account Members

*   `accounts_find_member` — Search account members across accounts
*   `accounts_add_members` — Add members to an account

## Trust Center access requests

*   `access_requests_list` — List access requests
*   `access_requests_approve` — Approve an access request

## Trust Center Updates

*   `tcu_list_topics` — List update topics
*   `tcu_get_topic` — Get a topic by ID
*   `tcu_create_topic` — Create a new topic
*   `tcu_update_topic` — Update a topic
*   `tcu_create_update` — Post an update under a topic

## Trust Center content

*   `trust_center_content_get` — Retrieve Trust Center products and items
*   `trust_center_content_update_item` — Update a Trust Center item's title, description, maturity, or shared products. Changes go live immediately.
*   `trust_center_content_link_documents` — Link or unlink documents on a Trust Center item. Changes go live immediately.

## Content Collections

*   `content_collections_list` — List content collections
*   `content_collections_get` — Retrieve one content collection by ID, including its sources and tag filters
*   `content_collections_create` — Create a content collection
*   `content_collections_edit_tags` — Add or remove tag filters on a content collection

## Documents

*   `documents_download` — Retrieve short-lived download links for one or more documents
*   `documents_list` — List documents
*   `documents_get` — Get a document by ID
*   `documents_create` — Create a document
*   `documents_update` — Update a document
*   `documents_upload_request` — Create a document upload request

## Knowledge Base

*   `kb_search` — Search KB entries
*   `kb_list` — List KB entries with filters (tag, source, product, assignee, health, review cadence)
*   `kb_create_entry` — Create a new KB entry
*   `kb_update_entry` — Update an existing KB entry's content
*   `kb_get_suggestions` — Pull AI-generated KB suggestions
*   `kb_handle_suggestion` — Approve or dismiss a KB suggestion
*   `kb_sources_list` — List available KB content sources
*   `kb_get_entries_assignees` — Get assignees for KB entries
*   `kb_set_assignees` — Set assignees on a KB entry
*   `kb_create_suggestion` — Create a KB suggestion for human review instead of writing to the KB directly
*   `kb_bulk_edit_tags` — Add or remove tags on multiple KB entries at once, leaving other tags in place

## Questionnaires

*   `questionnaires_list` — List questionnaires
*   `questionnaires_get` — Get a questionnaire by ID
*   `questionnaires_get_sheets` — List the sheets in a questionnaire
*   `questionnaires_get_sheet_rows` — Get the rows in a sheet
*   `questionnaires_get_row_assignees` — Get assignees on a row
*   `questionnaires_set_row_assignees` — Set assignees on a row
*   `questionnaires_bulk_assign_rows` — Bulk-assign rows to members
*   `questionnaires_edit_answer_and_comment` — Update the answer and/or comment on a questionnaire row. This clears the row's AI answer sources and confidence, the same as editing it in the app.
*   `questionnaires_retry_autofill_rows`¹ — Rerun AI autofill on selected rows. This overwrites current answers and comments.

## Org / admin

*   `org_get_me` — Get your own profile
*   `org_get_my_permissions` — Get your effective permissions
*   `org_get_info` — Get org info
*   `org_get_settings` — Get org settings
*   `org_get_nda` — Get the NDA
*   `org_list_members` — List org members
*   `org_invite_members` — Invite new members
*   `org_list_invites` — List pending invites
*   `org_list_roles` — List roles
*   `org_get_role` — Get a role by ID
*   `org_create_role` — Create a role
*   `org_update_role` — Update a role

## Trust Library reference

*   `tags_list` — List available tags
*   `products_list` — List available products
*   `search_assignees` — Search assignable members and groups by name or email.
*   `tags_create` — Creates a tag, or returns the existing tag if one with that name already exists
*   `tags_update` — Renames a tag
*   `tags_delete` — Deletes one or more tags. Tags still linked to a content collection are skipped.

## Trust Library Search

*   `trust_library_search` — Search Trust Library content by relevance and return matching items without generating an answer

## Permission Profiles

*   `permission_profiles_link_documents`² — Link documents to a permission profile

## Translations

*   `translations_explore`³ — Review Trust Center translations, including supported languages, coverage, and existing translated values.
*   `translations_apply`³ — Add, update, or remove translated values for Trust Center content

¹ Only available to orgs with Questionnaire Assistance.

² Behind the `enableDocumentInPermissionProfile` feature flag.

³ Behind the `enableClientTranslationsExtApi` feature flag.

**Reviewing a vendor’s Trust Center?** Use the [Vendor Trust Center MCP User Guide](https://help.safebase.io/en/articles/16203034-safebase-vendor-trust-center-mcp-user-guide-early-preview).

* * *

Related Articles

*   [SafeBase - Vendor Trust Center MCP User Guide (Early Preview)](https://help.safebase.io/en/articles/16203034-safebase-vendor-trust-center-mcp-user-guide-early-preview)
*   [Release Notes - August 2024](https://help.safebase.io/en/articles/16203096-release-notes-august-2024)
*   [Integration - SafeBase API](https://help.safebase.io/en/articles/16203145-integration-safebase-api)
*   [Integration - SafeBase Bot for Slack - Setup & Use](https://help.safebase.io/en/articles/16203163-integration-safebase-bot-for-slack-setup-use)
*   [SafeBase - Start To Finish](https://help.safebase.io/en/articles/16203234-safebase-start-to-finish)

Did this answer your question?

Disappointed Reaction😞Neutral Reaction😐Smiley Reaction😃

Table of contents

[Overview](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_faf6e111db)[What is it?](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_2564efb2be)[Who can use it?](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_c6e0da9afb)[Disclaimer](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_cbc051243b)[Setup](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_f64762a6e4)[Prerequisites](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_1ddb47cb40)[Client-specific setup](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_8262d643dd)[ChatGPT (Plus / Pro / Business / Enterprise / Edu)](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_de03bccc8c)[Claude Code](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_744c1cb66b)[Cursor](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_759b1d9403)[Tools Supported](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_3a5bc13888)[Ask AI](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_4509c16f06)[Accounts](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_ac5d132a44)[Account Members](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_fb40874afe)[Trust Center access requests](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_fba8d08b42)[Trust Center Updates](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_16d63c6dc9)[Trust Center content](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_84a1b3e351)[Content Collections](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_620f29be0c)[Documents](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_7157339c1e)[Knowledge Base](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_454e6bfedd)[Questionnaires](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_fbca9beb22)[Org / admin](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_cd960cf286)[Trust Library reference](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_a17639611a)[Trust Library Search](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_af0100003a)[Permission Profiles](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_4dabdac0f6)[Translations](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide#h_6776b6296a)

[![Image 2: SafeBase Help Center](https://downloads.intercomcdn.com/i/o/am69aeco/937034/e8edc5ee61a0bf7d3a81a178c397/2166c043d05a8b3df6f8ecc87e07abb5.png)](https://help.safebase.io/en/)

Resources

*   [Blog](https://safebase.io/blog)
*   [Resource Center](https://safebase.io/resources)
*   [Women in Trust](https://safebase.io/women-in-trust)

Company

*   [Home](https://safebase.io/)
*   [About Us](https://safebase.io/about)
*   [Careers](https://safebase.io/careers)
*   [Pricing](https://safebase.io/pricing)
*   [Trust Center](https://trust.safebase.io/)
