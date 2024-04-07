#!/usr/bin/env bash

docker="podman compose -f ./docker/base.yml"

set -xeo nounset

$docker down
$docker build db
$docker up -d db
