BEGIN TRANSACTION;

CREATE TABLE IF NOT EXISTS blog_posts
(
    post_id       INTEGER PRIMARY KEY,
    title         TEXT    NOT NULL UNIQUE,
    summary       TEXT    NOT NULL,
    upload_date   TEXT    NOT NULL,
    revision_date TEXT    NOT NULL,
    body          TEXT    NOT NULL,
    likes         INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_blog_posts_upload_date
    ON blog_posts (upload_date DESC);

CREATE INDEX IF NOT EXISTS idx_blog_posts_likes
    ON blog_posts (likes DESC);

CREATE VIRTUAL TABLE IF NOT EXISTS blog_posts_fts USING fts5
(
    title,
    summary,
    body,
    content = 'blog_posts',
    content_rowid = 'post_id'
);

-- 1. Sync on Insert (Unchanged, standard INSERT syntax)
CREATE TRIGGER IF NOT EXISTS blog_posts_ai
    AFTER INSERT
    ON blog_posts
BEGIN
    INSERT INTO blog_posts_fts(rowid, title, summary, body)
    VALUES (new.post_id, new.title, new.summary, new.body);
END;

-- 2. Sync on Delete (Fires BEFORE the row is gone, allowing a standard DELETE)
CREATE TRIGGER IF NOT EXISTS blog_posts_bd
    BEFORE DELETE
    ON blog_posts
BEGIN
    DELETE FROM blog_posts_fts WHERE rowid = old.post_id;
END;

-- 3. Sync on Update: Part 1 (Removes the old FTS entry before the data changes)
CREATE TRIGGER IF NOT EXISTS blog_posts_bu
    BEFORE UPDATE OF title, summary, body
    ON blog_posts
BEGIN
    DELETE FROM blog_posts_fts WHERE rowid = old.post_id;
END;

-- 4. Sync on Update: Part 2 (Inserts the new FTS entry after the data changes)
CREATE TRIGGER IF NOT EXISTS blog_posts_au
    AFTER UPDATE OF title, summary, body
    ON blog_posts
BEGIN
    INSERT INTO blog_posts_fts(rowid, title, summary, body)
    VALUES (new.post_id, new.title, new.summary, new.body);
END;

CREATE TABLE IF NOT EXISTS tags
(
    tag_id   INTEGER PRIMARY KEY,
    tag_name TEXT NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_tags_tag_id ON tags (tag_name);

CREATE TABLE IF NOT EXISTS posts_tags
(
    post_id INTEGER NOT NULL,
    tag_id  INTEGER NOT NULL,
    PRIMARY KEY (post_id, tag_id),
    FOREIGN KEY (post_id)
        REFERENCES blog_posts (post_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,
    FOREIGN KEY (tag_id)
        REFERENCES tags (tag_id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_posts_tags_tag_id ON posts_tags (tag_id, post_id);


CREATE TABLE IF NOT EXISTS citations
(
    citation_id INTEGER PRIMARY KEY,
    rizz        INTEGER NOT NULL,
    author      TEXT    NOT NULL,
    source      TEXT    NOT NULL,
    body        TEXT    NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_citations_author ON citations (rizz DESC);
CREATE INDEX IF NOT EXISTS idx_citations_author ON citations (author, source);
CREATE INDEX IF NOT EXISTS idx_citations_source ON citations (source);
COMMIT;
