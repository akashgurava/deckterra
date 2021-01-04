CREATE TABLE IF NOT EXISTS 'cards' (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    deck_code TEXT NOT NULL,
    code TEXT NOT NULL,
    "set" INTEGER NOT NULL,
    faction INTEGER NOT NULL,
    "number" INTEGER NOT NULL,
    "count" INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS 'decks' (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    uid TEXT NOT NULL,
    deck_code TEXT NOT NULL,
    title TEXT NOT NULL,
    "description" TEXT NOT NULL,
    rating INTEGER NOT NULL,
    mode TEXT NOT NULL,
    playstyle TEXT NOT NULL,
    changed_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL,
    is_draft BOOLEAN NOT NULL,
    is_private BOOLEAN NOT NULL,
    is_riot BOOLEAN NOT NULL
);