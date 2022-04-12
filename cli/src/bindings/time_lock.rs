pub use timelock_mod::*;
#[allow(clippy::too_many_arguments)]
mod timelock_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "TimeLock was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static TIMELOCK_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"minDelay\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"proposers\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"executors\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"target\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"CallExecuted\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"uint256\",\n        \"name\": \"index\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"address\",\n        \"name\": \"target\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"bytes32\",\n        \"name\": \"predecessor\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"delay\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"CallScheduled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"Cancelled\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"oldDuration\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"indexed\": false,\n        \"internalType\": \"uint256\",\n        \"name\": \"newDuration\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"MinDelayChange\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"previousAdminRole\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"newAdminRole\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"RoleAdminChanged\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RoleGranted\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"internalType\": \"address\",\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"RoleRevoked\",\n    \"type\": \"event\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"DEFAULT_ADMIN_ROLE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"EXECUTOR_ROLE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"PROPOSER_ROLE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"TIMELOCK_ADMIN_ROLE\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"cancel\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"target\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"predecessor\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"salt\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"execute\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"targets\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"values\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"bytes[]\",\n        \"name\": \"datas\",\n        \"type\": \"bytes[]\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"predecessor\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"salt\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"executeBatch\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getMinDelay\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"duration\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"getRoleAdmin\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"getTimestamp\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"timestamp\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"grantRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"hasRole\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"target\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"predecessor\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"salt\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"hashOperation\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"hash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"targets\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"values\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"bytes[]\",\n        \"name\": \"datas\",\n        \"type\": \"bytes[]\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"predecessor\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"salt\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"hashOperationBatch\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"hash\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"stateMutability\": \"pure\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"isOperation\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"pending\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"isOperationDone\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"done\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"isOperationPending\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"pending\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"id\",\n        \"type\": \"bytes32\"\n      }\n    ],\n    \"name\": \"isOperationReady\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"ready\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"renounceRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"role\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"account\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"revokeRole\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"target\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"predecessor\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"salt\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"delay\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"schedule\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address[]\",\n        \"name\": \"targets\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"internalType\": \"uint256[]\",\n        \"name\": \"values\",\n        \"type\": \"uint256[]\"\n      },\n      {\n        \"internalType\": \"bytes[]\",\n        \"name\": \"datas\",\n        \"type\": \"bytes[]\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"predecessor\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"bytes32\",\n        \"name\": \"salt\",\n        \"type\": \"bytes32\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"delay\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"scheduleBatch\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes4\",\n        \"name\": \"interfaceId\",\n        \"type\": \"bytes4\"\n      }\n    ],\n    \"name\": \"supportsInterface\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bool\",\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"newDelay\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"updateDelay\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"stateMutability\": \"payable\",\n    \"type\": \"receive\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct TimeLock<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for TimeLock<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for TimeLock<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(TimeLock))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> TimeLock<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), TIMELOCK_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `DEFAULT_ADMIN_ROLE` (0xa217fddf) function"]
        pub fn default_admin_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([162, 23, 253, 223], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `EXECUTOR_ROLE` (0x07bd0265) function"]
        pub fn executor_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([7, 189, 2, 101], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `PROPOSER_ROLE` (0x8f61f4f5) function"]
        pub fn proposer_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([143, 97, 244, 245], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `TIMELOCK_ADMIN_ROLE` (0x0d3cf6fc) function"]
        pub fn timelock_admin_role(&self) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([13, 60, 246, 252], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `cancel` (0xc4d252f5) function"]
        pub fn cancel(&self, id: [u8; 32]) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 210, 82, 245], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `execute` (0x134008d3) function"]
        pub fn execute(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 64, 8, 211], (target, value, data, predecessor, salt))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeBatch` (0xe38335e5) function"]
        pub fn execute_batch(
            &self,
            targets: ::std::vec::Vec<ethers::core::types::Address>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
            datas: ::std::vec::Vec<ethers::core::types::Bytes>,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [227, 131, 53, 229],
                    (targets, values, datas, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getMinDelay` (0xf27a0c92) function"]
        pub fn get_min_delay(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([242, 122, 12, 146], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getRoleAdmin` (0x248a9ca3) function"]
        pub fn get_role_admin(
            &self,
            role: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([36, 138, 156, 163], role)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTimestamp` (0xd45c4435) function"]
        pub fn get_timestamp(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([212, 92, 68, 53], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `grantRole` (0x2f2ff15d) function"]
        pub fn grant_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([47, 47, 241, 93], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hasRole` (0x91d14854) function"]
        pub fn has_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([145, 209, 72, 84], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hashOperation` (0x8065657f) function"]
        pub fn hash_operation(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [128, 101, 101, 127],
                    (target, value, data, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `hashOperationBatch` (0xb1c5f427) function"]
        pub fn hash_operation_batch(
            &self,
            targets: ::std::vec::Vec<ethers::core::types::Address>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
            datas: ::std::vec::Vec<ethers::core::types::Bytes>,
            predecessor: [u8; 32],
            salt: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash(
                    [177, 197, 244, 39],
                    (targets, values, datas, predecessor, salt),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOperation` (0x31d50750) function"]
        pub fn is_operation(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([49, 213, 7, 80], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOperationDone` (0x2ab0f529) function"]
        pub fn is_operation_done(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([42, 176, 245, 41], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOperationPending` (0x584b153e) function"]
        pub fn is_operation_pending(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([88, 75, 21, 62], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOperationReady` (0x13bc9f20) function"]
        pub fn is_operation_ready(
            &self,
            id: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([19, 188, 159, 32], id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceRole` (0x36568abe) function"]
        pub fn renounce_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([54, 86, 138, 190], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeRole` (0xd547741f) function"]
        pub fn revoke_role(
            &self,
            role: [u8; 32],
            account: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 71, 116, 31], (role, account))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `schedule` (0x01d5062a) function"]
        pub fn schedule(
            &self,
            target: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
            predecessor: [u8; 32],
            salt: [u8; 32],
            delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [1, 213, 6, 42],
                    (target, value, data, predecessor, salt, delay),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `scheduleBatch` (0x8f2a0bb0) function"]
        pub fn schedule_batch(
            &self,
            targets: ::std::vec::Vec<ethers::core::types::Address>,
            values: ::std::vec::Vec<ethers::core::types::U256>,
            datas: ::std::vec::Vec<ethers::core::types::Bytes>,
            predecessor: [u8; 32],
            salt: [u8; 32],
            delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [143, 42, 11, 176],
                    (targets, values, datas, predecessor, salt, delay),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `supportsInterface` (0x01ffc9a7) function"]
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `updateDelay` (0x64d62353) function"]
        pub fn update_delay(
            &self,
            new_delay: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 214, 35, 83], new_delay)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `CallExecuted` event"]
        pub fn call_executed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CallExecutedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `CallScheduled` event"]
        pub fn call_scheduled_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, CallScheduledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Cancelled` event"]
        pub fn cancelled_filter(&self) -> ethers::contract::builders::Event<M, CancelledFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `MinDelayChange` event"]
        pub fn min_delay_change_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, MinDelayChangeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleAdminChanged` event"]
        pub fn role_admin_changed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleAdminChangedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleGranted` event"]
        pub fn role_granted_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleGrantedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RoleRevoked` event"]
        pub fn role_revoked_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RoleRevokedFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, TimeLockEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "CallExecuted",
        abi = "CallExecuted(bytes32,uint256,address,uint256,bytes)"
    )]
    pub struct CallExecutedFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub index: ethers::core::types::U256,
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "CallScheduled",
        abi = "CallScheduled(bytes32,uint256,address,uint256,bytes,bytes32,uint256)"
    )]
    pub struct CallScheduledFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
        #[ethevent(indexed)]
        pub index: ethers::core::types::U256,
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub delay: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "Cancelled", abi = "Cancelled(bytes32)")]
    pub struct CancelledFilter {
        #[ethevent(indexed)]
        pub id: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "MinDelayChange", abi = "MinDelayChange(uint256,uint256)")]
    pub struct MinDelayChangeFilter {
        pub old_duration: ethers::core::types::U256,
        pub new_duration: ethers::core::types::U256,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "RoleAdminChanged",
        abi = "RoleAdminChanged(bytes32,bytes32,bytes32)"
    )]
    pub struct RoleAdminChangedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub previous_admin_role: [u8; 32],
        #[ethevent(indexed)]
        pub new_admin_role: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "RoleGranted", abi = "RoleGranted(bytes32,address,address)")]
    pub struct RoleGrantedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "RoleRevoked", abi = "RoleRevoked(bytes32,address,address)")]
    pub struct RoleRevokedFilter {
        #[ethevent(indexed)]
        pub role: [u8; 32],
        #[ethevent(indexed)]
        pub account: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TimeLockEvents {
        CallExecutedFilter(CallExecutedFilter),
        CallScheduledFilter(CallScheduledFilter),
        CancelledFilter(CancelledFilter),
        MinDelayChangeFilter(MinDelayChangeFilter),
        RoleAdminChangedFilter(RoleAdminChangedFilter),
        RoleGrantedFilter(RoleGrantedFilter),
        RoleRevokedFilter(RoleRevokedFilter),
    }
    impl ethers::contract::EthLogDecode for TimeLockEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = CallExecutedFilter::decode_log(log) {
                return Ok(TimeLockEvents::CallExecutedFilter(decoded));
            }
            if let Ok(decoded) = CallScheduledFilter::decode_log(log) {
                return Ok(TimeLockEvents::CallScheduledFilter(decoded));
            }
            if let Ok(decoded) = CancelledFilter::decode_log(log) {
                return Ok(TimeLockEvents::CancelledFilter(decoded));
            }
            if let Ok(decoded) = MinDelayChangeFilter::decode_log(log) {
                return Ok(TimeLockEvents::MinDelayChangeFilter(decoded));
            }
            if let Ok(decoded) = RoleAdminChangedFilter::decode_log(log) {
                return Ok(TimeLockEvents::RoleAdminChangedFilter(decoded));
            }
            if let Ok(decoded) = RoleGrantedFilter::decode_log(log) {
                return Ok(TimeLockEvents::RoleGrantedFilter(decoded));
            }
            if let Ok(decoded) = RoleRevokedFilter::decode_log(log) {
                return Ok(TimeLockEvents::RoleRevokedFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for TimeLockEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TimeLockEvents::CallExecutedFilter(element) => element.fmt(f),
                TimeLockEvents::CallScheduledFilter(element) => element.fmt(f),
                TimeLockEvents::CancelledFilter(element) => element.fmt(f),
                TimeLockEvents::MinDelayChangeFilter(element) => element.fmt(f),
                TimeLockEvents::RoleAdminChangedFilter(element) => element.fmt(f),
                TimeLockEvents::RoleGrantedFilter(element) => element.fmt(f),
                TimeLockEvents::RoleRevokedFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `DEFAULT_ADMIN_ROLE`function with signature `DEFAULT_ADMIN_ROLE()` and selector `[162, 23, 253, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "DEFAULT_ADMIN_ROLE", abi = "DEFAULT_ADMIN_ROLE()")]
    pub struct DefaultAdminRoleCall;
    #[doc = "Container type for all input parameters for the `EXECUTOR_ROLE`function with signature `EXECUTOR_ROLE()` and selector `[7, 189, 2, 101]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "EXECUTOR_ROLE", abi = "EXECUTOR_ROLE()")]
    pub struct ExecutorRoleCall;
    #[doc = "Container type for all input parameters for the `PROPOSER_ROLE`function with signature `PROPOSER_ROLE()` and selector `[143, 97, 244, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "PROPOSER_ROLE", abi = "PROPOSER_ROLE()")]
    pub struct ProposerRoleCall;
    #[doc = "Container type for all input parameters for the `TIMELOCK_ADMIN_ROLE`function with signature `TIMELOCK_ADMIN_ROLE()` and selector `[13, 60, 246, 252]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "TIMELOCK_ADMIN_ROLE", abi = "TIMELOCK_ADMIN_ROLE()")]
    pub struct TimelockAdminRoleCall;
    #[doc = "Container type for all input parameters for the `cancel`function with signature `cancel(bytes32)` and selector `[196, 210, 82, 245]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "cancel", abi = "cancel(bytes32)")]
    pub struct CancelCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `execute`function with signature `execute(address,uint256,bytes,bytes32,bytes32)` and selector `[19, 64, 8, 211]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "execute",
        abi = "execute(address,uint256,bytes,bytes32,bytes32)"
    )]
    pub struct ExecuteCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `executeBatch`function with signature `executeBatch(address[],uint256[],bytes[],bytes32,bytes32)` and selector `[227, 131, 53, 229]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "executeBatch",
        abi = "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)"
    )]
    pub struct ExecuteBatchCall {
        pub targets: ::std::vec::Vec<ethers::core::types::Address>,
        pub values: ::std::vec::Vec<ethers::core::types::U256>,
        pub datas: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getMinDelay`function with signature `getMinDelay()` and selector `[242, 122, 12, 146]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getMinDelay", abi = "getMinDelay()")]
    pub struct GetMinDelayCall;
    #[doc = "Container type for all input parameters for the `getRoleAdmin`function with signature `getRoleAdmin(bytes32)` and selector `[36, 138, 156, 163]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getRoleAdmin", abi = "getRoleAdmin(bytes32)")]
    pub struct GetRoleAdminCall {
        pub role: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getTimestamp`function with signature `getTimestamp(bytes32)` and selector `[212, 92, 68, 53]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTimestamp", abi = "getTimestamp(bytes32)")]
    pub struct GetTimestampCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `grantRole`function with signature `grantRole(bytes32,address)` and selector `[47, 47, 241, 93]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "grantRole", abi = "grantRole(bytes32,address)")]
    pub struct GrantRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hasRole`function with signature `hasRole(bytes32,address)` and selector `[145, 209, 72, 84]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "hasRole", abi = "hasRole(bytes32,address)")]
    pub struct HasRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `hashOperation`function with signature `hashOperation(address,uint256,bytes,bytes32,bytes32)` and selector `[128, 101, 101, 127]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "hashOperation",
        abi = "hashOperation(address,uint256,bytes,bytes32,bytes32)"
    )]
    pub struct HashOperationCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `hashOperationBatch`function with signature `hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)` and selector `[177, 197, 244, 39]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "hashOperationBatch",
        abi = "hashOperationBatch(address[],uint256[],bytes[],bytes32,bytes32)"
    )]
    pub struct HashOperationBatchCall {
        pub targets: ::std::vec::Vec<ethers::core::types::Address>,
        pub values: ::std::vec::Vec<ethers::core::types::U256>,
        pub datas: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isOperation`function with signature `isOperation(bytes32)` and selector `[49, 213, 7, 80]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isOperation", abi = "isOperation(bytes32)")]
    pub struct IsOperationCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isOperationDone`function with signature `isOperationDone(bytes32)` and selector `[42, 176, 245, 41]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isOperationDone", abi = "isOperationDone(bytes32)")]
    pub struct IsOperationDoneCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isOperationPending`function with signature `isOperationPending(bytes32)` and selector `[88, 75, 21, 62]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isOperationPending", abi = "isOperationPending(bytes32)")]
    pub struct IsOperationPendingCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `isOperationReady`function with signature `isOperationReady(bytes32)` and selector `[19, 188, 159, 32]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isOperationReady", abi = "isOperationReady(bytes32)")]
    pub struct IsOperationReadyCall {
        pub id: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `renounceRole`function with signature `renounceRole(bytes32,address)` and selector `[54, 86, 138, 190]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "renounceRole", abi = "renounceRole(bytes32,address)")]
    pub struct RenounceRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `revokeRole`function with signature `revokeRole(bytes32,address)` and selector `[213, 71, 116, 31]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "revokeRole", abi = "revokeRole(bytes32,address)")]
    pub struct RevokeRoleCall {
        pub role: [u8; 32],
        pub account: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `schedule`function with signature `schedule(address,uint256,bytes,bytes32,bytes32,uint256)` and selector `[1, 213, 6, 42]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "schedule",
        abi = "schedule(address,uint256,bytes,bytes32,bytes32,uint256)"
    )]
    pub struct ScheduleCall {
        pub target: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
        pub delay: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `scheduleBatch`function with signature `scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)` and selector `[143, 42, 11, 176]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "scheduleBatch",
        abi = "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)"
    )]
    pub struct ScheduleBatchCall {
        pub targets: ::std::vec::Vec<ethers::core::types::Address>,
        pub values: ::std::vec::Vec<ethers::core::types::U256>,
        pub datas: ::std::vec::Vec<ethers::core::types::Bytes>,
        pub predecessor: [u8; 32],
        pub salt: [u8; 32],
        pub delay: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `supportsInterface`function with signature `supportsInterface(bytes4)` and selector `[1, 255, 201, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    #[doc = "Container type for all input parameters for the `updateDelay`function with signature `updateDelay(uint256)` and selector `[100, 214, 35, 83]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "updateDelay", abi = "updateDelay(uint256)")]
    pub struct UpdateDelayCall {
        pub new_delay: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum TimeLockCalls {
        DefaultAdminRole(DefaultAdminRoleCall),
        ExecutorRole(ExecutorRoleCall),
        ProposerRole(ProposerRoleCall),
        TimelockAdminRole(TimelockAdminRoleCall),
        Cancel(CancelCall),
        Execute(ExecuteCall),
        ExecuteBatch(ExecuteBatchCall),
        GetMinDelay(GetMinDelayCall),
        GetRoleAdmin(GetRoleAdminCall),
        GetTimestamp(GetTimestampCall),
        GrantRole(GrantRoleCall),
        HasRole(HasRoleCall),
        HashOperation(HashOperationCall),
        HashOperationBatch(HashOperationBatchCall),
        IsOperation(IsOperationCall),
        IsOperationDone(IsOperationDoneCall),
        IsOperationPending(IsOperationPendingCall),
        IsOperationReady(IsOperationReadyCall),
        RenounceRole(RenounceRoleCall),
        RevokeRole(RevokeRoleCall),
        Schedule(ScheduleCall),
        ScheduleBatch(ScheduleBatchCall),
        SupportsInterface(SupportsInterfaceCall),
        UpdateDelay(UpdateDelayCall),
    }
    impl ethers::core::abi::AbiDecode for TimeLockCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <DefaultAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::DefaultAdminRole(decoded));
            }
            if let Ok(decoded) =
                <ExecutorRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::ExecutorRole(decoded));
            }
            if let Ok(decoded) =
                <ProposerRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::ProposerRole(decoded));
            }
            if let Ok(decoded) =
                <TimelockAdminRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::TimelockAdminRole(decoded));
            }
            if let Ok(decoded) = <CancelCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::Cancel(decoded));
            }
            if let Ok(decoded) =
                <ExecuteCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::Execute(decoded));
            }
            if let Ok(decoded) =
                <ExecuteBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::ExecuteBatch(decoded));
            }
            if let Ok(decoded) =
                <GetMinDelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::GetMinDelay(decoded));
            }
            if let Ok(decoded) =
                <GetRoleAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::GetRoleAdmin(decoded));
            }
            if let Ok(decoded) =
                <GetTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::GetTimestamp(decoded));
            }
            if let Ok(decoded) =
                <GrantRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::GrantRole(decoded));
            }
            if let Ok(decoded) =
                <HasRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::HasRole(decoded));
            }
            if let Ok(decoded) =
                <HashOperationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::HashOperation(decoded));
            }
            if let Ok(decoded) =
                <HashOperationBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::HashOperationBatch(decoded));
            }
            if let Ok(decoded) =
                <IsOperationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::IsOperation(decoded));
            }
            if let Ok(decoded) =
                <IsOperationDoneCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::IsOperationDone(decoded));
            }
            if let Ok(decoded) =
                <IsOperationPendingCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::IsOperationPending(decoded));
            }
            if let Ok(decoded) =
                <IsOperationReadyCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::IsOperationReady(decoded));
            }
            if let Ok(decoded) =
                <RenounceRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::RenounceRole(decoded));
            }
            if let Ok(decoded) =
                <RevokeRoleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::RevokeRole(decoded));
            }
            if let Ok(decoded) =
                <ScheduleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::Schedule(decoded));
            }
            if let Ok(decoded) =
                <ScheduleBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::ScheduleBatch(decoded));
            }
            if let Ok(decoded) =
                <SupportsInterfaceCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::SupportsInterface(decoded));
            }
            if let Ok(decoded) =
                <UpdateDelayCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(TimeLockCalls::UpdateDelay(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for TimeLockCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                TimeLockCalls::DefaultAdminRole(element) => element.encode(),
                TimeLockCalls::ExecutorRole(element) => element.encode(),
                TimeLockCalls::ProposerRole(element) => element.encode(),
                TimeLockCalls::TimelockAdminRole(element) => element.encode(),
                TimeLockCalls::Cancel(element) => element.encode(),
                TimeLockCalls::Execute(element) => element.encode(),
                TimeLockCalls::ExecuteBatch(element) => element.encode(),
                TimeLockCalls::GetMinDelay(element) => element.encode(),
                TimeLockCalls::GetRoleAdmin(element) => element.encode(),
                TimeLockCalls::GetTimestamp(element) => element.encode(),
                TimeLockCalls::GrantRole(element) => element.encode(),
                TimeLockCalls::HasRole(element) => element.encode(),
                TimeLockCalls::HashOperation(element) => element.encode(),
                TimeLockCalls::HashOperationBatch(element) => element.encode(),
                TimeLockCalls::IsOperation(element) => element.encode(),
                TimeLockCalls::IsOperationDone(element) => element.encode(),
                TimeLockCalls::IsOperationPending(element) => element.encode(),
                TimeLockCalls::IsOperationReady(element) => element.encode(),
                TimeLockCalls::RenounceRole(element) => element.encode(),
                TimeLockCalls::RevokeRole(element) => element.encode(),
                TimeLockCalls::Schedule(element) => element.encode(),
                TimeLockCalls::ScheduleBatch(element) => element.encode(),
                TimeLockCalls::SupportsInterface(element) => element.encode(),
                TimeLockCalls::UpdateDelay(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for TimeLockCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                TimeLockCalls::DefaultAdminRole(element) => element.fmt(f),
                TimeLockCalls::ExecutorRole(element) => element.fmt(f),
                TimeLockCalls::ProposerRole(element) => element.fmt(f),
                TimeLockCalls::TimelockAdminRole(element) => element.fmt(f),
                TimeLockCalls::Cancel(element) => element.fmt(f),
                TimeLockCalls::Execute(element) => element.fmt(f),
                TimeLockCalls::ExecuteBatch(element) => element.fmt(f),
                TimeLockCalls::GetMinDelay(element) => element.fmt(f),
                TimeLockCalls::GetRoleAdmin(element) => element.fmt(f),
                TimeLockCalls::GetTimestamp(element) => element.fmt(f),
                TimeLockCalls::GrantRole(element) => element.fmt(f),
                TimeLockCalls::HasRole(element) => element.fmt(f),
                TimeLockCalls::HashOperation(element) => element.fmt(f),
                TimeLockCalls::HashOperationBatch(element) => element.fmt(f),
                TimeLockCalls::IsOperation(element) => element.fmt(f),
                TimeLockCalls::IsOperationDone(element) => element.fmt(f),
                TimeLockCalls::IsOperationPending(element) => element.fmt(f),
                TimeLockCalls::IsOperationReady(element) => element.fmt(f),
                TimeLockCalls::RenounceRole(element) => element.fmt(f),
                TimeLockCalls::RevokeRole(element) => element.fmt(f),
                TimeLockCalls::Schedule(element) => element.fmt(f),
                TimeLockCalls::ScheduleBatch(element) => element.fmt(f),
                TimeLockCalls::SupportsInterface(element) => element.fmt(f),
                TimeLockCalls::UpdateDelay(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<DefaultAdminRoleCall> for TimeLockCalls {
        fn from(var: DefaultAdminRoleCall) -> Self {
            TimeLockCalls::DefaultAdminRole(var)
        }
    }
    impl ::std::convert::From<ExecutorRoleCall> for TimeLockCalls {
        fn from(var: ExecutorRoleCall) -> Self {
            TimeLockCalls::ExecutorRole(var)
        }
    }
    impl ::std::convert::From<ProposerRoleCall> for TimeLockCalls {
        fn from(var: ProposerRoleCall) -> Self {
            TimeLockCalls::ProposerRole(var)
        }
    }
    impl ::std::convert::From<TimelockAdminRoleCall> for TimeLockCalls {
        fn from(var: TimelockAdminRoleCall) -> Self {
            TimeLockCalls::TimelockAdminRole(var)
        }
    }
    impl ::std::convert::From<CancelCall> for TimeLockCalls {
        fn from(var: CancelCall) -> Self {
            TimeLockCalls::Cancel(var)
        }
    }
    impl ::std::convert::From<ExecuteCall> for TimeLockCalls {
        fn from(var: ExecuteCall) -> Self {
            TimeLockCalls::Execute(var)
        }
    }
    impl ::std::convert::From<ExecuteBatchCall> for TimeLockCalls {
        fn from(var: ExecuteBatchCall) -> Self {
            TimeLockCalls::ExecuteBatch(var)
        }
    }
    impl ::std::convert::From<GetMinDelayCall> for TimeLockCalls {
        fn from(var: GetMinDelayCall) -> Self {
            TimeLockCalls::GetMinDelay(var)
        }
    }
    impl ::std::convert::From<GetRoleAdminCall> for TimeLockCalls {
        fn from(var: GetRoleAdminCall) -> Self {
            TimeLockCalls::GetRoleAdmin(var)
        }
    }
    impl ::std::convert::From<GetTimestampCall> for TimeLockCalls {
        fn from(var: GetTimestampCall) -> Self {
            TimeLockCalls::GetTimestamp(var)
        }
    }
    impl ::std::convert::From<GrantRoleCall> for TimeLockCalls {
        fn from(var: GrantRoleCall) -> Self {
            TimeLockCalls::GrantRole(var)
        }
    }
    impl ::std::convert::From<HasRoleCall> for TimeLockCalls {
        fn from(var: HasRoleCall) -> Self {
            TimeLockCalls::HasRole(var)
        }
    }
    impl ::std::convert::From<HashOperationCall> for TimeLockCalls {
        fn from(var: HashOperationCall) -> Self {
            TimeLockCalls::HashOperation(var)
        }
    }
    impl ::std::convert::From<HashOperationBatchCall> for TimeLockCalls {
        fn from(var: HashOperationBatchCall) -> Self {
            TimeLockCalls::HashOperationBatch(var)
        }
    }
    impl ::std::convert::From<IsOperationCall> for TimeLockCalls {
        fn from(var: IsOperationCall) -> Self {
            TimeLockCalls::IsOperation(var)
        }
    }
    impl ::std::convert::From<IsOperationDoneCall> for TimeLockCalls {
        fn from(var: IsOperationDoneCall) -> Self {
            TimeLockCalls::IsOperationDone(var)
        }
    }
    impl ::std::convert::From<IsOperationPendingCall> for TimeLockCalls {
        fn from(var: IsOperationPendingCall) -> Self {
            TimeLockCalls::IsOperationPending(var)
        }
    }
    impl ::std::convert::From<IsOperationReadyCall> for TimeLockCalls {
        fn from(var: IsOperationReadyCall) -> Self {
            TimeLockCalls::IsOperationReady(var)
        }
    }
    impl ::std::convert::From<RenounceRoleCall> for TimeLockCalls {
        fn from(var: RenounceRoleCall) -> Self {
            TimeLockCalls::RenounceRole(var)
        }
    }
    impl ::std::convert::From<RevokeRoleCall> for TimeLockCalls {
        fn from(var: RevokeRoleCall) -> Self {
            TimeLockCalls::RevokeRole(var)
        }
    }
    impl ::std::convert::From<ScheduleCall> for TimeLockCalls {
        fn from(var: ScheduleCall) -> Self {
            TimeLockCalls::Schedule(var)
        }
    }
    impl ::std::convert::From<ScheduleBatchCall> for TimeLockCalls {
        fn from(var: ScheduleBatchCall) -> Self {
            TimeLockCalls::ScheduleBatch(var)
        }
    }
    impl ::std::convert::From<SupportsInterfaceCall> for TimeLockCalls {
        fn from(var: SupportsInterfaceCall) -> Self {
            TimeLockCalls::SupportsInterface(var)
        }
    }
    impl ::std::convert::From<UpdateDelayCall> for TimeLockCalls {
        fn from(var: UpdateDelayCall) -> Self {
            TimeLockCalls::UpdateDelay(var)
        }
    }
}
