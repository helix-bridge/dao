#!/usr/bin/env bash

set -eox pipefail

ETH_FROM=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
WRING=0xE7578598Aac020abFB918f33A20faD5B71d670b4
BACKINGSAFEBOX=0x0000000000d2B6dB21aED6e5aCcbFeCbBA37b51C

wad=$(seth --to-wei 562875623.961051257 ether)

data=$(seth calldata "transfer(address,uint256)" $BACKINGSAFEBOX $wad)

seth call -F $WALLET $WRING $data --chain darwinia

# 1. send 562875623 wring to BACKINGSAFEBOX
seth send -F $ETH_FROM $WALLET "submitTransaction(address,uint,bytes)" $WRING 0 $data --chain darwinia

count=$(seth call -F $ETH_FROM $WALLET "transactionCount()(uint)" --chain darwinia)
seth call -F $ETH_FROM $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain darwinia
