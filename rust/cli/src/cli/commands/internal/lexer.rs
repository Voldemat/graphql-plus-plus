use crate::cli::utils;

#[derive(clap::Subcommand)]
pub enum Commands {
    Parse(ParseArgs),
}

#[derive(clap::Args)]
pub struct ParseArgs {
    #[arg(
        short,
        long,
        help = "filepath to graphql file, \"-\" for stdin",
        default_value = "-"
    )]
    filepath: std::path::PathBuf,
    #[arg(short, long, help = "Prettify output")]
    pretty: bool,
}

fn parse(args: &ParseArgs) {
    let buffer = utils::read_buffer_from_filepath(&args.filepath);
    let tokens =
        libgql::lexer::utils::parse_buffer_into_tokens(&buffer).unwrap();
    utils::print_result(args.pretty, tokens);
}

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Parse(args) => parse(args),
        }
    }
}
