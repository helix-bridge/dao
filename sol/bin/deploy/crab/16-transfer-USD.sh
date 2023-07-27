#!/usr/bin/env bash

set -eox pipefail

USDT=0x6a2d262D56735DbA19Dd70682B39F6bE9a931D98
USDC=0x81ECac0D6Be0550A00FF064a4f9dd2400585FE9c
WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
ECHO=0x7aE77149ed38c5dD313e9069d790Ce7085caf0A6
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
SALT=0x0000000000000000000000000000000000000000000000000000000000000010
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

BALANCE1=$(seth call $USDT "balanceOf(address)(uint)" $TIMELOCK)
BALANCE2=$(seth call $USDC "balanceOf(address)(uint)" $TIMELOCK)

# transfer USDT to ECHO
data1=$(seth calldata "transfer(address,uint)" "$ECHO" "$BALANCE1")
# transfer USDC to ECHO
data2=$(seth calldata "transfer(address,uint)" "$ECHO" "$BALANCE2")

targets=[$USDT,$USDC]
values=[0,0]
datas=[$data1,$data2]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data

# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data
# count=$(seth call $WALLET "transactionCount()(uint)")
# seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT
