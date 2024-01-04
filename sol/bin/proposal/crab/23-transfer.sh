#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
LP=0x05F0Bc920A23D1662764907910b150C819C110aa
TREASURY=0x6D6f646c64612f74727372790000000000000000

SALT=0x0000000000000000000000000000000000000000000000000000000000000033
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

wad=$(seth call $LP "balanceOf(address)(uint)" $TIMELOCK --chain crab)
target=$LP
value=0
data1=$(seth calldata "transfer(address,uint)" "$TREASURY" "$wad")

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data --chain crab
# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
# count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

# op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain crab)
# date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain crab) '+%Y-%m-%d %H:%M:%S'

seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain crab
