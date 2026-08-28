use std::collections::HashMap;

pub const CLI_VERSION: &str = match option_env!("CLI_VERSION") {
    Some(v) => v,
    None => "unspecified",
};

#[derive(serde::Deserialize)]
pub struct GraphqlFormattingServerConfig {}

#[derive(serde::Deserialize)]
pub struct GraphqlFormattingClientConfig {}

#[derive(serde::Deserialize)]
pub struct GraphqlFormattingSharedConfig {
    pub max_line_width: codeform::ir::shared::LineWidth,
    pub indent_width: std::num::NonZeroU8,
}

#[derive(serde::Deserialize)]
pub struct GraphqlFormattingConfig {
    pub shared: GraphqlFormattingSharedConfig,
    pub server: Option<GraphqlFormattingServerConfig>,
    pub client: Option<GraphqlFormattingClientConfig>,
}

#[derive(serde::Deserialize)]
pub struct InputsConfig {
    pub graphql: Vec<std::path::PathBuf>,
    #[serde(alias = "jsonSchema", default)]
    pub json_schema: Vec<std::path::PathBuf>,
}

#[derive(serde::Deserialize)]
pub struct OutputsConfig {
    pub filepath: std::path::PathBuf,
    #[serde(alias = "onlyUsedInOperations")]
    pub only_used_in_operations: bool,

    #[serde(default)]
    pub pretty: bool,
}

#[derive(serde::Deserialize)]
pub struct ServerConfig {
    pub inputs: InputsConfig,
    pub outputs: Option<OutputsConfig>,
}

#[derive(serde::Deserialize)]
pub struct ClientConfig {
    pub inputs: InputsConfig,
    pub outputs: Option<OutputsConfig>,
}

type OperationsMapInputsConfig = HashMap<String, Vec<String>>;

#[derive(serde::Deserialize)]
pub struct OperationsMapConfig {
    pub inputs: OperationsMapInputsConfig,
    pub outputs: String,
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub version: Option<String>,
    pub formatting: Option<GraphqlFormattingConfig>,
    pub server: Option<ServerConfig>,
    pub client: Option<ClientConfig>,
    #[serde(alias = "operationsMap")]
    pub operations_map: Option<OperationsMapConfig>,
}

impl Config {
    pub fn from_yaml_file_path(config_path: &std::path::Path) -> Config {
        let buffer = std::fs::read_to_string(config_path).unwrap();
        let config: Config = serde_yaml::from_str(&buffer).unwrap();
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
}
