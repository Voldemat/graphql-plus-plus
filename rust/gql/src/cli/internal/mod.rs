pub mod lexer;
pub mod parsers;

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Lexer(lexer::Commands),
    #[command(subcommand)]
    Parsers(parsers::Commands),
}

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Lexer(lexer) => lexer.execute(),
            Commands::Parsers(parsers) => parsers.execute(),
        }
    }
}
