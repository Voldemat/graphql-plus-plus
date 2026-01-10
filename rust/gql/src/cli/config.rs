use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct InputsConfig {
    pub graphql: Vec<std::path::PathBuf>,
    #[serde(alias="jsonSchema", default)]
    pub json_schema: Vec<std::path::PathBuf>
}

#[derive(serde::Deserialize)]
pub struct OutputsConfig {
    pub filepath: std::path::PathBuf,
    #[serde(alias="onlyUsedInOperations")]
    pub only_used_in_operations: bool
}

#[derive(serde::Deserialize)]
pub struct ServerConfig {
    pub inputs: InputsConfig,
    pub outputs: Option<OutputsConfig>
}

#[derive(serde::Deserialize)]
pub struct ClientConfig {
    pub inputs: InputsConfig,
    pub outputs: Option<OutputsConfig>
}

type OperationsMapInputsConfig = HashMap<String, Vec<String>>;

#[derive(serde::Deserialize)]
pub struct OperationsMapConfig {
    pub inputs: OperationsMapInputsConfig,
    pub outputs: String
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub client: Option<ClientConfig>,
    #[serde(alias="operationsMap")]
    pub operations_map: Option<OperationsMapConfig>,
}
