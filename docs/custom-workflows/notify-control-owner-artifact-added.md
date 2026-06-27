Title: Notify Control Owner When an Artifact Is Added to Evidence

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/

Published Time: Fri, 26 Jun 2026 18:55:00 GMT

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#objective)Objective

Automatically notify the **control owner** whenever a new artifact is **uploaded to an evidence item** linked to their control. This workflow improves visibility into documentation changes, supports timely evidence review, and ensures owners stay informed of updates that may affect control readiness.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#prerequisites)Prerequisites

Before configuring this workflow, ensure you have:

*   **Admin** or **Workspace Manager** permissions.
*    Identified which evidence items and controls you want monitored.
*    Verified available notification channels:
    *   **Email** (always available)
    *   **Slack message** (Slack integration required)
    *   **Microsoft Teams message** (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#workflow-overview)Workflow Overview

This workflow notifies control owners when:

*   **Object Type:** Evidence
*   **Scope:** All manual evidence
*   **Trigger Event:** New artifact uploaded
*   **Action:** Send notification to the control owner

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#1-create-the-workflow)1. Create the Workflow

1.    Navigate to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Configure:
    *   **Name:**`Notify when artifact added to evidence`
    *   **Object Type:**`Evidence`

![Image 1: Create Workflow](https://cdn.drata.com/developers/custom_workflows/recipe_9/create_workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    Under the **Start** step, choose **All manual evidence** .
2.    Click **Continue** to proceed.

![Image 2: Start](https://cdn.drata.com/developers/custom_workflows/recipe_9/start.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#3-select-the-trigger)3. Select the Trigger

1.    Choose **New artifact uploaded** as the trigger.
2.    This event fires whenever a user uploads a new artifact file to an existing evidence item.

![Image 3: Trigger](https://cdn.drata.com/developers/custom_workflows/recipe_9/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#send-notification-email-example)Send Notification (Email Example)

1.    Add a **Send notification** step.
2.    Select your preferred notification method:
    *   **Email**
    *   **Slack message**
    *   **Microsoft Teams message**

3.    For this example, configure **Email** .
4.    Set **Control Owner** as the recipient.
5.    Use dynamic variables such as:
    *   `{{evidence_name}}`
    *   `{{new_artifact_uploaded_file_name}}`
    *   `{{new_artifact_uploaded_by}}`
    *   `{{new_artifact_uploaded_on}}`
    *   `{{evidence_linked_control_name}}`

**Example Email Subject:**`New artifact uploaded to evidence: {{evidence_name}}`

**Example Email Body:**

Copy

Copied

```
Hello,

A new artifact has been uploaded to the evidence item "{{evidence_name}}" linked to control "{{evidence_linked_control_name}}".

Uploaded By: {{new_artifact_uploaded_by}}
File Name: {{new_artifact_uploaded_file_name}}
Uploaded On: {{new_artifact_uploaded_on}}

Please review this update to ensure all documentation remains accurate and complete.

Thank you,
Compliance Team
```

![Image 4: Send Email](https://cdn.drata.com/developers/custom_workflows/recipe_9/send_email.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#5-review-and-publish)5. Review and Publish

1.    Confirm workflow details:
    *    Scope: **All manual evidence**
    *    Trigger: **New artifact uploaded**
    *    Notification: **Control Owner**

2.    Select **Publish** to activate.
3.    Or save as **Draft** if further team review is needed.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owner-artifact-added/#validation--testing)Validation & Testing

1.    Upload a new artifact to a **test evidence item** .
2.    Confirm the workflow fires by checking **Settings → Workflows → Run History** .
3.    Verify the control owner receives the notification.
4.    Adjust message content or add additional recipients if desired.
