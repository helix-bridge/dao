#!/usr/bin/env bash

set -eox pipefail

TIMELOCK=0xa546f42beb3dea617b0f3ca6995c7df5dfcad29d
WRING=0xe7578598aac020abfb918f33a20fad5b71d670b4
KTON=0x0000000000000000000000000000000000000402
OLD_BACKING=0xD1B10B114f1975d8BCc6cb6FC43519160e2AA978
NEW_BACKING=0x2B496f19A420C02490dB859fefeCCD71eDc2c046

SALT=0x0000000000000000000000000000000000000000000000000000000000000003
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

balance_wring=$(seth call $WRING "balanceOf(address)(uint)" $OLD_BACKING --chain darwinia)
balance_kton=$( seth call $KTON  "balanceOf(address)(uint)" $OLD_BACKING --chain darwinia)

targets=[$OLD_BACKING,$OLD_BACKING]
values=[0,0]
data1=$(seth calldata "rescueFunds(address,address,uint256)" $WRING $NEW_BACKING $balance_wring)
data2=$(seth calldata "rescueFunds(address,address,uint256)" $KTON  $NEW_BACKING $balance_kton)

datas=[$data1,$data2]

seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY

id=$(seth call $TIMELOCK "hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia)
date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $id --chain darwinia) '+%Y-%m-%d %H:%M:%S'

# seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia
