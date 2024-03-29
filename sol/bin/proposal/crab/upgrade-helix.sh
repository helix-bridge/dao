#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
ADMIN=0x9FA16146BA5b7d144927E46acd9ba143f7205Fc8
PROXY=0x8738A64392b71617aF4C685d0E827855c741fDF7
LOGIC=0x31588a248550a90Bb6Bf784434e17D9470Ea0659

SALT=0x0000000000000000000000000000000000000000000000000000000000000010
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$ADMIN
value=0
data1=$(seth calldata "upgrade(address,address)" "$PROXY" "$LOGIC")

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data
# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data

# op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT)
# date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash) '+%Y-%m-%d %H:%M:%S'

seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT
