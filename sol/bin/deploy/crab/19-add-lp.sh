#!/usr/bin/env bash

set -eox pipefail

WCRAB=0x2d2b97ea380b0185e9fdf8271d1afb5d2bf18329
xWRING=0x273131f7cb50ac002bdd08ca721988731f7e1092
ROUTER=0xaf5caa87a7d3718622604268c43ff6ce9d2cec3c
WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
SWAPTOPRICE=0xED9Bf1C1B000C41b60206F997Cbe05d4E362e84D
SALT=0x0000000000000000000000000000000000000000000000000000000000000031
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

DEADLINE=1696783073
wad=$(seth --to-wei 50000 ether)
amount1Desired=$(seth --to-wei  4000000 ether)
amount2Desired=$(seth --to-wei 40000000 ether)
amount1Min=$(seth --to-wei  3500000 ether)
amount2Min=$(seth --to-wei 35000000 ether)

data1=$(seth calldata "approve(address,uint)" $SWAPTOPRICE $wad)
data2=$(seth calldata "approve(address,uint)" $ROUTER $amount1Desired)
data3=$(seth calldata "approve(address,uint)" $ROUTER $amount2Desired)
data4=$(seth calldata "swapToPrice(address,address,uint256,uint256,uint256,uint256,address,uint256)" $xWRING $WCRAB 1 10 $wad 0 $TIMELOCK $DEADLINE)
data5=$(seth calldata "addLiquidity(address,address,uint,uint,uint,uint,address,uint)" $xWRING $WCRAB $amount1Desired $amount2Desired $amount1Min $amount2Min $TIMELOCK $DEADLINE)
data6=$(seth calldata "approve(address,uint)" $SWAPTOPRICE 0)

targets=[$xWRING,$xWRING,$WCRAB,$SWAPTOPRICE,$ROUTER,$xWRING]
values=[0,0,0,0,0,0]
datas=[$data1,$data2,$data3,$data4,$data5,$data6]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

# seth call $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain crab
