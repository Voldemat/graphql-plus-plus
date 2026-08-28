mod format;
mod generate;
mod internal;
mod validate;

#[derive(clap::Subcommand)]
#[command(about)]
pub enum Commands {
    #[command(subcommand)]
    Internal(internal::Commands),
    Generate(generate::Args),
    Validate(validate::Args),
    Format(format::Args),
}

impl Commands {
    pub fn execute(self: Self) {
        match self {
            Commands::Internal(internal) => internal.execute(),
            Commands::Generate(args) => args.execute(),
            Commands::Validate(args) => args.execute(),
            Commands::Format(args) => args.execute(),
        }
    }
}
