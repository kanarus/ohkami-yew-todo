-- Cloudflare D1; SQLite3

CREATE TABLE IF NOT EXISTS users (
    id    TEXT NOT NULL, -- uuid v4
    token TEXT NOT NULL, -- active JWT token

    PRIMARY KEY (id),
    UNIQUE (token)
);

CREATE TABLE IF NOT EXISTS todos (
    id           TEXT NOT NULL, -- uuid v4
    user_id      TEXT NOT NULL, -- uuid v4
    content      TEXT NOT NULL,
    completed_at INTEGER,       -- nullable unix timestamp (secs)

    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS tags (
    id   INTEGER NOT NULL, -- like autoincremented (https://www.sqlite.org/autoinc.html)
    name TEXT    NOT NULL,

    PRIMARY KEY (id),
    UNIQUE (name)
);

CREATE TABLE IF NOT EXISTS todo_tags (
    todo_id TEXT    NOT NULL, -- uuid v4
    tag_id  INTEGER NOT NULL,

    UNIQUE (todo_id, tag_id),
    FOREIGN KEY (todo_id) REFERENCES todos (id),
    FOREIGN KEY (tag_id)  REFERENCES tags (id)
);
