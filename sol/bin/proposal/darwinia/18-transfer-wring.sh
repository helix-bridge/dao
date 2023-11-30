#!/usr/bin/env bash

set -eox pipefail

WALLET=0x394607dAC12ea8C6095D68E3a7E5c9d40Cf0df9A
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
WRING=0xE7578598Aac020abFB918f33A20faD5B71d670b4

wad=$(seth --to-wei 1000000 ether)

# data=$(seth calldata "transfer(address,uint256)" $TIMELOCK $wad)

data=$(seth calldata "deposit()")

# seth call -F $WALLET $WRING $data --chain darwinia

# 1. wrap 1M wring
# 2. send 1M wring to timelock
seth send $WALLET "submitTransaction(address,uint,bytes)" $WRING $wad $data --chain darwinia

count=$(seth call $WALLET "transactionCount()(uint)" --chain darwinia)
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain darwinia
