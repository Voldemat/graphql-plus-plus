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
        if let Some(formatting_server_config) = formatting_config.server {
            let Some(config_server) = conf.server.as_ref() else {
                eprintln!("config.server is not defined");
                return;
            };
            errors.extend(server::format_config(
                config_directory_path,
                self.check,
                config_server,
                &formatting_config.shared,
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
                &formatting_config.shared,
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
