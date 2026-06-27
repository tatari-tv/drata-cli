Title: Notify Control Owners When Evidence Renewals Are Approaching

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#objective)Objective

Automatically alert **control owners** when evidence linked to their controls is approaching its **upcoming renewal date**. This workflow ensures proactive evidence management, reduces the likelihood of expired documentation, and helps maintain audit readiness.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#prerequisites)Prerequisites

Before configuring this workflow, ensure you have:

*   **Admin** or **Workspace Manager** permissions.
*    Identified which evidence items require renewal reminders.
*    Verified available notification channels:
    *   **Email** (always available)
    *   **Slack message** (Slack integration required)
    *   **Microsoft Teams message** (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#workflow-overview)Workflow Overview

This workflow notifies control owners when:

*   **Object Type:** Evidence
*   **Scope:** All manual evidence
*   **Trigger Event:** Upcoming renewal
*   **Trigger Condition:** Number of days before the renewal date (user-defined)
*   **Action:** Send notification (Email, Slack, or Teams)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#1-create-the-workflow)1. Create the Workflow

1.    Navigate to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Configure:
    *   **Name:**`Notify control owners when evidence renewals are approaching`
    *   **Object Type:**`Evidence`

![Image 1: Create Workflow](https://cdn.drata.com/developers/custom_workflows/recipe_5/create_workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    Under the **Start** step, select **All manual evidence** .
2.    Click **Continue** .

![Image 2: Start](https://cdn.drata.com/developers/custom_workflows/recipe_5/start.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#3-select-the-trigger)3. Select the Trigger

1.    Choose **Upcoming renewal** as the evidence trigger.
2.    Enter the number of days **before the renewal date** you want the workflow to notify (e.g., 30 days).

![Image 3: Trigger](https://cdn.drata.com/developers/custom_workflows/recipe_5/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#send-notification-email-example)Send Notification (Email Example)

1.    Add a **Send notification** step.
2.    Choose your delivery method:
    *   **Email**
    *   **Slack message**
    *   **Microsoft Teams message**

3.    For this example, configure **Email** .
4.    Set **Control Owners** as recipients.
5.    Use dynamic variables such as:
    *   `{{evidence_name}}`
    *   `{{evidence_owner_name}}`
    *   `{{evidence_renewal_date}}`
    *   `{{evidence_linked_control_name}}`

**Example Email Subject:**`Upcoming evidence renewal: {{evidence_name}}`

**Example Email Body:**

Copy

Copied

```
Hello,

The evidence item "{{evidence_name}}" linked to control "{{evidence_linked_control_name}}" is approaching its renewal date ({{evidence_renewal_date}}).

Please review and renew this evidence to maintain compliance and control readiness.

Thank you,
Compliance Team
```

![Image 4: Send Email](https://cdn.drata.com/developers/custom_workflows/recipe_5/send_email.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#5-review-and-publish)5. Review and Publish

1.    Review the workflow details:
    *    Scope: **All manual evidence**
    *    Trigger: **Upcoming renewal**
    *    Action: Notify **control owners**

2.    Select **Publish** to activate the workflow.
3.    Or save as **Draft** if further review is needed.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-control-owners-evidence-renewals-approaching/#validation--testing)Validation & Testing

To validate workflow behavior:

1.    Identify or create a **test evidence item** with an upcoming renewal date.
2.    Link it to a control.
3.    Adjust the renewal date or trigger window to force activation.
4.    Navigate to **Settings → Workflows → Run History** to verify execution.
5.    Confirm the notification was received by the intended recipients.
6.    Adjust wording or timing as needed.
