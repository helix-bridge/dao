#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
xRING=0x7399Ea6C9d35124d893B8d9808930e9d3F211501
xRINGMigrator=0xdED6Edd731f5F59fEB2555Ec3f1b6C085Dc6E42E

data1=$(seth calldata "approve(address,uint)" $xRINGMigrator 27433616720449600)
data2=$(seth calldata "migrateAll()")

# seth call -F $WALLET $xRING $data1
# seth call -F $WALLET $xRINGMigrator $data2

seth send $WALLET "submitTransaction(address,uint,bytes)" $xRING 0 $data1
seth send $WALLET "submitTransaction(address,uint,bytes)" $xRINGMigrator 0 $data2

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))
