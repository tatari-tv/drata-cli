Title: Integration - Drata

URL Source: https://help.safebase.io/en/articles/16203172-integration-drata

Markdown Content:
## Overview

SafeBase's Drata integration gives customers a seamless way to sync security policies, monitored controls, and company metadata into SafeBase. With this integration, customers can directly sync content housed within the Drata platform to SafeBase's Trust Center Document Library (DL) and enhance AI-powered Questionnaire Assistance (AIQA) by sourcing accurate, verified data directly from the Drata platform.

It is important to note that this integration is one-way, meaning that content from Trust Center does not sync back to Drata. Subprocessors must also be added manually in SafeBase, as they do not sync automatically from Drata.

The integration syncs data on a daily schedule, running overnight during US hours. After the daily sync, users must manually link documents to Trust Center items for visibility, and optionally upload other documents.
​

The integration automatically syncs published policies, controls (for AIQA), and company metadata from Drata to SafeBase, reducing the need for manual updates.

## Getting Started

The SafeBase team requires two items from your Drata tenant's [Settings](https://help.drata.com/en/articles/4760117-settings-page). Both of these are found in the Drata app, in Settings >[Company Info](https://app.drata.com/account-settings/company-info).

## Configuration

Once the SafeBase team has confirmed that the integration has been enabled, a SafeBase user with the Admin role can navigate to the **Settings > Integrations** section of the Trust Center, and click **Connect**.
​

[![Image 1](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649422/a77cd9722f058ff0f82a607c1ffd/upload_9464782806993890062.png?expires=1793948400&signature=fbad91bee844d1bd1bb69a78a64f128d405a53ffbaf76bb4e450e3879f7c0601&req=diUvE896lIVdW%2FMW1HO4zRxNYYLTaGtTt1zyZupduoBV8I%2Bnw0X6cV9pr6KW%0ADTREmykQcX89rYjqXgc%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649422/a77cd9722f058ff0f82a607c1ffd/upload_9464782806993890062.png?expires=1793948400&signature=fbad91bee844d1bd1bb69a78a64f128d405a53ffbaf76bb4e450e3879f7c0601&req=diUvE896lIVdW%2FMW1HO4zRxNYYLTaGtTt1zyZupduoBV8I%2Bnw0X6cV9pr6KW%0ADTREmykQcX89rYjqXgc%3D%0A)

## Options

Once connected, admins will be presented with three options:

[![Image 2](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649442/86785a5a1606d0410d8b393b2315/upload_15623779619469003413.png?expires=1793948400&signature=c54e0c8defde13202d6abaac659237ec32f4705d04b1b289302184b644e39753&req=diUvE896lIVbW%2FMW1HO4zaSeK2AaRxzKukHmiAyPL9WGAn2QsqdvZyZupBfD%0AttIcaE0idcPix4Zk30Q%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649442/86785a5a1606d0410d8b393b2315/upload_15623779619469003413.png?expires=1793948400&signature=c54e0c8defde13202d6abaac659237ec32f4705d04b1b289302184b644e39753&req=diUvE896lIVbW%2FMW1HO4zaSeK2AaRxzKukHmiAyPL9WGAn2QsqdvZyZupBfD%0AttIcaE0idcPix4Zk30Q%3D%0A)

If connecting the integration fails for any reason, you may continue the same chat or email with our Support team.

## Post-Configuration Manual Actions

After syncing data from Drata to SafeBase’s Documents section, the following steps are recommended to fully configure your Trust Center:

## Subprocessor Management

To add or edit subprocessors in your SafeBase Trust Center:

To remove a subprocessor:

## Drata Evidence Library Sync

Customers can sync certain artifacts from their Drata Evidence Library into SafeBase's Documents section. There are some key limitations:

Drata personnel will apply certain settings on your Drata tenant and SafeBase organization to enable the sync. You must also ensure the Drata integration in SafeBase's Settings area is connected (it is OK if the three individual toggles are off).

Once these settings are in place, you will see a new column called "SafeBase Trust Library Status" in Drata's Evidence Library table view.

[![Image 3](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649463/00315c55f297a68edc95a847d668/upload_6328174347423344040.png?expires=1793948400&signature=bad5dbe35ffe68dbe1ef01aee3cc9da533fbea68ca5f715c67e217dcfd612229&req=diUvE896lIVZWvMW1HO4zTafglhCaBWrZtTqGwFucYn%2FxzhxS2A2Lr8udG90%0AFnEGw3hkiGIPghUYjOs%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649463/00315c55f297a68edc95a847d668/upload_6328174347423344040.png?expires=1793948400&signature=bad5dbe35ffe68dbe1ef01aee3cc9da533fbea68ca5f715c67e217dcfd612229&req=diUvE896lIVZWvMW1HO4zTafglhCaBWrZtTqGwFucYn%2FxzhxS2A2Lr8udG90%0AFnEGw3hkiGIPghUYjOs%3D%0A)

You will also see a new tab called "SafeBase Trust Library" within the view of a single artifact.

[![Image 4](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649487/ee0493594fcbd986f1c01a9e40ec/upload_7419797857326581464.png?expires=1793948400&signature=0d6e61a3f3299a5183d4d892aac79959c4c126bc04a73d49f46fa37977113f2e&req=diUvE896lIVXXvMW1HO4zXX3wEeI5KPEw%2FPIqwFN4YKzBVVFe27ZpyyRgmZW%0AJrVP8IHIvhJ%2F9OI3144%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649487/ee0493594fcbd986f1c01a9e40ec/upload_7419797857326581464.png?expires=1793948400&signature=0d6e61a3f3299a5183d4d892aac79959c4c126bc04a73d49f46fa37977113f2e&req=diUvE896lIVXXvMW1HO4zXX3wEeI5KPEw%2FPIqwFN4YKzBVVFe27ZpyyRgmZW%0AJrVP8IHIvhJ%2F9OI3144%3D%0A)

### Bulk Sync

In the main table view of Evidence Library, use the **Filters** button at the upper left and choose **Source** ->**File**. Apply other filters as needed. Use the checkboxes in the left column to select at least one artifact. Now, a "Sync to Trust Library" button will appear above the table. Click this button.

[![Image 5](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649514/f8f9cf3f518d20f2b47744260168/upload_10494498747121839709.png?expires=1793948400&signature=5b4131cca558dacc79ccdf76e62b45b8b9f50285005e97bb6759133cd8b3a182&req=diUvE896lIReXfMW1HO4zcQBYR4fq%2B%2BZpFcntc04xIPqa5NfBeBcKIjPmpnu%0At%2BeUDdOhr%2Bt8EB7dZ3k%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649514/f8f9cf3f518d20f2b47744260168/upload_10494498747121839709.png?expires=1793948400&signature=5b4131cca558dacc79ccdf76e62b45b8b9f50285005e97bb6759133cd8b3a182&req=diUvE896lIReXfMW1HO4zcQBYR4fq%2B%2BZpFcntc04xIPqa5NfBeBcKIjPmpnu%0At%2BeUDdOhr%2Bt8EB7dZ3k%3D%0A)

In the modal, choose to mark selected pieces of evidence to be **Synced**or **Not Synced**. Selecting **Synced** will display the toast message that the sync is in progress.

[![Image 6](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649531/3b99b62b4a2763ab533854c0a713/upload_5896275690707637353.png?expires=1793948400&signature=455f57119a5a718b6a505a78c41a3a7222f64e315dd275c76fb7b216ab0211d2&req=diUvE896lIRcWPMW1HO4ze9ND2IXUthiViq%2FJMA%2BTyn8owHXxNYfx02NYAAj%0A1yEbo07OEQYYMI8ERWM%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649531/3b99b62b4a2763ab533854c0a713/upload_5896275690707637353.png?expires=1793948400&signature=455f57119a5a718b6a505a78c41a3a7222f64e315dd275c76fb7b216ab0211d2&req=diUvE896lIRcWPMW1HO4ze9ND2IXUthiViq%2FJMA%2BTyn8owHXxNYfx02NYAAj%0A1yEbo07OEQYYMI8ERWM%3D%0A)

[![Image 7](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649544/f9ac62164eb81121c54115174907/upload_6607084355620494182.png?expires=1793948400&signature=8b6c654381386e542328e35c6f296e51574a470ffca42d1fb5b1a78cbe2f3b93&req=diUvE896lIRbXfMW1HO4zela1PK4K33IpR%2FHkvQBW%2F1urprAb2UnM74SDuzP%0Aj2IxSKEpGUOaYY3xqcQ%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649544/f9ac62164eb81121c54115174907/upload_6607084355620494182.png?expires=1793948400&signature=8b6c654381386e542328e35c6f296e51574a470ffca42d1fb5b1a78cbe2f3b93&req=diUvE896lIRbXfMW1HO4zela1PK4K33IpR%2FHkvQBW%2F1urprAb2UnM74SDuzP%0Aj2IxSKEpGUOaYY3xqcQ%3D%0A)

The table will update the "SafeBase Trust Library Status" to "In progress" for all pieces of evidence that are being synced.

[![Image 8](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649566/d49da6e88a1a73c5f7f05e3a89cd/upload_6435636772549931884.png?expires=1793948400&signature=b4a716426af6be44d1641b95b76d7152cf876076d3893298a1bc46714589f673&req=diUvE896lIRZX%2FMW1HO4zYA1VGVYDRyguUs4IUIK%2BVyNFmVyOGH7FwNRnXBr%0AS%2B5FEE%2BuPCHhgbWLxd8%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649566/d49da6e88a1a73c5f7f05e3a89cd/upload_6435636772549931884.png?expires=1793948400&signature=b4a716426af6be44d1641b95b76d7152cf876076d3893298a1bc46714589f673&req=diUvE896lIRZX%2FMW1HO4zYA1VGVYDRyguUs4IUIK%2BVyNFmVyOGH7FwNRnXBr%0AS%2B5FEE%2BuPCHhgbWLxd8%3D%0A)

Any pieces of evidence that are not eligible to be synced will remain as `-` for the

SafeBase Trust Library Status.

[![Image 9](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649601/b19ae19b06547c9ebf3daab232dc/upload_2771859456799306270.png?expires=1793948400&signature=3b70e7c0089b8221c79d0744425dc12104d684cace18122a41d6afa6e9c9e544&req=diUvE896lIdfWPMW1HO4zatTw6%2FeemuF1vnvrAmuQzm9a3k%2BnQlXn0yrl9Ko%0AaANfidHO66%2FLvjnKvK8%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649601/b19ae19b06547c9ebf3daab232dc/upload_2771859456799306270.png?expires=1793948400&signature=3b70e7c0089b8221c79d0744425dc12104d684cace18122a41d6afa6e9c9e544&req=diUvE896lIdfWPMW1HO4zatTw6%2FeemuF1vnvrAmuQzm9a3k%2BnQlXn0yrl9Ko%0AaANfidHO66%2FLvjnKvK8%3D%0A)

Once the syncing is complete, refresh the page to see that the "SafeBase Trust Library Status" field now shows "Synced."

[![Image 10](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649614/2bf7fd28c2e9e767e199e697aa51/upload_17599595075170394399.png?expires=1793948400&signature=161e07b02c1f8a2ae66e76246c203a18f0729a63f72f88d9371311e5e795385f&req=diUvE896lIdeXfMW1HO4zfLPKnHZBbB%2B0vEEWlM2kPGmFPvMUQLr1OS7r31l%0AJKqrcC5dO5mle7EU0EQ%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649614/2bf7fd28c2e9e767e199e697aa51/upload_17599595075170394399.png?expires=1793948400&signature=161e07b02c1f8a2ae66e76246c203a18f0729a63f72f88d9371311e5e795385f&req=diUvE896lIdeXfMW1HO4zfLPKnHZBbB%2B0vEEWlM2kPGmFPvMUQLr1OS7r31l%0AJKqrcC5dO5mle7EU0EQ%3D%0A)

### Individual Sync

Within the view of a single artifact, you will see a new field labeled "SafeBase Trust Library Status" as well as a new tab labeled "SafeBase Trust Library."

[![Image 11](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649636/76e27b4bf71fdf34bf2378a35efa/upload_14490146258630377351.png?expires=1793948400&signature=994d2ca6c4c89a24e10ab63581673c3829731bbac3d75de79a28b46d098588d9&req=diUvE896lIdcX%2FMW1HO4zW9Pw9kJTieWz%2BFvakvW0xH8%2BMYzl4dnJR7h7wNT%0A%2B2%2B7uktTH0vLs0cwQow%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649636/76e27b4bf71fdf34bf2378a35efa/upload_14490146258630377351.png?expires=1793948400&signature=994d2ca6c4c89a24e10ab63581673c3829731bbac3d75de79a28b46d098588d9&req=diUvE896lIdcX%2FMW1HO4zW9Pw9kJTieWz%2BFvakvW0xH8%2BMYzl4dnJR7h7wNT%0A%2B2%2B7uktTH0vLs0cwQow%3D%0A)

Click into the "SafeBase Trust Library" tab. You will see the current sync status of this artifact. Click the Edit button to turn the sync on or off.

[![Image 12](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649646/b09aec5ad1fcdc5d8a96b5bbb208/upload_17337070320042263640.png?expires=1793948400&signature=5fe54e893ac321f4d4d54f7a03c3296a7ac63e5ae91648ab3af285024495380e&req=diUvE896lIdbX%2FMW1HO4zSe7lVeswIp%2F3tHAXMGN72E7b%2B5IxCqMSQ%2Bq2ksE%0ARH06XrBtfk9MHXtvWqk%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649646/b09aec5ad1fcdc5d8a96b5bbb208/upload_17337070320042263640.png?expires=1793948400&signature=5fe54e893ac321f4d4d54f7a03c3296a7ac63e5ae91648ab3af285024495380e&req=diUvE896lIdbX%2FMW1HO4zSe7lVeswIp%2F3tHAXMGN72E7b%2B5IxCqMSQ%2Bq2ksE%0ARH06XrBtfk9MHXtvWqk%3D%0A)

Check the box to sync the current artifact, or uncheck it to turn off the sync. Then click **Save**.

[![Image 13](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649653/31b4015b01430677f3826974ec41/upload_4092103903787145023.png?expires=1793948400&signature=fb4f7e013cc463136e9b5b2cba844e7a9adadb2c0f91f26e187561a911c0dc2a&req=diUvE896lIdaWvMW1HO4zYztPIzmkeJIqD%2BiHeH8ItjZg8NSu3QpbipbMd5H%0AKLH8%2FOzD6VhcsOqXEXQ%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649653/31b4015b01430677f3826974ec41/upload_4092103903787145023.png?expires=1793948400&signature=fb4f7e013cc463136e9b5b2cba844e7a9adadb2c0f91f26e187561a911c0dc2a&req=diUvE896lIdaWvMW1HO4zYztPIzmkeJIqD%2BiHeH8ItjZg8NSu3QpbipbMd5H%0AKLH8%2FOzD6VhcsOqXEXQ%3D%0A)

### Sync When Adding Evidence

On the main Drata Evidence Library screen, click the **Add Evidence** button at the upper right. In the **Artifact** ->**Source** dropdown, choose "File." Then attach a file of one of the supported formats. You will now see a "Sync this evidence's current artifact to SafeBase Trust Library" checkbox at the bottom of the "Artifact" section of the drawer. Check that box to sync this artifact.

[![Image 14](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649666/a77729f0bbfeb14f62b2c64b6e39/upload_911903220224951005.png?expires=1793948400&signature=3f2cfd133a317a31f0b2ee082dbdfc1a4a33f2185aefb65af6b154e8827ccec8&req=diUvE896lIdZX%2FMW1HO4zfvtE6uh7Av9bujNK6wIv%2BqksbJIpZNwshsiyHRD%0A%2BXEI2VGZWvG%2BRv5tQNs%3D%0A)](https://drata.intercom-attachments-1.com/i/o/am69aeco/2585649666/a77729f0bbfeb14f62b2c64b6e39/upload_911903220224951005.png?expires=1793948400&signature=3f2cfd133a317a31f0b2ee082dbdfc1a4a33f2185aefb65af6b154e8827ccec8&req=diUvE896lIdZX%2FMW1HO4zfvtE6uh7Av9bujNK6wIv%2BqksbJIpZNwshsiyHRD%0A%2BXEI2VGZWvG%2BRv5tQNs%3D%0A)

* * *

Related Articles

*   [Release Notes – April 2026](https://help.safebase.io/en/articles/16203045-release-notes-april-2026)
*   [Integration - SafeBase Bot for Slack - Setup & Use](https://help.safebase.io/en/articles/16203163-integration-safebase-bot-for-slack-setup-use)
*   [Release Notes - February 2025](https://help.safebase.io/en/articles/16203187-release-notes-february-2025)
*   [Integration - Google Drive Sync (Early Access)](https://help.safebase.io/en/articles/16203192-integration-google-drive-sync-early-access)
*   [Self Migration from the Legacy Drata Trust Center to SafeBase](https://help.safebase.io/en/articles/17764087-self-migration-from-the-legacy-drata-trust-center-to-safebase)
