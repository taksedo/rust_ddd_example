#!/bin/sh

docker build --file tools/docker/Dockerfile -t rust_ddd_example .

#docker run -it --rm -p 8080:8080 --name rust_ddd_example rust_ddd_example
docker compose up --file docker/docker-compose.yml -d