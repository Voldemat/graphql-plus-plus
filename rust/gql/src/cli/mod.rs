pub mod config;
pub mod internal;
pub mod utils;

#[derive(clap::Parser)]
#[command(version, about)]
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

#[derive(clap::Subcommand)]
pub enum Commands {
    #[command(subcommand)]
    Internal(internal::Commands),
    Generate(MainArgs),
    Validate(MainArgs),
}

impl CLI {
    pub fn execute(self: &Self) {
        self.command.execute();
    }
}

fn parse_config(args: &MainArgs) -> config::Config {
    let buffer = std::fs::read_to_string(&args.config).unwrap();
    return serde_yaml::from_str(&buffer).unwrap();
}

fn generate(args: &MainArgs) {
    let config = parse_config(args);
    utils::run_config_action(
        &args.config,
        &config,
        Box::new(|json_string, filepath, _| {
            if filepath == "-" {
                println!("{}", json_string);
            } else {
                let final_filepath =
                    std::path::Path::join(args.config.parent().unwrap(), filepath);
                std::fs::write(final_filepath, json_string).unwrap();
            }
        }),
    )
    .unwrap();
}

fn validate(args: &MainArgs) {
    let config = parse_config(args);
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

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Internal(internal) => internal.execute(),
            Commands::Generate(args) => generate(args),
            Commands::Validate(args) => validate(args),
        }
    }
}
