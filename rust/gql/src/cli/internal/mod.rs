pub mod lexer;
pub mod parsers;
pub mod executor;

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Lexer(lexer::Commands),
    #[command(subcommand)]
    Parsers(parsers::Commands),
    #[command(subcommand)]
    Executor(executor::Commands),
}

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Lexer(lexer) => lexer.execute(),
            Commands::Parsers(parsers) => parsers.execute(),
            Commands::Executor(e) => e.execute(),
        }
    }
}
