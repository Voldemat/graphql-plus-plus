use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ResolversConfig {
    pub context_type: String
}

#[derive(Debug)]
pub struct Config {
    pub scalars_mapping: HashMap<String, String>,
    pub scalar_type: String,
    pub resolvers: ResolversConfig,
    pub field_to_resolver: HashSet<(String, String)>
}
