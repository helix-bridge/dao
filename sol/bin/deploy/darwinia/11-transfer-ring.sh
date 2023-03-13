#!/usr/bin/env bash

set -eox pipefail

WALLET=0x394607dAC12ea8C6095D68E3a7E5c9d40Cf0df9A
LNBRIDGE=0x3B9E571AdeCB0c277486036D6097E9C2CCcfa9d9

wad=$(seth --to-wei 2000000 ether)

# dao2 transfer 2M RING to LN bridge
seth send $WALLET "submitTransaction(address,uint,bytes)" $LNBRIDGE $wad 0x

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))
