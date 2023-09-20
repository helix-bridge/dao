#!/usr/bin/env bash

set -eox pipefail

HELIX=0xCF8923ebF4244cedC647936a0281dd10bDFCBF18
XIAOCH=0x88a39B052d477CfdE47600a7C9950a441Ce61cb4
WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
SALT=0x000000000000000000000000000000000000000000000000000000000000002f
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

# grantRole operator to XIAOCH
target=$HELIX
value=0
data1=$(seth calldata "grantRole(bytes32,address)" "0x97667070c54ef182b0f5858b034beac1b6f3089aa2d3188bb1e8929f4fa9b929" "$XIAOCH")

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data

# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT)
date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash) '+%Y-%m-%d %H:%M:%S'

seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT
