Title: SafeBase API - Managing Documents

URL Source: https://help.safebase.io/en/articles/16203215-safebase-api-managing-documents

Markdown Content:
Copyright (c) 2023, Intercom, Inc. (legal@intercom.io) with Reserved Font Name "Inter". This Font Software is licensed under the SIL Open Font License, Version 1.1.Copyright (c) 2023, Intercom, Inc. (legal@intercom.io) with Reserved Font Name "Inter". This Font Software is licensed under the SIL Open Font License, Version 1.1.[Skip to main content](https://help.safebase.io/en/articles/16203215-safebase-api-managing-documents#main-content)

1.   [All Collections](https://help.safebase.io/en/)
2.   [Integrations](https://help.safebase.io/en/collections/19708610-integrations)
3.   SafeBase API - Managing Documents

General walkthrough on how to utilize SafeBase's API to manage documents in the Trust Center.

September 24, 2026

## Overview

Security and compliance documentation often lives across disparate and niche repositories where native file integration often does not exist.
​
With the SafeBase documents API, organizations can now integrate these repositories with SafeBase to keep documents in sync without managing files across multiple systems.
​

This API workflow allows teams to programmatically add, replace, and delete the documentation in SafeBase.

## Guide

This guide outlines the high-level workflows for uploading, replacing, and removing documents using our `documents` endpoints.

**API documentation for all of SafeBase’s endpoints [here](https://docs.safebase.io/v1.20.0/reference/getaccounts).**

## Generate an API key

Admins can follow [this guide](https://help.safebase.io/en/articles/6119329-integration-safebase-api) to generate an API key.

## Upload a New Document

SafeBase uses a secure, three step process for handling file uploads. Instead of sending the file data directly to our API, users will first request a secure upload location and then confirm the creation of the document.

### The Workflow

## Replacing an Existing File

To replace an existing document, e.g., replacing "Security Policy v1" with "Security Policy v2", without creating a duplicate record or breaking existing links, follow a similar flow to uploading a new file.

### The Workflow

## Deleting a Document

If a document is no longer needed, it can permanently remove it from the Trust Center using its unique ID.

### The Workflow:

## **Video Walkthrough**

A walkthrough of the Documents APIs can be found [here](https://www.loom.com/share/d3df5ff0dd034d17b319698f5f8c4425).

* * *

Related Articles

*   [SafeBase MCP Server Guide](https://help.safebase.io/en/articles/16203026-safebase-mcp-server-guide)
*   [Integration - Salesforce - "Add to SafeBase Button" Component](https://help.safebase.io/en/articles/16203039-integration-salesforce-add-to-safebase-button-component)
*   [Integration - SafeBase API](https://help.safebase.io/en/articles/16203145-integration-safebase-api)
*   [Integration - Drata](https://help.safebase.io/en/articles/16203172-integration-drata)
*   [SafeBase - Start To Finish](https://help.safebase.io/en/articles/16203234-safebase-start-to-finish)
