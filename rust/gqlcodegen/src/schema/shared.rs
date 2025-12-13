#[derive(Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(tag = "_type")]
pub enum InputType {
    InputType { name: String },
    Scalar { name: String },
    Enum { name: String },
}

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(untagged)]
pub enum Literal {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub enum ArrayLiteral {
    String(Vec<String>),
    Int(Vec<i32>),
    Float(Vec<f32>),
    Bool(Vec<bool>),
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct LiteralFieldSpec<T> {
    #[serde(rename(deserialize = "type"))]
    pub field_type: T,
    #[serde(rename(deserialize = "defaultValue"))]
    pub default_value: Option<Literal>,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct ArrayFieldSpec<T> {
    pub nullable: bool,
    #[serde(rename(deserialize = "type"))]
    pub field_type: T,
    #[serde(rename(deserialize = "defaultValue"))]
    pub default_value: Option<ArrayLiteral>,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(tag = "_type")]
pub enum InputFieldSpec {
    #[serde(rename(deserialize = "literal"))]
    Literal(LiteralFieldSpec<InputType>),
    #[serde(rename(deserialize = "array"))]
    Array(ArrayFieldSpec<InputType>),
}

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Field<T> {
    pub nullable: bool,
    pub spec: T,
}

pub type InputField = Field<InputFieldSpec>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input_type() {
        let _: InputType = serde_json_path_to_error::from_str(
            r#"
        {
            "_type": "Scalar",
            "name": "Int"
        }
        "#,
        )
        .unwrap();
    }
}
