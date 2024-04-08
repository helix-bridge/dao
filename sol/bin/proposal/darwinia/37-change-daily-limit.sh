#!/usr/bin/env bash

set -eox pipefail

TIMELOCK=0x000000000f681D85374225EdEeADC25560C1fb3F
BRIDGE=0xf6372ab2d35B32156A19F2d2F23FA6dDeFBE58bd

SALT=0x0000000000000000000000000000000000000000000000000000000000000004
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$BRIDGE
value=0
data1=$(seth calldata "setDailyLimit(address,uint256)" 0x656567Eb75b765FC320783cc6EDd86bD854b2305 60000000000000000000000000)

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain darwinia)
# date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain darwinia) '+%Y-%m-%d %H:%M:%S'
# seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT
