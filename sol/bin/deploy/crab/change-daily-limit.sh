#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
Erc20Sub2SubMappingTokenFactoryProxy=0x8738A64392b71617aF4C685d0E827855c741fDF7
xWRING=0x273131F7CB50ac002BDd08cA721988731F7e1092

SALT=0x000000000000000000000000000000000000000000000000000000000000000d
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=$Erc20Sub2SubMappingTokenFactoryProxy
value=0
data1=$(seth calldata "changeDailyLimit(address,uint256)" $xWRING 3000000000000000000000000)

data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

seth call -F $WALLET $TIMELOCK $data

# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data

# seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT
