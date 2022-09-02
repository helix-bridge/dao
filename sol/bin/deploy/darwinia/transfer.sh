#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D

SALT=0x0000000000000000000000000000000000000000000000000000000000000001
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6 #echo
value=$(seth --to-wei 0.1 ether)
data1=0x

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK $value $data
count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

# seth call $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT
