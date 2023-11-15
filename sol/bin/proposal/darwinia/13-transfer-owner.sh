#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
BridgeProxyAdmin=0x86c85A9bf5DEAfdeD40D9C70517883D95F50Df03
MESSAGE_DAO=0xB29DA7C1b1514AB342afbE6AB915252Ad3f87E4d

data=$(seth calldata "transferOwnership(address)" $MESSAGE_DAO)
# helix-dao-multisig transfer BridgeProxyAdmin's ownership to message-dao
seth send $WALLET "submitTransaction(address,uint,bytes)" $BridgeProxyAdmin 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))
