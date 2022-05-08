#!/bin/bash

R=$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM$RANDOM

item="{\"name\":\"food - $R\", \"price\":$RANDOM, \"tax\":$RANDOM, \"type\":\"food\"}"
curl -X POST -H "Content-Type: application/json" -d "$item" "http://localhost:8080/api/items"
