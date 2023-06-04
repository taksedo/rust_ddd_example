#!/bin/sh

cd tests/performance || echo "Destination folder not found"

docker compose -d --file ./tests/performance/docker-compose.yml up