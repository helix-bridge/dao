#!/usr/bin/env bash

set -eox pipefail

USDT=0x6a2d262D56735DbA19Dd70682B39F6bE9a931D98
USDC=0x81ECac0D6Be0550A00FF064a4f9dd2400585FE9c
xRING=0x7399Ea6C9d35124d893B8d9808930e9d3F211501
ROUTER=0xaf5caa87a7d3718622604268c43ff6ce9d2cec3c
WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
SALT=0x000000000000000000000000000000000000000000000000000000000000000b
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

data1=$(seth calldata "approve(address,uint)" $ROUTER 50000000000)
data2=$(seth calldata "approve(address,uint)" $ROUTER 2689963906064308)
data3=$(seth calldata "addLiquidity(address,address,uint,uint,uint,uint,address,uint)" $USDT $xRING 50000000000 2689963906064308 47500000000 2555465710761092 $TIMELOCK 1649865176)

data4=$(seth calldata "approve(address,uint)" $ROUTER 50000000000)
# value 16163549256800000000000000
data5=$(seth calldata "addLiquidityETH(address,uint,uint,uint,address,uint)" $USDC 50000000000 47500000000 15355371793960000000000000 $TIMELOCK 1649865176)

targets=[$USDT,$xRING,$ROUTER,$USDC,$ROUTER]
values=[0,0,0,0,16163549256800000000000000]
datas=[$data1,$data2,$data3,$data4,$data5]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data

seth call $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT
