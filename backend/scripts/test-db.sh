#!/bin/bash

for _ in {1..10}; do
    item="{\"name\":\"food - $RANDOM\", \"price\":$RANDOM, \"tax\":$RANDOM, \"type\":\"food\"}"
    curl -X POST -H "Content-Type: application/json" -d "$item" "http://localhost:8080/api/items"

    item="{\"name\":\"drink - $RANDOM\", \"price\":$RANDOM, \"tax\":$RANDOM, \"type\":\"drink\"}"
    curl -X POST -H "Content-Type: application/json" -d "$item" "http://localhost:8080/api/items"

    item="{\"name\":\"other - $RANDOM\", \"price\":$RANDOM, \"tax\":$RANDOM, \"type\":\"other\"}"
    curl -X POST -H "Content-Type: application/json" -d "$item" "http://localhost:8080/api/items"
done