Title: Notify Reminder to the Control Owner on Evidence Past Due

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/

Published Time: Fri, 26 Jun 2026 18:55:00 GMT

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#objective)Objective

Automatically notify the **control owner** when linked evidence is **past its renewal date**. This workflow ensures control owners are aware of overdue evidence and can take corrective action quickly, reducing audit risk and maintaining control readiness.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#prerequisites)Prerequisites

Before setting up this workflow, ensure:

*    You have **Admin** or **Workspace Manager** permissions.
*    You know which controls and evidence types should be monitored.
*    Your notification channels are available:
    *   **Email** (always available)
    *   **Slack message** (Slack integration required)
    *   **Microsoft Teams message** (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#workflow-overview)Workflow Overview

This workflow executes when:

*   **Object Type:** Evidence
*   **Scope:** All manual evidence
*   **Trigger Event:** Renewal past due
*   **Actions:**
    *    Notify control owners of overdue evidence

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#1-create-the-workflow)1. Create the Workflow

1.    Navigate to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Configure:
    *   **Name:**`Notify reminder to the control owner on evidence past due`
    *   **Object Type:**`Evidence`

![Image 1: Create Workflow](https://cdn.drata.com/developers/custom_workflows/recipe_4/create_workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    In the **Start** step, choose **All manual evidence** .
2.    Select **Continue** to proceed.

![Image 2: Start](https://cdn.drata.com/developers/custom_workflows/recipe_4/start.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#3-select-the-trigger)3. Select the Trigger

1.    Choose **Renewal past due** as the trigger.
2.    This trigger fires when the evidence's renewal date has passed without replacement or update.

![Image 3: Trigger](https://cdn.drata.com/developers/custom_workflows/recipe_4/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#send-notification-email-example)Send Notification (Email Example)

1.    Add a **Send notification** step.
2.    Select your notification method:
    *   **Email**
    *   **Slack message**
    *   **Microsoft Teams message**

3.    For this example, configure **Email** .
4.    Set **Control Owner** as the recipient.
5.    Compose your message using dynamic evidence variables such as:
    *   `{{evidence_name}}`
    *   `{{evidence_owner_name}}`
    *   `{{evidence_renewal_date}}`
    *   `{{evidence_linked_control_name}}`

**Example Email Subject:**`Evidence renewal past due: {{evidence_name}}`

**Example Email Body:**

Copy

Copied

```
Hello,

The evidence item "{{evidence_name}}" linked to control "{{evidence_linked_control_name}}" is past its renewal date ({{evidence_renewal_date}}).

Please review this item and update it as soon as possible to maintain control readiness.

Thank you,
Compliance Team
```

![Image 4: Send Email](https://cdn.drata.com/developers/custom_workflows/recipe_4/send_email.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#5-review-and-publish)5. Review and Publish

1.    Review the workflow configuration:
    *    Scope: **All manual evidence**
    *    Trigger: **Renewal past due**
    *    Notification recipients: **Control owners**

2.    Select **Publish** to activate the workflow.
3.    Save as **Draft** if additional internal review is required.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-evidence-past-due/#validation--testing)Validation & Testing

To ensure the workflow is functioning correctly:

1.    Locate or create a **test evidence item** with a renewal date in the past.
2.    Link it to a control.
3.    Confirm workflow execution under **Settings → Workflows → Run History** .
4.    Validate the control owner received the notification.
5.    Adjust messaging or recipients as needed.
