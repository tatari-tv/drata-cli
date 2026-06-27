Title: Create Task and Notify When Control Changes to "Not Ready"

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#objective)Objective

Automatically alert stakeholders and create an actionable task whenever a **control's readiness changes to "Not Ready"**. This workflow ensures that regressions in control readiness are immediately surfaced and assigned for follow-up—supporting continuous compliance and timely remediation.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#prerequisites)Prerequisites

Before configuring this workflow, ensure:

*    You have **Admin** or **Workspace Manager** permissions.
*    You know who should respond when a control becomes not ready (e.g., Control Owner, Admin, Security Team).
*    Notification channels are available:
    *   **Email** (always available)
    *   **Slack message** (Slack integration required)
    *   **Microsoft Teams message** (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#workflow-overview)Workflow Overview

This workflow runs when:

*   **Object Type:** Control
*   **Scope:** All controls
*   **Trigger Event:** Control readiness changed to "Not Ready"
*   **Actions:**
    *    Create a task for the appropriate owner
    *    Send a notification (Email, Slack, or Teams)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#1-create-the-workflow)1. Create the Workflow

1.    Navigate to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Configure:
    *   **Name:**`Create task and notify when control "not ready"`
    *   **Object Type:**`Control`

![Image 1: Create Workflow](https://cdn.drata.com/developers/custom_workflows/recipe_6/create_workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    Set the scope to **All controls** .
2.    Select **Continue** .

![Image 2: Start](https://cdn.drata.com/developers/custom_workflows/recipe_6/start.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#3-select-the-trigger)3. Select the Trigger

1.    Choose **Control readiness changed to "Not ready"** as the trigger.
2.    This event fires whenever a control's readiness status drops from Ready → Not Ready.

![Image 3: Trigger](https://cdn.drata.com/developers/custom_workflows/recipe_6/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#create-task)Create Task

1.    Add a **Create task** step.
2.    Configure the task:
    *   **Title:**`Control marked as not ready: {{control_name}}`
    *   **Description:**

Copy

Copied

```
The control "{{control_name}}" has changed to a Not Ready state.

Please review this control, identify the underlying issues, and take corrective action.
```

    *   **Assign to:**
    *   **Control Owner** , or
    *    Specific role (e.g., Admin, Security Team)
    *   **Due Date:** Set an appropriate follow-up timeframe (e.g., 5 days from task creation)

![Image 4: Create Task](https://cdn.drata.com/developers/custom_workflows/recipe_6/create_task.png)

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#send-notification-email-example)Send Notification (Email Example)

1.    Add a **Send notification** step.
2.    Select method:
    *   **Email**
    *   **Slack message**
    *   **Microsoft Teams message**

3.    For this example, configure **Email** .
4.    Choose recipient(s), such as:
    *   **Control Owner**
    *   **Admin**
    *   **Compliance Team**

5.    Use dynamic variables such as:
    *   `{{control_name}}`
    *   `{{control_readiness}}`
    *   `{{date_changed}}`
    *   `{{updated_by}}`

**Example Email Subject:**`Control moved to Not Ready: {{control_name}}`

**Example Email Body:**

Copy

Copied

```
Hello,

The control "{{control_name}}" has changed to a "Not Ready" state.

Date Changed: {{date_changed}}
Updated By: {{updated_by}}

A task has been created for follow-up. Please review and take appropriate action.

Thank you,
Compliance Team
```

![Image 5: Send Email](https://cdn.drata.com/developers/custom_workflows/recipe_6/send_email.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#5-review-and-publish)5. Review and Publish

1.    Confirm the workflow configuration:
    *    Scope: **All controls**
    *    Trigger: **Control readiness changed to Not Ready**
    *    Steps: **Create task** and **Send notification**

2.    Select **Publish** to activate the workflow.
3.    Or save as **Draft** to review with stakeholders.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-notify-control-not-ready/#validation--testing)Validation & Testing

To ensure proper operation:

1.    Select a **test control** and manually update it to "Not Ready."
2.    Go to **Settings → Workflows → Run History** to verify the workflow executed.
3.    Confirm:
    *    A **task** was created and assigned correctly.
    *    The **notification** was successfully delivered.

4.    Adjust message content or recipients as needed.
