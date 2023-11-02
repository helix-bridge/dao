#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
xWRING=0x273131f7cb50ac002bdd08ca721988731f7e1092
XIAOCH=0x3B9E571AdeCB0c277486036D6097E9C2CCcfa9d9

SALT=0x0000000000000000000000000000000000000000000000000000000000000032
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$xWRING
value=0
# transfer 2M xWRING to XIAOCH on crab
amount=$(seth --to-wei 2000000 ether)
data1=$(seth calldata "transfer(address,uint)" "$XIAOCH" "$amount")

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

seth call -F $WALLET $TIMELOCK $data --chain crab
seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

# op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain crab)
# date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain crab) '+%Y-%m-%d %H:%M:%S'

# seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain crab
