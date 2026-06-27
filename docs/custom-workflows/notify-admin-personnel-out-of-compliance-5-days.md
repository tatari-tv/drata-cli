Title: Notify Admin When Personnel Is Out of Compliance by 5 Days

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/

Published Time: Fri, 26 Jun 2026 18:55:00 GMT

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#objective)Objective

Automatically alert **Admins** when any employee has been **out of compliance for 5 days** across required trainings, policy acknowledgements, or other compliance-related obligations. This workflow improves visibility into long-standing compliance gaps and enables timely follow-up before risks escalate.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#prerequisites)Prerequisites

Before configuring this workflow, ensure you have:

*   **Admin** or **Workspace Manager** permissions in Drata.
*    Identified which **compliance categories** you want to monitor (e.g., Security Training, Policies, MFA, Device Compliance).
*    Verified which notification channels are available in your environment:
    *   **Email** (always available)
    *   **Slack message** (Slack integration required)
    *   **Microsoft Teams message** (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#workflow-overview)Workflow Overview

This workflow notifies Admins when:

*   **Object Type:** Personnel
*   **Scope:** All personnel
*   **Trigger Event:** Out of compliance
*   **Duration Threshold:** 5 continuous days out of compliance
*   **Action:** Send notification (Email, Slack, or Teams)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#1-create-the-workflow)1. Create the Workflow

1.    Navigate to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Configure:
    *   **Name:**`Notify admin when personnel is out of compliance by 5 days`
    *   **Object Type:**`Personnel`

![Image 1: Create Workflow](https://cdn.drata.com/developers/custom_workflows/recipe_2/create_workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    In the **Start** step, set the scope to **All personnel** .
2.    Select **Continue** .

![Image 2: Start](https://cdn.drata.com/developers/custom_workflows/recipe_2/start.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#3-select-the-trigger)3. Select the Trigger

1.    Choose **Out of compliance** as the personnel trigger.
2.    Select the **compliance categories** you want this workflow to monitor.
3.    Set the **number of days out of compliance** to **5** so the workflow triggers only when the issue persists for five days.

![Image 3: Trigger](https://cdn.drata.com/developers/custom_workflows/recipe_2/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#send-notification-email-example)Send Notification (Email Example)

1.    Add a **Send notification** step.
2.    Select a notification method:
    *   **Email**
    *   **Slack message**
    *   **Microsoft Teams message**

3.    For this example, configure **Email** .
4.    Set **Admin** as the recipient.
5.    Compose the email using dynamic variables such as:
    *   `{{personnel_name}}`
    *   `{{personnel_categories_out_of_compliance}}`

**Example Email Subject:**`{{personnel_name}} out of compliance for 5 days`

**Example Email Body:**

Copy

Copied

```
Hi Admin Team,

{{personnel_name}} has been out of compliance for 5 days.

Categories affected:
{{personnel_categories_out_of_compliance}}

Please review their status and take appropriate action.

Thank you,
Compliance Team
```

![Image 4: Send Email](https://cdn.drata.com/developers/custom_workflows/recipe_2/send_email.png)

> Optional: Add additional notification steps (Slack or Teams) in parallel if your organization uses those channels.

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#5-review-and-publish)5. Review and Publish

1.    Review all workflow settings, including:
    *    Scope: **All personnel**
    *    Trigger: **Out of compliance** , set to **5 days**
    *    Action: **Send notification to Admin**

2.    Select **Publish** to activate the workflow.
3.    Alternatively, save as **Draft** if internal approval is required before activation.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-admin-personnel-out-of-compliance-5-days/#validation--testing)Validation & Testing

To confirm the workflow is working as expected:

1.    Identify or create a **test personnel record** and configure it to be out of compliance in one of the selected categories.
2.    Adjust test data or timing to simulate being out of compliance for 5 days (as appropriate).
3.    Navigate to **Settings → Workflows → Run History** to verify that the workflow executed.
4.    Confirm the Admin received the notification email.
5.    Refine the subject line, body text, or recipients based on stakeholder feedback.
