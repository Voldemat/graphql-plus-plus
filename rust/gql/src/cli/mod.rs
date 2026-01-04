pub mod internal;
pub mod utils;

#[derive(clap::Parser)]
#[command(version, about)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Internal(internal::Commands),
}

impl CLI {
    pub fn execute(self: &Self) {
        self.command.execute();
    }
}

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Internal(internal) => internal.execute(),
        }
    }
}
