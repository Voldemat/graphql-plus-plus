pub mod executor;
pub mod lexer;
pub mod parsers;
pub mod diff;


#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Lexer(lexer::Commands),
    #[command(subcommand)]
    Parsers(parsers::Commands),
    #[command(subcommand)]
    Executor(executor::Commands),
    Diff(diff::DiffArgs),
}


impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Lexer(lexer) => lexer.execute(),
            Commands::Parsers(parsers) => parsers.execute(),
            Commands::Executor(e) => e.execute(),
            Commands::Diff(args) => diff::command(args),
        }
    }
}
