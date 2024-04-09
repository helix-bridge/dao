#!/usr/bin/env bash

set -eo pipefail

ETH_FROM=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
WRING=0xe7578598aac020abfb918f33a20fad5b71d670b4
xWCRAB=0x656567eb75b765fc320783cc6edd86bd854b2305
TREASURY=0x6d6f646c64612f74727372790000000000000000
 
BRIDGE0=0xa64d1c284280b22f921e7b2a55040c7bbfd4d9d0
BRIDGE1=0xf6372ab2d35b32156a19f2d2f23fa6ddefbe58bd

SALT=0x0000000000000000000000000000000000000000000000000000000000000025
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

BALANCE0=$(seth call $WRING "balanceOf(address)(uint)" $TIMELOCK --chain darwinia)
echo "WRING balance: $BALANCE0"
BALANCE1=$(seth call $xWCRAB "balanceOf(address)(uint)" $TIMELOCK --chain darwinia)
echo "xWCRAB balance: $BALANCE1"

targets=[$WRING,$xWCRAB,$BRIDGE0,$BRIDGE1]
values=[0,0,2000000000000000000,2000000000000000000]
data1=$(seth calldata "approve(address,uint)" $BRIDGE0 $BALANCE0)
data2=$(seth calldata "approve(address,uint)" $BRIDGE1 $BALANCE1)
data3=$(seth calldata "lockAndXIssue(uint,address,address,address,uint,uint,bytes,bytes)" 44 $WRING $TREASURY $ETH_FROM $BALANCE0 1712643486 0x 0x00000000000000000000000000000000000000000000000000000000000540d40000000000000000000000007ae77149ed38c5dd313e9069d790ce7085caf0a600000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000000)
data4=$(seth calldata "burnAndXUnlock(address,address,address,uint,uint,bytes,bytes)" $xWCRAB 0x004D0dE211BC148c3Ce696C51Cbc85BD421727E9 $ETH_FROM $BALANCE1 1712643825 $TREASURY 0x00000000000000000000000000000000000000000000000000000000000554470000000000000000000000007ae77149ed38c5dd313e9069d790ce7085caf0a600000000000000000000000000000000000000000000000000000000000000600000000000000000000000000000000000000000000000000000000000000000)

datas=[$data1,$data2,$data3,$data4]

echo ""
echo "scheduleBatch Parameters:"
echo "target: $targets"
echo "values: $values"
echo "datas:  $datas"
echo ""

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

echo "Helix Multisig execute target: $TIMELOCK"
echo "Helix Multisig execute data: $data"

seth call -F $WALLET $TIMELOCK $data --chain darwinia

seth send -F $ETH_FROM $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain darwinia
count=$(seth call $WALLET "transactionCount()(uint)" --chain darwinia)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain darwinia
