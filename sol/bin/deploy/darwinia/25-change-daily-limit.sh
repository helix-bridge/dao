#!/usr/bin/env bash

set -eox pipefail

ETH_FROM=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
WRING=0xE7578598Aac020abFB918f33A20faD5B71d670b4
LNBRIDGE=0xD1B10B114f1975d8BCc6cb6FC43519160e2AA978

SALT=0x0000000000000000000000000000000000000000000000000000000000000034
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$LNBRIDGE
value=0
data1=$(seth calldata "changeDailyLimit(address,uint256)" $WRING 35000000000000000000000000)

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data

seth send -F $ETH_FROM $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain darwinia
count=$(seth call $WALLET "transactionCount()(uint)" --chain darwinia)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain darwinia

# op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain darwinia)
# date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain darwinia) '+%Y-%m-%d %H:%M:%S'

# seth send -F $ETH_FROM $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain darwinia
