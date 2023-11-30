#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
S2SMAPPINGTOKENFACTORYPROXY=0x3CC8913088F79831c8335f0307f4FC92d79C1ac7

SALT=0x000000000000000000000000000000000000000000000000000000000000000c
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$S2SMAPPINGTOKENFACTORYPROXY
value=0
data1=$(seth calldata "pause()")

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data
# count=$(seth call $WALLET "transactionCount()(uint)")
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT
