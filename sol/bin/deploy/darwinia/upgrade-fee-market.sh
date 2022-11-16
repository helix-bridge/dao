#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
BridgeProxyAdmin=0x86c85A9bf5DEAfdeD40D9C70517883D95F50Df03
FeeMarketProxy=0xcA927Df15afb7629b79dA4713a871190315c7409
NewFeeMarket=0xD1182CddB414FfC00eF2Ce18299A1E12e1a552BE

data=$(seth calldata "upgrade(address,address)" "$FeeMarketProxy" "$NewFeeMarket")

seth call -F $WALLET $BridgeProxyAdmin $data

seth send $WALLET "submitTransaction(address,uint,bytes)" $BridgeProxyAdmin 0 $data
count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))
