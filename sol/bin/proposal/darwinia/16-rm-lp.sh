#!/usr/bin/env bash

set -eox pipefail

LP=0xdfA67E8f80F6B37a21e5C3baeEb0eCa893652Ab9
WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
ECHO=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
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

# seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT

xWCRAB=0x656567eb75b765fc320783cc6edd86bd854b2305
WRING=0xe7578598aac020abfb918f33a20fad5b71d670b4
PAIR=0xdfA67E8f80F6B37a21e5C3baeEb0eCa893652Ab9
ROUTER=0xB899409cDA0fFA2bF87F9c7b31f3c77D6A3a0bB0
