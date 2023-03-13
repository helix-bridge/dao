#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
LNBRIDGE=0x3B9E571AdeCB0c277486036D6097E9C2CCcfa9d9

wad=$(seth --to-wei 2000000 ether)

# dao2 transfer 2M RING to LN bridge
seth send $WALLET "submitTransaction(address,uint,bytes)" $LNBRIDGE $wad 0x

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))
