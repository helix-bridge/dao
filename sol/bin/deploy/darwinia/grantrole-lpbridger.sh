#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
ENDPOINT=0x9C80EdD342b5D179c3a87946fC1F0963BfcaAa09

SALT=0x0000000000000000000000000000000000000000000000000000000000000005
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

targets=[$ENDPOINT,$ENDPOINT]
values=[0,0]
data1=$(seth calldata "grantRole(bytes32,address)" 0x90753073214e51464983fa1109c0af25023e18af68c70d66fc572842c1a4c667 0x84f7a56483C100ECb12CbB4A31b7873dAE0d8E9B)
data2=$(seth calldata "grantRole(bytes32,address)" 0x843c3a00fa95510a35f425371231fd3fe4642e719cb4595160763d6d02594b50 0x84f7a56483C100ECb12CbB4A31b7873dAE0d8E9B)
datas=[$data1,$data2]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

seth call -F $WALLET $TIMELOCK $data

seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data
count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

# seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT
