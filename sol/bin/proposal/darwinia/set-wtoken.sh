#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
PROXY=0xF3c1444CD449bD66Ef6DA7CA6c3E7884840A3995
WTOKEN=0xE7578598Aac020abFB918f33A20faD5B71d670b4

SALT=0x0000000000000000000000000000000000000000000000000000000000000004
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$PROXY
value=0
data1=$(seth calldata "setWToken(address)" "$WTOKEN")

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data
# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data
# count=$(seth call $WALLET "transactionCount()(uint)")
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT)
date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash) '+%Y-%m-%d %H:%M:%S'

seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT
