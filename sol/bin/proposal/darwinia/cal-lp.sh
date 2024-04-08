#!/usr/bin/env bash

set -eo pipefail


WRING=0xe7578598aac020abfb918f33a20fad5b71d670b4
xWCRAB=0x656567eb75b765fc320783cc6edd86bd854b2305
PAIR=0xdfa67e8f80f6b37a21e5c3baeeb0eca893652ab9
TIMELOCK=0xa546f42beb3dea617b0f3ca6995c7df5dfcad29d

cal() {
  python3 -c "print(int($1))"
}

LP=$(seth call $PAIR "balanceOf(address)(uint)" $TIMELOCK --chain darwinia)
TOTALSUPPLY0=$(seth call $PAIR "totalSupply()(uint)" --chain darwinia)
BALANCE0=$(seth call $WRING "balanceOf(address)(uint)" $PAIR --chain darwinia)
BALANCE1=$(seth call $xWCRAB "balanceOf(address)(uint)" $PAIR --chain darwinia)
WRING_AMOUT_OUT=$(cal "$LP * $BALANCE0 / $TOTALSUPPLY0 * 0.997")
xWCRAB_AMOUT_OUT=$(cal "$LP * $BALANCE1 / $TOTALSUPPLY0 * 0.997")
echo "WRING:  $(seth --to-fix 18 $WRING_AMOUT_OUT)"
echo "xWCRAB: $(seth --to-fix 18 $xWCRAB_AMOUT_OUT)"
