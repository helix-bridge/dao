#!/usr/bin/env bash

set -eox pipefail

TIMELOCK=0x000000000f681D85374225EdEeADC25560C1fb3F

SALT=0x0000000000000000000000000000000000000000000000000000000000000004
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

target=0x65be094765731f394bc6d9df53bdf3376f1fc8b0
value=0
data1=$(seth calldata 'transferOwnership(address)' 0xD0a0899c5dc2FEb253D57Ab0b7c6d1b1Fcbbf824)



data=$(seth calldata "schedule(address,uint256,bytes,bytes32,bytes32,uint256)" $target $value $data1 $PREDECESSOR $SALT $DELAY)

seth send $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value $data1 $PREDECESSOR $SALT --chain darwinia

