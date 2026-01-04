pub mod file;

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    File(file::Commands),
}

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::File(file) => file.execute(),
        }
    }
}

