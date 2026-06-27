Title: Review Updated Risk Treatment Plan

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#objective)Objective

Automatically alert risk stakeholders and generate a review task whenever a **risk's treatment option is updated** (e.g., Accept → Mitigate). This workflow ensures changes to risk treatment strategy are reviewed promptly, supporting effective risk governance and maintaining alignment across the security, compliance, and risk teams.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#prerequisites)Prerequisites

Before configuring this workflow:

*    You must have **Admin** or **Workspace Manager** permissions.
*    Your organization must use risk treatment options such as _Accept_ , _Avoid_ , _Mitigate_ , or _Transfer_ .
*    Notification and task assignment roles must be configured:
    *   **Email** (always available)
    *   **Slack message** (Slack integration required)
    *   **Microsoft Teams message** (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#workflow-overview)Workflow Overview

This workflow runs when:

*   **Object Type:** Risk
*   **Scope:** All risks
*   **Trigger Event:** Treatment option changed
*   **Action:**
    *    Create a task to review the updated treatment plan
    *    (Optional) Send a notification to risk stakeholders

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#1-create-the-workflow)1. Create the Workflow

1.    Navigate to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Configure:
    *   **Name:**`Review updated risk treatment plan`
    *   **Object Type:**`Risk`

![Image 1: Create Workflow](https://cdn.drata.com/developers/custom_workflows/recipe_11/create_workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    Under **Start** , select **All risks** .
2.    Click **Continue** .

![Image 2: Start](https://cdn.drata.com/developers/custom_workflows/recipe_11/start.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#3-select-the-trigger)3. Select the Trigger

1.    Choose **Treatment option changed** as the trigger.
2.    Select which treatment option changes should initiate this workflow (e.g., _Any change_ , or specific transitions like _Accept → Mitigate_ ).

![Image 3: Trigger](https://cdn.drata.com/developers/custom_workflows/recipe_11/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#create-task)Create Task

1.    Add a **Create task** step.
2.    Configure the task details:
    *   **Title:**`Review updated treatment plan for {{risk_name}}`
    *   **Description:**

Copy

Copied

```
The treatment option for {{risk_name}} has been updated.

Previous Option: {{old_treatment_option}}
New Option: {{new_treatment_option}}

Please review the updated treatment plan and determine if further mitigation, documentation, or approval is required.
```

    *   **Assigned To:**
        *   **Risk Owner**
        *   **Risk Reviewer**
        *    Or a designated role (e.g., Admin, Security Team)

    *   **Due Date:** Set a reasonable timeframe (e.g., 5–7 days from creation).

![Image 4: Create Task](https://cdn.drata.com/developers/custom_workflows/recipe_11/create_task.png)

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#send-notification-email-example)Send Notification (Email Example)

To notify additional stakeholders (e.g., Risk Committee):

1.    Add a **Send notification** step.
2.    Select a method:
    *   **Email**
    *   **Slack message**
    *   **Microsoft Teams message**

3.    For this example, configure **Email** .
4.    Example subject and body:

**Subject:**`Risk treatment plan updated: {{risk_name}}`

**Body:**

Copy

Copied

```
The treatment plan for {{risk_name}} has been updated.

Old Treatment Option: {{old_treatment_option}}
New Treatment Option: {{new_treatment_option}}

A task has been created for follow-up review. Please take any required action.
```

![Image 5: Send Email](https://cdn.drata.com/developers/custom_workflows/recipe_11/send_email.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#5-review-and-publish)5. Review and Publish

1.    Review:
    *    Scope: **All risks**
    *    Trigger: **Treatment option changed**
    *    Steps: **Task creation** , plus optional notifications

2.    Select **Publish** to activate.
3.    Save as **Draft** if internal review is required.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/review-updated-risk-treatment-plan/#validation--testing)Validation & Testing

To ensure proper setup:

1.    Modify the treatment option of a **test risk** .
2.    Navigate to **Settings → Workflows → Run History** to verify the workflow fired.
3.    Confirm:
    *    A **task** was created and assigned correctly
    *    Any notifications were delivered

4.    Adjust task details or messaging as needed.
