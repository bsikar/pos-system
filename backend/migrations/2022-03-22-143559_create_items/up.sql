-- Your SQL goes here
CREATE TABLE IF NOT EXISTS items (
    "name" TEXT PRIMARY KEY NOT NULL, -- name of item
    price BIGINT NOT NULL CHECK (price >= 0), -- price of item in cents
    tax REAL NOT NULL -- tax
);