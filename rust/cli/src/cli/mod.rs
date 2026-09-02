mod commands;
mod config;
mod format_error;
mod shared;
mod utils;

#[derive(clap::Parser)]
#[command(version = config::CLI_VERSION, about, disable_help_subcommand = true)]
pub struct CLI {
    #[command(subcommand)]
    pub command: commands::Commands,
}

impl CLI {
    pub fn execute(self: Self) {
        self.command.execute();
    }
}
