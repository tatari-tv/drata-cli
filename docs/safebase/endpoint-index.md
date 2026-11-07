# SafeBase API v1 - Endpoint Index

Generated from `spec/safebase-openapi-v1.json` (OpenAPI 3.1.0, spec version 1.4.1).

Servers:
- `https://app.safebase.io/api/ext/v1/rest`

## Account Members

| Method | Path | Summary |
|---|---|---|
| `POST` | `/accounts/{id}/members/notify` | Notify a member |
| `POST` | `/accounts/{id}/members` | Add a member |
| `DELETE` | `/accounts/{id}/members` | Delete a member |
| `GET` | `/accounts/members` | Search for members |

## Accounts

| Method | Path | Summary |
|---|---|---|
| `POST` | `/accounts` | Create account |
| `GET` | `/accounts` | Get accounts |
| `DELETE` | `/accounts/{id}` | Delete account by ID |
| `GET` | `/accounts/{id}` | Get account by ID |
| `PATCH` | `/accounts/{id}` | Edit account |
| `GET` | `/accounts/{id}/url` | Get account page URL |

## Documents

| Method | Path | Summary |
|---|---|---|
| `GET` | `/document/{id}` | Get a document by ID or external ID |
| `DELETE` | `/document/{id}` | Delete a document by ID |
| `PATCH` | `/document/{id}` | Update document by ID |
| `PUT` | `/documents/upload` | Create document |
| `POST` | `/documents/upload` | Request file upload |
| `GET` | `/documents` | Get all documents |
| `GET` | `/document-types` | Get document types |

## Knowledge Base

| Method | Path | Summary |
|---|---|---|
| `POST` | `/kb/entries` | Create Knowledge Base Entry |
| `GET` | `/kb/search` | Search Knowledge Base |
| `PATCH` | `/kb/entries/{id}` | Update Knowledge Base Entry |

## Organization

| Method | Path | Summary |
|---|---|---|
| `GET` | `/organizations` | Get current Organization |
| `GET` | `/organizations/members/me` | Get current Organization Member |
| `GET` | `/organizations/nda` | Get Organization NDA settings |
| `GET` | `/organizations/settings` | Get Organization settings |

## Portals

| Method | Path | Summary |
|---|---|---|
| `GET` | `/portals/default/products` | Get All Products |

## Products

| Method | Path | Summary |
|---|---|---|
| `GET` | `/products` | Get products |

## Questionnaires

| Method | Path | Summary |
|---|---|---|
| `GET` | `/questionnaires/{id}` | Get completed questionnaire |
| `POST` | `/questionnaires` | Upload Questionnaire |

## Requests

| Method | Path | Summary |
|---|---|---|
| `POST` | `/requests/{requestId}/approve` | Approve access request |
| `POST` | `/requests/{requestId}/decline` | Decline access request |
| `GET` | `/requests` | Get access requests |

## Tags

| Method | Path | Summary |
|---|---|---|
| `GET` | `/tags` | Get tags |

## Trust Center Updates

| Method | Path | Summary |
|---|---|---|
| `POST` | `/tcu/topics` | Create topic |
| `GET` | `/tcu/topics` | List topics |
| `POST` | `/tcu/topics/{topicId}/updates` | Create update |
| `DELETE` | `/tcu/topics/{topicId}` | Delete topic |
| `GET` | `/tcu/topics/{topicId}` | Get topic by ID |
| `PATCH` | `/tcu/topics/{topicId}` | Update topic |
| `DELETE` | `/tcu/topics/{topicId}/updates/{updateId}` | Delete update |
| `GET` | `/tcu/topics/{topicId}/updates/{updateId}` | Get update by ID |
| `PATCH` | `/tcu/topics/{topicId}/updates/{updateId}` | Edit update |

## Webpages

| Method | Path | Summary |
|---|---|---|
| `POST` | `/webpages` | Create webpage |
| `GET` | `/webpages` | List webpages |
| `GET` | `/webpage/{id}` | Get a webpage by ID |
| `DELETE` | `/webpage/{id}` | Delete a webpage by ID |
| `PATCH` | `/webpage/{id}` | Update a webpage by ID |
| `POST` | `/webpage/{id}/refresh` | Refresh a webpage |
