#!/usr/bin/env bash

set -eox pipefail

ETH_FROM=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
xWCRAB=0x656567eb75b765fc320783cc6edd86bd854b2305
LNBRIDGE=0xF3c1444CD449bD66Ef6DA7CA6c3E7884840A3995

SALT=0x0000000000000000000000000000000000000000000000000000000000000036
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

wad=$(seth --to-wei 3874220.445832665103537196 ether)
fee=$(seth --to-wei 160 ether)
targets=[$xWCRAB,$LNBRIDGE]
values=[0,$fee]
data1=$(seth calldata "approve(address,uint)" $LNBRIDGE $wad)
data2=$(seth calldata "lockAndRemoteIssuing(uint32,uint256,address,address,uint256)" 6501 2000000 $xWCRAB 0x2401224012bAE7C2f217392665CA7abC16dCDE1e $wad)
datas=[$data1,$data2]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data

# seth send -F $ETH_FROM $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain darwinia
# count=$(seth call $WALLET "transactionCount()(uint)" --chain darwinia)
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain darwinia

# op_hash=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)(bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain darwinia)
# date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash --chain darwinia) '+%Y-%m-%d %H:%M:%S'

seth send -F $ETH_FROM $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia
