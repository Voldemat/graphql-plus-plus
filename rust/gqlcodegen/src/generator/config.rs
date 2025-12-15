use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    pub scalars_mapping: HashMap<String, String>,
    pub scalar_type_override: Option<String>
}
