#!/usr/bin/env bash

set -eox pipefail

TIMELOCK=0x000000000f681D85374225EdEeADC25560C1fb3F

SALT=0x0000000000000000000000000000000000000000000000000000000000000002
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

targets=[0x601dE3B81c7cE04BecE3b29e5cEe4F3251d250dB,0xFd626a860F4d4bDB94aF200c218ce62c9647c8b2,0xFd626a860F4d4bDB94aF200c218ce62c9647c8b2,0xf16D9682C4883415aee6e78EB0D6ae3507014e42]
values=[0,0,0,0]
data1=$(seth calldata "upgrade(address,address)" 0xFd626a860F4d4bDB94aF200c218ce62c9647c8b2 0x40Cea5Ae9b0aa1a57279DED050379094E7b77C25)
data2=$(seth calldata "setwToken(address)" 0xE7578598Aac020abFB918f33A20faD5B71d670b4)
data3=$(seth calldata "migrate(address)" 0xa64D1c284280b22f921E7B2A55040C7bbfD4d9d0)
data4=$(seth calldata "transferxTokenOwnership(address,address)" 0x656567Eb75b765FC320783cc6EDd86bD854b2305 0xf6372ab2d35B32156A19F2d2F23FA6dDeFBE58bd)

datas=[$data1,$data2,$data3,$data4]

seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY

id=$(seth call $TIMELOCK "hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia)

seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia
