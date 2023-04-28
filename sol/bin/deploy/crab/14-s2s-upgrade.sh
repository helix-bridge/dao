#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
Erc20Sub2SubBackingProxy=0xCF8923ebF4244cedC647936a0281dd10bDFCBF18
Erc20Sub2SubMappingTokenFactoryProxy=0x8738A64392b71617aF4C685d0E827855c741fDF7
ENDPOINT=0x3D400e863F4c8a194b869F2aA7f0C9a2194Ad5Ce


SALT=0x000000000000000000000000000000000000000000000000000000000000000e
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

targets=[$Erc20Sub2SubBackingProxy,$Erc20Sub2SubMappingTokenFactoryProxy]
values=[0,0]
data1=$(seth calldata "setMessageEndpoint(address)" $ENDPOINT)
data2=$(seth calldata "setMessageEndpoint(address)" $ENDPOINT)
datas=[$data1,$data2]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data

seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data
count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

# seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT
