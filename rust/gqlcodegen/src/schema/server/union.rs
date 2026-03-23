#[derive(Debug, serde::Deserialize)]
pub struct Union {
    pub name: String,
    pub items: indexmap::IndexMap<String, String>,
}
