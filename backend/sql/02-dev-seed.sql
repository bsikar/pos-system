-- dev seed
INSERT INTO purchase (id, ctime, items, total) VALUES (100, now(), '{"items": [{"name": "test 1", "price": 100, "quantity": 1}]}', 100); -- test entry
INSERT INTO purchase (id, ctime, items, total) VALUES (101, now(), '{"items": [{"name": "test 2", "price": 200, "quantity": 2}]}', 400); -- test entry