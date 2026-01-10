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
    let mut registry =
        libgql::parsers::schema::type_registry::TypeRegistry::new();
    let server_schema = utils::load_server_schema_from_inputs(
        &mut registry,
        args.config.parent().unwrap(),
        &config.server.inputs,
    )
    .unwrap();
    let client_schema = config.client.map(|client_config| {
        utils::load_client_schema_from_inputs(
            &mut registry,
            args.config.parent().unwrap(),
            &client_config.inputs,
        )
        .unwrap()
    });
    if let Some(outputs) = config.server.outputs {
        let json_string = libgql::json::serializers::schema::server::serialize(
            &server_schema,
            if outputs.only_used_in_operations {
                client_schema.as_ref()
            } else {
                None
            },
        ).unwrap();
    }
}

fn validate(args: &MainArgs) {
    let config = parse_config(args);
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
