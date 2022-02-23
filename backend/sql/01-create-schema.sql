-- create entry
CREATE TABLE purchase (
    id bigserial NOT NULL, -- id of entry
    ctime timestamp with time zone DEFAULT now() NOT NULL, -- creation time
    items json NOT NULL, -- items in entry
    total bigint NOT NULL -- amount of items in entry in cents
);
ALTER SEQUENCE purchase_id_seq RESTART WITH 1000; -- start id at 1000 so our test data wont overlap