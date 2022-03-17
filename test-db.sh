#! /bin/sh

echo "Test DB"

echo "post item single donut hole"
curl -X POST -H "Content-Type: application/json" -d '{"name":"single donut hole", "price":30}' http://localhost:3030/api/items
echo "\n"

echo "post item dozen donut hole"
curl -X POST -H "Content-Type: application/json" -d '{"name":"half dozen donut hole", "price":150}' http://localhost:3030/api/items
echo "\n"

echo "post new purchase"
curl -X POST -H "Content-Type: application/json" -d '{"items": [{"name": "single donut hole","price": 30,"quantity": 2},{"name": "half dozen donut hole","price": 150,"quantity": 1}]}' http://localhost:3030/api/purchases
echo "\n"

echo "get all purchases"
curl -X GET http://localhost:3030/api/purchases
echo "\n"