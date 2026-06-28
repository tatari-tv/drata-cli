Title: Notify and Create Task When Risk Residual Score Changes to a Higher Level

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/

Published Time: Fri, 26 Jun 2026 18:55:00 GMT

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#objective)Objective

Automatically notify stakeholders and create a follow-up task whenever a **risk's residual score increases** (e.g., from Medium → High). This workflow helps ensure that elevated risks receive timely review, improving risk visibility and supporting proactive remediation in your risk management program.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#prerequisites)Prerequisites

Before setting up this workflow, ensure:

*    You have **Admin** or **Workspace Manager** permissions in Drata.
*    You have identified which **risks** should be monitored for score changes.
*    You know the **score thresholds** used in your organization (labels may be customized).
*    Your notification channels are ready:
    *   **Email** (always available)
    *   **Slack** (Slack integration required)
    *   **Microsoft Teams** (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#workflow-overview)Workflow Overview

This workflow executes when:

*   **Object Type:** Risk
*   **Scope:** All risks
*   **Trigger Event:** Residual score changed
*   **Trigger Condition:** New residual score meets or exceeds selected threshold (fires on any change where the new score crosses or stays above the threshold, including decreases that remain above it)
*   **Actions:**
    *    Send notification (Email, Slack, or Teams)
    *    Create task for risk owner or designated role

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#1-create-the-workflow)1. Create the Workflow

1.    Navigate to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Configure:
    *   **Name:**`Notify and create task when risk residual score changes to a higher level`
    *   **Object Type:**`Risk`

![Image 1: Create Workflow](https://cdn.drata.com/developers/custom_workflows/recipe_3/create_workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    Under **Start** , set scope to **All risks** .
2.    Click **Continue** to proceed.

![Image 2: Start](https://cdn.drata.com/developers/custom_workflows/recipe_3/start.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#3-select-the-trigger)3. Select the Trigger

1.    Choose **Residual score changed** as the trigger.
2.    Define the condition:
    *    Under **New score is** , choose the threshold you want to monitor (e.g., _High_ , _Critical_ ).
    *    Configure the operator (e.g., _Greater than or equal to_ ).

This triggers the workflow whenever the new score meets or exceeds the threshold. Note that this condition fires on any score change where the new value is at or above the threshold, not only when the score increases; a decrease from Critical to High would also trigger if both values exceed the threshold. If you only want to notify on upward changes, you would need an additional old-vs-new comparison condition when the platform supports it.

![Image 3: Trigger](https://cdn.drata.com/developers/custom_workflows/recipe_3/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#create-task)Create Task

1.    Add a **Create task** step.
2.    Configure the task:
    *   **Title:**`Review elevated residual risk score for {{risk_name}}`
    *   **Description:**

Copy

Copied

```
The residual score for {{risk_name}} has increased to {{new_residual_score_value}}.

Please review the risk, evaluate required mitigation, and determine next steps.
```

    *    Assign to:
        *   **Risk Owner** , or
        *    A specific user or role (e.g., Admin, Risk Manager)

    *    Set a due date (e.g., _5 days from task creation_ ).

![Image 4: Create Task](https://cdn.drata.com/developers/custom_workflows/recipe_3/create_task.png)

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#send-notification-email-example)Send Notification (Email Example)

1.    Add a **Send notification** step.
2.    Select a delivery method:
    *   **Email**
    *   **Slack**
    *   **Microsoft Teams**

3.    For this example, configure **Email** .
4.    Choose recipient(s), such as:
    *   **Risk Owner**
    *   **Admin**
    *   **Security Team distribution list**

5.    Example Email Subject: `Residual risk score elevated for {{risk_name}}`
6.    Example Email Body: Copy

Copied ```
The residual score for {{risk_name}} has increased.

Previous Score: {{old_residual_score_value}}
New Score: {{new_residual_score_value}}

Please review the task created for this risk to determine required next steps.
```

![Image 5: Send Email](https://cdn.drata.com/developers/custom_workflows/recipe_3/send_email.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#5-review-and-publish)5. Review and Publish

1.    Review the workflow details:
    *    Scope: **All risks**
    *    Trigger: Residual score changed → threshold condition
    *    Steps: Task creation + notification

2.    Select **Publish** to activate the workflow.
3.    Save as **Draft** if stakeholder review is required.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-create-task-risk-residual-score-change/#validation--testing)Validation & Testing

To confirm the workflow is working properly:

1.    Modify a **test risk** so its residual score crosses the monitored threshold.
2.    Go to **Settings → Workflows → Run History** to confirm the workflow executed.
3.    Verify:
    *    The task was created and assigned correctly.
    *    The notification email was delivered to the intended recipients.

4.    Iterate message content or task details as needed.
