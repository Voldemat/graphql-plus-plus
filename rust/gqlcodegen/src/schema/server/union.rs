use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct Union {
    pub name: String,
    pub items: HashMap<String, String>,
}

