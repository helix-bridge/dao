use super::EthereumOpts;
use crate::bindings::time_lock::{
    CallScheduledFilter, TimeLock as TimeLockContract, TimeLockEvents,
};
use crate::cmd::conf::*;
use crate::cmd::utils::Bytes;
use crate::graphql::proposal::{proposal_view, ProposalView};
use async_recursion::async_recursion;
use cast::SimpleCast;
use ethers::core::types::transaction::eip2718::TypedTransaction;
use ethers::core::types::transaction::request::TransactionRequest;
use ethers::prelude::*;
use graphql_client::{GraphQLQuery, Response};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter, Result};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{convert::TryFrom, str::FromStr, sync::Arc};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "TimeLock related commands.")]
pub enum TimeLock {
    #[structopt(name = "min-delay")]
    MinDelay,
    #[structopt(name = "proposal")]
    Proposals(Proposal),
    #[structopt(name = "role")]
    Roles(Role),
}

#[derive(StructOpt)]
#[structopt(about = "TimeLock proposal related commands.")]
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
    },
    #[structopt(about = "Schedule an operation containing a batch of transactions.")]
    ScheduleBatch { args: Vec<String> },
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

#[derive(StructOpt)]
#[structopt(about = "TimeLock role related commands.")]
pub enum Role {
    IsAdmin {
        account: Address,
    },
    IsProposer {
        account: Address,
    },
    IsExecutor {
        account: Address,
    },
    Grant {
        #[structopt(about = "1: admin, 2: proposer, 3: executor")]
        role: u8,
        account: Address,
    },
    Revoke {
        #[structopt(about = "1: admin, 2: proposer, 3: executor")]
        role: u8,
        account: Address,
    },
}

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
pub enum ProposalStatus {
    Pending,
    Ready,
    Executed,
    Cancelled,
}

#[derive(Hash, Clone, Debug, Eq, PartialEq)]
pub struct ProposalItem {
    id: [u8; 32],
    index: U256,
    target: Address,
    value: U256,
    data: ethers::prelude::Bytes,
    predecessor: [u8; 32],
    delay: U256,
    timestamp: u64,
    status: ProposalStatus,
}

impl ProposalItem {
    fn from(filter: &CallScheduledFilter) -> Self {
        ProposalItem {
            id: filter.id,
            index: filter.index,
            target: filter.target,
            value: filter.value,
            data: filter.data.clone(),
            predecessor: filter.predecessor,
            delay: filter.delay,
            timestamp: 0,
            status: ProposalStatus::Pending,
        }
    }
}

impl Display for ProposalItem {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "id: {}\nindex: {}\ntarget: {:?}\nvalue: {}\ndata: {}\npredecessor: {}\ntimestamp: {}\nstatus: {:?}",
            hex::encode(self.id),
            self.index,
            self.target,
            self.value,
            self.data,
            hex::encode(self.predecessor),
			self.timestamp,
            self.status
        )
    }
}

impl TimeLock {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            TimeLock::MinDelay => {
                let time_lock = init_timelock_call().await?;
                let min_delay = time_lock.get_min_delay().call().await?;
                println!("{}", min_delay);
            }
            TimeLock::Proposals(_p) => _p.run().await?,
            TimeLock::Roles(_r) => _r.run().await?,
        }
        Ok(())
    }
}

impl Role {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Role::IsAdmin { account } => {
                let time_lock = init_timelock_call().await?;
                let timelock_admin_role = time_lock.timelock_admin_role().call().await?;
                let is = time_lock
                    .has_role(timelock_admin_role, account)
                    .call()
                    .await?;
                println!("{}", is);
            }
            Role::IsProposer { account } => {
                let time_lock = init_timelock_call().await?;
                let proposer_role = time_lock.proposer_role().call().await?;
                let is = time_lock.has_role(proposer_role, account).call().await?;
                println!("{}", is);
            }
            Role::IsExecutor { account } => {
                let time_lock = init_timelock_call().await?;
                let executor_role = time_lock.executor_role().call().await?;
                let is = time_lock.has_role(executor_role, account).call().await?;
                println!("{}", is);
            }
            Role::Grant { role, account } => {
                let time_lock = init_timelock_call().await?;
                let role = if role == 1 {
                    time_lock.timelock_admin_role().call().await?
                } else if role == 2 {
                    time_lock.proposer_role().call().await?
                } else if role == 3 {
                    time_lock.executor_role().call().await?
                } else {
                    panic!("unexpect role");
                };
                let calldata = time_lock.grant_role(role, account).calldata().unwrap();
                println!("{}", calldata);
            }
            Role::Revoke { role, account } => {
                let time_lock = init_timelock_call().await?;
                let role = if role == 1 {
                    time_lock.timelock_admin_role().call().await?
                } else if role == 2 {
                    time_lock.proposer_role().call().await?
                } else if role == 3 {
                    time_lock.executor_role().call().await?
                } else {
                    panic!("unexpect role");
                };
                let calldata = time_lock.revoke_role(role, account).calldata().unwrap();
                println!("{}", calldata);
            }
        }
        Ok(())
    }
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
                    load_proposals(
                        from_block, to_block, latest, no_done, no_ready, no_pending, no_cancel,
                    )
                    .await?;
                } else {
                    load_proposals_from_subgraph(no_done, no_ready, no_pending, no_cancel).await?;
                }
            }
            Proposal::Schedule {
                target,
                value,
                data,
                predecessor,
                salt,
                delay,
            } => {
                let time_lock = init_timelock_call().await?;
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
                println!("{}", payload);
            }
            Proposal::ScheduleBatch { args } => {
                let sig = "scheduleBatch(address[],uint256[],bytes[],bytes32,bytes32,uint256)"
                    .to_string();
                println!("{}", SimpleCast::calldata(sig, &args)?);
            }
            Proposal::Cancel { id } => {
                let time_lock = init_timelock_call().await?;
                let calldata = time_lock.cancel(*id.as_fixed_bytes()).calldata().unwrap();
                println!("{}", calldata);
            }
            Proposal::Execute {
                target,
                value,
                data,
                predecessor,
                salt,
                eth,
            } => {
                let time_lock = init_timelock_send(&eth.private_key).await?;
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

pub async fn load_proposals_from_subgraph(
    no_done: bool,
    no_ready: bool,
    no_pending: bool,
    no_cancel: bool,
) -> eyre::Result<()> {
    let client = reqwest::Client::new();
    let q = ProposalView::build_query(proposal_view::Variables {});
    let res = client.post(SUBGRAPG_URL).json(&q).send().await?;
    let response_body: Response<proposal_view::ResponseData> = res.json().await?;
    let response_data: proposal_view::ResponseData = response_body.data.unwrap();
    let mut statuses: HashSet<proposal_view::Status> = [
        proposal_view::Status::Executed,
        proposal_view::Status::Pending,
        proposal_view::Status::Ready,
        proposal_view::Status::Cancelled,
    ]
    .iter()
    .cloned()
    .collect();
    if no_done {
        statuses.remove(&proposal_view::Status::Executed);
    }
    if no_pending {
        statuses.remove(&proposal_view::Status::Pending);
    }
    if no_ready {
        statuses.remove(&proposal_view::Status::Ready);
    }
    if no_cancel {
        statuses.remove(&proposal_view::Status::Cancelled);
    }
    for p in response_data.proposals.iter() {
        if statuses.contains(&p.status) {
            println!(
                "============================================================================="
            );
            println!("{}", p);
        }
    }
    println!("Proposals Count: {}", response_data.proposals.len());
    Ok(())
}

pub async fn load_proposals(
    from_block: String,
    to_block: Option<String>,
    latest: Option<String>,
    no_done: bool,
    no_ready: bool,
    no_pending: bool,
    no_cancel: bool,
) -> eyre::Result<()> {
    let time_lock = init_timelock_call().await?;
    let _to_block = if let Some(to_block) = to_block {
        U64::from_str_radix(&to_block, 10)?
    } else {
        time_lock.client().get_block_number().await?
    };
    let mut _from_block = U64::from_str_radix(&from_block, 10)?;
    if let Some(latest) = latest {
        _from_block = _to_block - U64::from_str_radix(&latest, 10)?;
    }
    let now = timestamp();
    let mut proposals: HashMap<[u8; 32], ProposalItem> = HashMap::new();
    let mut events = load_events(time_lock.clone(), &_from_block, &_to_block).await;
    events.sort_by(|a, b| a.1.block_number.cmp(&b.1.block_number));
    for event in events {
        match &event.0 {
            TimeLockEvents::CallScheduledFilter(data) => {
                let mut proposal = ProposalItem::from(data);
                let ts = time_lock.get_timestamp(proposal.id).call().await?;
                proposal.timestamp = ts.as_u64();
                if ts.as_u64() < now {
                    proposal.status = ProposalStatus::Ready;
                }
                proposals.insert(data.id, proposal);
            }
            TimeLockEvents::CallExecutedFilter(data) => {
                if proposals.contains_key(&data.id) {
                    proposals.get_mut(&data.id).unwrap().status = ProposalStatus::Executed;
                } else {
                    println!("proposal not exist");
                }
            }
            TimeLockEvents::CancelledFilter(data) => {
                if proposals.contains_key(&data.id) {
                    proposals.get_mut(&data.id).unwrap().status = ProposalStatus::Cancelled;
                } else {
                    println!("proposal not exist");
                }
            }
            TimeLockEvents::MinDelayChangeFilter(_) => {}
            TimeLockEvents::RoleAdminChangedFilter(_) => {}
            TimeLockEvents::RoleGrantedFilter(_) => {}
            TimeLockEvents::RoleRevokedFilter(_) => {}
        }
    }
    let mut statuses: HashSet<ProposalStatus> = [
        ProposalStatus::Executed,
        ProposalStatus::Pending,
        ProposalStatus::Ready,
        ProposalStatus::Cancelled,
    ]
    .iter()
    .cloned()
    .collect();
    if no_done {
        statuses.remove(&ProposalStatus::Executed);
    }
    if no_pending {
        statuses.remove(&ProposalStatus::Pending);
    }
    if no_ready {
        statuses.remove(&ProposalStatus::Ready);
    }
    if no_cancel {
        statuses.remove(&ProposalStatus::Cancelled);
    }
    proposals.retain(|_, v| -> bool { statuses.contains(&v.status) });
    for p in proposals.values() {
        println!("=============================================================================");
        println!("{}", p);
    }
    Ok(())
}

pub fn timestamp() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs()
}

#[async_recursion]
pub async fn load_events(
    contract: TimeLockContract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
    from_block: &U64,
    to_block: &U64,
) -> Vec<(TimeLockEvents, LogMeta)> {
    let events = contract
        .events()
        .from_block(from_block)
        .to_block(to_block)
        .query_with_meta()
        .await;

    match events {
        Ok(result) => result,
        Err(err) => {
            println!("Query err: {:?}", err);

            let mid_block = (from_block + to_block) / 2u64;
            if mid_block == *from_block || mid_block == *to_block {
                panic!("range is already narrow");
            }

            let mut left_part = load_events(contract.clone(), from_block, &mid_block).await;

            let mut right_part = load_events(contract.clone(), &(mid_block + 1u64), to_block).await;
            left_part.append(&mut right_part);
            left_part
        }
    }
}

pub async fn init_timelock_call(
) -> eyre::Result<TimeLockContract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>>
{
    Ok(init_timelock_send(DEFAULT_PRIVATE_KEY).await?)
}

pub async fn init_timelock_send(
    private_key: &str,
) -> eyre::Result<TimeLockContract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>>
{
    let provider = Provider::<Http>::try_from(ETH_RPC_URL)?;
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let key = private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);
    let to = Address::from_str(WORMHOLE_DAO_TIME_LOCK)?;
    let client = SignerMiddleware::new(provider, key);
    let client = Arc::new(client);
    let time_lock = TimeLockContract::new(to, client);
    Ok(time_lock)
}
