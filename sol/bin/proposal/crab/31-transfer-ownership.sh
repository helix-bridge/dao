#!/usr/bin/env bash

set -eox pipefail

OWNER=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x000000000f681D85374225EdEeADC25560C1fb3F
SPHINX_DAO=0xD0a0899c5dc2FEb253D57Ab0b7c6d1b1Fcbbf824

SALT=0x0000000000000000000000000000000000000000000000000000000000000000
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

targets=[0x65be094765731f394bc6d9df53bdf3376f1fc8b0]
values=[0]
data1=$(seth calldata "transferOwnership(address)" $SPHINX_DAO)

datas=[$data1]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

seth call -F $WALLET $TIMELOCK $data --chain crab
seth send -F $OWNER $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

# seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain crab
