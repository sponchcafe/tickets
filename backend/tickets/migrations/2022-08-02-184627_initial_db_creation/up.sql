-- Your SQL goes here

PRAGMA encoding = "UTF-8";

CREATE TABLE users (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    email TEXT
);

CREATE TABLE tickets (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    body TEXT,
    created INTEGER NOT NULL,
    updated INTEGER,
    creator INTEGER NOT NULL,
    assginee INTEGER,
    stage INTEGER NOT NULL,
    order_index INTEGER NOT NULL
);