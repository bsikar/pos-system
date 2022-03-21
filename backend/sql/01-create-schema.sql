-- create entry
CREATE TABLE IF NOT EXISTS purchase (
    id BIGSERIAL NOT NULL, -- id of entry
    ctime TIMESTAMP NOT NULL, -- creation time
    items JSON NOT NULL, -- items in entry
    total BIGINT NOT NULL -- amount of items in entry in cents
);
-- ALTER SEQUENCE purchase_id_seq RESTART WITH 1000; -- start id at 1000 so our test data wont overlap

-- create entry
CREATE TABLE IF NOT EXISTS item (
    "name" TEXT NOT NULL, -- name of item
    price BIGINT NOT NULL, -- price of item in cents
    tax REAL NOT NULL -- tax
);