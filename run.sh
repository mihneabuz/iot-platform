#!/bin/bash

docker swarm init
docker stack deploy --compose-file stack.yml iot-platform
