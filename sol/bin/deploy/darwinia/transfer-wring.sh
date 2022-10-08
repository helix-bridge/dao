#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
BACKING=0xD1B10B114f1975d8BCc6cb6FC43519160e2AA978
WRING=0xE7578598Aac020abFB918f33A20faD5B71d670b4

wad=$(seth --to-wei 20000000 ether)

data=$(seth calldata "transfer(address,uint256)" $BACKING $wad)

seth call -F $WALLET $WRING $data

# 1. send 2kw wring to backing
seth send $WALLET "submitTransaction(address,uint,bytes)" $WRING 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))
