Title: Changelog

URL Source: https://developers.drata.com/changelog/

Markdown Content:
Last updated 1 year ago

All notable changes are documented in this changelog.

## [](https://developers.drata.com/changelog/#2025-06-09)2025-06-09

### [](https://developers.drata.com/changelog/#renamed-permissions)Renamed Permissions

Some of the permissions names have been changed to be more consistent and descriptive. There is no effect on existing API keys. We're providing a list of the changes to make it easier to update any documentation that might have referreed to the old names.

#### [](https://developers.drata.com/changelog/#assets)Assets

| Old Name | New Name |
| --- | --- |
| Asset inventory | List Assets |
| Add asset | Create Asset |

#### [](https://developers.drata.com/changelog/#company-settings)Company Settings

| Old Name | New Name |
| --- | --- |
| List company info | Get Company Settings |

#### [](https://developers.drata.com/changelog/#connections)Connections

| Old Name | New Name |
| --- | --- |
| Get managed connections | List Connections |
| Manage infrastructure accounts | Manage Infrastructure User Identities |
| Manage version control accounts | Manage Version Control User Identities |

#### [](https://developers.drata.com/changelog/#controls)Controls

| Old Name | New Name |
| --- | --- |
| Control details | Get Control |
| Download control evidence | Download All Control evidence |
| Controls list | List Controls |
| Delete a Control Note | Delete Control Note |
| Get Controls Notes | Get Control Note |
| Add a Control Note | Create Control Note |
| Modify a Control Note | Update Control Note |
| Add control | Create Control |
| Control owners | Manage Control Owners |
| Update control info | Update Control |

#### [](https://developers.drata.com/changelog/#custom-connections-data)Custom Connections Data

| Old Name | New Name |
| --- | --- |
| Add Data | Create Data |
| Update Data | Create and Update Data |

#### [](https://developers.drata.com/changelog/#customer-request)Customer Request

| Old Name | New Name |
| --- | --- |
| Get Customer Requests | Get Customer Request |
| Update Customer Request Details | Update Customer Request |

#### [](https://developers.drata.com/changelog/#devices)Devices

| Old Name | New Name |
| --- | --- |
| Delete devices | Delete Device |
| Get devices | List Devices |
| Create devices | Create Device |

#### [](https://developers.drata.com/changelog/#event-tracking)Event Tracking

| Old Name | New Name |
| --- | --- |
| Event details | Get Event |
| Events list | List Events |

#### [](https://developers.drata.com/changelog/#evidence-library)Evidence Library

| Old Name | New Name |
| --- | --- |
| Add Evidence | Create Evidence |

#### [](https://developers.drata.com/changelog/#monitoring)Monitoring

| Old Name | New Name |
| --- | --- |
| Monitor details | Get Monitor |
| Monitor list | List Monitors |
| Test now | Trigger Monitor Test |

#### [](https://developers.drata.com/changelog/#personnel)Personnel

| Old Name | New Name |
| --- | --- |
| Personnel background check | Create Background Check |
| Personnel details | Get Personnel |
| Personnel list | List Personnel |
| Set hire and separation dates | Update Personnel's Hire and Separation Dates |
| Set Employment Status | Update Personnel's Employment Status |
| Reset Personnels IdP/HRIS Sync | Reset Personnel's IdP/HRIS Sync Status |

#### [](https://developers.drata.com/changelog/#policies)Policies

| Old Name | New Name |
| --- | --- |
| Policy Version | Get Policy Version |

#### [](https://developers.drata.com/changelog/#questionnaires)Questionnaires

| Old Name | New Name |
| --- | --- |
| List questionnaires answers by vendor | List Questionnaire Answers by Vendor |

#### [](https://developers.drata.com/changelog/#risk-management)Risk Management

| Old Name | New Name |
| --- | --- |
| Risk details | Get Risk |
| Risk register | List Risks |
| Edit risk details | Update Risk |
| Add risk | Create Risk |

#### [](https://developers.drata.com/changelog/#roles)Roles

| Old Name | New Name |
| --- | --- |
| Role details | Get Role |
| Role list | List Roles |

#### [](https://developers.drata.com/changelog/#trust-center-pro---trust-center)Trust Center Pro -> Trust Center

| Old Name | New Name |
| --- | --- |
| Continuous monitoring details | List Monitoring Controls |
| Get list of private documents in Trust Center | List Private Documents |
| Create a request | Create Access Request |
| NDA settings | Manage NDA Settings |
| Approve or deny access request | Manage Access Request |

#### [](https://developers.drata.com/changelog/#user-identities)User Identities

| Old Name | New Name |
| --- | --- |
| Permission to read all of user identities | List User Identities |
| Permission to delete an user identity | Delete User Identity |
| Permission to read an user identity | Get User Identity |
| Permission to add an user identity | Create User Identity |
| Permission to update an user identity | Update User Identity |

#### [](https://developers.drata.com/changelog/#user-policies)User Policies

| Old Name | New Name |
| --- | --- |
| User policies | List User's Assigned Policies |

#### [](https://developers.drata.com/changelog/#users)Users

| Old Name | New Name |
| --- | --- |
| User details | Get User |
| Remove user evidence | Delete User Document |
| User documents by type | List User Documents |
| User evidence download | Download User Document |
| User list | List Users |
| User compliance | Create User Document |

#### [](https://developers.drata.com/changelog/#vendors)Vendors

| Old Name | New Name |
| --- | --- |
| Vendor questionnaire status | List Vendor Questionnaire |
| Vendors statistics | Get Vendors Statistics |
| Vendor details | Get Vendor |
| Vendor directory | List Vendors |
| Update vendor status | Update Vendor's Status |
| Upload report to vendor | Upload Vendor Report |
| Create a security review | Create Security Review |
| Add vendor | Create Vendor |
| Update vendor details | Update Vendor |

## [](https://developers.drata.com/changelog/#2024-09-24)2024-09-24

### [](https://developers.drata.com/changelog/#removed)Removed

**Removed Properties**

*   `DeviceResponsePublicDto.admins`
*   `DeviceResponsePublicDto.memory`
*   `DeviceResponsePublicDto.processor`
*   `DeviceResponsePublicDto.hddSize`
*   `DeviceResponsePublicDto.graphics`
*   `PersonnelDetailsResponsePublicDto.admins`
*   `PersonnelDetailsResponsePublicDto.memory`
*   `PersonnelDetailsResponsePublicDto.processor`
*   `PersonnelDetailsResponsePublicDto.hddSize`
*   `PersonnelDetailsResponsePublicDto.graphics`

**Affected Endpoints**

| Request | Endpoint |
| --- | --- |
| `POST` | /background-check/{userId}/manual |
| `POST` | /assets |
| `GET` | /personnel/{id} |
| `GET` | /personnel/{email}/email |
| `PUT` | /personnel/{id}/contract-dates |
| `PUT` | /personnel/{id}/status |
| `PUT` | /personnel/reset-sync/{personnelId} |
| `DELETE` | /assets/{id} |

## [](https://developers.drata.com/changelog/#2024-09-06)2024-09-06

Below is the route that was added to the Open API:

### [](https://developers.drata.com/changelog/#added)Added

*   Trust Center Pro endpoint for retrieving Trust Center usage data.

| Request |
| --- |
| `GET` /public/trust-center/reports |

## [](https://developers.drata.com/changelog/#2023-08-24)2023-08-24

### [](https://developers.drata.com/changelog/#changed)Changed

*   Reports & Docs was updated to Evidence Library in the Drata web app.

Below are the routes that were updated for the Open API:

| Request | Old | New |
| --- | --- | --- |
| `GET` | /public/workspaces/{workspaceId}/reports | /public/workspaces/{workspaceId}/evidence-library |
| `POST` | /public/workspaces/{workspaceId}/reports | /public/workspaces/{workspaceId}/evidence-library |
| `PUT` | /public/workspaces/{workspaceId}/reports/{id} | /public/workspaces/{workspaceId}/evidence-library/{id} |
| `DELETE` | /public/workspaces/{workspaceId}/reports/{id} | /public/workspaces/{workspaceId}/evidence-library/{id} |
