#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
WCRAB=0x2D2b97EA380b0185e9fDF8271d1AFB5d2Bf18329
OLD_BRIDGE=0xCF8923ebF4244cedC647936a0281dd10bDFCBF18
NEW_BRIDGE=0xFd626a860F4d4bDB94aF200c218ce62c9647c8b2
OLD_ISSUING=0x8738A64392b71617aF4C685d0E827855c741fDF7

SALT=0x0000000000000000000000000000000000000000000000000000000000000035
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

amount=63623373485905886516839027

targets=[$OLD_BRIDGE,$WCRAB,$NEW_BRIDGE,$OLD_ISSUING]
values=[0,0,$amount,0]
data1=$(seth calldata "rescueFunds(address,address,uint256)" $WCRAB $TIMELOCK $amount)
data2=$(seth calldata "withdraw(uint256)" $amount) 
data3=0x
data4=$(seth calldata "transferMappingTokenOwnership(address,address)" 0x273131F7CB50ac002BDd08cA721988731F7e1092 0xf16D9682C4883415aee6e78EB0D6ae3507014e42)

datas=[$data1,$data2,$data3,$data4]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data --chain crab
# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
# count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain crab
