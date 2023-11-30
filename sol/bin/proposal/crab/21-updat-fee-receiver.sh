#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x000000000f681D85374225EdEeADC25560C1fb3F
LnBridge=0x94C614DAeFDbf151E1BB53d6A201ae5fF56A9337
from=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6

SALT=0x0000000000000000000000000000000000000000000000000000000000000000
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$LnBridge
value=0
# LnBridge set updateFeeReceiver to TIMELOCK
data1=$(seth calldata "updateFeeReceiver(address)" $TIMELOCK)

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data --chain crab
# seth send -F $from $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
# count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain crab)
date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain crab) '+%Y-%m-%d %H:%M:%S'

seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain crab
