#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
BACKING=0xD1B10B114f1975d8BCc6cb6FC43519160e2AA978
KTON=0x0000000000000000000000000000000000000402

wad=38851683489530000000000

data=$(seth calldata "transfer(address,uint256)" $BACKING $wad)

seth call -F $WALLET $KTON $data

# 1. send 38851.68348953 kton to backing
seth send $WALLET "submitTransaction(address,uint,bytes)" $KTON 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))
