-- dev seed
INSERT INTO purchase (id, ctime, items, total)
VALUES
(
    100,
    now(),
    '[
        {
            "name": "test 1",
            "price": 100,
            "quantity": 1
        }
    ]',
    100
); -- test entry

INSERT INTO purchase (id, ctime, items, total)
VALUES
(
    101,
    now(),
    '[
        {
            "name": "test 2",
            "price": 200,
            "quantity": 2
        }
    ]',
    400
); -- test entry

INSERT INTO purchase (id, ctime, items, total)
VALUES
(
    102,
    now(),
    '[
        {
            "name": "test 3-1",
            "price": 200,
            "quantity": 2
        },
        {
            "name": "test 3-2",
            "price": 300,
            "quantity": 1
        }
    ]',
    700
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
    'half dozen glazed donut',
    625
); -- test entry

INSERT INTO item ("name", price)
VALUES
(
    'dozen glazed donut',
    1099
); -- test entry