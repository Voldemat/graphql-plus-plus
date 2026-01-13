use crate::schema::shared::Literal;

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "_type")]
pub enum ArgumentValue {
    #[serde(rename(deserialize = "ref"))]
    Ref { name: String },
    #[serde(rename(deserialize = "literal"))]
    Literal { value: Literal }
}

#[derive(Debug, serde::Deserialize)]
pub struct Argument {
    name: String,
    value: ArgumentValue
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argument() {
        let _: Argument = serde_json_path_to_error::from_str(
            r##"
              {
                "name": "limit",
                "value": {
                  "_type": "literal",
                  "value": 10000000
                }
              }
        "##,
        )
        .unwrap();
    }
}
