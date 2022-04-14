#! /bin/sh

echo "Test DB"

echo "post item single donut hole"
curl -X POST -H "Content-Type: application/json" -d '{"name":"single donut hole", "price":30, "tax":1.0, "type_":"food"}' http://localhost:8080/api/items
echo "\n"

echo "post item dozen donut hole"
curl -X POST -H "Content-Type: application/json" -d '{"name":"half dozen donut hole", "price":150, "tax":1.0, "type_":"food"}' http://localhost:8080/api/items
echo "\n"

echo "post new purchase"
curl -X POST -H "Content-Type: application/json" -d '{"items": [{"name": "single donut hole","price": 30,"quantity": 2,"tax":1.0, "type":"food"},{"name": "half dozen donut hole","price": 150,"quantity": 1,"tax":1.0,"type":"food"}]}' http://localhost:8080/api/purchases
echo "\n"

echo "get all purchases"
curl -X GET http://localhost:8080/api/purchases
echo "\n"