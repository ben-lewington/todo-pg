#!/usr/bin/env bash

source .env

set -xeo nounset

source ./bin/container-db.sh

sleep 2

if [[ $# -ge 1 ]]; then
    DATABASE_URL="${DATABASE_URL}" cargo watch -x r
else
    DATABASE_URL="${DATABASE_URL}" cargo r
fi
