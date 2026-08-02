# Client API Documentation

Base URL: `http://localhost:8080`

All endpoints return JSON.

## Endpoints

### GET /posts

Returns a list of post previews.

Query parameters:

- `page_size` (required, integer): number of posts per page.
- `page` (required, integer): page number (0-indexed).
- `search` (optional, string): full-text search query.
- `tag` (optional, repeated): filter by tag name(s). Can be specified multiple times.

If `search` is provided, `tag` cannot be used (returns error).

Response 200:

```json
{
  "posts": [
    {
      "post_id": 1,
      "title": "Example",
      "summary": "Summary",
      "upload_date": "2024-01-01",
      "revision_date": "2024-01-02",
      "likes": 10
    }
  ]
}
```

### GET /posts/{postid}

Returns a single post with full details.

Path parameters:

- `postid` (integer): ID of the post.

Response 200:

```json
{
  "post": {
    "post_id": 1,
    "title": "Example",
    "summary": "Summary",
    "upload_date": "2024-01-01",
    "revision_date": "2024-01-02",
    "body": "Full body text",
    "tags": ["tag1", "tag2"],
    "likes": 10
  }
}
```

### PUT /posts/{postid}/like

Increments the like count for the post.

Path parameters:

- `postid` (integer): ID of the post.

No request body.

Response 200 (empty body).

### PUT /posts/{postid}/unlike

Decrements the like count (minimum 0).

Path parameters:

- `postid` (integer): ID of the post.

No request body.

Response 200 (empty body).

### GET /citations

Returns a list of citations.

Query parameters:

- `author` (optional, string): filter by author prefix.
- `source` (optional, string): filter by source prefix.
- `page_size` (required, integer): number of citations per page.
- `page` (required, integer): page number (0-indexed).

Response 200:

```json
{
  "citations": [
    {
      "citation_id": 1,
      "author": "Author",
      "rizz": 5,
      "source": "Source",
      "body": "Citation text"
    }
  ]
}
```

### GET /citations/{citation_id}

Returns a single citation.

Path parameters:

- `citation_id` (integer): ID of the citation.

Response 200:

```json
{
  "citation": {
    "citation_id": 1,
    "author": "Author",
    "rizz": 5,
    "source": "Source",
    "body": "Citation text"
  }
}
```

### GET /portfolio

Returns the portfolio projects.

No query parameters.

Response 200:

```json
{
  "portfolio": [
    {
      "project_id": 1,
      "rizz": 3,
      "project_name": "Project",
      "note": "Note"
    }
  ]
}
```

### GET /portfolio/{id}

Returns a single portfolio project.

Path parameters:

- `id` (integer): ID of the project.

Response 200:

```json
{
  "project": {
    "project_id": 1,
    "rizz": 3,
    "project_name": "Project",
    "note": "Note"
  }
}
```

### GET /tags

Returns a list of tags.

Query parameters:

- `q` (optional, string): filter by tag name prefix.

Response 200:

```json
{
  "tags": [
    {
      "tag_id": 1,
      "tag_name": "rust"
    }
  ]
}
```

### DELETE /tags/{tag_name}

Deletes a tag by its name.

Path parameters:

- `tag_name` (string): name of the tag to delete. URL-encode the tag name when necessary.

No request body.

Response 200 (empty body).

## Error Responses

All errors return HTTP 500 with a plain text error message.
