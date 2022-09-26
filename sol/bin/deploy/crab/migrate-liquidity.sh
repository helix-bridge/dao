#!/usr/bin/env bash

set -eox pipefail

USDT=0x6a2d262D56735DbA19Dd70682B39F6bE9a931D98
xRING=0x7399Ea6C9d35124d893B8d9808930e9d3F211501
USDT_xRING_PAIR=0x29BF15B31029c889E397Cc153C1F225e200581BC
WCRAB=0x2d2b97ea380b0185e9fdf8271d1afb5d2bf18329
WCRAB_xRING_PAIR=0xF157c9393255Db1728bC6483c3545Ca8a1655a0F
xRING_MIGRATOR=0xdED6Edd731f5F59fEB2555Ec3f1b6C085Dc6E42E
xWRING=0x273131f7cb50ac002bdd08ca721988731f7e1092
ROUTER=0xaf5caa87a7d3718622604268c43ff6ce9d2cec3c
WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
SALT=0x000000000000000000000000000000000000000000000000000000000000000e
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800
DEADLINE=1664359200

cal() {
  python3 -c "print(int($1))"
}

xRING_BL0=$(seth call $xRING "balanceOf(address)(uint)" $TIMELOCK)

USDT_xRING_LIQ=$(seth call $USDT_xRING_PAIR "balanceOf(address)(uint)" $TIMELOCK)
TOTALSUPPLY0=$(seth call $USDT_xRING_PAIR "totalSupply()(uint)")
BALANCE0=$(seth call $USDT "balanceOf(address)(uint)" $USDT_xRING_PAIR)
BALANCE1=$(seth call $xRING "balanceOf(address)(uint)" $USDT_xRING_PAIR)
USDT_AMOUT_OUT=$(cal "$USDT_xRING_LIQ * $BALANCE0 / $TOTALSUPPLY0 * 0.99")
xRING_AMOUT_OUT0=$(cal "$USDT_xRING_LIQ * $BALANCE1 / $TOTALSUPPLY0 * 0.99")

data1=$(seth calldata "approve(address,uint)" $ROUTER $USDT_xRING_LIQ)
data2=$(seth calldata "removeLiquidity(address,address,uint,uint,uint,address,uint)" $USDT $xRING $USDT_xRING_LIQ $USDT_AMOUT_OUT $xRING_AMOUT_OUT0 $TIMELOCK $DEADLINE)

WCRAB_xRING_LIQ=$(seth call $WCRAB_xRING_PAIR "balanceOf(address)(uint)" $TIMELOCK)
TOTALSUPPLY1=$(seth call $WCRAB_xRING_PAIR "totalSupply()(uint)")
BALANCE0=$(seth call $WCRAB "balanceOf(address)(uint)" $WCRAB_xRING_PAIR)
BALANCE1=$(seth call $xRING "balanceOf(address)(uint)" $WCRAB_xRING_PAIR)
WCRAB_AMOUT_OUT=$(cal "$WCRAB_xRING_LIQ * $BALANCE0 / $TOTALSUPPLY1 * 0.99")
xRING_AMOUT_OUT1=$(cal "$WCRAB_xRING_LIQ * $BALANCE1 / $TOTALSUPPLY1 * 0.99")

data3=$(seth calldata "approve(address,uint)" $ROUTER $WCRAB_xRING_PAIR)
data4=$(seth calldata "removeLiquidity(address,address,uint,uint,uint,address,uint)" $WCRAB $xRING $WCRAB_xRING_PAIR $WCRAB_AMOUT_OUT $xRING_AMOUT_OUT1  $TIMELOCK $DEADLINE)

MigrateBL=$(cal "($xRING_BL0 + $xRING_AMOUT_OUT0 + $xRING_AMOUT_OUT1) * 1.1")

data5=$(seth calldata "approve(address,uint)" $xRING_MIGRATOR $MigrateBL)
data6=$(seth calldata "migrateAll()")

data7=$(seth calldata "approve(address,uint)" $ROUTER $USDT_AMOUT_OUT)
data8=$(seth calldata "approve(address,uint)" $ROUTER $(($xRING_AMOUT_OUT0 + $xRING_AMOUT_OUT1)))
data9=$(seth calldata "approve(address,uint)" $ROUTER $WCRAB_AMOUT_OUT)

USDT_MIN=$(cal "$USDT_AMOUT_OUT * 0.99")
xWRING_MIN0=$(cal "$xRING_AMOUT_OUT0 * 0.99")
data10=$(seth calldata "addLiquidity(address,address,uint,uint,uint,uint,address,uint)" $USDT $xWRING $USDT_AMOUT_OUT $xRING_AMOUT_OUT0 $USDT_MIN $xWRING_MIN0 $TIMELOCK $DEADLINE)

WCRAB_MIN=$(cal "$WCRAB_AMOUT_OUT *0.99")
xWRING_MIN1=$(cal "$xRING_AMOUT_OUT1 *0.99")
data11=$(seth calldata "addLiquidity(address,address,uint,uint,uint,uint,address,uint)" $WCRAB $xWRING $WCRAB_AMOUT_OUT $xRING_AMOUT_OUT1 $WCRAB_MIN $xWRING_MIN1 $TIMELOCK $DEADLINE)

targets=[$USDT_xRING_PAIR,$ROUTER,$WCRAB_xRING_PAIR,$ROUTER,$xRING,$xRING_MIGRATOR,$USDT,$xWRING,$WCRAB,$ROUTER,$ROUTER]
values=[0,0,0,0,0,0,0,0,0,0,0]
datas=[$data1,$data2,$data3,$data4,$data5,$data6,$data7,$data8,$data9,$data10,$data11]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

seth call -F $WALLET $TIMELOCK $data

seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

# seth call $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT
