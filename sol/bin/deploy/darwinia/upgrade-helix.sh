#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
ADMIN=0x91331B6Dd589163aF02Fb13E0466c3ba10eE310f
PROXY=0xF3c1444CD449bD66Ef6DA7CA6c3E7884840A3995
LOGIC=0x3141fEB844049B7b0f922542a4cf5Bcae9e1367d

SALT=0x0000000000000000000000000000000000000000000000000000000000000003
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$ADMIN
value=0
data1=$(seth calldata "upgrade(address,address)" "$PROXY" "$LOGIC")

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

seth call -F $WALLET $TIMELOCK $data

seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

# seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT
