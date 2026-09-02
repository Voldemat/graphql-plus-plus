mod client;
mod server;
pub mod shared;
mod text_diff;

#[derive(clap::Args)]
pub struct Args {
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

impl Args {
    pub fn execute(self: &Self) {
        let config_directory_path = self.config.parent().unwrap();
        let conf =
            crate::cli::config::Config::from_yaml_file_path(&self.config);
        let mut errors = Vec::<String>::new();
        let Some(formatting_config) = conf.formatting else {
            eprintln!("No formatting config is defined");
            return;
        };
        let shared_formatter_config =
            libgql::formatter::shared::config::Config {
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
            let Some(config_server) = conf.server.as_ref() else {
                eprintln!("config.server is not defined");
                return;
            };
            errors.extend(server::format_config(
                config_directory_path,
                self.check,
                config_server,
                &shared_formatter_config,
                &hir_to_lir_config,
                &lir_printer_config,
                &formatting_server_config,
            ))
        }

        if let Some(formatting_client_config) = formatting_config.client {
            let Some(config_client) = conf.client.as_ref() else {
                eprintln!("config.client is not defined");
                return;
            };
            errors.extend(client::format_config(
                config_directory_path,
                self.check,
                config_client,
                &shared_formatter_config,
                &hir_to_lir_config,
                &lir_printer_config,
                &formatting_client_config,
            ))
        }
        if errors.len() != 0 {
            for e in errors {
                eprintln!("{}", e);
            }
            std::process::exit(1);
        }
    }
}
