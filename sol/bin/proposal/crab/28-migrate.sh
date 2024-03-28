#!/usr/bin/env bash

set -eox pipefail

OWNER=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x000000000f681D85374225EdEeADC25560C1fb3F

SALT=0x0000000000000000000000000000000000000000000000000000000000000002
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

targets=[0x601dE3B81c7cE04BecE3b29e5cEe4F3251d250dB,0xFd626a860F4d4bDB94aF200c218ce62c9647c8b2,0xFd626a860F4d4bDB94aF200c218ce62c9647c8b2,0xf16D9682C4883415aee6e78EB0D6ae3507014e42]
values=[0,0,0,0]
data1=$(seth calldata "upgrade(address,address)" 0xFd626a860F4d4bDB94aF200c218ce62c9647c8b2 0x40Cea5Ae9b0aa1a57279DED050379094E7b77C25)
data2=$(seth calldata "setwToken(address)" 0x2D2b97EA380b0185e9fDF8271d1AFB5d2Bf18329)
data3=$(seth calldata "migrate(address)" 0xa64D1c284280b22f921E7B2A55040C7bbfD4d9d0)
data4=$(seth calldata "transferxTokenOwnership(address,address)" 0x273131F7CB50ac002BDd08cA721988731F7e1092 0xf6372ab2d35B32156A19F2d2F23FA6dDeFBE58bd)

datas=[$data1,$data2,$data3,$data4]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

id=$(seth call $TIMELOCK "hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain crab)

# seth call -F $WALLET $TIMELOCK $data --chain crab
# seth send -F $OWNER $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
# count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
# seth call -F $OWNER $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain crab
