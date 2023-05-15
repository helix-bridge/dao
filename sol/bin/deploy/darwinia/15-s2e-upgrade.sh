#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
Erc20Sub2EthBackingProxy=0xD1B10B114f1975d8BCc6cb6FC43519160e2AA978
ENDPOINT=0xaa69C8955636A623b2AB0AEE7366B4bB3677051D


SALT=0x000000000000000000000000000000000000000000000000000000000000000f
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$Erc20Sub2EthBackingProxy
value=0
data1=$(seth calldata "setMessageEndpoint(address)" $ENDPOINT)

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain darwinia
# count=$(seth call $WALLET "transactionCount()(uint)" --chain darwinia)
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain darwinia

op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain darwinia)
date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain darwinia) '+%Y-%m-%d %H:%M:%S'

seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain darwinia

