#[derive(Debug, serde::Deserialize)]
pub struct Enum {
    name: String,
    values: Vec<String>,
}
