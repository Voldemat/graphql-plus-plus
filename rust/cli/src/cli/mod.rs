pub mod config;
pub mod internal;
pub mod text_diff;
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
    let config: config::Config = serde_yaml::from_str(&buffer).unwrap();
    if let Some(config_version) = &config.version
        && config_version != CLI_VERSION
    {
        eprintln!(
            "Version mismatch. cli version: {} and config version {}",
            CLI_VERSION, config_version
        );
        std::process::exit(1);
    }
    config
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

pub fn buffer_to_server_ast<'buffer>(
    graphql_path: &std::path::PathBuf,
    buffer: &'buffer str,
) -> Result<Vec<libgql::parsers::file::server::ast::ASTNode<'buffer>>, String> {
    let source_file =
        std::sync::Arc::new(libgql::parsers::file::shared::ast::SourceFile {
            filepath: graphql_path.clone(),
            buffer: buffer,
        });
    let tokens =
        libgql::lexer::utils::parse_buffer_into_tokens(&source_file.buffer)
            .unwrap();
    libgql::parsers::file::server::Parser::new(
        libgql::parsers::file::tokens_sources::VecTokensSource::new(
            tokens,
            source_file.clone(),
        ),
    )
    .parse_ast_nodes()
    .map_err(|e| {
        utils::format_parse_error(
            &format!("{:?}", e),
            e.get_location(),
            &source_file,
        )
    })
}

pub fn buffer_to_client_ast<'buffer>(
    graphql_path: &std::path::PathBuf,
    buffer: &'buffer str,
) -> Result<Vec<libgql::parsers::file::client::ast::ASTNode<'buffer>>, String> {
    let source_file =
        std::sync::Arc::new(libgql::parsers::file::shared::ast::SourceFile {
            filepath: graphql_path.clone(),
            buffer: buffer,
        });
    let tokens =
        libgql::lexer::utils::parse_buffer_into_tokens(&source_file.buffer)
            .unwrap();
    libgql::parsers::file::client::Parser::new(
        libgql::parsers::file::tokens_sources::VecTokensSource::new(
            tokens,
            source_file.clone(),
        ),
    )
    .parse_ast_nodes()
    .map_err(|e| {
        utils::format_parse_error(
            &format!("{:?}", e),
            &e.get_location(),
            &source_file,
        )
    })
}

fn server_nodes_to_lir_nodes<'buffer>(
    shared_formatter_config: &libgql::formatter::shared::config::Config,
    hir_to_lir_config: &codeform::hir_to_lir::config::Config,
    ast_nodes: &[libgql::parsers::file::server::ast::ASTNode<'buffer>],
) -> Vec<codeform::ir::lir::node::Node<'buffer>> {
    let hir_nodes = libgql::formatter::server::nodes::format_nodes(
        &libgql::formatter::server::config::Config {
            shared: &shared_formatter_config,
        },
        &ast_nodes,
    );
    let mut hir_to_lir_state = codeform::hir_to_lir::state::State::default();
    codeform::hir_to_lir::mappers::nodes::lower(
        hir_to_lir_config,
        &mut hir_to_lir_state,
        hir_nodes,
    )
}

fn client_nodes_to_lir_nodes<'buffer>(
    shared_formatter_config: &libgql::formatter::shared::config::Config,
    hir_to_lir_config: &codeform::hir_to_lir::config::Config,
    ast_nodes: &[libgql::parsers::file::client::ast::ASTNode<'buffer>],
) -> Vec<codeform::ir::lir::node::Node<'buffer>> {
    let hir_nodes = libgql::formatter::client::nodes::format_nodes(
        &libgql::formatter::client::config::Config {
            shared: &shared_formatter_config,
        },
        &ast_nodes,
    );
    let mut hir_to_lir_state = codeform::hir_to_lir::state::State::default();
    codeform::hir_to_lir::mappers::nodes::lower(
        hir_to_lir_config,
        &mut hir_to_lir_state,
        hir_nodes,
    )
}

fn format_action(
    graphql_path: &std::path::Path,
    lir_printer_config: &codeform::lir_printer::Config,
    lir_nodes: Vec<codeform::ir::lir::node::Node>,
) -> Result<(), String> {
    let mut writer =
        std::io::BufWriter::new(std::fs::File::create(graphql_path).unwrap());
    let mut printer_state = codeform::lir_printer::State::default();
    codeform::lir_printer::print_nodes(
        &mut writer,
        &lir_printer_config,
        &mut printer_state,
        &lir_nodes,
    )
    .unwrap();
    Ok(())
}

fn format_check_action(
    graphql_path: &std::path::Path,
    initial_buffer: &str,
    lir_printer_config: &codeform::lir_printer::Config,
    lir_nodes: Vec<codeform::ir::lir::node::Node>,
) -> Result<(), String> {
    let mut writer = std::io::BufWriter::new(Vec::<u8>::new());
    let mut printer_state = codeform::lir_printer::State::default();
    codeform::lir_printer::print_nodes(
        &mut writer,
        &lir_printer_config,
        &mut printer_state,
        &lir_nodes,
    )
    .unwrap();
    let formatted_string =
        String::from_utf8(writer.into_inner().unwrap()).unwrap();
    match text_diff::get_diff_string(initial_buffer, &formatted_string) {
        None => Ok(()),
        Some(diff_string) => Err(format!(
            "{}\n{}",
            console::style(format!("{}:", graphql_path.to_string_lossy()))
                .blue(),
            diff_string
        )),
    }
}

fn format_command(args: FormatArgs) {
    let config = parse_config(&args.config);
    let mut errors = Vec::<String>::new();
    let Some(formatting_config) = config.formatting else {
        eprintln!("No formatting config is defined");
        return;
    };
    let shared_formatter_config = libgql::formatter::shared::config::Config {
        indent_width: codeform::ir::shared::IndentWidth::from_u8(
            formatting_config.shared.indent_width.into(),
        )
        .unwrap(),
    };
    let hir_to_lir_config = codeform::hir_to_lir::config::Config {
        indent_width: shared_formatter_config.indent_width,
        max_width: formatting_config.shared.max_line_width,
    };
    let lir_printer_config = codeform::lir_printer::Config {
        indent_width: shared_formatter_config.indent_width,
        new_line_control_sequence: b"\n",
    };
    if let Some(formatting_server_config) = formatting_config.server {
        let Some(config_server) = config.server.as_ref() else {
            eprintln!("config.server is not defined");
            return;
        };
        for graphql_path in utils::resolve_paths(
            &args.config.parent().unwrap(),
            &config_server.inputs.graphql,
        ) {
            let buffer = std::fs::read_to_string(&graphql_path).unwrap();
            let ast_nodes = match buffer_to_server_ast(&graphql_path, &buffer) {
                Ok(nodes) => nodes,
                Err(error) => {
                    errors.push(error);
                    continue;
                }
            };
            let lir_nodes = server_nodes_to_lir_nodes(
                &shared_formatter_config,
                &hir_to_lir_config,
                &ast_nodes,
            );
            if args.check {
                if let Err(error) = format_check_action(
                    &graphql_path,
                    &buffer,
                    &lir_printer_config,
                    lir_nodes,
                ) {
                    errors.push(error);
                }
            } else {
                if let Err(error) =
                    format_action(&graphql_path, &lir_printer_config, lir_nodes)
                {
                    errors.push(error);
                }
            }
        }
    }

    if let Some(formatting_client_config) = formatting_config.client {
        let Some(config_client) = config.client.as_ref() else {
            eprintln!("config.client is not defined");
            return;
        };
        for graphql_path in utils::resolve_paths(
            &args.config.parent().unwrap(),
            &config_client.inputs.graphql,
        ) {
            let buffer = std::fs::read_to_string(&graphql_path).unwrap();
            let ast_nodes = match buffer_to_client_ast(&graphql_path, &buffer) {
                Ok(nodes) => nodes,
                Err(error) => {
                    errors.push(error);
                    continue;
                }
            };
            let lir_nodes = client_nodes_to_lir_nodes(
                &shared_formatter_config,
                &hir_to_lir_config,
                &ast_nodes,
            );
            if args.check {
                if let Err(error) = format_check_action(
                    &graphql_path,
                    &buffer,
                    &lir_printer_config,
                    lir_nodes,
                ) {
                    errors.push(error);
                }
            } else {
                if let Err(error) =
                    format_action(&graphql_path, &lir_printer_config, lir_nodes)
                {
                    errors.push(error);
                }
            }
        }
    }
    if errors.len() != 0 {
        for e in errors {
            eprintln!("{}", e);
        }
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
