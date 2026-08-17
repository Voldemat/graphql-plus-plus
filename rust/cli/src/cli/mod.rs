pub mod config;
pub mod internal;
pub mod utils;

const CLI_VERSION: &str = match option_env!("CLI_VERSION") {
    Some(v) => v,
    None => "unspecified",
};

#[derive(clap::Parser)]
#[command(version = CLI_VERSION, about, disable_help_subcommand = true)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Args)]
pub struct MainArgs {
    #[arg(
        short,
        long,
        help = "path to yaml config file",
        default_value = "./gql.yaml"
    )]
    pub config: std::path::PathBuf,
}

#[derive(clap::Args)]
pub struct FormatArgs {
    #[arg(
        short,
        long,
        help = "path to yaml config file",
        default_value = "./gql.yaml"
    )]
    pub config: std::path::PathBuf,
    #[arg(
        long,
        help = "check formatting and print the diff",
        default_value = "false"
    )]
    pub check: bool,
}

#[derive(clap::Subcommand)]
#[command(about)]
pub enum Commands {
    #[command(subcommand)]
    Internal(internal::Commands),
    Generate(MainArgs),
    Validate(MainArgs),
    Format(FormatArgs),
}

impl CLI {
    pub fn execute(self: Self) {
        self.command.execute();
    }
}

fn parse_config(config_path: &std::path::Path) -> config::Config {
    let buffer = std::fs::read_to_string(config_path).unwrap();
    return serde_yaml::from_str(&buffer).unwrap();
}

fn generate(args: MainArgs) {
    let config = parse_config(&args.config);
    utils::run_config_action(
        &args.config,
        &config,
        Box::new(|json_string, filepath, _| {
            if filepath == "-" {
                println!("{}", json_string);
            } else {
                let final_filepath = std::path::Path::join(
                    args.config.parent().unwrap(),
                    filepath,
                );
                std::fs::write(final_filepath, json_string).unwrap();
            }
        }),
    )
    .unwrap();
}

fn validate(args: MainArgs) {
    let config = parse_config(&args.config);
    utils::run_config_action(
        &args.config,
        &config,
        Box::new(|json_string, filepath, schema_name| {
            if filepath == "-" {
                return;
            }
            let final_filepath =
                std::path::Path::join(args.config.parent().unwrap(), filepath);
            utils::does_file_have_changes(
                &final_filepath,
                json_string,
                schema_name,
            )
            .unwrap();
        }),
    )
    .unwrap();
}

fn format_command(args: FormatArgs) {
    let config = parse_config(&args.config);
    let mut errors = Vec::<String>::new();
    let Some(formatting_config) = config.server.formatting else {
        eprintln!("No server.formatting config is defined");
        return;
    };
    let formatter_config = libgql::formatter::config::Config {
        indent_width: codeform::ir::shared::IndentWidth::from_u8(
            formatting_config.indent_width.into(),
        )
        .unwrap(),
    };
    let hir_to_lir_config = codeform::hir_to_lir::config::Config {
        indent_width: formatter_config.indent_width,
        max_width: formatting_config.max_line_width,
    };
    let lir_printer_config = codeform::lir_printer::Config {
        indent_width: formatter_config.indent_width,
        new_line_control_sequence: b"\n",
    };
    let mut is_success = true;
    for graphql_path in utils::resolve_paths(
        &args.config.parent().unwrap(),
        &config.server.inputs.graphql,
    ) {
        let buffer = std::fs::read_to_string(&graphql_path).unwrap();
        let source_file = std::sync::Arc::new(
            libgql::parsers::file::shared::ast::SourceFile {
                filepath: graphql_path.clone(),
                buffer: buffer.as_str(),
            },
        );
        let tokens =
            libgql::lexer::utils::parse_buffer_into_tokens(&source_file.buffer)
                .unwrap();
        let file_nodes = match libgql::parsers::file::server::Parser::new(
            libgql::parsers::file::tokens_sources::VecTokensSource::new(
                tokens,
                source_file.clone(),
            ),
        )
        .parse_ast_nodes()
        {
            Ok(n) => n,
            Err(e) => {
                errors.push(utils::format_parse_error(
                    &format!("{:?}", e),
                    e.get_location(),
                    &source_file,
                ));
                continue;
            }
        };
        let hir_nodes = libgql::formatter::server::nodes::format_nodes(
            &formatter_config,
            &file_nodes,
        );
        let mut hir_to_lir_state =
            codeform::hir_to_lir::state::State::default();
        let lir_nodes = codeform::hir_to_lir::mappers::nodes::lower(
            &hir_to_lir_config,
            &mut hir_to_lir_state,
            hir_nodes,
        );
        let mut printer_state = codeform::lir_printer::State::default();
        if args.check {
            let mut writer = std::io::BufWriter::new(Vec::<u8>::new());
            codeform::lir_printer::print_nodes(
                &mut writer,
                &lir_printer_config,
                &mut printer_state,
                &lir_nodes,
            )
            .unwrap();
            let formatted_string =
                String::from_utf8(writer.into_inner().unwrap()).unwrap();
            let text_diff =
                similar::TextDiff::from_lines(&buffer, &formatted_string);
            if text_diff.ratio() != 1.0 {
                is_success = false;
                println!(
                    "{}",
                    console::style(
                        format!("{}:", graphql_path.to_string_lossy())
                    ).blue()
                );

                // 3. Iterate over grouped hunks and print changes with line numbers
                for hunk in text_diff.grouped_ops(3) {
                    for op in hunk {
                        for change in text_diff.iter_changes(&op) {
                            // Format line numbers based on change type
                            let old_ln = change
                                .old_index()
                                .map(|idx| (idx + 1).to_string())
                                .unwrap_or_default();
                            let new_ln = change
                                .new_index()
                                .map(|idx| (idx + 1).to_string())
                                .unwrap_or_default();

                            match change.tag() {
                                similar::ChangeTag::Delete => {
                                    print!(
                                        "{:3} {:3} | {}",
                                        console::style(&old_ln).red(),
                                        console::style(&new_ln).dim(),
                                        console::style(format!("- {}", change)).red()
                                    );
                                }
                                similar::ChangeTag::Insert => {
                                    print!(
                                        "{:3} {:3} | {}",
                                        console::style(&old_ln).dim(),
                                        console::style(&new_ln).green(),
                                        console::style(format!("+ {}", change)).green()
                                    );
                                }
                                similar::ChangeTag::Equal => {
                                    print!(
                                        "{:3} {:3} |  {}",
                                        console::style(&old_ln).dim(),
                                        console::style(&new_ln).dim(),
                                        change
                                    );
                                }
                            }
                        }
                    }
                }
            }
        } else {
            let mut writer = std::io::BufWriter::new(
                std::fs::File::create(graphql_path).unwrap(),
            );
            codeform::lir_printer::print_nodes(
                &mut writer,
                &lir_printer_config,
                &mut printer_state,
                &lir_nodes,
            )
            .unwrap();
        }
    }
    if !is_success {
        std::process::exit(1);
    }
}

impl Commands {
    pub fn execute(self: Self) {
        match self {
            Commands::Internal(internal) => internal.execute(),
            Commands::Generate(args) => generate(args),
            Commands::Validate(args) => validate(args),
            Commands::Format(args) => format_command(args),
        }
    }
}
