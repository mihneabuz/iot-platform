#!/bin/bash

docker swarm init
docker stack deploy --compose-file stack.yml sprc3
