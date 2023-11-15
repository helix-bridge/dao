#!/usr/bin/env bash

set -eo pipefail

chain=$1
addr=0x00B1dBC6167Ba640A7Dd0fF578ED2657eB53b961
salt=0x7694ea1f493b7903c7f4284747cd12fceebc894225cfced65c14c02bac91aea1
c3=0x0000000000C76fe1798a428F60b27c6724e03408
deployer=0x0f14341A7f464320319025540E8Fe48Ad0fe5aec

alex=0x00E3993566b34e5367d1C602439997BD08c11FF7
denny=0x52386BE2397e8EAc26298F733b390684203fB580
echo=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
ranji=0xe59261f6D4088BcD69985A3D369Ff14cC54EF1E5
xiaoch=0xd2c7008400F54aA70Af01CF8C747a4473246593E

bytecode=$(jq -r ".contracts[\"src/solc_0.8/TimeLock.f.sol\"].TimeLock.evm.bytecode.object" out/dapp.sol.json)
args=$(ethabi encode params -v uint256 1800 \
  -v address[] "[Bd1a110ec476b4775c43905000288881367B1a88]" \
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
