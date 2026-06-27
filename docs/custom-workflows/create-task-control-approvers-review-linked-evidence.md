Title: Create Task for Control Approvers to Review Newly Linked Evidence

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/

Published Time: Fri, 26 Jun 2026 18:55:00 GMT

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#objective)Objective

Automatically create a task for **control approvers** whenever new evidence is **linked to a control**. This workflow ensures approvers are immediately aware of new documentation impacting their controls, enabling timely review, approval, or follow-up actions to maintain compliance and audit readiness.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#prerequisites)Prerequisites

Before creating this workflow, ensure:

*    You have **Admin** or **Workspace Manager** permissions.
*    Your organization uses **control approvals** and has designated approvers.
*    Notification and task assignment roles are set up correctly:
    *   **Email** (always available)
    *   **Slack message** (Slack integration required)
    *   **Microsoft Teams message** (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#workflow-overview)Workflow Overview

This workflow executes when:

*   **Object Type:** Evidence
*   **Scope:** Selected evidence or all manual evidence
*   **Trigger Event:** Evidence linked to a control
*   **Actions:**
    *    Create a task assigned to **control approvers**
    *    (Optional) Notify additional recipients

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#1-create-the-workflow)1. Create the Workflow

1.    Navigate to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Configure:
    *   **Name:**`Create task for control approvers to review newly linked evidence`
    *   **Object Type:**`Evidence`

![Image 1: Create Workflow](https://cdn.drata.com/developers/custom_workflows/recipe_10/create_workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    Under **Start** , choose one of:
    *   **All manual evidence**
    *   **Source of evidence**
    *   **Framework evidence**
    *   **Selected evidence**

2.    Select **Continue** .

![Image 2: Start](https://cdn.drata.com/developers/custom_workflows/recipe_10/start.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#3-select-the-trigger)3. Select the Trigger

1.    Choose **Evidence linked to a control** as the trigger.
2.    This fires whenever evidence becomes associated with a control.

![Image 3: Trigger](https://cdn.drata.com/developers/custom_workflows/recipe_10/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#create-task)Create Task

1.    Add a **Create task** step.
2.    Configure the task details:
    *   **Title:**`Review newly linked evidence for {{evidence_linked_control_name}}`
    *   **Description:**

Copy

Copied

```
New evidence {{evidence_name}} has been linked to control {{evidence_linked_control_name}}.

Please review the attached evidence and confirm accuracy, completeness, and relevance.
```

    *   **Assigned To:**
        *   **Control Approvers** (recommended)
        *    Or a specific user/role (Admin, Compliance Team)

    *   **Due Date:** Set a reasonable follow-up window (e.g., 3–5 days from task creation)

![Image 4: Create Task](https://cdn.drata.com/developers/custom_workflows/recipe_10/create_task.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#5-review-and-publish)5. Review and Publish

1.    Review:
    *    Scope
    *    Trigger
    *    Task configuration
    *    Optional notifications

2.    Select **Publish** to activate the workflow.
3.    Or save as **Draft** if additional stakeholder review is required.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/create-task-control-approvers-review-linked-evidence/#validation--testing)Validation & Testing

To verify correct operation:

1.    Choose a **test evidence item** and link it to a control.
2.    Navigate to **Settings → Workflows → Run History** to confirm the workflow executed.
3.    Check that:
    *    A **task** was created and assigned to **control approvers** .
    *    Any configured notifications were sent successfully.

4.    Adjust task details, assignment, or messaging as needed.
