-- Your SQL goes here
CREATE TABLE IF NOT EXISTS purchases (
    id BIGSERIAL PRIMARY KEY NOT NULL, -- id of entry
    ctime TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP, -- creation time
    items JSONB NOT NULL, -- items in entry
    total BIGINT NOT NULL -- amount of items in entry in cents
);