#!/usr/bin/env bash

set -eo pipefail

USDT=0x6a2d262D56735DbA19Dd70682B39F6bE9a931D98
USDC=0x81ECac0D6Be0550A00FF064a4f9dd2400585FE9c
xWRING=0x273131f7cb50ac002bdd08ca721988731f7e1092
WCRAB=0x2d2b97ea380b0185e9fdf8271d1afb5d2bf18329
USDT_xWRING_PAIR=0xb0fe91f4fa6ac7bedFB741FDf2d521766db4a925
USDC_CRAB_PAIR=0x75D4D59991D388Dd0e1c5224AF605a3E79e1f17e
TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e

cal() {
  python3 -c "print(int($1))"
}

USDT_xWRING_LIQ=$(seth call $USDT_xWRING_PAIR "balanceOf(address)(uint)" $TIMELOCK)
TOTALSUPPLY0=$(seth call $USDT_xWRING_PAIR "totalSupply()(uint)")
BALANCE0=$(seth call $USDT "balanceOf(address)(uint)" $USDT_xWRING_PAIR)
BALANCE1=$(seth call $xWRING "balanceOf(address)(uint)" $USDT_xWRING_PAIR)
USDT_AMOUT_OUT=$(cal "$USDT_xWRING_LIQ * $BALANCE0 / $TOTALSUPPLY0 * 0.99")
echo "USDT: $(seth --to-fix 6 $USDT_AMOUT_OUT)"

USDC_CRAB_LIQ=$(seth call $USDC_CRAB_PAIR "balanceOf(address)(uint)" $TIMELOCK)
TOTALSUPPLY0=$(seth call $USDC_CRAB_PAIR "totalSupply()(uint)")
BALANCE0=$(seth call $USDC "balanceOf(address)(uint)" $USDC_CRAB_PAIR)
BALANCE1=$(seth call $WCRAB "balanceOf(address)(uint)" $USDC_CRAB_PAIR)
USDC_AMOUT_OUT=$(cal "$USDC_CRAB_LIQ * $BALANCE0 / $TOTALSUPPLY0 * 0.99")
echo "USDC: $(seth --to-fix 6 $USDC_AMOUT_OUT)"
