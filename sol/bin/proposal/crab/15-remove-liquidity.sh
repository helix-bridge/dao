#!/usr/bin/env bash

set -eox pipefail

USDT=0x6a2d262D56735DbA19Dd70682B39F6bE9a931D98
USDC=0x81ECac0D6Be0550A00FF064a4f9dd2400585FE9c
xWRING=0x273131f7cb50ac002bdd08ca721988731f7e1092
WCRAB=0x2d2b97ea380b0185e9fdf8271d1afb5d2bf18329
USDT_xWRING_PAIR=0xb0fe91f4fa6ac7bedFB741FDf2d521766db4a925
USDC_CRAB_PAIR=0x75D4D59991D388Dd0e1c5224AF605a3E79e1f17e
ROUTER=0xaf5caa87a7d3718622604268c43ff6ce9d2cec3c
WALLET=0x0050F880c35c31c13BFd9cBb7D28AafaEcA3abd2
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
SALT=0x000000000000000000000000000000000000000000000000000000000000000f
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800
DEADLINE=1690425021

cal() {
  python3 -c "print(int($1))"
}

USDT_xWRING_LIQ=$(seth call $USDT_xWRING_PAIR "balanceOf(address)(uint)" $TIMELOCK)
TOTALSUPPLY0=$(seth call $USDT_xWRING_PAIR "totalSupply()(uint)")
BALANCE0=$(seth call $xWRING "balanceOf(address)(uint)" $USDT_xWRING_PAIR)
BALANCE1=$(seth call $USDT "balanceOf(address)(uint)" $USDT_xWRING_PAIR)
xRING_AMOUT_OUT0=$(cal "$USDT_xWRING_LIQ * $BALANCE0 / $TOTALSUPPLY0 * 0.99")
USDT_AMOUT_OUT=$(cal "$USDT_xWRING_LIQ * $BALANCE1 / $TOTALSUPPLY0 * 0.99")

data1=$(seth calldata "approve(address,uint)" $ROUTER $USDT_xWRING_LIQ)
data2=$(seth calldata "removeLiquidity(address,address,uint,uint,uint,address,uint)" $xWRING $USDT $USDT_xWRING_LIQ $xRING_AMOUT_OUT0 $USDT_AMOUT_OUT $TIMELOCK $DEADLINE)

USDC_CRAB_LIQ=$(seth call $USDC_CRAB_PAIR "balanceOf(address)(uint)" $TIMELOCK)
TOTALSUPPLY1=$(seth call $USDC_CRAB_PAIR "totalSupply()(uint)")
BALANCE0=$(seth call $WCRAB "balanceOf(address)(uint)" $USDC_CRAB_PAIR)
BALANCE1=$(seth call $USDC "balanceOf(address)(uint)" $USDC_CRAB_PAIR)
WCRAB_AMOUT_OUT=$(cal "$USDC_CRAB_LIQ * $BALANCE0 / $TOTALSUPPLY1 * 0.99")
USDC_AMOUT_OUT1=$(cal "$USDC_CRAB_LIQ * $BALANCE1 / $TOTALSUPPLY1 * 0.99")

data3=$(seth calldata "approve(address,uint)" $ROUTER $USDC_CRAB_LIQ)
data4=$(seth calldata "removeLiquidity(address,address,uint,uint,uint,address,uint)" $WCRAB $USDC $USDC_CRAB_LIQ $WCRAB_AMOUT_OUT $USDC_AMOUT_OUT1  $TIMELOCK $DEADLINE)

targets=[$USDT_xWRING_PAIR,$ROUTER,$USDC_CRAB_PAIR,$ROUTER]
values=[0,0,0,0]
datas=[$data1,$data2,$data3,$data4]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

# seth call -F $WALLET $TIMELOCK $data

seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

# seth call $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT
