#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
WRING=0xE7578598Aac020abFB918f33A20faD5B71d670b4

value=982875623961051257000000000
data=$(seth calldata "deposit()")

seth call -F $WALLET -V $value $WRING $data

# 1. wrap all ring to wring
seth send $WALLET "submitTransaction(address,uint,bytes)" $WRING $value $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))
