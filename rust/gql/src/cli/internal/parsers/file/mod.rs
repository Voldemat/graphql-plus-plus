pub mod client;
pub mod server;

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Client(client::Commands),
    #[command(subcommand)]
    Server(server::Commands),
}

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Client(client) => client.execute(),
            Commands::Server(server) => server.execute(),
        }
    }
}

