-- dev seed
INSERT INTO purchase (id, ctime, items, total)
VALUES
(
    100,
    now(),
    '[
        {
            "name": "single glazed donut",
            "price": 120,
            "quantity": 1
        }
    ]',
    120
); -- test entry

INSERT INTO purchase (id, ctime, items, total)
VALUES
(
    101,
    now(),
    '[
        {
            "name": "half dozen glazed donuts",
            "price": 625,
            "quantity": 2
        }
    ]',
    1250
); -- test entry

INSERT INTO purchase (id, ctime, items, total)
VALUES
(
    102,
    now(),
    '[
        {
            "name": "half dozen glazed donuts",
            "price": 625,
            "quantity": 1
        },
        {
            "name": "dozen glazed donuts",
            "price": 1099,
            "quantity": 2
        }
    ]',
    2823
); -- test entry

INSERT INTO item ("name", price)
VALUES
(
    'single glazed donut',
    120
); -- test entry

INSERT INTO item ("name", price)
VALUES
(
    'half dozen glazed donuts',
    625
); -- test entry

INSERT INTO item ("name", price)
VALUES
(
    'dozen glazed donuts',
    1099
); -- test entry