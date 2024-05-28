CREATE TABLE IF NOT EXISTS users (
    id TEXT NOT NULL, -- uuid v4

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS cards (
    id           TEXT NOT NULL, -- uuid v4
    user_id      TEXT NOT NULL, -- uuid v4
    title        TEXT NOT NULL DEFAULT '',
    created_at   INTEGER NOT NULL, -- unix timestamp (secs)

    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS todos (
    id           INTEGER NOT NULL,
    card_id      TEXT NOT NULL, -- uuid v4
    content      TEXT NOT NULL DEFAULT '',
    completed_at INTEGER, -- nullable unix timestamp (secs)

    PRIMARY KEY (id)
);
