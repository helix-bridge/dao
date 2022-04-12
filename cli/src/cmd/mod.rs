mod conf;
mod multi_sig_wallet;
mod pausable;
mod proposal;
mod seth;
mod time_lock;
mod utils;

use structopt::StructOpt;

#[derive(StructOpt)]
pub struct EthereumOpts {
    #[structopt(
        long = "private-key",
        env = "ETH_PRIVATE_KEY",
        help = "Your private key string"
    )]
    private_key: String,
}

#[derive(StructOpt)]
#[structopt(about = "Dao utilities")]
pub enum Command {
    #[structopt(name = "wallet")]
    MultiSigWallet(multi_sig_wallet::MultiSigWallet),
    #[structopt(name = "timelock")]
    TimeLock(time_lock::TimeLock),
    #[structopt(name = "pausable")]
    Pausable(pausable::Pausable),
    #[structopt(name = "proposal")]
    Proposal(proposal::Proposal),
    #[structopt(name = "seth")]
    Seth(seth::Cast),
}

impl Command {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Command::MultiSigWallet(cmd) => cmd.run().await?,
            Command::TimeLock(cmd) => cmd.run().await?,
            Command::Pausable(cmd) => cmd.run().await?,
            Command::Proposal(cmd) => cmd.run().await?,
            Command::Seth(cmd) => cmd.run().await?,
        }
        Ok(())
    }
}

pub fn parse_args() -> Command {
    Command::from_args()
}
