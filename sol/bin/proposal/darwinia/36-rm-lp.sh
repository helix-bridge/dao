#!/usr/bin/env bash

set -eox pipefail

PAIR=0xdfA67E8f80F6B37a21e5C3baeEb0eCa893652Ab9
ROUTER=0xB899409cDA0fFA2bF87F9c7b31f3c77D6A3a0bB0
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
xWCRAB=0x656567eb75b765fc320783cc6edd86bd854b2305
WRING=0xe7578598aac020abfb918f33a20fad5b71d670b4
SALT=0x0000000000000000000000000000000000000000000000000000000000000024
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800
DEADLINE=1712682153

cal() {
  python3 -c "print(int($1))"
}

LP=$(seth call $PAIR "balanceOf(address)(uint)" $TIMELOCK --chain darwinia)
TOTALSUPPLY0=$(seth call $PAIR "totalSupply()(uint)" --chain darwinia)
BALANCE0=$(seth call $WRING "balanceOf(address)(uint)" $PAIR --chain darwinia)
BALANCE1=$(seth call $xWCRAB "balanceOf(address)(uint)" $PAIR --chain darwinia)
WRING_AMOUT_OUT=$(cal "$LP * $BALANCE0 / $TOTALSUPPLY0 * 0.99")
xWCRAB_AMOUT_OUT=$(cal "$LP * $BALANCE1 / $TOTALSUPPLY0 * 0.99")
echo "WRING:  $(seth --to-fix 18 $WRING_AMOUT_OUT)"
echo "xWCRAB: $(seth --to-fix 18 $xWCRAB_AMOUT_OUT)"

targets=[$PAIR,$ROUTER]
values=[0,0]
data1=$(seth calldata "approve(address,uint)" $ROUTER $LP)
data2=$(seth calldata "removeLiquidity(address,address,uint,uint,uint,address,uint)" $WRING $xWCRAB $LP $WRING_AMOUT_OUT $xWCRAB_AMOUT_OUT $TIMELOCK $DEADLINE)

datas=[$data1,$data2]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)


id=$(seth call $TIMELOCK "hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia)
date -r $(seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $id --chain darwinia) '+%Y-%m-%d %H:%M:%S'

# seth send -F $ETH_FROM $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT --chain darwinia

