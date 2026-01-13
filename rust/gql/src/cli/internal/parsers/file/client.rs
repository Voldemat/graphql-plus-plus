use std::rc::Rc;

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
        help = "filepath to tokens json file, - for stdin",
        default_value = "-"
    )]
    filepath: std::path::PathBuf,
    #[arg(short, long, help = "Prettify output")]
    pretty: bool,
}

fn parse(args: &ParseArgs) {
    let buffer = utils::read_buffer_from_filepath(&args.filepath);
    let tokens: Vec<libgql::lexer::tokens::Token> =
        serde_json::from_str(&buffer).unwrap();
    let source_file = Rc::new(libgql::parsers::file::shared::ast::SourceFile {
        buffer,
        filepath: args.filepath.clone()
    });
    let nodes = libgql::parsers::file::client::Parser::new(
        libgql::parsers::file::tokens_sources::VecTokensSource::new(
            tokens,
            source_file
        )
    ).parse_ast_nodes().unwrap();
    utils::print_result(args.pretty, &nodes);
}

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Parse(args) => parse(args),
        }
    }
}
