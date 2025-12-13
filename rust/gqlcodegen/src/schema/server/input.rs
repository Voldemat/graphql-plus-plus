use std::collections::HashMap;

use crate::schema::shared::InputField;

#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct Input {
    name: String,
    fields: HashMap<String, InputField>,
}

#[cfg(test)]
mod tests {
    use crate::schema::shared::{
        ArrayFieldSpec, InputField, InputFieldSpec, InputType, LiteralFieldSpec,
    };

    use super::*;

    #[test]
    fn test_parse_input_schema() {
        let schema: Input = serde_json_path_to_error::from_str(
            r#"
{
    "name": "GroupIn",
    "fields": {
      "limitOfDownloadsPerDay": {
        "nullable": false,
        "spec": {
          "_type": "literal",
          "type": {
            "_type": "Scalar",
            "name": "Int"
          }
        }
      },
      "name": {
        "nullable": false,
        "spec": {
          "_type": "literal",
          "type": {
            "_type": "Scalar",
            "name": "String"
          }
        }
      },
      "tagIds": {
        "nullable": false,
        "spec": {
          "_type": "array",
          "nullable": false,
          "type": {
            "_type": "Scalar",
            "name": "UUID"
          }
        }
      }
    }
  }
        "#,
        )
        .unwrap();
        let tag_ids_field = InputField {
            nullable: false,
            spec: InputFieldSpec::Array(ArrayFieldSpec::<InputType> {
                nullable: false,
                field_type: InputType::Scalar {
                    name: "UUID".into(),
                },
                default_value: None,
            }),
        };
        let limit_of_downloads_per_day_field = InputField {
            nullable: false,
            spec: InputFieldSpec::Literal(LiteralFieldSpec::<InputType> {
                field_type: InputType::Scalar { name: "Int".into() },
                default_value: None,
            }),
        };
        let name_field = InputField {
            nullable: false,
            spec: InputFieldSpec::Literal(LiteralFieldSpec::<InputType> {
                field_type: InputType::Scalar {
                    name: "String".into(),
                },
                default_value: None,
            }),
        };
        assert_eq!(
            schema,
            Input {
                name: "GroupIn".into(),
                fields: HashMap::<String, InputField>::from([
                    (String::from("tagIds"), tag_ids_field),
                    (
                        String::from("limitOfDownloadsPerDay"),
                        limit_of_downloads_per_day_field,
                    ),
                    (String::from("name"), name_field),
                ])
            }
        );
    }
}
