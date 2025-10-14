pub mod address;
pub mod generate;
pub mod send;

#[derive(Debug, clap::Args)]
pub struct Wallet {
    #[command(subcommand)]
    pub command: WalletCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum WalletCommand {
    /// Get the address from an encrypted key file
    Address(address::Address),
    /// Generate a new encrypted key pair
    Generate(generate::Generate),
    /// Send a payment transaction
    Send(send::Send),
}

impl Wallet {
    pub fn run(self) -> anyhow::Result<()> {
        match self.command {
            WalletCommand::Address(cmd) => cmd.run(),
            WalletCommand::Generate(cmd) => cmd.run(),
            WalletCommand::Send(cmd) => cmd.run(),
        }
    }
}
