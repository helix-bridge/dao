#!/usr/bin/env bash

set -eox pipefail

ETH_FROM=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
WRING=0xE7578598Aac020abFB918f33A20faD5B71d670b4

OLD_BRIDGE=0xF3c1444CD449bD66Ef6DA7CA6c3E7884840A3995
NEW_BRIDGE=0xFd626a860F4d4bDB94aF200c218ce62c9647c8b2
OLD_ISSUING=0x8c585F9791EE5b4B23fe82888cE576DBB69607eB

SALT=0x0000000000000000000000000000000000000000000000000000000000000037
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

amount=31031939912263749723848643

targets=[$OLD_BRIDGE,$WRING,$NEW_BRIDGE,$OLD_ISSUING]
values=[0,0,$amount,0]
data1=$(seth calldata "rescueFunds(address,address,uint256)" $WRING $TIMELOCK $amount)
data2=$(seth calldata "withdraw(uint256)" $amount) 
data3=0x
data4=$(seth calldata "transferMappingTokenOwnership(address,address)" 0x656567Eb75b765FC320783cc6EDd86bD854b2305 0xf16D9682C4883415aee6e78EB0D6ae3507014e42)

datas=[$data1,$data2,$data3,$data4]
data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

seth call -F $WALLET $TIMELOCK $data --chain darwinia

seth send -F $ETH_FROM $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain darwinia
count=$(seth call $WALLET "transactionCount()(uint)" --chain darwinia)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain darwinia

# op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain darwinia)
# date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain darwinia) '+%Y-%m-%d %H:%M:%S'

# seth send -F $ETH_FROM $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia
