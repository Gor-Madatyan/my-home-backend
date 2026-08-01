# My-Home Admin Server API

Base URL: `http://localhost:9090`

All endpoints are intended for internal administrative use only.

## Common Conventions

- **Authentication**: Not required in the current implementation.
- **Request Headers**: `Content-Type: application/json` must be set for PUT endpoints.
- **Success Response**: All endpoints return `HTTP 200 OK` with an empty response body when the operation succeeds.
- **Error Response**: If a database error occurs, the server returns `HTTP 500 Internal Server Error` with a plain text description of the error.

---

## `PUT /citations`

Upsert (insert or replace) a citation.

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `citation_id` | integer | No | If present, the citation with this ID is replaced. If absent, a new citation is created. |
| `author` | string | Yes | Author name |
| `rizz` | integer (unsigned 16-bit) | Yes | A numeric score (0–65535) |
| `source` | string | Yes | Source or publication name |
| `body` | string | Yes | Citation text |

**Example**

```json
{
  "citation_id": 12,
  "author": "Marie Curie",
  "rizz": 42,
  "source": "Nobel Lecture",
  "body": "Nothing in life is to be feared, it is only to be understood."
}
```

**Responses**

- `200 OK` – empty body on success
- `500 Internal Server Error` – on database error

---

## `DELETE /citations/{citation_path}`

Delete a citation by its internal numeric ID.

**Path Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `citation_path` | integer | ID of the citation to delete |

**Example**

`DELETE /citations/12`

**Responses**

- `200 OK` – empty body on success
- `500 Internal Server Error` – on database error

---

## `PUT /portfolio`

Upsert (insert or replace) a portfolio project.

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `project_id` | integer | No | If present, replaces the project with this ID. If absent, a new project is added. |
| `rizz` | integer | Yes | A numeric score used for ordering |
| `project_name` | string | Yes | Display name of the project |
| `note` | string | Yes | Description or notes about the project |

**Example**

```json
{
  "project_id": 3,
  "rizz": 99,
  "project_name": "Static Site Generator",
  "note": "A small Rust program that builds my static website."
}
```

**Responses**

- `200 OK` – empty body on success
- `500 Internal Server Error` – on database error

---

## `DELETE /portfolio/{project_path}`

Delete a portfolio project by its numeric ID.

**Path Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `project_path` | integer | ID of the project to delete |

**Example**

`DELETE /portfolio/3`

**Responses**

- `200 OK` – empty body on success
- `500 Internal Server Error` – on database error

---

## `PUT /tags`

Upsert (insert or replace) a tag.

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `tag_name` | string | Yes | Name of the tag |
| `tag_id` | integer | No | If present, replaces the tag with this ID. If absent, a new tag is created. |

**Example**

```json
{
  "tag_id": 17,
  "tag_name": "rust"
}
```

**Responses**

- `200 OK` – empty body on success
- `500 Internal Server Error` – on database error

---

## `DELETE /tags/{tag_path}`

Delete a tag by its numeric ID.

**Path Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `tag_path` | integer | ID of the tag to delete |

**Example**

`DELETE /tags/17`

**Responses**

- `200 OK` – empty body on success
- `500 Internal Server Error` – on database error

---

## `PUT /posts`

Upsert (insert or replace) a blog post and manage its associated tags.

**Request Body**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `post_id` | integer | No | If present, replaces the post with this ID. If absent, a new post is created. |
| `title` | string | Yes | Post title |
| `summary` | string | Yes | Short summary or description |
| `upload_date` | string | Yes | Date the post was uploaded (format depends on client; usually ISO 8601) |
| `revision_date` | string | Yes | Date of last revision |
| `body` | string | Yes | Full content of the post |
| `tags` | string | Yes | Comma-separated list of tag names. Spaces are trimmed; empty elements are ignored. |
| `likes` | integer | Yes | Initial number of likes |

**Processing Details**

1. The post is inserted or replaced according to `post_id`.
2. The `tags` string is split on commas (`,`), trimmed, and empty items removed.
3. For each resulting tag name, the system ensures a row exists in the `tags` table (creating it if needed) and creates an association in the `posts_tags` join table between the post and the tag.
4. The operation is performed inside a database transaction, so either all changes are committed or none are.

**Example**

```json
{
  "post_id": 101,
  "title": "Hello, Rust!",
  "summary": "First post with tags",
  "upload_date": "2026-08-02T10:00:00Z",
  "revision_date": "2026-08-02T10:00:00Z",
  "body": "This is a paragraph.",
  "tags": "rust, axum, sqlx",
  "likes": 0
}
```

**Responses**

- `200 OK` – empty body on success
- `500 Internal Server Error` – on database error

---

## `DELETE /posts/{post_path}`

Delete a blog post by its numeric ID.

**Path Parameters**

| Parameter | Type | Description |
|-----------|------|-------------|
| `post_path` | integer | ID of the post to delete |

**Example**

`DELETE /posts/101`

**Responses**

- `200 OK` – empty body on success
- `500 Internal Server Error` – on database error

---

## Notes

- The server binds to `localhost:9090` unless configured otherwise.
- The SQLite database used by the server is expected to already contain the required tables.
- The `TagDraft` type in the furniture crate defines the request body for tag upsert operations.
