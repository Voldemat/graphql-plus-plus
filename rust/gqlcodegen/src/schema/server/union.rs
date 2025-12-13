use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct Union {
    name: String,
    items: HashMap<String, String>,
}

