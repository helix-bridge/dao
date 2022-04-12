use crate::bindings::multi_sig_wallet::MultiSigWallet as MultiSigWalletContract;
use crate::cmd::conf::*;
use crate::cmd::utils::Bytes;
use ethers::prelude::*;
use std::{convert::TryFrom, str::FromStr, sync::Arc};
use structopt::StructOpt;

use super::EthereumOpts;

#[derive(StructOpt)]
#[structopt(about = "MultiSigWallet related commands.")]
pub enum MultiSigWallet {
    #[structopt(name = "owner")]
    Owners(Owner),
    #[structopt(about = "Required confirmations")]
    #[structopt(name = "threshold")]
    Threshold,
    #[structopt(about = "Set required confirmations")]
    #[structopt(name = "set-threshold")]
    SetThreshold {
        #[structopt(about = "New threshold required")]
        required: String,
    },
    #[structopt(name = "tx")]
    Transactions(Tx),
}

#[derive(StructOpt)]
#[structopt(about = "Owners management related commands.")]
pub enum Owner {
    #[structopt(about = "Owner list.")]
    List,
    #[structopt(about = "Add new owner")]
    Add { new_owner: Address },
    #[structopt(about = "Remove an owner")]
    Remove { rm_owner: Address },
    #[structopt(about = "Replace an owner with a new owner")]
    Replace {
        old_owner: Address,
        new_owner: Address,
    },
}

#[derive(StructOpt)]
#[structopt(about = "MultiSigWallet transaction related commands.")]
pub enum Tx {
    #[structopt(about = "Transaction list.")]
    List {
        #[structopt(default_value = "0")]
        from: String,

        to: Option<String>,
        #[structopt(long)]
        latest: Option<String>,
        #[structopt(long)]
        no_pending: bool,
        #[structopt(long)]
        no_executed: bool,
    },
    #[structopt(about = "Submit and confirm a transaction.")]
    Submit {
        destination: Address,
        value: String,
        data: Bytes,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Confirm a transaction.")]
    Confirm {
        #[structopt(about = "Transaction ID.")]
        tx_id: String,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Execute a confirmed transaction.")]
    Execute {
        #[structopt(about = "Transaction ID.")]
        tx_id: String,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
    #[structopt(about = "Revoke a confirmation for a transaction.")]
    Revoke {
        #[structopt(about = "Transaction ID.")]
        tx_id: String,
        #[structopt(flatten)]
        eth: EthereumOpts,
    },
}

impl MultiSigWallet {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Self::Owners(_owner) => _owner.run().await?,
            Self::Transactions(_sub) => _sub.run().await?,
            Self::Threshold => {
                let multi_sig_wallet = init_wallet_call().await?;
                let threshold = multi_sig_wallet.required().call().await?;
                println!("{}", threshold);
            }
            Self::SetThreshold { required } => {
                let multi_sig_wallet = init_wallet_call().await?;
                let _required = U256::from_str_radix(&required, 10)?;
                let calldata = multi_sig_wallet
                    .change_requirement(_required)
                    .calldata()
                    .unwrap();
                println!("{}", calldata);
            }
        }
        Ok(())
    }
}

impl Owner {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Owner::List => {
                let multi_sig_wallet = init_wallet_call().await?;
                let owners = multi_sig_wallet.get_owners().call().await?;
                println!("{:?}", owners);
            }
            Owner::Add { new_owner } => {
                let multi_sig_wallet = init_wallet_call().await?;
                let calldata = multi_sig_wallet.add_owner(new_owner).calldata().unwrap();
                println!("{}", calldata);
            }
            Owner::Remove { rm_owner } => {
                let multi_sig_wallet = init_wallet_call().await?;
                let calldata = multi_sig_wallet.remove_owner(rm_owner).calldata().unwrap();
                println!("{}", calldata);
            }
            Owner::Replace {
                old_owner,
                new_owner,
            } => {
                let multi_sig_wallet = init_wallet_call().await?;
                let calldata = multi_sig_wallet
                    .replace_owner(old_owner, new_owner)
                    .calldata()
                    .unwrap();
                println!("{}", calldata);
            }
        }
        Ok(())
    }
}

impl Tx {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Tx::List {
                from,
                to,
                latest,
                no_pending,
                no_executed,
            } => {
                let multi_sig_wallet = init_wallet_call().await?;
                let tx_count = if let Some(_to) = to {
                    U256::from_str_radix(&_to, 10)?
                } else {
                    multi_sig_wallet.transaction_count().call().await?
                };
                let _from = if let Some(latest) = latest {
                    tx_count - U256::from_str_radix(&latest, 10)?
                } else {
                    U256::from_str_radix(&from, 10)?
                };
                let ids = multi_sig_wallet
                    .get_transaction_ids(_from, tx_count, !no_pending, !no_executed)
                    .call()
                    .await?;
                for id in ids {
                    // multicall
                    let tx = multi_sig_wallet.transactions(id).call().await?;
                    if !no_executed && tx.3 || !no_pending && !tx.3 {
                        let comfirms = multi_sig_wallet.get_confirmations(id).call().await?;
                        println!("========================================================");
                        println!("id            : {}", id);
                        println!("destination   : {:?}", tx.0);
                        println!("value         : {}", tx.1);
                        println!("data          : {}", tx.2);
                        println!("Confirmations : {:?}", comfirms);
                        println!("executed      : {}", tx.3);
                    }
                }
            }
            Tx::Submit {
                destination,
                value,
                data,
                eth,
            } => {
                let multi_sig_wallet = init_wallet_send(&eth.private_key).await?;
                let calldata = ethers::prelude::Bytes::from(data.0);
                let _value = U256::from_str_radix(&value, 10)?;
                let call = multi_sig_wallet.submit_transaction(destination, _value, calldata);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Tx::Confirm { tx_id, eth } => {
                let multi_sig_wallet = init_wallet_send(&eth.private_key).await?;
                let _tx_id = U256::from_str_radix(&tx_id, 10)?;
                let call = multi_sig_wallet.confirm_transaction(_tx_id).gas(500_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Tx::Execute { tx_id, eth } => {
                let _tx_id = U256::from_str_radix(&tx_id, 10)?;
                let multi_sig_wallet = init_wallet_send(&eth.private_key).await?;
                let call = multi_sig_wallet.execute_transaction(_tx_id).gas(500_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
            Tx::Revoke { tx_id, eth } => {
                let _tx_id = U256::from_str_radix(&tx_id, 10)?;
                let multi_sig_wallet = init_wallet_send(&eth.private_key).await?;
                let call = multi_sig_wallet.revoke_confirmation(_tx_id).gas(100_000);
                let pending_tx = call.send().await?;
                println!("{:?}", *pending_tx);
            }
        }
        Ok(())
    }
}

pub async fn init_wallet_call() -> eyre::Result<
    MultiSigWalletContract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
> {
    Ok(init_wallet_send(DEFAULT_PRIVATE_KEY).await?)
}

pub async fn init_wallet_send(
    private_key: &str,
) -> eyre::Result<
    MultiSigWalletContract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>,
> {
    let provider = Provider::<Http>::try_from(ETH_RPC_URL)?;
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let key = private_key
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);
    let to = Address::from_str(WORMHOLE_DAO_MULTISIG)?;
    let client = SignerMiddleware::new(provider, key);
    let client = Arc::new(client);
    let multi_sig_wallet = MultiSigWalletContract::new(to, client);
    Ok(multi_sig_wallet)
}
