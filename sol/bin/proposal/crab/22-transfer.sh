#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
xWRING=0x273131F7CB50ac002BDd08cA721988731F7e1092
WCRAB=0x2D2b97EA380b0185e9fDF8271d1AFB5d2Bf18329
TREASURY=0x6D6f646c64612f74727372790000000000000000

SALT=0x0000000000000000000000000000000000000000000000000000000000000032
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

wad1=$(seth --to-wei 116071744.692448891745222351 ether)
wad2=$(seth --to-wei 22547187.460827013241845523 ether)
wad3=$(seth --to-wei 136709456.741191076045095525 ether)
targets=[$WCRAB,$xWRING,$TREASURY]
values=[0,0,$wad3]
data1=$(seth calldata "withdraw(uint)" "$wad1")
data2=$(seth calldata "transfer(address,uint)" "$TREASURY" "$wad2")
data3=0x
datas=[$data1,$data2,$data3]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

seth call -F $WALLET $TIMELOCK $data --chain crab
seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

# op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain crab)
# date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain crab) '+%Y-%m-%d %H:%M:%S'

# seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain crab
