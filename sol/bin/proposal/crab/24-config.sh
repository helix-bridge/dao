#!/usr/bin/env bash

set -eox pipefail

WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x000000000f681d85374225edeeadc25560c1fb3f
LN=0x94c614daefdbf151e1bb53d6a201ae5ff56a9337

SALT=0x0000000000000000000000000000000000000000000000000000000000000034
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

targets=[$LN,$LN]
values=[0,0]
data1=$(seth calldata "setReceiveService(uint,address,address)" 46 0x94c614daefdbf151e1bb53d6a201ae5ff56a9337 0x65Be094765731F394bc6d9DF53bDF3376F1Fc8B0)
data2=$(seth calldata "setSendService(uint,address,address)" 46 0x94c614daefdbf151e1bb53d6a201ae5ff56a9337 0x65Be094765731F394bc6d9DF53bDF3376F1Fc8B0)

datas=[$data1,$data2]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data --chain crab
# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data --chain crab
# count=$(seth call $WALLET "transactionCount()(uint)" --chain crab)
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 )) --chain crab

seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain crab
