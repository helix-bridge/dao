pub use multisigwallet_mod::*;
#[allow(clippy::too_many_arguments)]
mod multisigwallet_mod {
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
    #[doc = "MultiSigWallet was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MULTISIGWALLET_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"owners\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"removeOwner\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"revokeConfirmation\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"isOwner\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"confirmations\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"pending\",\n        \"type\": \"bool\"\n      },\n      {\n        \"name\": \"executed\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"getTransactionCount\",\n    \"outputs\": [\n      {\n        \"name\": \"count\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"addOwner\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"isConfirmed\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getConfirmationCount\",\n    \"outputs\": [\n      {\n        \"name\": \"count\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"transactions\",\n    \"outputs\": [\n      {\n        \"name\": \"destination\",\n        \"type\": \"address\"\n      },\n      {\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      },\n      {\n        \"name\": \"executed\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"getOwners\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"from\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"name\": \"to\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"name\": \"pending\",\n        \"type\": \"bool\"\n      },\n      {\n        \"name\": \"executed\",\n        \"type\": \"bool\"\n      }\n    ],\n    \"name\": \"getTransactionIds\",\n    \"outputs\": [\n      {\n        \"name\": \"_transactionIds\",\n        \"type\": \"uint256[]\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [\n      {\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getConfirmations\",\n    \"outputs\": [\n      {\n        \"name\": \"_confirmations\",\n        \"type\": \"address[]\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"transactionCount\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"_required\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"changeRequirement\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"confirmTransaction\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"destination\",\n        \"type\": \"address\"\n      },\n      {\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"name\": \"data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"submitTransaction\",\n    \"outputs\": [\n      {\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"MAX_OWNER_COUNT\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": true,\n    \"inputs\": [],\n    \"name\": \"required\",\n    \"outputs\": [\n      {\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      },\n      {\n        \"name\": \"newOwner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"replaceOwner\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"constant\": false,\n    \"inputs\": [\n      {\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"executeTransaction\",\n    \"outputs\": [],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"name\": \"_owners\",\n        \"type\": \"address[]\"\n      },\n      {\n        \"name\": \"_required\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"payable\": false,\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"payable\": true,\n    \"stateMutability\": \"payable\",\n    \"type\": \"fallback\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Confirmation\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": true,\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Revocation\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Submission\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Execution\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"name\": \"transactionId\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"ExecutionFailure\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"name\": \"sender\",\n        \"type\": \"address\"\n      },\n      {\n        \"indexed\": false,\n        \"name\": \"value\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"Deposit\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnerAddition\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": true,\n        \"name\": \"owner\",\n        \"type\": \"address\"\n      }\n    ],\n    \"name\": \"OwnerRemoval\",\n    \"type\": \"event\"\n  },\n  {\n    \"anonymous\": false,\n    \"inputs\": [\n      {\n        \"indexed\": false,\n        \"name\": \"required\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"RequirementChange\",\n    \"type\": \"event\"\n  }\n]\n") . expect ("invalid abi")
        });
    #[derive(Clone)]
    pub struct MultiSigWallet<M>(ethers::contract::Contract<M>);
    impl<M> std::ops::Deref for MultiSigWallet<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MultiSigWallet<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MultiSigWallet))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MultiSigWallet<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            let contract =
                ethers::contract::Contract::new(address.into(), MULTISIGWALLET_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `MAX_OWNER_COUNT` (0xd74f8edd) function"]
        pub fn max_owner_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([215, 79, 142, 221], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `addOwner` (0x7065cb48) function"]
        pub fn add_owner(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 101, 203, 72], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `changeRequirement` (0xba51a6df) function"]
        pub fn change_requirement(
            &self,
            required: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 81, 166, 223], required)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `confirmTransaction` (0xc01a8c84) function"]
        pub fn confirm_transaction(
            &self,
            transaction_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 26, 140, 132], transaction_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `confirmations` (0x3411c81c) function"]
        pub fn confirmations(
            &self,
            p0: ethers::core::types::U256,
            p1: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([52, 17, 200, 28], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executeTransaction` (0xee22610b) function"]
        pub fn execute_transaction(
            &self,
            transaction_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([238, 34, 97, 11], transaction_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getConfirmationCount` (0x8b51d13f) function"]
        pub fn get_confirmation_count(
            &self,
            transaction_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([139, 81, 209, 63], transaction_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getConfirmations` (0xb5dc40c3) function"]
        pub fn get_confirmations(
            &self,
            transaction_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([181, 220, 64, 195], transaction_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getOwners` (0xa0e67e2b) function"]
        pub fn get_owners(
            &self,
        ) -> ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<ethers::core::types::Address>,
        > {
            self.0
                .method_hash([160, 230, 126, 43], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTransactionCount` (0x54741525) function"]
        pub fn get_transaction_count(
            &self,
            pending: bool,
            executed: bool,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([84, 116, 21, 37], (pending, executed))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getTransactionIds` (0xa8abe69a) function"]
        pub fn get_transaction_ids(
            &self,
            from: ethers::core::types::U256,
            to: ethers::core::types::U256,
            pending: bool,
            executed: bool,
        ) -> ethers::contract::builders::ContractCall<M, ::std::vec::Vec<ethers::core::types::U256>>
        {
            self.0
                .method_hash([168, 171, 230, 154], (from, to, pending, executed))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isConfirmed` (0x784547a7) function"]
        pub fn is_confirmed(
            &self,
            transaction_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([120, 69, 71, 167], transaction_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isOwner` (0x2f54bf6e) function"]
        pub fn is_owner(
            &self,
            p0: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([47, 84, 191, 110], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owners` (0x025e7c27) function"]
        pub fn owners(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([2, 94, 124, 39], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `removeOwner` (0x173825d9) function"]
        pub fn remove_owner(
            &self,
            owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 56, 37, 217], owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `replaceOwner` (0xe20056e6) function"]
        pub fn replace_owner(
            &self,
            owner: ethers::core::types::Address,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 0, 86, 230], (owner, new_owner))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `required` (0xdc8452cd) function"]
        pub fn required(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([220, 132, 82, 205], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `revokeConfirmation` (0x20ea8d86) function"]
        pub fn revoke_confirmation(
            &self,
            transaction_id: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 234, 141, 134], transaction_id)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitTransaction` (0xc6427474) function"]
        pub fn submit_transaction(
            &self,
            destination: ethers::core::types::Address,
            value: ethers::core::types::U256,
            data: ethers::core::types::Bytes,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([198, 66, 116, 116], (destination, value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transactionCount` (0xb77bf600) function"]
        pub fn transaction_count(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([183, 123, 246, 0], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transactions` (0x9ace38c2) function"]
        pub fn transactions(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<
            M,
            (
                ethers::core::types::Address,
                ethers::core::types::U256,
                ethers::core::types::Bytes,
                bool,
            ),
        > {
            self.0
                .method_hash([154, 206, 56, 194], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `Confirmation` event"]
        pub fn confirmation_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ConfirmationFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Deposit` event"]
        pub fn deposit_filter(&self) -> ethers::contract::builders::Event<M, DepositFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Execution` event"]
        pub fn execution_filter(&self) -> ethers::contract::builders::Event<M, ExecutionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ExecutionFailure` event"]
        pub fn execution_failure_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ExecutionFailureFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnerAddition` event"]
        pub fn owner_addition_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnerAdditionFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnerRemoval` event"]
        pub fn owner_removal_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnerRemovalFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `RequirementChange` event"]
        pub fn requirement_change_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, RequirementChangeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Revocation` event"]
        pub fn revocation_filter(&self) -> ethers::contract::builders::Event<M, RevocationFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Submission` event"]
        pub fn submission_filter(&self) -> ethers::contract::builders::Event<M, SubmissionFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MultiSigWalletEvents> {
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
    #[ethevent(name = "Confirmation", abi = "Confirmation(address,uint256)")]
    pub struct ConfirmationFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub transaction_id: ethers::core::types::U256,
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
    #[ethevent(name = "Deposit", abi = "Deposit(address,uint256)")]
    pub struct DepositFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
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
    #[ethevent(name = "Execution", abi = "Execution(uint256)")]
    pub struct ExecutionFilter {
        #[ethevent(indexed)]
        pub transaction_id: ethers::core::types::U256,
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
    #[ethevent(name = "ExecutionFailure", abi = "ExecutionFailure(uint256)")]
    pub struct ExecutionFailureFilter {
        #[ethevent(indexed)]
        pub transaction_id: ethers::core::types::U256,
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
    #[ethevent(name = "OwnerAddition", abi = "OwnerAddition(address)")]
    pub struct OwnerAdditionFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
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
    #[ethevent(name = "OwnerRemoval", abi = "OwnerRemoval(address)")]
    pub struct OwnerRemovalFilter {
        #[ethevent(indexed)]
        pub owner: ethers::core::types::Address,
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
    #[ethevent(name = "RequirementChange", abi = "RequirementChange(uint256)")]
    pub struct RequirementChangeFilter {
        pub required: ethers::core::types::U256,
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
    #[ethevent(name = "Revocation", abi = "Revocation(address,uint256)")]
    pub struct RevocationFilter {
        #[ethevent(indexed)]
        pub sender: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub transaction_id: ethers::core::types::U256,
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
    #[ethevent(name = "Submission", abi = "Submission(uint256)")]
    pub struct SubmissionFilter {
        #[ethevent(indexed)]
        pub transaction_id: ethers::core::types::U256,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MultiSigWalletEvents {
        ConfirmationFilter(ConfirmationFilter),
        DepositFilter(DepositFilter),
        ExecutionFilter(ExecutionFilter),
        ExecutionFailureFilter(ExecutionFailureFilter),
        OwnerAdditionFilter(OwnerAdditionFilter),
        OwnerRemovalFilter(OwnerRemovalFilter),
        RequirementChangeFilter(RequirementChangeFilter),
        RevocationFilter(RevocationFilter),
        SubmissionFilter(SubmissionFilter),
    }
    impl ethers::contract::EthLogDecode for MultiSigWalletEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ConfirmationFilter::decode_log(log) {
                return Ok(MultiSigWalletEvents::ConfirmationFilter(decoded));
            }
            if let Ok(decoded) = DepositFilter::decode_log(log) {
                return Ok(MultiSigWalletEvents::DepositFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFilter::decode_log(log) {
                return Ok(MultiSigWalletEvents::ExecutionFilter(decoded));
            }
            if let Ok(decoded) = ExecutionFailureFilter::decode_log(log) {
                return Ok(MultiSigWalletEvents::ExecutionFailureFilter(decoded));
            }
            if let Ok(decoded) = OwnerAdditionFilter::decode_log(log) {
                return Ok(MultiSigWalletEvents::OwnerAdditionFilter(decoded));
            }
            if let Ok(decoded) = OwnerRemovalFilter::decode_log(log) {
                return Ok(MultiSigWalletEvents::OwnerRemovalFilter(decoded));
            }
            if let Ok(decoded) = RequirementChangeFilter::decode_log(log) {
                return Ok(MultiSigWalletEvents::RequirementChangeFilter(decoded));
            }
            if let Ok(decoded) = RevocationFilter::decode_log(log) {
                return Ok(MultiSigWalletEvents::RevocationFilter(decoded));
            }
            if let Ok(decoded) = SubmissionFilter::decode_log(log) {
                return Ok(MultiSigWalletEvents::SubmissionFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MultiSigWalletEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MultiSigWalletEvents::ConfirmationFilter(element) => element.fmt(f),
                MultiSigWalletEvents::DepositFilter(element) => element.fmt(f),
                MultiSigWalletEvents::ExecutionFilter(element) => element.fmt(f),
                MultiSigWalletEvents::ExecutionFailureFilter(element) => element.fmt(f),
                MultiSigWalletEvents::OwnerAdditionFilter(element) => element.fmt(f),
                MultiSigWalletEvents::OwnerRemovalFilter(element) => element.fmt(f),
                MultiSigWalletEvents::RequirementChangeFilter(element) => element.fmt(f),
                MultiSigWalletEvents::RevocationFilter(element) => element.fmt(f),
                MultiSigWalletEvents::SubmissionFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `MAX_OWNER_COUNT`function with signature `MAX_OWNER_COUNT()` and selector `[215, 79, 142, 221]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "MAX_OWNER_COUNT", abi = "MAX_OWNER_COUNT()")]
    pub struct MaxOwnerCountCall;
    #[doc = "Container type for all input parameters for the `addOwner`function with signature `addOwner(address)` and selector `[112, 101, 203, 72]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "addOwner", abi = "addOwner(address)")]
    pub struct AddOwnerCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `changeRequirement`function with signature `changeRequirement(uint256)` and selector `[186, 81, 166, 223]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "changeRequirement", abi = "changeRequirement(uint256)")]
    pub struct ChangeRequirementCall {
        pub required: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `confirmTransaction`function with signature `confirmTransaction(uint256)` and selector `[192, 26, 140, 132]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "confirmTransaction", abi = "confirmTransaction(uint256)")]
    pub struct ConfirmTransactionCall {
        pub transaction_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `confirmations`function with signature `confirmations(uint256,address)` and selector `[52, 17, 200, 28]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "confirmations", abi = "confirmations(uint256,address)")]
    pub struct ConfirmationsCall(
        pub ethers::core::types::U256,
        pub ethers::core::types::Address,
    );
    #[doc = "Container type for all input parameters for the `executeTransaction`function with signature `executeTransaction(uint256)` and selector `[238, 34, 97, 11]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "executeTransaction", abi = "executeTransaction(uint256)")]
    pub struct ExecuteTransactionCall {
        pub transaction_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getConfirmationCount`function with signature `getConfirmationCount(uint256)` and selector `[139, 81, 209, 63]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getConfirmationCount", abi = "getConfirmationCount(uint256)")]
    pub struct GetConfirmationCountCall {
        pub transaction_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getConfirmations`function with signature `getConfirmations(uint256)` and selector `[181, 220, 64, 195]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getConfirmations", abi = "getConfirmations(uint256)")]
    pub struct GetConfirmationsCall {
        pub transaction_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `getOwners`function with signature `getOwners()` and selector `[160, 230, 126, 43]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getOwners", abi = "getOwners()")]
    pub struct GetOwnersCall;
    #[doc = "Container type for all input parameters for the `getTransactionCount`function with signature `getTransactionCount(bool,bool)` and selector `[84, 116, 21, 37]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "getTransactionCount", abi = "getTransactionCount(bool,bool)")]
    pub struct GetTransactionCountCall {
        pub pending: bool,
        pub executed: bool,
    }
    #[doc = "Container type for all input parameters for the `getTransactionIds`function with signature `getTransactionIds(uint256,uint256,bool,bool)` and selector `[168, 171, 230, 154]`"]
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
        name = "getTransactionIds",
        abi = "getTransactionIds(uint256,uint256,bool,bool)"
    )]
    pub struct GetTransactionIdsCall {
        pub from: ethers::core::types::U256,
        pub to: ethers::core::types::U256,
        pub pending: bool,
        pub executed: bool,
    }
    #[doc = "Container type for all input parameters for the `isConfirmed`function with signature `isConfirmed(uint256)` and selector `[120, 69, 71, 167]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isConfirmed", abi = "isConfirmed(uint256)")]
    pub struct IsConfirmedCall {
        pub transaction_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `isOwner`function with signature `isOwner(address)` and selector `[47, 84, 191, 110]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "isOwner", abi = "isOwner(address)")]
    pub struct IsOwnerCall(pub ethers::core::types::Address);
    #[doc = "Container type for all input parameters for the `owners`function with signature `owners(uint256)` and selector `[2, 94, 124, 39]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "owners", abi = "owners(uint256)")]
    pub struct OwnersCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `removeOwner`function with signature `removeOwner(address)` and selector `[23, 56, 37, 217]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "removeOwner", abi = "removeOwner(address)")]
    pub struct RemoveOwnerCall {
        pub owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `replaceOwner`function with signature `replaceOwner(address,address)` and selector `[226, 0, 86, 230]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "replaceOwner", abi = "replaceOwner(address,address)")]
    pub struct ReplaceOwnerCall {
        pub owner: ethers::core::types::Address,
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `required`function with signature `required()` and selector `[220, 132, 82, 205]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "required", abi = "required()")]
    pub struct RequiredCall;
    #[doc = "Container type for all input parameters for the `revokeConfirmation`function with signature `revokeConfirmation(uint256)` and selector `[32, 234, 141, 134]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "revokeConfirmation", abi = "revokeConfirmation(uint256)")]
    pub struct RevokeConfirmationCall {
        pub transaction_id: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `submitTransaction`function with signature `submitTransaction(address,uint256,bytes)` and selector `[198, 66, 116, 116]`"]
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
        name = "submitTransaction",
        abi = "submitTransaction(address,uint256,bytes)"
    )]
    pub struct SubmitTransactionCall {
        pub destination: ethers::core::types::Address,
        pub value: ethers::core::types::U256,
        pub data: ethers::core::types::Bytes,
    }
    #[doc = "Container type for all input parameters for the `transactionCount`function with signature `transactionCount()` and selector `[183, 123, 246, 0]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transactionCount", abi = "transactionCount()")]
    pub struct TransactionCountCall;
    #[doc = "Container type for all input parameters for the `transactions`function with signature `transactions(uint256)` and selector `[154, 206, 56, 194]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "transactions", abi = "transactions(uint256)")]
    pub struct TransactionsCall(pub ethers::core::types::U256);
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MultiSigWalletCalls {
        MaxOwnerCount(MaxOwnerCountCall),
        AddOwner(AddOwnerCall),
        ChangeRequirement(ChangeRequirementCall),
        ConfirmTransaction(ConfirmTransactionCall),
        Confirmations(ConfirmationsCall),
        ExecuteTransaction(ExecuteTransactionCall),
        GetConfirmationCount(GetConfirmationCountCall),
        GetConfirmations(GetConfirmationsCall),
        GetOwners(GetOwnersCall),
        GetTransactionCount(GetTransactionCountCall),
        GetTransactionIds(GetTransactionIdsCall),
        IsConfirmed(IsConfirmedCall),
        IsOwner(IsOwnerCall),
        Owners(OwnersCall),
        RemoveOwner(RemoveOwnerCall),
        ReplaceOwner(ReplaceOwnerCall),
        Required(RequiredCall),
        RevokeConfirmation(RevokeConfirmationCall),
        SubmitTransaction(SubmitTransactionCall),
        TransactionCount(TransactionCountCall),
        Transactions(TransactionsCall),
    }
    impl ethers::core::abi::AbiDecode for MultiSigWalletCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <MaxOwnerCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::MaxOwnerCount(decoded));
            }
            if let Ok(decoded) =
                <AddOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::AddOwner(decoded));
            }
            if let Ok(decoded) =
                <ChangeRequirementCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::ChangeRequirement(decoded));
            }
            if let Ok(decoded) =
                <ConfirmTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::ConfirmTransaction(decoded));
            }
            if let Ok(decoded) =
                <ConfirmationsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::Confirmations(decoded));
            }
            if let Ok(decoded) =
                <ExecuteTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::ExecuteTransaction(decoded));
            }
            if let Ok(decoded) =
                <GetConfirmationCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::GetConfirmationCount(decoded));
            }
            if let Ok(decoded) =
                <GetConfirmationsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::GetConfirmations(decoded));
            }
            if let Ok(decoded) =
                <GetOwnersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::GetOwners(decoded));
            }
            if let Ok(decoded) =
                <GetTransactionCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::GetTransactionCount(decoded));
            }
            if let Ok(decoded) =
                <GetTransactionIdsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::GetTransactionIds(decoded));
            }
            if let Ok(decoded) =
                <IsConfirmedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::IsConfirmed(decoded));
            }
            if let Ok(decoded) =
                <IsOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::IsOwner(decoded));
            }
            if let Ok(decoded) = <OwnersCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::Owners(decoded));
            }
            if let Ok(decoded) =
                <RemoveOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::RemoveOwner(decoded));
            }
            if let Ok(decoded) =
                <ReplaceOwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::ReplaceOwner(decoded));
            }
            if let Ok(decoded) =
                <RequiredCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::Required(decoded));
            }
            if let Ok(decoded) =
                <RevokeConfirmationCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::RevokeConfirmation(decoded));
            }
            if let Ok(decoded) =
                <SubmitTransactionCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::SubmitTransaction(decoded));
            }
            if let Ok(decoded) =
                <TransactionCountCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::TransactionCount(decoded));
            }
            if let Ok(decoded) =
                <TransactionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MultiSigWalletCalls::Transactions(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MultiSigWalletCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MultiSigWalletCalls::MaxOwnerCount(element) => element.encode(),
                MultiSigWalletCalls::AddOwner(element) => element.encode(),
                MultiSigWalletCalls::ChangeRequirement(element) => element.encode(),
                MultiSigWalletCalls::ConfirmTransaction(element) => element.encode(),
                MultiSigWalletCalls::Confirmations(element) => element.encode(),
                MultiSigWalletCalls::ExecuteTransaction(element) => element.encode(),
                MultiSigWalletCalls::GetConfirmationCount(element) => element.encode(),
                MultiSigWalletCalls::GetConfirmations(element) => element.encode(),
                MultiSigWalletCalls::GetOwners(element) => element.encode(),
                MultiSigWalletCalls::GetTransactionCount(element) => element.encode(),
                MultiSigWalletCalls::GetTransactionIds(element) => element.encode(),
                MultiSigWalletCalls::IsConfirmed(element) => element.encode(),
                MultiSigWalletCalls::IsOwner(element) => element.encode(),
                MultiSigWalletCalls::Owners(element) => element.encode(),
                MultiSigWalletCalls::RemoveOwner(element) => element.encode(),
                MultiSigWalletCalls::ReplaceOwner(element) => element.encode(),
                MultiSigWalletCalls::Required(element) => element.encode(),
                MultiSigWalletCalls::RevokeConfirmation(element) => element.encode(),
                MultiSigWalletCalls::SubmitTransaction(element) => element.encode(),
                MultiSigWalletCalls::TransactionCount(element) => element.encode(),
                MultiSigWalletCalls::Transactions(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MultiSigWalletCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MultiSigWalletCalls::MaxOwnerCount(element) => element.fmt(f),
                MultiSigWalletCalls::AddOwner(element) => element.fmt(f),
                MultiSigWalletCalls::ChangeRequirement(element) => element.fmt(f),
                MultiSigWalletCalls::ConfirmTransaction(element) => element.fmt(f),
                MultiSigWalletCalls::Confirmations(element) => element.fmt(f),
                MultiSigWalletCalls::ExecuteTransaction(element) => element.fmt(f),
                MultiSigWalletCalls::GetConfirmationCount(element) => element.fmt(f),
                MultiSigWalletCalls::GetConfirmations(element) => element.fmt(f),
                MultiSigWalletCalls::GetOwners(element) => element.fmt(f),
                MultiSigWalletCalls::GetTransactionCount(element) => element.fmt(f),
                MultiSigWalletCalls::GetTransactionIds(element) => element.fmt(f),
                MultiSigWalletCalls::IsConfirmed(element) => element.fmt(f),
                MultiSigWalletCalls::IsOwner(element) => element.fmt(f),
                MultiSigWalletCalls::Owners(element) => element.fmt(f),
                MultiSigWalletCalls::RemoveOwner(element) => element.fmt(f),
                MultiSigWalletCalls::ReplaceOwner(element) => element.fmt(f),
                MultiSigWalletCalls::Required(element) => element.fmt(f),
                MultiSigWalletCalls::RevokeConfirmation(element) => element.fmt(f),
                MultiSigWalletCalls::SubmitTransaction(element) => element.fmt(f),
                MultiSigWalletCalls::TransactionCount(element) => element.fmt(f),
                MultiSigWalletCalls::Transactions(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<MaxOwnerCountCall> for MultiSigWalletCalls {
        fn from(var: MaxOwnerCountCall) -> Self {
            MultiSigWalletCalls::MaxOwnerCount(var)
        }
    }
    impl ::std::convert::From<AddOwnerCall> for MultiSigWalletCalls {
        fn from(var: AddOwnerCall) -> Self {
            MultiSigWalletCalls::AddOwner(var)
        }
    }
    impl ::std::convert::From<ChangeRequirementCall> for MultiSigWalletCalls {
        fn from(var: ChangeRequirementCall) -> Self {
            MultiSigWalletCalls::ChangeRequirement(var)
        }
    }
    impl ::std::convert::From<ConfirmTransactionCall> for MultiSigWalletCalls {
        fn from(var: ConfirmTransactionCall) -> Self {
            MultiSigWalletCalls::ConfirmTransaction(var)
        }
    }
    impl ::std::convert::From<ConfirmationsCall> for MultiSigWalletCalls {
        fn from(var: ConfirmationsCall) -> Self {
            MultiSigWalletCalls::Confirmations(var)
        }
    }
    impl ::std::convert::From<ExecuteTransactionCall> for MultiSigWalletCalls {
        fn from(var: ExecuteTransactionCall) -> Self {
            MultiSigWalletCalls::ExecuteTransaction(var)
        }
    }
    impl ::std::convert::From<GetConfirmationCountCall> for MultiSigWalletCalls {
        fn from(var: GetConfirmationCountCall) -> Self {
            MultiSigWalletCalls::GetConfirmationCount(var)
        }
    }
    impl ::std::convert::From<GetConfirmationsCall> for MultiSigWalletCalls {
        fn from(var: GetConfirmationsCall) -> Self {
            MultiSigWalletCalls::GetConfirmations(var)
        }
    }
    impl ::std::convert::From<GetOwnersCall> for MultiSigWalletCalls {
        fn from(var: GetOwnersCall) -> Self {
            MultiSigWalletCalls::GetOwners(var)
        }
    }
    impl ::std::convert::From<GetTransactionCountCall> for MultiSigWalletCalls {
        fn from(var: GetTransactionCountCall) -> Self {
            MultiSigWalletCalls::GetTransactionCount(var)
        }
    }
    impl ::std::convert::From<GetTransactionIdsCall> for MultiSigWalletCalls {
        fn from(var: GetTransactionIdsCall) -> Self {
            MultiSigWalletCalls::GetTransactionIds(var)
        }
    }
    impl ::std::convert::From<IsConfirmedCall> for MultiSigWalletCalls {
        fn from(var: IsConfirmedCall) -> Self {
            MultiSigWalletCalls::IsConfirmed(var)
        }
    }
    impl ::std::convert::From<IsOwnerCall> for MultiSigWalletCalls {
        fn from(var: IsOwnerCall) -> Self {
            MultiSigWalletCalls::IsOwner(var)
        }
    }
    impl ::std::convert::From<OwnersCall> for MultiSigWalletCalls {
        fn from(var: OwnersCall) -> Self {
            MultiSigWalletCalls::Owners(var)
        }
    }
    impl ::std::convert::From<RemoveOwnerCall> for MultiSigWalletCalls {
        fn from(var: RemoveOwnerCall) -> Self {
            MultiSigWalletCalls::RemoveOwner(var)
        }
    }
    impl ::std::convert::From<ReplaceOwnerCall> for MultiSigWalletCalls {
        fn from(var: ReplaceOwnerCall) -> Self {
            MultiSigWalletCalls::ReplaceOwner(var)
        }
    }
    impl ::std::convert::From<RequiredCall> for MultiSigWalletCalls {
        fn from(var: RequiredCall) -> Self {
            MultiSigWalletCalls::Required(var)
        }
    }
    impl ::std::convert::From<RevokeConfirmationCall> for MultiSigWalletCalls {
        fn from(var: RevokeConfirmationCall) -> Self {
            MultiSigWalletCalls::RevokeConfirmation(var)
        }
    }
    impl ::std::convert::From<SubmitTransactionCall> for MultiSigWalletCalls {
        fn from(var: SubmitTransactionCall) -> Self {
            MultiSigWalletCalls::SubmitTransaction(var)
        }
    }
    impl ::std::convert::From<TransactionCountCall> for MultiSigWalletCalls {
        fn from(var: TransactionCountCall) -> Self {
            MultiSigWalletCalls::TransactionCount(var)
        }
    }
    impl ::std::convert::From<TransactionsCall> for MultiSigWalletCalls {
        fn from(var: TransactionsCall) -> Self {
            MultiSigWalletCalls::Transactions(var)
        }
    }
}
