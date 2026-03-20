use std::collections::HashMap;

#[derive(Debug)]
pub struct ResolversConfig {
    pub context_type: String
}

#[derive(Debug)]
pub struct Config {
    pub scalars_mapping: HashMap<String, String>,
    pub scalar_type: String,
    pub resolvers: ResolversConfig
}
