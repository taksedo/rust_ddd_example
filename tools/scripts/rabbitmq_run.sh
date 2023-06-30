#!/bin/sh

docker run --rm --name rabbitmq-stream-go-client-test -d -p 5552:5552 -p 5672:5672 -p 15672:15672 -e RABBITMQ_SERVER_ADDITIONAL_ERL_ARGS="-rabbitmq_stream advertised_host localhost" -e RABBITMQ_DEFAULT_USER="guest" -e RABBITMQ_DEFAULT_PASS="guest" --pull always pivotalrabbitmq/rabbitmq-stream
