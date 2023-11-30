#!/usr/bin/env bash

set -eox pipefail

ETH_FROM=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
WRING=0xE7578598Aac020abFB918f33A20faD5B71d670b4
LNBRIDGE=0xF3c1444CD449bD66Ef6DA7CA6c3E7884840A3995

SALT=0x0000000000000000000000000000000000000000000000000000000000000035
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

wad=$(seth --to-wei 177225.031173004421091494 ether)
amount=$(seth --to-wei 176905.031173004421091494 ether)
fee=$(seth --to-wei 160 ether)
targets=[$WRING,$LNBRIDGE]
values=[0,$fee]
data1=$(seth calldata "withdraw(uint)" $wad)
data2=$(seth calldata "lockAndRemoteIssuingNative(uint32,uint256,address,uint256)" 6501 2000000 0x2401224012bAE7C2f217392665CA7abC16dCDE1e $amount)
datas=[$data1,$data2]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

seth call -F $WALLET $TIMELOCK $data

seth send -F $ETH_FROM $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain darwinia
count=$(seth call $WALLET "transactionCount()(uint)" --chain darwinia)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain darwinia

# op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain darwinia)
# date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain darwinia) '+%Y-%m-%d %H:%M:%S'

# seth send -F $ETH_FROM $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia
