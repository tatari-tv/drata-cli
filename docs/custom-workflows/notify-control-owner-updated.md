Title: Notify When Control Owner Is Updated

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/

Published Time: Fri, 26 Jun 2026 18:55:00 GMT

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#objective)Objective

Automatically notify relevant stakeholders whenever the **owner of a control changes**. This workflow improves visibility into ownership updates, ensures new owners are aware of their responsibilities, and helps teams maintain accurate role assignments across the GRC program.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#prerequisites)Prerequisites

Before configuring this workflow:

*    You must have **Admin** or **Workspace Manager** permissions.
*    You should know who needs to be notified of control owner changes (e.g., Admins, Compliance Team).
*    Your notification channels must be available:
    *   **Email** (always available)
    *   **Slack message** (Slack integration required)
    *   **Microsoft Teams message** (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#workflow-overview)Workflow Overview

This workflow runs when:

*   **Object Type:** Control
*   **Scope:** All controls
*   **Trigger Event:** Control owner updated
*   **Action:** Notify selected recipients of the update

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#1-create-the-workflow)1. Create the Workflow

1.    Go to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Configure:
    *   **Name:**`Notify when control owner updated`
    *   **Object Type:**`Control`

![Image 1: Create Workflow](https://cdn.drata.com/developers/custom_workflows/recipe_8/create_workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    Under **Start** , choose **All controls** to monitor all current and future controls.
2.    Select **Continue** .

![Image 2: Start](https://cdn.drata.com/developers/custom_workflows/recipe_8/start.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#3-select-the-trigger)3. Select the Trigger

1.    Choose **Control owner updated** as the trigger.
2.    This event fires whenever a control's owner field changes.

![Image 3: Trigger](https://cdn.drata.com/developers/custom_workflows/recipe_8/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#send-notification-email-example)Send Notification (Email Example)

1.    Add a **Send notification** step.
2.    Select a communication method:
    *   **Email**
    *   **Slack message**
    *   **Microsoft Teams message**

3.    For this example, use **Email** .
4.    Choose recipients such as:
    *   **Admin**
    *   **Compliance Team**
    *   **New Control Owner**

5.    Use dynamic variables such as:
    *   `{{control_name}}`
    *   `{{old_control_owner}}`
    *   `{{new_control_owner}}`
    *   `{{updated_by}}`

**Example Email Subject:**`Control owner updated: {{control_name}}`

**Example Email Body:**

Copy

Copied

```
Hello,

The control "{{control_name}}" has had its owner updated.

Previous Owner: {{old_control_owner}}
New Owner: {{new_control_owner}}
Updated By: {{updated_by}}

Please review responsibilities and ensure all required tasks are reassigned appropriately.

Thank you,
Compliance Team
```

![Image 4: Send Email](https://cdn.drata.com/developers/custom_workflows/recipe_8/send_email.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#5-review-and-publish)5. Review and Publish

1.    Confirm the workflow settings:
    *    Scope: **All controls**
    *    Trigger: **Control owner updated**
    *    Action: Notify appropriate recipients

2.    Select **Publish** to activate.
3.    Save as **Draft** if additional internal review is required.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-updated/#validation--testing)Validation & Testing

To ensure proper functionality:

1.    Select a **test control** and update its owner.
2.    Go to **Settings → Workflows → Run History** to verify the workflow executed.
3.    Confirm recipients received the notification.
4.    Adjust the message or recipients as needed.
