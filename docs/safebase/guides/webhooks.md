Title: Integration - Webhooks

URL Source: https://help.safebase.io/en/articles/16203084-integration-webhooks

Markdown Content:
Copyright (c) 2023, Intercom, Inc. (legal@intercom.io) with Reserved Font Name "Inter". This Font Software is licensed under the SIL Open Font License, Version 1.1.Copyright (c) 2023, Intercom, Inc. (legal@intercom.io) with Reserved Font Name "Inter". This Font Software is licensed under the SIL Open Font License, Version 1.1.[Skip to main content](https://help.safebase.io/en/articles/16203084-integration-webhooks#main-content)

[![Image 2: SafeBase Help Center](https://downloads.intercomcdn.com/i/o/am69aeco/936714/d04dfa8869410748abaf552c2b4d/2166c043d05a8b3df6f8ecc87e07abb5.png)](https://help.safebase.io/en/)

[Resource Center](https://drata.com/resources)[Login](https://app.safebase.io/)

[Resource Center](https://drata.com/resources)[Login](https://app.safebase.io/)

Search for articles...

1.   [All Collections](https://help.safebase.io/en/)
2.   [Integrations](https://help.safebase.io/en/collections/19708610-integrations)
3.   [Webhooks](https://help.safebase.io/en/collections/19708738-webhooks)
4.   Integration - Webhooks

# Integration - Webhooks

Learn how to use the Trust Center's Webhooks feature for custom workflows.

September 24, 2026

Table of contents

[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_a5e726f372)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_35b75db973)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_1cc12b5c55)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_93b5b0838f)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_c113f33b4c)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_e6da3119f7)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_c605b3af34)

# Overview

Webhooks in the Trust Center let organizations trigger automations in other tools whenever specific events happen in your Trust Center. They can typically be wired up through middleware like Zapier, or essentially anything that receive a webhook, and can be used to build helpful notifications or automations.
​

Common use cases include:

*   Posting a message in a specific Slack channel when a user signs an NDA
*   Sending a message in Microsoft Teams when someone views or downloads a SOC 2 report
*   Creating a new task in ClickUp or Asana when a user requests Trust Center access.
*   Generating an entry in a Google Sheet to document a new access request

Beyond those examples, webhooks can be used for things like routing approval requests, syncing access events to a SIEM or data warehouse, kicking off CRM updates, or notifying internal teams about questionnaire activity.

## Configuration

Webhooks are enabled in the **Integrations**tab of the **Settings**page.

*   Navigate to the Data section
*   Click on the **Connect button**
*   A modal will open with an **Add Endpoint**button

[![Image 3](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639830/bf882c2309abc99ca0e7a823e8cb/upload_5795706412528273273.png?expires=1793950200&signature=0baaddbe7a6eb2bcdd24cebec559d2f47d9aa25f4da9528c0d7c84b48014103b&req=diUvE899lIlcWfMW1HO4zZkGnykg77ESPajfq85eTqyNi6ZxsVZZSbHsGl4N%0APs5VpVfFtR8Dg0ONTwE%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639830/bf882c2309abc99ca0e7a823e8cb/upload_5795706412528273273.png?expires=1793950200&signature=0baaddbe7a6eb2bcdd24cebec559d2f47d9aa25f4da9528c0d7c84b48014103b&req=diUvE899lIlcWfMW1HO4zZkGnykg77ESPajfq85eTqyNi6ZxsVZZSbHsGl4N%0APs5VpVfFtR8Dg0ONTwE%3D%0A)

*   The **Event Catalog**tab lists all events.

    *   Use the Event Catalog to see the format of the data sent to the webhook. All data is returned as a JSON object.
​ [![Image 4](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639844/26e4eb4ec24beea180f2326b07fe/upload_1350733767648743449.png?expires=1793950200&signature=d9c91541c3826b7123a56449f749ec86c7196610752b450b3f4eb6711d326605&req=diUvE899lIlbXfMW1HO4zTqY%2B9UPD5q1iTrC6%2BYu4393h4LN0DoQm4%2BYdeVa%0AqfqX%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639844/26e4eb4ec24beea180f2326b07fe/upload_1350733767648743449.png?expires=1793950200&signature=d9c91541c3826b7123a56449f749ec86c7196610752b450b3f4eb6711d326605&req=diUvE899lIlbXfMW1HO4zTqY%2B9UPD5q1iTrC6%2BYu4393h4LN0DoQm4%2BYdeVa%0AqfqX%0A)

*   When adding an endpoint, specify the webhook URL, and which events should trigger it.

    *   In this example, the webhook is triggering when an NDA is completed
​ [![Image 5](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639858/c8291c104632487125d6eac70fa7/upload_14149225660727897623.png?expires=1793950200&signature=47f26f0f217601aee5daa0a56fcb7b95d50304078ca720755f2cd80bb9f65ea0&req=diUvE899lIlaUfMW1HO4zZTcIGVxZZbdX0FtRw8OVF923Z5NBFWj9hHb%2BrCQ%0AG4Ip%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639858/c8291c104632487125d6eac70fa7/upload_14149225660727897623.png?expires=1793950200&signature=47f26f0f217601aee5daa0a56fcb7b95d50304078ca720755f2cd80bb9f65ea0&req=diUvE899lIlaUfMW1HO4zZTcIGVxZZbdX0FtRw8OVF923Z5NBFWj9hHb%2BrCQ%0AG4Ip%0A)

*   After saving, there will be an option to configure rate limiting or pass a custom header
​ [![Image 6](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639877/d622031ec6d00fa464e3e71e0562/upload_9064515884658993576.png?expires=1793950200&signature=058af140013e76217ed3520df3126068a8fa2648c4b7a3f85cc00978ff7bf09f&req=diUvE899lIlYXvMW1HO4zQpqikT2EGKfTZr20aWuE%2Fni10N761kdRRY%2FwZEc%0A%2F7YH%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639877/d622031ec6d00fa464e3e71e0562/upload_9064515884658993576.png?expires=1793950200&signature=058af140013e76217ed3520df3126068a8fa2648c4b7a3f85cc00978ff7bf09f&req=diUvE899lIlYXvMW1HO4zQpqikT2EGKfTZr20aWuE%2Fni10N761kdRRY%2FwZEc%0A%2F7YH%0A)

## Testing

*   Test events can be sent to ensure that the webhook and any subsequent workflow logic are functioning as intended using the Testing tab
​ [![Image 7](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639897/b9a204b4c1c82fcd8034f857ec09/upload_11032508910518254442.png?expires=1793950200&signature=edc4920cef4a0b2528c031a19654ebda0496abea8505b958ac3092942bb76692&req=diUvE899lIlWXvMW1HO4zcP0S9VYCRXDqyAkb4cEBb%2FfXFSFDcICjMeTtBlZ%0AKP%2F3%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639897/b9a204b4c1c82fcd8034f857ec09/upload_11032508910518254442.png?expires=1793950200&signature=edc4920cef4a0b2528c031a19654ebda0496abea8505b958ac3092942bb76692&req=diUvE899lIlWXvMW1HO4zcP0S9VYCRXDqyAkb4cEBb%2FfXFSFDcICjMeTtBlZ%0AKP%2F3%0A)

## Logging

*   The Logs tab allows users to view a history of recent webhook events and whether they were successful or not.
*   We recommend firing a few test events when configuring a new webhook endpoint for the first time.
​ [![Image 8](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639919/91ad66c1f1430c5b322e6d4e056a/upload_12346223023234692389.png?expires=1793950200&signature=92114ace66ecbd289f402ead7ac50057a47a83c62ca8d39fc4397fc86b75922b&req=diUvE899lIheUPMW1HO4zb0s%2FQFp8eZWGdgs%2BQDd6MfBdTNkQMMr91OwVNxf%0A00cQ%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585639919/91ad66c1f1430c5b322e6d4e056a/upload_12346223023234692389.png?expires=1793950200&signature=92114ace66ecbd289f402ead7ac50057a47a83c62ca8d39fc4397fc86b75922b&req=diUvE899lIheUPMW1HO4zb0s%2FQFp8eZWGdgs%2BQDd6MfBdTNkQMMr91OwVNxf%0A00cQ%0A)

* * *

## Event Catalog

For a full list of available webhook events, please visit our [Event Catalog](https://www.svix.com/event-types/us/org_2Lmnm325Gqjzoty7vhdMgL1HOxh/).

* * *

## Example Webhook Workflows

Our Webhooks feature can be used with various existing tools to automate Customer Trust workflows.
​
Here are example custom workflows organizations can create using this feature:

*   Slack has the ability to listen to [incoming webhooks](https://api.slack.com/messaging/webhooks).

    *   Create a custom Slack app with an incoming webhook and post a message to a channel/custom DM group with custom formatting if a user has signed an NDA or downloaded a SIG questionnaire.

*   Orgs can use Webhooks to send SafeBase-related activity to SIEM tools to act as an audit log.

    *   For example, send file download events to Splunk using an [HTTP Event Collector](https://docs.splunk.com/Documentation/Splunk/8.2.2/Data/UsetheHTTPEventCollector). Note: use the [/raw](https://help.safebase.io/en/articles/16203084-integration-webhooks) endpoint.

*   A legal team may want a record of any Clickwrap NDAs in a system such as Dropbox or Box. Zapier can be used to store completed NDA information in these third-party tools.
*   Zapier is the most commonly used no-code integration middleware that we have seen our customers use. Some possible Zapier workflows include:

    *   Some customers do not like to use Slack or Emails for SafeBase notifications. Webhooks can be used to [create a new ClickUp task](https://help.safebase.io/en/articles/7260845-webhooks-clickup-access-requests-via-zapier) whenever a user submits an Access Request.
    *   Customers may already be tracking inbound questionnaires using an existing project management tool. Webhooks can be used to create a [new Asana task](https://zapier.com/apps/asana/integrations/webhook) whenever a user submits a new questionnaire.
    *   Some of our customers do not use a major CRM solution such as Salesforce and HubSpot. The Webhooks feature can be used to [generate a new entry in a Google Sheet](https://zapier.com/apps/google-sheets/integrations/webhook) as a way to capture potential new leads when an access request is submitted.
    *   Please see the [Zapier Webhooks overview](https://zapier.com/apps/webhook/integrations) for even more ideas.

* * *

## Troubleshooting

*   If using a Splunk HTTP collector, use the `/raw` endpoint rather than `/event`.
*   Refer to [https://docs.svix.com/receiving/source-ips](https://docs.svix.com/receiving/source-ips) for a list of allow-list IPs
*   If the event name is not part of the payload when configuring a webhook, this is normal. The test event does not have the event name, but a real event will.

* * *

Related Articles

*   [Release Notes – November 2025](https://help.safebase.io/en/articles/16203052-release-notes-november-2025)
*   [Release Notes - April 2023](https://help.safebase.io/en/articles/16203114-release-notes-april-2023)
*   [Trust Center - Customer Self-Serve Questionnaire Submission](https://help.safebase.io/en/articles/16203152-trust-center-customer-self-serve-questionnaire-submission)
*   [Integration - SafeBase Bot for Slack - Setup & Use](https://help.safebase.io/en/articles/16203163-integration-safebase-bot-for-slack-setup-use)
*   [Trust Center Adoption - Process Changes](https://help.safebase.io/en/articles/16203222-trust-center-adoption-process-changes)

Did this answer your question?

Disappointed Reaction😞Neutral Reaction😐Smiley Reaction😃

Table of contents

[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_a5e726f372)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_35b75db973)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_1cc12b5c55)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_93b5b0838f)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_c113f33b4c)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_e6da3119f7)[](https://help.safebase.io/en/articles/16203084-integration-webhooks#h_c605b3af34)

[![Image 9: SafeBase Help Center](https://downloads.intercomcdn.com/i/o/am69aeco/937034/e8edc5ee61a0bf7d3a81a178c397/2166c043d05a8b3df6f8ecc87e07abb5.png)](https://help.safebase.io/en/)

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
