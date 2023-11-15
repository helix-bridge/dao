#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
Erc20Sub2SubBackingProxy=0xF3c1444CD449bD66Ef6DA7CA6c3E7884840A3995
Erc20Sub2SubMappingTokenFactoryProxy=0x8c585F9791EE5b4B23fe82888cE576DBB69607eB
ENDPOINT=0xD1564Ff64445394380D38C77AF0907221b0d38d9


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

# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data
# count=$(seth call $WALLET "transactionCount()(uint)")
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia
