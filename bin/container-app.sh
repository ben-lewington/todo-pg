#!/usr/bin/env bash

docker="docker compose -f ./docker/base.yml"

set -xeo nounset

$docker down -v --remove-orphans
$docker build
$docker up -d
$docker run --rm --no-deps app /bin/bash
