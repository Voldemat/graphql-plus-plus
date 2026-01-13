#[derive(Debug)]
pub enum DirectiveLocation {
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDefinition,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
}

struct DirectiveLocationVisitor;

impl<'de> serde::de::Visitor<'de> for DirectiveLocationVisitor {
    type Value = DirectiveLocation;

    fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        formatter.write_str("A directive location enum")
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match value.as_str() {
            "SCHEMA" => Ok(DirectiveLocation::Schema),
            "SCALAR" => Ok(DirectiveLocation::Scalar),
            "OBJECT" => Ok(DirectiveLocation::Object),
            "FIELD_DEFINITION" => Ok(DirectiveLocation::FieldDefinition),
            "ARGUMENT_DEFINITION" => Ok(DirectiveLocation::ArgumentDefinition),
            "INTERFACE" => Ok(DirectiveLocation::Interface),
            "UNION" => Ok(DirectiveLocation::Union),
            "ENUM" => Ok(DirectiveLocation::Enum),
            "ENUM_VALUE" => Ok(DirectiveLocation::EnumValue),
            "INPUT_OBJECT" => Ok(DirectiveLocation::InputObject),
            "INPUT_FIELD_DEFINITION" => {
                Ok(DirectiveLocation::InputFieldDefinition)
            }
            _ => Err(serde::de::Error::custom("Unknown directive location")),
        }
    }
}

impl<'de> serde::Deserialize<'de> for DirectiveLocation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(DirectiveLocationVisitor)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Directive {
    name: String,
    locations: Vec<DirectiveLocation>,
}

