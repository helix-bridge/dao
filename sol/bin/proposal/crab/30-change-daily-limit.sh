#!/usr/bin/env bash

set -eox pipefail

OWNER=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x000000000f681D85374225EdEeADC25560C1fb3F
BRIDGE=0xa64D1c284280b22f921E7B2A55040C7bbfD4d9d0

SALT=0x0000000000000000000000000000000000000000000000000000000000000001
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$BRIDGE
value=0
data1=$(seth calldata "setDailyLimit(address,uint256)" 0x2D2b97EA380b0185e9fDF8271d1AFB5d2Bf18329 60000000000000000000000000)

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

seth call -F $TIMELOCK $BRIDGE $data1 --chain crab
seth call -F $WALLET $TIMELOCK $data --chain crab
seth send -F $OWNER $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

# op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain crab)
# date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain darwinia) '+%Y-%m-%d %H:%M:%S'

# seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain crab

