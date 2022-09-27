#!/usr/bin/env bash

set -eo pipefail

TIMELOCK=0x2401224012bAE7C2f217392665CA7abC16dCDE1e
SALT=0x000000000000000000000000000000000000000000000000000000000000000e
PREDECESSOR=0x0000000000000000000000000000000000000000000000000000000000000000
DELAY=1800

targets=[0x29BF15B31029c889E397Cc153C1F225e200581BC,0xaf5caa87a7d3718622604268c43ff6ce9d2cec3c,0xF157c9393255Db1728bC6483c3545Ca8a1655a0F,0xaf5caa87a7d3718622604268c43ff6ce9d2cec3c,0x7399Ea6C9d35124d893B8d9808930e9d3F211501,0xdED6Edd731f5F59fEB2555Ec3f1b6C085Dc6E42E,0x6a2d262D56735DbA19Dd70682B39F6bE9a931D98,0x273131f7cb50ac002bdd08ca721988731f7e1092,0x2d2b97ea380b0185e9fdf8271d1afb5d2bf18329,0xaf5caa87a7d3718622604268c43ff6ce9d2cec3c,0xaf5caa87a7d3718622604268c43ff6ce9d2cec3c]
values=[0,0,0,0,0,0,0,0,0,0,0]
datas=[0x095ea7b3000000000000000000000000af5caa87a7d3718622604268c43ff6ce9d2cec3c00000000000000000000000000000000000000000000000000000a8c37131847,0xbaa2abde0000000000000000000000006a2d262d56735dba19dd70682b39f6be9a931d980000000000000000000000007399ea6c9d35124d893b8d9808930e9d3f21150100000000000000000000000000000000000000000000000000000a8c371318470000000000000000000000000000000000000000000000000000000679e568f20000000000000000000000000000000000000000000000000010ec1516b49c070000000000000000000000002401224012bae7c2f217392665ca7abc16dcde1e0000000000000000000000000000000000000000000000000000000063341b20,0x095ea7b3000000000000000000000000af5caa87a7d3718622604268c43ff6ce9d2cec3c000000000000000000000000000000000000000000000001fbf85dfb67dcb9d7,0xbaa2abde0000000000000000000000002d2b97ea380b0185e9fdf8271d1afb5d2bf183290000000000000000000000007399ea6c9d35124d893b8d9808930e9d3f211501000000000000000000000000000000000000000000000001fbf85dfb67dcb9d70000000000000000000000000000000000000000001aba9e26e75c3500000000000000000000000000000000000000000000000000000000000fc0f8c15e0ee90000000000000000000000002401224012bae7c2f217392665ca7abc16dcde1e0000000000000000000000000000000000000000000000000000000063341b20,0x095ea7b3000000000000000000000000ded6edd731f5f59feb2555ec3f1b6c085dc6e42e000000000000000000000000000000000000000000000000002574a912521572,0x4a77f870,0x095ea7b3000000000000000000000000af5caa87a7d3718622604268c43ff6ce9d2cec3c0000000000000000000000000000000000000000000000000000000679e568f2,0x095ea7b3000000000000000000000000af5caa87a7d3718622604268c43ff6ce9d2cec3c00000000000000000000000000000000000000000007cb2650dbf0f57c538600,0x095ea7b3000000000000000000000000af5caa87a7d3718622604268c43ff6ce9d2cec3c0000000000000000000000000000000000000000001aba9e26e75c3500000000,0xe8e337000000000000000000000000006a2d262d56735dba19dd70682b39f6be9a931d98000000000000000000000000273131f7cb50ac002bdd08ca721988731f7e10920000000000000000000000000000000000000000000000000000000679e568f200000000000000000000000000000000000000000003f0a43b33ec85fc538600000000000000000000000000000000000000000000000000000000066951322200000000000000000000000000000000000000000003e68e1be40b70200000000000000000000000000000002401224012bae7c2f217392665ca7abc16dcde1e0000000000000000000000000000000000000000000000000000000063341b20,0xe8e337000000000000000000000000002d2b97ea380b0185e9fdf8271d1afb5d2bf18329000000000000000000000000273131f7cb50ac002bdd08ca721988731f7e10920000000000000000000000000000000000000000001aba9e26e75c350000000000000000000000000000000000000000000000000003da8215a8046f800000000000000000000000000000000000000000000000001a31c3f2ecf9150000000000000000000000000000000000000000000000000003c6c729b40458c00000000000000000000000000000002401224012bae7c2f217392665ca7abc16dcde1e0000000000000000000000000000000000000000000000000000000063341b20]

data=$(seth calldata "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)" $targets $values $datas $PREDECESSOR $SALT $DELAY)

op_hash=$(seth call $TIMELOCK "hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)(bytes32)" $targets $values $datas $PREDECESSOR $SALT)
seth call $TIMELOCK "getTimestamp(bytes32)(uint)" $op_hash

seth send $TIMELOCK "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)" $targets $values $datas $PREDECESSOR $SALT