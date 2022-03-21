-- create entry
CREATE TABLE IF NOT EXISTS purchase (
    id bigserial NOT NULL, -- id of entry
    ctime timestamp NOT NULL, -- creation time
    items json NOT NULL, -- items in entry
    total bigint NOT NULL -- amount of items in entry in cents
);
ALTER SEQUENCE purchase_id_seq RESTART WITH 1000; -- start id at 1000 so our test data wont overlap

-- create entry
CREATE TABLE IF NOT EXISTS item (
    "name" text NOT NULL, -- name of item
    price bigint NOT NULL, -- price of item in cents
    tax real NOT NULL -- tax
);