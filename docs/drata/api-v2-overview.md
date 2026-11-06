Title: Drata API

URL Source: https://developers.drata.com/openapi/reference/v2/overview/

Published Time: Thu, 05 Nov 2026 22:05:16 GMT

Markdown Content:
Download OpenAPI specification:[Download](blob:https://developers.drata.com/6313e21a-744c-4570-866c-c964264aa074)

### What's New in V2

*   Support for Custom Fields
*   Get and acknowledge user's assigned policies
*   Payloads are streamlined to include only the essential information. You can expand related objects and collections using the `expand` query parameter.
*   Cursor-based pagination for greater efficiency and stability with large datasets

### Getting Started

Please visit our help articles to learn how to [create an API key](https://help.drata.com/en/articles/6695964) or [set up OAuth](https://help.drata.com/en/articles/13521519-set-up-oauth-for-the-drata-api).

### Upgrading From Version 1

API V2 is designed to provide a faster an more flexible way of accessing Drata. There are several differences from V1:

*   The listing endpoints now use **cursor-based pagination**. This provides faster responses and ensures that all records can be retrieved even, if they are changed during the process.

```
async function fetchAll() {
  // The first request doesn't send a value for the cursor.
  let cursor = undefined;
  do {
    const query = new URLSearchParams({ cursor });
    const resp = await fetch(
      `https://public-api.drata.com/public/v2/users?${query}`,
      {
        method: 'GET',
        headers: { Authorization: 'Bearer <YOUR_API_KEY_HERE>' }
      }
    );
    const data = await resp.json();
    console.log(data);

    // If there's a cursor value returned, then there are more results.
    // Pass that back as a query parameter to get the next page of data.
    cursor = data?.pagination?.cursor;
  } while (cursor)
}
fetchAll();
```

*   You'll notice smaller responses. Most endpoints accept an `expand` query parameter that let's you specify which related objects and collections you want to expand.
