use crate::schema::shared::{
    ArrayFieldSpec, Field, InputField, LiteralFieldSpec,
};

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
#[serde(tag = "_type")]
pub enum ObjectType {
    ObjectType { name: String },
    InterfaceType { name: String },
    Scalar { name: String },
    Union { name: String },
    Enum { name: String },
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "_type")]
pub enum ObjectNonCallableFieldSpec {
    #[serde(rename(deserialize = "literal"))]
    Literal(LiteralFieldSpec<ObjectType>),
    #[serde(rename(deserialize = "array"))]
    Array(ArrayFieldSpec<ObjectNonCallableFieldSpec>),
}

#[derive(Debug, serde::Deserialize)]
pub struct CallableFieldSpec {
    #[serde(rename(deserialize = "returnType"))]
    pub return_type: ObjectNonCallableFieldSpec,
    pub arguments: indexmap::IndexMap<String, InputField>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "_type")]
pub enum ObjectFieldSpec {
    #[serde(rename(deserialize = "literal"))]
    Literal(LiteralFieldSpec<ObjectType>),
    #[serde(rename(deserialize = "array"))]
    Array(ArrayFieldSpec<ObjectNonCallableFieldSpec>),
    #[serde(rename(deserialize = "callable"))]
    Callable(CallableFieldSpec),
}

impl ObjectFieldSpec {
    pub fn get_arguments(
        self: &Self,
    ) -> Option<&indexmap::IndexMap<String, InputField>> {
        match self {
            Self::Literal(_) => None,
            Self::Array(_) => None,
            Self::Callable(callable) => Some(&callable.arguments),
        }
    }
}

pub type ObjectField = Field<ObjectFieldSpec>;

#[derive(Debug, serde::Deserialize)]
pub struct Object {
    pub name: String,
    pub implements: indexmap::IndexMap<String, String>,
    pub fields: indexmap::IndexMap<String, ObjectField>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_object() {
        let _: Object = serde_json_path_to_error::from_str(
            r##"
{
  "name": "User",
  "implements": {},
  "fields": {
    "createdAt": {
      "nullable": false,
      "spec": {
        "_type": "literal",
        "type": {
          "_type": "Scalar",
          "name": "Datetime"
        },
        "invocations": {}
      }
    },
    "email": {
      "nullable": false,
      "spec": {
        "_type": "literal",
        "type": {
          "_type": "Scalar",
          "name": "String"
        },
        "invocations": {}
      }
    },
    "id": {
      "nullable": false,
      "spec": {
        "_type": "literal",
        "type": {
          "_type": "Scalar",
          "name": "UUID"
        },
        "invocations": {}
      }
    },
    "name": {
      "nullable": false,
      "spec": {
        "_type": "literal",
        "type": {
          "_type": "Scalar",
          "name": "String"
        },
        "invocations": {}
      }
    },
    "tenGroups": {
      "nullable": false,
      "spec": {
        "_type": "array",
        "nullable": false,
        "type": {
          "_type": "literal",
          "type": {
            "_type": "ObjectType",
            "name": "Group",
            "$ref": "#/server/objects/Group"
          }
        }
      }
    }
  }
}
        "##,
        )
        .unwrap();
    }
}
