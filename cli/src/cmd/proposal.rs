use super::EthereumOpts;
use crate::cmd::conf::*;
use crate::cmd::multi_sig_wallet;
use crate::cmd::time_lock;
use crate::cmd::utils::Bytes;
use cast::SimpleCast;
use ethers::core::types::transaction::eip2718::TypedTransaction;
use ethers::core::types::transaction::request::TransactionRequest;
use ethers::prelude::*;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Proposal related commands")]
pub enum Proposal {
    #[structopt(about = "Proposal list.")]
    List {
        #[structopt(long, short)]
        no_subgraph: bool,
        #[structopt(default_value = "8317180")]
        #[structopt(long, short)]
        from_block: String,
        #[structopt(long, short)]
        to_block: Option<String>,
        #[structopt(long)]
        latest: Option<String>,
        #[structopt(long)]
        no_done: bool,
        #[structopt(long)]
        no_ready: bool,
        #[structopt(long)]
        no_pending: bool,
        #[structopt(long)]
        no_cancel: bool,
    },
    #[structopt(about = "Schedule an proposal containing a single transaction.")]
    Schedule {
        #[structopt(
            about = "The address of the smart contract that the timelock should operate on."
        )]
        target: Address,
        #[structopt(
            about = "In wei, that should be sent with the transaction. Most of the time this will be 0."
        )]
        value: String,
        #[structopt(
            about = "Containing the encoded function selector and parameters of the call by abi.encode."
        )]
        data: Bytes,
        #[structopt(about = "That specifies a dependency between operations.")]
        predecessor: H256,
        #[structopt(
            about = "Used to disambiguate two otherwise identical proposals. This can be any random value."
        )]
        salt: H256,
        #[structopt(about = "Delay time to execute the proposal, should be larger than minDelay")]
        delay: String,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Schedule an operation containing a batch of transactions.")]
    ScheduleBatch {
        args: Vec<String>,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Execute an (ready) operation containing a batch of transactions.")]
    ExecuteBatch {
        args: Vec<String>,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Cancel an proposal.")]
    Cancel {
        #[structopt(about = "Proposal ID")]
        id: H256,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Execute an (ready) proposal containing a single transaction.")]
    Execute {
        #[structopt(
            about = "The address of the smart contract that the timelock should operate on."
        )]
        target: Address,
        #[structopt(
            about = "In wei, that should be sent with the transaction. Most of the time this will be 0."
        )]
        value: String,
        #[structopt(
            about = "Containing the encoded function selector and parameters of the call by abi.encode."
        )]
        data: Bytes,
        #[structopt(about = "That specifies a dependency between operations.")]
        predecessor: H256,
        #[structopt(
            about = "Used to disambiguate two otherwise identical proposals. This can be any random value."
        )]
        salt: H256,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
}

impl Proposal {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Proposal::List {
                no_subgraph,
                from_block,
                to_block,
                latest,
                no_done,
                no_ready,
                no_pending,
                no_cancel,
            } => {
                if no_subgraph {
                    time_lock::load_proposals(
                        from_block, to_block, latest, no_done, no_ready, no_pending, no_cancel,
                    )
                    .await?
                } else {
                    time_lock::load_proposals_from_subgraph(
                        no_done, no_ready, no_pending, no_cancel,
                    )
                    .await?;
                }
            }
            Proposal::Schedule {
                target,
                value,
                data,
                predecessor,
                salt,
                delay,
                eth,
            } => {
                let time_lock = time_lock::init_timelock_call().await?;
                let calldata = ethers::prelude::Bytes::from(data.0);
                let _value = U256::from_str_radix(&value, 10)?;
                let _delay = U256::from_str_radix(&delay, 10)?;
                let payload = time_lock
                    .schedule(
                        target,
                        _value,
                        calldata,
                        *predecessor.as_fixed_bytes(),
                        *salt.as_fixed_bytes(),
                        _delay,
                    )
                    .calldata()
                    .unwrap();

                let multi_sig_wallet = multi_sig_wallet::init_wallet_send(&eth.private_key).await?;
                let destination = Address::from_str(WORMHOLE_DAO_TIME_LOCK)?;
                let call =
                    multi_sig_wallet.submit_transaction(destination, U256::default(), payload);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Proposal::Cancel { id, eth } => {
                let time_lock = time_lock::init_timelock_call().await?;
                let calldata = time_lock.cancel(*id.as_fixed_bytes()).calldata().unwrap();

                let multi_sig_wallet = multi_sig_wallet::init_wallet_send(&eth.private_key).await?;
                let destination = Address::from_str(WORMHOLE_DAO_TIME_LOCK)?;
                let call =
                    multi_sig_wallet.submit_transaction(destination, U256::default(), calldata);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Proposal::Execute {
                target,
                value,
                data,
                predecessor,
                salt,
                eth,
            } => {
                let time_lock = time_lock::init_timelock_send(&eth.private_key).await?;
                let calldata = ethers::prelude::Bytes::from(data.0);
                let _value = U256::from_str_radix(&value, 10)?;
                let call = time_lock
                    .execute(
                        target,
                        _value,
                        calldata,
                        *predecessor.as_fixed_bytes(),
                        *salt.as_fixed_bytes(),
                    )
                    .gas(500_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Proposal::ScheduleBatch { args, eth } => {
                let sig = "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)"
                    .to_string();
                let calldata = SimpleCast::calldata(sig, &args)?;
                let multi_sig_wallet = multi_sig_wallet::init_wallet_send(&eth.private_key).await?;
                let destination = Address::from_str(WORMHOLE_DAO_TIME_LOCK)?;
                let calldata = calldata.strip_prefix("0x").unwrap_or(&calldata);
                let calldata = hex::decode(calldata)?;
                let calldata = ethers::prelude::Bytes::from(calldata);
                let call =
                    multi_sig_wallet.submit_transaction(destination, U256::default(), calldata);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Proposal::ExecuteBatch { args, eth } => {
                let provider = Provider::<Http>::try_from(ETH_RPC_URL)?;
                let chain_id = provider.get_chainid().await.unwrap().as_u64();
                let key = eth
                    .private_key
                    .parse::<LocalWallet>()
                    .unwrap()
                    .with_chain_id(chain_id);
                let sig = "executeBatch(address[],uint256[],bytes[],bytes32,bytes32)".to_string();
                let to = Address::from_str(WORMHOLE_DAO_TIME_LOCK)?;
                let from = key.address();
                let data = SimpleCast::calldata(sig, &args)?;
                let data = data.strip_prefix("0x").unwrap_or(&data);
                let data = hex::decode(data)?;
                let data = ethers::prelude::Bytes::from(data);
                let tx = TransactionRequest::new().from(from).to(to).data(data);
                let client = SignerMiddleware::new(provider, key);
                let pending_tx = client
                    .send_transaction(TypedTransaction::Legacy(tx), None)
                    .await?;
                println!("{:?}", *pending_tx);
            }
        }
        Ok(())
    }
}
