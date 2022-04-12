use crate::bindings::pausable::Pausable as PausableContract;
use crate::cmd::conf::*;
use ethers::prelude::*;
use std::{convert::TryFrom, sync::Arc};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "Pausable related commands")]
pub enum Pausable {
    #[structopt(about = "Show paused status")]
    Paused {
        addr: Address,
    },
    Pause {
        to: Address,
    },
    Unpause {
        to: Address,
    },
}

impl Pausable {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Pausable::Paused { addr } => {
                let pausable = init_pausable_call(addr).await?;
                let paused = pausable.paused().call().await?;
                println!("{}", paused);
            }
            Pausable::Pause { to } => {
                let pausable = init_pausable_call(to).await?;
                let calldata = pausable.pause().calldata().unwrap();
                println!("{}", calldata);
            }
            Pausable::Unpause { to } => {
                let pausable = init_pausable_call(to).await?;
                let calldata = pausable.unpause().calldata().unwrap();
                println!("{}", calldata);
            }
        }
        Ok(())
    }
}

pub async fn init_pausable_call(
    to: Address,
) -> eyre::Result<PausableContract<SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>>>
{
    let provider = Provider::<Http>::try_from(ETH_RPC_URL)?;
    let chain_id = provider.get_chainid().await.unwrap().as_u64();
    let key = DEFAULT_PRIVATE_KEY
        .parse::<LocalWallet>()
        .unwrap()
        .with_chain_id(chain_id);
    let client = SignerMiddleware::new(provider, key);
    let client = Arc::new(client);
    let pausable = PausableContract::new(to, client);
    Ok(pausable)
}
