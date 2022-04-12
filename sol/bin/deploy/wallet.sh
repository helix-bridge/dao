#!/usr/bin/env bash

set -eo pipefail

export NETWORK_NAME=${1?}

. $(dirname $0)/common.sh
