-- Cloudflare D1; SQLite3

CREATE TABLE IF NOT EXISTS users (
    id TEXT NOT NULL, -- uuid v4

    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS cards (
    id      TEXT NOT NULL, -- uuid v4
    user_id TEXT NOT NULL, -- uuid v4
    title   TEXT NOT NULL DEFAULT '',

    PRIMARY KEY (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);

CREATE TABLE IF NOT EXISTS todos (
    id           TEXT NOT NULL, -- uuid v4
    card_id      TEXT NOT NULL, -- uuid v4
    content      TEXT NOT NULL,
    completed_at INTEGER, -- nullable unix timestamp (secs)

    PRIMARY KEY (id),
    FOREIGN KEY (card_id) REFERENCES cards (id)
);
