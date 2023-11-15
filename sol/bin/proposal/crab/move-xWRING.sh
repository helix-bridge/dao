#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
xWRING=0x273131f7cb50ac002bdd08ca721988731f7e1092
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e


data=$(seth calldata "transfer(address,uint)" $TIMELOCK 27433616720449600000000000)

seth call -F $WALLET $xWRING $data
seth send $WALLET "submitTransaction(address,uint,bytes)" $xWRING 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))
