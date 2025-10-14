pub mod address;
pub mod send;

#[derive(Debug, clap::Args)]
pub struct Wallet {
    #[command(subcommand)]
    pub command: WalletCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum WalletCommand {
    /// Send a payment transaction
    Send(send::Send),
    /// Get the address from an encrypted key file
    Address(address::Address),
}

impl Wallet {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            WalletCommand::Send(cmd) => cmd.run(),
            WalletCommand::Address(cmd) => cmd.run(),
        }
    }
}
