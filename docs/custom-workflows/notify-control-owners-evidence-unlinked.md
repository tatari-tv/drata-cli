Title: Notify Control Owners When Evidence Is Unlinked From Their Control

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/

Published Time: Fri, 26 Jun 2026 18:55:00 GMT

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#objective)Objective

Automatically notify **control owners** when existing evidence is **unlinked** from one of their controls. This workflow helps owners quickly assess whether the removal was intentional, prevents gaps in documentation, and supports continuous audit readiness.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#prerequisites)Prerequisites

Before building this workflow, ensure:

*    You have **Admin** or **Workspace Manager** permissions.
*    You know which controls require monitoring for evidence removal.
*    Your notification channels are available:
    *   **Email** (always available)
    *   **Slack message** (Slack integration required)
    *   **Microsoft Teams message** (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#workflow-overview)Workflow Overview

This workflow runs when:

*   **Object Type:** Evidence
*   **Scope:** All manual evidence
*   **Trigger Event:** Evidence unlinked from a control
*   **Action:** Notify control owners of the unlinked evidence

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#1-create-the-workflow)1. Create the Workflow

1.    Navigate to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Configure:
    *   **Name:**`Notify control owners when evidence is unlinked from their control`
    *   **Object Type:**`Evidence`

![Image 1: Create Workflow](https://cdn.drata.com/developers/custom_workflows/recipe_7/create_workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    Under **Start** , select **All manual evidence** .
2.    Select **Continue** .

![Image 2: Start](https://cdn.drata.com/developers/custom_workflows/recipe_7/start.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#3-select-the-trigger)3. Select the Trigger

1.    Choose **Evidence unlinked from a control** as the trigger.
2.    This workflow fires whenever an existing evidence item is removed from a control.

![Image 3: Trigger](https://cdn.drata.com/developers/custom_workflows/recipe_7/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#send-notification-email-example)Send Notification (Email Example)

1.    Add a **Send notification** step.
2.    Choose a communication channel:
    *   **Email**
    *   **Slack message**
    *   **Microsoft Teams message**

3.    For example purposes, configure **Email** .
4.    Set **Control Owner** as the recipient.
5.    Include dynamic variables such as:
    *   `{{evidence_name}}`
    *   `{{evidence_unlinked_control_name}}`
    *   `{{evidence_unlinked_control_on}}`
    *   `{{evidence_unlinked_control_by}}`

**Example Email Subject:**`Evidence unlinked from your control: {{evidence_name}}`

**Example Email Body:**

Copy

Copied

```
Hello,

The evidence item "{{evidence_name}}" has been unlinked from control "{{evidence_unlinked_control_name}}".

Unlinked on: {{evidence_unlinked_control_on}}
Unlinked by: {{evidence_unlinked_control_by}}

If this change was not expected, please review and take appropriate action.

Thank you,
Compliance Team
```

![Image 4: Send Email](https://cdn.drata.com/developers/custom_workflows/recipe_7/send_email.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#5-review-and-publish)5. Review and Publish

1.    Review your selections:
    *    Scope: **All manual evidence**
    *    Trigger: **Evidence unlinked from a control**
    *    Step(s): Notification to **control owners**

2.    Select **Publish** to activate the workflow.
3.    Save as **Draft** if more internal review is required.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-unlinked/#validation--testing)Validation & Testing

To ensure this workflow operates correctly:

1.    Take a **test evidence item** and unlink it from a control.
2.    Go to **Settings → Workflows → Run History** to confirm the workflow triggered.
3.    Check that the control owner received the notification.
4.    Adjust message content or assign additional recipients as needed.
