#!/bin/bash

set -e

if [ $(id -u) -ne 0 ]
  then echo Please run this script as root
  exit 1
fi

docker compose build
docker stack deploy -c docker-compose.yaml 64
