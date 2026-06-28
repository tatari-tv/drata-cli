Title: Notify Current Employees When Non-Compliant / Failure

URL Source: https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/

Markdown Content:
Last updated 6 months ago

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#objective)Objective

Automatically alert employees when they become **non-compliant** with required training, policies, or onboarding actions.

 This workflow ensures **timely action**, **clear accountability**, and minimizes the risk of **missed compliance obligations** by notifying personnel as soon as they fall out of compliance.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#prerequisites)Prerequisites

Before configuring this workflow, make sure you have:

*   **Admin** or **Workspace Manager** access, which is required to create and publish workflows.
*    Identified the **personnel categories** you want to monitor (e.g., Security Training, Code of Conduct, Policy Acknowledgements).
*    Confirmed the **communication channel** you'll use:
    *    Email (always available)
    *    Slack channel or DM (Slack integration required)
    *    Microsoft Teams channel (Teams integration required)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#workflow-overview)Workflow Overview

This workflow sends an automated notification whenever:

*   **Object Type:** Personnel
*   **Trigger Event:** Out of compliance
*   **Steps Executed:** Send notification (email, Slack, or Teams)

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#step-by-step-configuration)Step-by-Step Configuration

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#1-create-the-workflow)1. Create the Workflow

1.    Navigate to **Settings → Workflows** .
2.    Select **Create Workflow** .
3.    Name the workflow (e.g., _Notify Employees When They Become Non-Compliant_ ).
4.    Choose **Personnel** as the object type.

![Image 1: alt text](https://cdn.drata.com/developers/custom_workflows/recipe_1/create-workflow.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#2-define-the-workflow-scope)2. Define the Workflow Scope

1.    Select which personnel the workflow applies to:
    *   **All personnel** , or
    *    Filter by **status** if you only want to target active employees.

![Image 2: alt text](https://cdn.drata.com/developers/custom_workflows/recipe_1/personnel-scope.png)

2.    Confirm your selection to move to the trigger configuration.

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#3-select-the-trigger)3. Select the Trigger

1.    Choose **Out of compliance** from the Personnel triggers list.
2.    Configure the trigger details:
    *    Select the **Compliance Categories** that should fire this workflow (e.g., Security Training).
    *    Set the **number of days** the person must be out of compliance before the workflow triggers (e.g., _0 days_ for immediate notification).

![Image 3: alt text](https://cdn.drata.com/developers/custom_workflows/recipe_1/trigger.png)

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#4-add-steps)4. Add Steps

#### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#send-notification)Send Notification

1.    Select a notification method: **Email** , **Slack message** , or **Microsoft Teams message** .
![Image 4: alt text](https://cdn.drata.com/developers/custom_workflows/recipe_1/action.png)

2.    Choose the recipient:
    *   **Personnel** (the non-compliant employee)
    *    Optionally add **additional recipients** such as Admins or managers (the manager option is only available if the HRIS is connected to Drata)

3.    Compose the message using dynamic variables for clarity:
    *    Example Subject: **You Are Out of Compliance With a Required Item**
    *    Example Message:

Copy

Copied

```
Hi {{personnel_name}},

Our system has detected that you are out of compliance for:
- Category: {{personnel_categories_out_of_compliance}}

Please complete the outstanding requirement as soon as possible to regain compliance status.

Thank you,
Compliance Team
```

![Image 5: alt text](https://cdn.drata.com/developers/custom_workflows/recipe_1/create-email-to-notify.png)

4.    Save the step.
5.   **Optional** : Add additional in-parallel steps, e.g., Send Slack message to notify the user over Slack.

### [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#5-review-and-publish)5. Review and Publish

1.    Review the workflow's scope, trigger, and notification content.
2.    Select **Publish** to activate the workflow.
3.    Optionally, save as a draft if you need to review the messaging with stakeholders.

## [](https://developers.drata.com/developer-portal/v2/custom-workflows/notify-employees-non-compliant/#validation--testing)Validation & Testing

To confirm the workflow is functioning properly:

1.   **Simulate the trigger** by marking a test employee as out of compliance or adjusting a compliance item.
2.    Navigate to **Workflows → Run History** to confirm the workflow fired.
3.    Verify the **email, Slack, or Teams notification** was received by the intended user.
4.    Adjust messaging or recipients as necessary based on validation.
