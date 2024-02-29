#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x000000000f681D85374225EdEeADC25560C1fb3F

SALT=0x0000000000000000000000000000000000000000000000000000000000000000
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

targets=[0xFd626a860F4d4bDB94aF200c218ce62c9647c8b2,0xf16D9682C4883415aee6e78EB0D6ae3507014e42]
values=[0,0]
data1=$(seth calldata "acceptOwnership()")

datas=[$data1,$data1]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

seth call -F $WALLET $TIMELOCK $data --chain crab
seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

# seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain crab
