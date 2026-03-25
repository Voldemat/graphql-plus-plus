use indexmap::{IndexMap, IndexSet};

use crate::parsers::schema::{
    server::{self, ast},
    shared,
};

fn parse_literal(
    value: &serde_json::Value,
) -> Result<shared::ast::Literal, String> {
    if let Some(v) = value.as_i64() {
        return Ok(shared::ast::Literal::Int(v));
    };
    if let Some(v) = value.as_f64() {
        return Ok(shared::ast::Literal::Float(v));
    };
    if let Some(v) = value.as_bool() {
        return Ok(shared::ast::Literal::Boolean(v));
    };
    if let Some(v) = value.as_str() {
        return Ok(shared::ast::Literal::String(v.into()));
    };
    return Err(format!("Unexpected literal value: {}", value));
}

fn parse_optional_literal(
    value: &serde_json::Value,
) -> Result<Option<shared::ast::Literal>, String> {
    if value.is_null() {
        Ok(None)
    } else {
        Ok(Some(parse_literal(value)?))
    }
}

fn parse_array_literal(
    value: &serde_json::Value,
) -> Result<shared::ast::ArrayLiteral, String> {
    let Some(arr) = value.as_array() else {
        return Err("Expected an array for array literal".into());
    };
    let first_v = arr.get(0).unwrap();
    if first_v.is_i64() {
        return Ok(shared::ast::ArrayLiteral::Int(
            arr.iter().map(|a| a.as_i64().unwrap()).collect(),
        ));
    };
    if first_v.is_f64() {
        return Ok(shared::ast::ArrayLiteral::Float(
            arr.iter().map(|a| a.as_f64().unwrap()).collect(),
        ));
    };
    if first_v.is_boolean() {
        return Ok(shared::ast::ArrayLiteral::Boolean(
            arr.iter().map(|a| a.as_bool().unwrap()).collect(),
        ));
    };
    if first_v.is_string() {
        return Ok(shared::ast::ArrayLiteral::String(
            arr.iter().map(|a| a.as_str().unwrap().into()).collect(),
        ));
    };
    return Err(format!("Unexpected array literal value: {}", first_v));
}

fn parse_optional_array_literal(
    value: &serde_json::Value,
) -> Result<Option<shared::ast::ArrayLiteral>, String> {
    if value.is_null() {
        Ok(None)
    } else {
        Ok(Some(parse_array_literal(value)?))
    }
}

fn parse_object_type_spec(
    value: &serde_json::Value,
) -> Result<server::ast::ObjectTypeSpec, String> {
    let Some(t) = value["_type"].as_str() else {
        return Err(
            "Expected to have _type string descriminator in object type spec"
                .into(),
        );
    };
    match t {
        "ObjectType" => {
            return Ok(server::ast::ObjectTypeSpec::ObjectType(
                value["name"].as_str().unwrap().to_string(),
            ));
        }
        "Interface" => {
            return Ok(server::ast::ObjectTypeSpec::Interface(
                value["name"].as_str().unwrap().to_string(),
            ));
        }
        "Enum" => {
            return Ok(server::ast::ObjectTypeSpec::Enum(
                value["name"].as_str().unwrap().to_string(),
            ));
        }
        "Union" => {
            return Ok(server::ast::ObjectTypeSpec::Union(
                value["name"].as_str().unwrap().to_string(),
            ));
        }
        "Scalar" => {
            return Ok(server::ast::ObjectTypeSpec::Scalar(
                value["name"].as_str().unwrap().to_string(),
            ));
        }
        _ => return Err(format!("Unknown ObjectTypeSpec _type: {}", t)),
    }
}

fn parse_object_literal_field_spec(
    value: &serde_json::Value,
) -> Result<shared::ast::LiteralFieldSpec<server::ast::ObjectTypeSpec>, String>
{
    return Ok(shared::ast::LiteralFieldSpec {
        default_value: None,
        directive_invocations: IndexMap::new(),
        r#type: parse_object_type_spec(&value["type"])?,
    });
}

fn parse_object_array_field_spec(
    value: &serde_json::Value,
) -> Result<shared::ast::ArrayFieldSpec<server::ast::ObjectTypeSpec>, String> {
    return Ok(shared::ast::ArrayFieldSpec {
        default_value: None,
        directive_invocations: Vec::new(),
        r#type: Box::new(parse_non_callable_object_field_spec(&value["type"])?),
        nullable: value["nullable"].as_bool().unwrap(),
    });
}

fn parse_non_callable_object_field_spec(
    value: &serde_json::Value,
) -> Result<
    shared::ast::NonCallableFieldSpec<server::ast::ObjectTypeSpec>,
    String,
> {
    let t = value["_type"].as_str().unwrap();
    match t {
        "literal" => Ok(parse_object_literal_field_spec(value)?.into()),
        "array" => Ok(parse_object_array_field_spec(value)?.into()),
        _ => Err(format!(
            "Unknown _type for NonNullableObjectFieldSpec: {}",
            t
        )),
    }
}

fn parse_input_type_spec(
    value: &serde_json::Value,
) -> Result<shared::ast::InputTypeSpec, String> {
    let Some(t) = value["_type"].as_str() else {
        return Err(
            "Expected to have _type string descriminator in input type spec"
                .into(),
        );
    };
    match t {
        "InputType" => {
            return Ok(shared::ast::InputTypeSpec::InputType(
                value["name"].as_str().unwrap().to_string(),
            ));
        }
        "Enum" => {
            return Ok(shared::ast::InputTypeSpec::Enum(
                value["name"].as_str().unwrap().to_string(),
            ));
        }
        "Scalar" => {
            return Ok(shared::ast::InputTypeSpec::Scalar(
                value["name"].as_str().unwrap().to_string(),
            ));
        }
        _ => return Err(format!("Unknown InputTypeSpec _type: {}", t)),
    }
}

fn parse_input_literal_field_spec(
    value: &serde_json::Value,
) -> Result<shared::ast::LiteralFieldSpec<shared::ast::InputTypeSpec>, String> {
    return Ok(shared::ast::LiteralFieldSpec {
        default_value: Some(parse_optional_literal(&value["default_value"])?),
        directive_invocations: IndexMap::new(),
        r#type: parse_input_type_spec(&value["type"])?,
    });
}

fn parse_input_array_field_spec(
    value: &serde_json::Value,
) -> Result<shared::ast::ArrayFieldSpec<shared::ast::InputTypeSpec>, String> {
    return Ok(shared::ast::ArrayFieldSpec {
        default_value: Some(parse_optional_array_literal(
            &value["default_value"],
        )?),
        directive_invocations: Vec::new(),
        r#type: Box::new(parse_input_field_spec(&value["type"])?),
        nullable: value["nullable"].as_bool().unwrap(),
    });
}

fn parse_input_field_spec(
    value: &serde_json::Value,
) -> Result<shared::ast::InputFieldSpec, String> {
    let t = value["_type"].as_str().unwrap();
    match t {
        "literal" => Ok(parse_input_literal_field_spec(value)?.into()),
        "array" => Ok(parse_input_array_field_spec(value)?.into()),
        _ => Err(format!(
            "Unknown _type for NonNullableInputFieldSpec: {}",
            t
        )),
    }
}

fn parse_input_field_definition(
    name: &str,
    value: &serde_json::Value,
) -> Result<shared::ast::FieldDefinition<shared::ast::InputFieldSpec>, String> {
    return Ok(shared::ast::FieldDefinition {
        name: name.into(),
        spec: parse_input_field_spec(&value["spec"])?,
        nullable: value["nullable"].as_bool().unwrap(),
    });
}

fn parse_arguments(
    value: &serde_json::Value,
) -> Result<
    IndexMap<String, shared::ast::FieldDefinition<shared::ast::InputFieldSpec>>,
    String,
> {
    let mut args = IndexMap::<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >::new();
    for (key, v) in value.as_object().unwrap() {
        args.insert(key.clone(), parse_input_field_definition(key, v)?);
    }
    return Ok(args);
}

fn parse_object_callable_field_spec(
    value: &serde_json::Value,
) -> Result<server::ast::CallableFieldSpec, String> {
    return Ok(server::ast::CallableFieldSpec {
        return_type: parse_non_callable_object_field_spec(
            &value["returnType"],
        )?,
        arguments: parse_arguments(&value["arguments"])?,
    });
}

fn parse_object_field_spec(
    value: &serde_json::Value,
) -> Result<server::ast::ObjectFieldSpec, String> {
    let t = value["_type"].as_str().ok_or(
        "ObjectFieldSpec is expected to have \"_type\" string descriminator",
    )?;
    match t {
        "literal" => Ok(parse_object_literal_field_spec(value)?.into()),
        "callable" => Ok(parse_object_callable_field_spec(value)?.into()),
        "array" => Ok(parse_object_array_field_spec(value)?.into()),
        _ => Err(
            format!("Invalid ObjectFieldSpec _type descriminator: {}", t)
                .into(),
        ),
    }
}

fn parse_object_field(
    name: &str,
    value: &serde_json::Value,
) -> Result<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>, String>
{
    let nullable = value["nullable"]
        .as_bool()
        .ok_or("FieldDefinition is expected to have \"nullable\" bool value")?;
    return Ok(shared::ast::FieldDefinition {
        name: name.into(),
        nullable,
        spec: parse_object_field_spec(&value["spec"])?,
    });
}

fn parse_object_fields(
    map: &serde_json::Value,
) -> Result<
    IndexMap<
        String,
        shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>,
    >,
    String,
> {
    let mut fields = IndexMap::<
        String,
        shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>,
    >::new();
    for (key, value) in map.as_object().unwrap() {
        fields.insert(key.into(), parse_object_field(key, value)?);
    }

    return Ok(fields);
}

fn parse_implements_map(
    value: &serde_json::Value,
) -> indexmap::IndexSet<String> {
    return value
        .as_object()
        .unwrap()
        .iter()
        .map(|(item, _)| item.clone())
        .collect();
}

fn parse_object(
    value: &serde_json::Value,
) -> Result<server::ast::ObjectType, String> {
    Ok(server::ast::ObjectType {
        name: value["name"].as_str().unwrap().to_string(),
        fields: parse_object_fields(&value["fields"])?,
        implements: parse_implements_map(&value["implements"]),
        directives: Vec::new(),
    })
}

fn parse_objects(
    map: &serde_json::Value,
) -> Result<IndexMap<String, ast::ObjectType>, String> {
    let mut objects = IndexMap::<String, ast::ObjectType>::new();
    for (key, value) in map.as_object().unwrap() {
        objects.insert(key.clone(), parse_object(value)?);
    }
    return Ok(objects);
}

fn parse_enum(value: &serde_json::Value) -> Result<shared::ast::Enum, String> {
    return Ok(shared::ast::Enum {
        name: value["name"].as_str().unwrap().to_string(),
        values: value["values"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect(),
    });
}

fn parse_enums(
    map: &serde_json::Value,
) -> Result<IndexMap<String, shared::ast::Enum>, String> {
    let mut enums = IndexMap::<String, shared::ast::Enum>::new();
    for (key, value) in map.as_object().unwrap() {
        enums.insert(key.clone(), parse_enum(value)?);
    }
    return Ok(enums);
}

fn parse_input_fields(
    v: &serde_json::Value,
) -> Result<
    IndexMap<String, shared::ast::FieldDefinition<shared::ast::InputFieldSpec>>,
    String,
> {
    let mut fields = IndexMap::<
        String,
        shared::ast::FieldDefinition<shared::ast::InputFieldSpec>,
    >::new();
    for (key, value) in v.as_object().unwrap() {
        fields.insert(key.clone(), parse_input_field_definition(key, value)?);
    }
    return Ok(fields);
}

fn parse_inputs(
    map: &serde_json::Value,
) -> Result<IndexMap<String, shared::ast::InputType>, String> {
    let mut inputs = IndexMap::<String, shared::ast::InputType>::new();
    for (key, value) in map.as_object().unwrap() {
        inputs.insert(
            key.clone(),
            shared::ast::InputType {
                name: key.clone(),
                fields: parse_input_fields(&value["fields"])?,
            },
        );
    }
    return Ok(inputs);
}

fn parse_interfaces(
    map: &serde_json::Value,
) -> Result<IndexMap<String, server::ast::Interface>, String> {
    let mut interfaces = IndexMap::<String, server::ast::Interface>::new();
    for (key, value) in map.as_object().unwrap() {
        parse_object_fields(&value["fields"])?;
        interfaces.insert(
            key.clone(),
            server::ast::Interface {
                name: key.clone(),
                fields: parse_object_fields(&value["fields"])?,
                directives: Vec::new(),
            },
        );
    }
    return Ok(interfaces);
}

fn parse_items(value: &serde_json::Value) -> Result<IndexSet<String>, String> {
    let mut items = IndexSet::<String>::new();
    for key in value.as_object().unwrap().keys() {
        items.insert(key.clone());
    }
    return Ok(items);
}

fn parse_union(
    value: &serde_json::Value,
) -> Result<server::ast::Union, String> {
    Ok(server::ast::Union {
        name: value["name"].as_str().unwrap().to_string(),
        items: parse_items(&value["items"])?,
    })
}

fn parse_unions(
    map: &serde_json::Value,
) -> Result<IndexMap<String, ast::Union>, String> {
    let mut unions = IndexMap::<String, ast::Union>::new();
    for (key, value) in map.as_object().unwrap() {
        unions.insert(key.clone(), parse_union(value)?);
    }
    return Ok(unions);
}

pub fn parse_server_schema(
    registry: &mut server::type_registry::TypeRegistry,
    value: serde_json::Value,
) -> Result<(), String> {
    let new_scalars: Vec<String> = value["scalars"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    for scalar in &new_scalars {
        registry.scalars.insert(scalar.clone());
    }
    registry.objects = parse_objects(&value["objects"])?;
    registry.interfaces = parse_interfaces(&value["interfaces"])?;
    registry.unions = parse_unions(&value["unions"])?;
    registry.inputs = parse_inputs(&value["inputs"])?;
    registry.enums = parse_enums(&value["enums"])?;
    Ok(())
}
