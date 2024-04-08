#!/usr/bin/env bash

set -eox pipefail

BRIDGE=0xf6372ab2d35B32156A19F2d2F23FA6dDeFBE58bd

target=$BRIDGE
value=0
data1=$(seth calldata "setDailyLimit(address,uint256)" 0x656567Eb75b765FC320783cc6EDd86bD854b2305 60000000000000000000000000)
