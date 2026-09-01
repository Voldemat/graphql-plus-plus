const GQLUP_VERSION: &str = match option_env!("GQLUP_VERSION") {
    Some(v) => v,
    None => "unspecified",
};

#[derive(clap::Parser)]
#[command(version = GQLUP_VERSION, about, disable_help_subcommand = true)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(about = "Check for a newer gql release and install it")]
    Update,
}

impl CLI {
    pub fn execute(self: Self) {
        let force = matches!(self.command, Some(Commands::Update));
        if let Err(error) = crate::install::run(force) {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
