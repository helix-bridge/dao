#!/usr/bin/env bash

set -eox pipefail

xWCRAB=0x656567eb75b765fc320783cc6edd86bd854b2305
WRING=0xe7578598aac020abfb918f33a20fad5b71d670b4
ROUTER=0xB899409cDA0fFA2bF87F9c7b31f3c77D6A3a0bB0
WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
SWAPTOPRICE=0x4E319297F77EA8099520F3467a083050090a1d81
SALT=0x0000000000000000000000000000000000000000000000000000000000000030
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

DEADLINE=1696783073
wad=$(seth --to-wei 50000 ether)
amount1Desired=$(seth --to-wei  4000000 ether)
amount2Desired=$(seth --to-wei 40000000 ether)
amount1Min=0
amount2Min=0

data1=$(seth calldata "approve(address,uint)" $SWAPTOPRICE $wad)
data2=$(seth calldata "approve(address,uint)" $ROUTER $amount1Min)
data3=$(seth calldata "approve(address,uint)" $ROUTER $amount2Min)
data4=$(seth calldata "swapToPrice(address,address,uint256,uint256,uint256,uint256,address,uint256)" $WRING $xWCRAB 1 10 0 $wad $TIMELOCK $DEADLINE)
data5=$(seth calldata "addLiquidity(address,address,uint,uint,uint,uint,address,uint)" $WRING $xWCRAB $amount1Desired $amount2Desired $amount1Min $amount2Min $TIMELOCK $DEADLINE)
data6=$(seth calldata "approve(address,uint)" $SWAPTOPRICE 0)

targets=[$xWCRAB,$WRING,$xWCRAB,$SWAPTOPRICE,$ROUTER,$xWCRAB]
values=[0,0,0,0,0,0]
datas=[$data1,$data2,$data3,$data4,$data5,$data6]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain darwinia
count=$(seth call $WALLET "transactionCount()(uint)" --chain darwinia)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain darwinia

# seth call $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia
