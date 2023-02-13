#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
ADMIN=0x2FC7Def96561ca5e1f0Aa333cb78cFA51DeE3218
PROXY=0x84f7a56483C100ECb12CbB4A31b7873dAE0d8E9B
LOGIC=0xb95e74D3ECaA6270559466576bFD62CE9745764B

SALT=0x0000000000000000000000000000000000000000000000000000000000000006
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$ADMIN
value=0
data1=$(seth calldata "upgrade(address,address)" "$PROXY" "$LOGIC")

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data
# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data
# count=$(seth call $WALLET "transactionCount()(uint)")
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT)
date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash) '+%Y-%m-%d %H:%M:%S'

seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT
