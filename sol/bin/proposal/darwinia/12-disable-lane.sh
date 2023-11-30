#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
FEEMARKET=0xcA927Df15afb7629b79dA4713a871190315c7409
LANE=0xcA3749C8C3aF04278D596a3fBe461481B6aa1b01

data=$(seth calldata "setOutbound(address,uint256)" $LANE 0)
# dao disbale outboundlane
seth send $WALLET "submitTransaction(address,uint,bytes)" $FEEMARKET 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))
