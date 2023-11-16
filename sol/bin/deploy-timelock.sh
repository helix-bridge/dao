#!/usr/bin/env bash

set -eo pipefail

safe=${1:?}
chain=${2:?}
addr=0x000000000f681D85374225EdEeADC25560C1fb3F
salt=0xa4390aa887eb9fe9ed94283ed393bbfeb0b301e904fac48ba5d3c176f41851c1
c3=0x0000000000C76fe1798a428F60b27c6724e03408
deployer=0x0f14341A7f464320319025540E8Fe48Ad0fe5aec

alex=0x00E3993566b34e5367d1C602439997BD08c11FF7
denny=0x52386BE2397e8EAc26298F733b390684203fB580
ranji=0xe59261f6D4088BcD69985A3D369Ff14cC54EF1E5
xiaoch=0xd2c7008400F54aA70Af01CF8C747a4473246593E

bytecode=$(jq -r ".contracts[\"src/solc_0.8/TimeLock.f.sol\"].TimeLock.evm.bytecode.object" out/dapp.sol.json)
args=$(ethabi encode params -v uint256 1800 \
  -v address[] "[${safe:2},${deployer:2}]" \
  -v address[] "[00E3993566b34e5367d1C602439997BD08c11FF7,52386BE2397e8EAc26298F733b390684203fB580,7aE77149ed38c5dD313e9069d790Ce7085caf0A6,e59261f6D4088BcD69985A3D369Ff14cC54EF1E5,d2c7008400F54aA70Af01CF8C747a4473246593E]"
)
creationCode=0x$bytecode$args
# salt, creationCode
expect_addr=$(seth call $c3 "deploy(bytes32,bytes)(address)" $salt $creationCode --chain $chain)

set -x

if [[ $(seth --to-checksum-address "${addr}") == $(seth --to-checksum-address "${expect_addr}") ]]; then
  seth send $c3 "deploy(bytes32,bytes)" $salt $creationCode --chain $chain
else
  echo "Unexpected address."
fi
