#[derive(Debug, serde::Deserialize)]
pub struct Enum {
    pub name: String,
    pub values: Vec<String>,
}
