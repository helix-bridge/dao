#!/usr/bin/env bash

set -eox pipefail

ETH_FROM=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
WRING=0xE7578598Aac020abFB918f33A20faD5B71d670b4
BACKING=0xD1B10B114f1975d8BCc6cb6FC43519160e2AA978

wad=$(seth --to-wei 200000000 ether)

data=$(seth calldata "transfer(address,uint256)" $BACKING $wad)

# seth call -F $WALLET $WRING $data --chain darwinia

# 1. send 200M wring to backing
seth send -F $ETH_FROM $WALLET "submitTransaction(address,uint,bytes)" $WRING 0 $data --chain darwinia

count=$(seth call -F $ETH_FROM $WALLET "transactionCount()(uint)" --chain darwinia)
seth call -F $ETH_FROM $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain darwinia
