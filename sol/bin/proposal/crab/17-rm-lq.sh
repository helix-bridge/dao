#!/usr/bin/env bash

set -eox pipefail

LP=0x05F0Bc920A23D1662764907910b150C819C110aa
WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
ECHO=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
SALT=0x000000000000000000000000000000000000000000000000000000000000001f
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

BALANCE1=$(seth call $LP "balanceOf(address)(uint)" $TIMELOCK)

# transfer LP to ECHO
target=$LP
value=0
data1=$(seth calldata "transfer(address,uint)" "$ECHO" "$BALANCE1")

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data

# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT)
date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash) '+%Y-%m-%d %H:%M:%S'

seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT
