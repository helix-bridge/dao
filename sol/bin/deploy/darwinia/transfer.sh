#!/usr/bin/env bash

set -eox pipefail

WALLET=0xBd1a110ec476b4775c43905000288881367B1a88
TIMELOCK=0xA546F42BeB3DEa617b0F3Ca6995C7Df5dfCaD29D
WRING=0xE7578598Aac020abFB918f33A20faD5B71d670b4
Erc20Sub2SubBacking=0xF3c1444CD449bD66Ef6DA7CA6c3E7884840A3995
XRINGMigrator=0xdED6Edd731f5F59fEB2555Ec3f1b6C085Dc6E42E

SALT=0x0000000000000000000000000000000000000000000000000000000000000001
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

value1=$(seth --to-wei 0.1 ether)
data1=$(seth calldata "deposit()")
data2=$(seth calldata "approve(address,uint)" $Erc20Sub2SubBacking $value1)
value3=$(seth --to-wei 40 ether)
data3=$(seth calldata "lockAndRemoteIssuing(uint32,uint256,address,address,uint256)" \
  1241 5000000 $WRING $XRINGMigrator $value1)

targets=[$WRING,$WRING,$Erc20Sub2SubBacking]
values=[$value1,0,$value3]
datas=[$data1,$data2,$data3]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

id1=$(seth call $TIMELOCK "hashOperation(address,uint256,bytes,bytes32,bytes32)" $WRING $value1 $data1 $PREDECESSOR $SALT)

seth call -F $WALLET $TIMELOCK $data

# 1. WALLET send 0.1 RING to TIMELOCK
# seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK $value1 $data1
# 2.1 TIMELOCK send deposit 0.1 RING
# 2.2 TIMELOCK send approve 0.1 RING to Erc20Sub2SubBacking
# 2.3 TIMELOCK send lock    0.1 RING to Erc20Sub2SubBacking

seth send $WALLET "submitTransaction(address,uint,bytes)" $TIMELOCK 0 $data

count=$(seth call $WALLET "transactionCount()(uint)")
seth call $WALLET "transactions(uint)(address,uint,bytes,bool)" $(( $count - 1 ))

# seth call $TIMELOCK "execute(address,uint256,bytes,bytes32,bytes32)" $target $value1 $data1 $PREDECESSOR $SALT
