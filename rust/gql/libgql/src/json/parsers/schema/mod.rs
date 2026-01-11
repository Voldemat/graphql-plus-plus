use std::{cell::RefCell, rc::Rc};

use indexmap::IndexMap;

use crate::parsers::schema::{
    server::{self, ast},
    shared,
    type_registry::TypeRegistry,
};

fn parse_literal(
    value: &serde_json::Value,
) -> Result<shared::ast::Literal, String> {
    if let Some(v) = value.as_i64() {
        return Ok(shared::ast::Literal::Int(
            v.try_into()
                .map_err(|_| "Int value is larger than i32::Max".to_string())?,
        ));
    };
    if let Some(v) = value.as_f64() {
        return Ok(shared::ast::Literal::Float(v as f32));
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
            arr.iter()
                .map(|a| -> Result<i32, String> {
                    a.as_i64().unwrap().try_into().map_err(|_| {
                        "Int value is larger than i32::Max".to_string()
                    })
                })
                .collect::<Result<Vec<i32>, String>>()?,
        ));
    };
    if first_v.is_f64() {
        return Ok(shared::ast::ArrayLiteral::Float(
            arr.iter().map(|a| a.as_f64().unwrap() as f32).collect(),
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
    registry: &TypeRegistry,
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
            return Ok(registry
                .objects
                .get(value["name"].as_str().unwrap())
                .unwrap()
                .clone()
                .into());
        }
        "Interface" => {
            return Ok(registry
                .interfaces
                .get(value["name"].as_str().unwrap())
                .unwrap()
                .clone()
                .into());
        }
        "Enum" => {
            return Ok(registry
                .enums
                .get(value["name"].as_str().unwrap())
                .unwrap()
                .clone()
                .into());
        }
        "Union" => {
            return Ok(registry
                .unions
                .get(value["name"].as_str().unwrap())
                .unwrap()
                .clone()
                .into());
        }
        "Scalar" => {
            return Ok(server::ast::ObjectTypeSpec::Scalar {
                name: value["name"].as_str().unwrap().into(),
            });
        }
        _ => return Err(format!("Unknown ObjectTypeSpec _type: {}", t)),
    }
}

fn parse_object_literal_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> Result<shared::ast::LiteralFieldSpec<server::ast::ObjectTypeSpec>, String>
{
    return Ok(shared::ast::LiteralFieldSpec {
        default_value: parse_optional_literal(&value["default_value"])?,
        directive_invocations: IndexMap::new(),
        r#type: parse_object_type_spec(registry, &value["type"])?,
    });
}

fn parse_object_array_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> Result<shared::ast::ArrayFieldSpec<server::ast::ObjectTypeSpec>, String> {
    return Ok(shared::ast::ArrayFieldSpec {
        default_value: parse_optional_array_literal(&value["default_value"])?,
        directive_invocations: Vec::new(),
        r#type: parse_object_type_spec(registry, &value["type"])?,
        nullable: value["nullable"].as_bool().unwrap(),
    });
}

fn parse_non_nullable_object_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> Result<
    shared::ast::NonCallableFieldSpec<server::ast::ObjectTypeSpec>,
    String,
> {
    let t = value["_type"].as_str().unwrap();
    match t {
        "literal" => {
            Ok(parse_object_literal_field_spec(registry, value)?.into())
        }
        "array" => Ok(parse_object_array_field_spec(registry, value)?.into()),
        _ => Err(format!(
            "Unknown _type for NonNullableObjectFieldSpec: {}",
            t
        )),
    }
}

fn parse_input_type_spec(
    registry: &TypeRegistry,
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
            return Ok(registry
                .inputs
                .get(value["name"].as_str().unwrap())
                .unwrap()
                .clone()
                .into());
        }
        "Enum" => {
            return Ok(registry
                .enums
                .get(value["name"].as_str().unwrap())
                .unwrap()
                .clone()
                .into());
        }
        "Scalar" => {
            return Ok(shared::ast::InputTypeSpec::Scalar(
                value["name"].as_str().unwrap().into(),
            ));
        }
        _ => return Err(format!("Unknown InputTypeSpec _type: {}", t)),
    }
}

fn parse_input_literal_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> Result<shared::ast::LiteralFieldSpec<shared::ast::InputTypeSpec>, String> {
    return Ok(shared::ast::LiteralFieldSpec {
        default_value: parse_optional_literal(&value["default_value"])?,
        directive_invocations: IndexMap::new(),
        r#type: parse_input_type_spec(registry, &value["type"])?,
    });
}

fn parse_input_array_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> Result<shared::ast::ArrayFieldSpec<shared::ast::InputTypeSpec>, String> {
    return Ok(shared::ast::ArrayFieldSpec {
        default_value: parse_optional_array_literal(&value["default_value"])?,
        directive_invocations: Vec::new(),
        r#type: parse_input_type_spec(registry, &value["type"])?,
        nullable: value["nullable"].as_bool().unwrap(),
    });
}

fn parse_input_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> Result<shared::ast::InputFieldSpec, String> {
    let t = value["_type"].as_str().unwrap();
    match t {
        "literal" => {
            Ok(parse_input_literal_field_spec(registry, value)?.into())
        }
        "array" => Ok(parse_input_array_field_spec(registry, value)?.into()),
        _ => Err(format!(
            "Unknown _type for NonNullableInputFieldSpec: {}",
            t
        )),
    }
}

fn parse_input_field_definition(
    registry: &TypeRegistry,
    name: &str,
    value: &serde_json::Value,
) -> Result<shared::ast::FieldDefinition<shared::ast::InputFieldSpec>, String> {
    return Ok(shared::ast::FieldDefinition {
        name: name.into(),
        spec: parse_input_field_spec(registry, &value["spec"])?,
        nullable: value["nullable"].as_bool().unwrap(),
    });
}

fn parse_arguments(
    registry: &TypeRegistry,
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
        args.insert(
            key.clone(),
            parse_input_field_definition(registry, key, v)?,
        );
    }
    return Ok(args);
}

fn parse_object_callable_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> Result<server::ast::CallableFieldSpec, String> {
    return Ok(server::ast::CallableFieldSpec {
        return_type: parse_non_nullable_object_field_spec(
            registry,
            &value["returnType"],
        )?,
        arguments: parse_arguments(registry, &value["arguments"])?,
    });
}

fn parse_object_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> Result<server::ast::ObjectFieldSpec, String> {
    let t = value["_type"].as_str().ok_or(
        "ObjectFieldSpec is expected to have \"_type\" string descriminator",
    )?;
    match t {
        "literal" => {
            Ok(parse_object_literal_field_spec(registry, value)?.into())
        }
        "callable" => {
            Ok(parse_object_callable_field_spec(registry, value)?.into())
        }
        "array" => Ok(parse_object_array_field_spec(registry, value)?.into()),
        _ => Err(
            format!("Invalid ObjectFieldSpec _type descriminator: {}", t)
                .into(),
        ),
    }
}

fn parse_object_field(
    registry: &TypeRegistry,
    name: &str,
    value: &serde_json::Value,
) -> Result<
    Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
    String,
> {
    let nullable = value["nullable"]
        .as_bool()
        .ok_or("FieldDefinition is expected to have \"nullable\" bool value")?;
    return Ok(Rc::new(shared::ast::FieldDefinition {
        name: name.into(),
        nullable,
        spec: parse_object_field_spec(registry, &value["spec"])?,
    }));
}

fn parse_object_fields(
    registry: &TypeRegistry,
    map: &serde_json::Value,
) -> Result<
    IndexMap<
        String,
        Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
    >,
    String,
> {
    let mut fields = IndexMap::<
        String,
        Rc<shared::ast::FieldDefinition<server::ast::ObjectFieldSpec>>,
    >::new();
    for (key, value) in map.as_object().unwrap() {
        fields.insert(key.into(), parse_object_field(registry, key, value)?);
    }

    return Ok(fields);
}

fn parse_implements_map(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> indexmap::IndexMap<String, Rc<RefCell<server::ast::Interface>>> {
    return value
        .as_object()
        .unwrap()
        .iter()
        .map(|(key, _)| {
            (key.clone(), registry.interfaces.get(key).unwrap().clone())
        })
        .collect();
}

fn parse_object(
    registry: &TypeRegistry,
    object: &mut server::ast::ObjectType,
    value: &serde_json::Value,
) -> Result<(), String> {
    object.fields = parse_object_fields(registry, &value["fields"])?;
    object.implements = parse_implements_map(registry, &value["implements"]);
    return Ok(());
}

fn parse_objects(
    registry: &TypeRegistry,
    map: &serde_json::Value,
) -> Result<IndexMap<String, Rc<RefCell<ast::ObjectType>>>, String> {
    let mut objects = IndexMap::<String, Rc<RefCell<ast::ObjectType>>>::new();
    for (key, value) in map.as_object().unwrap() {
        let object_rc = registry.objects.get(key).unwrap();
        parse_object(registry, &mut object_rc.borrow_mut(), value)?;
        objects.insert(key.clone(), object_rc.clone());
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
    registry: &mut TypeRegistry,
    map: &serde_json::Value,
) -> Result<IndexMap<String, Rc<shared::ast::Enum>>, String> {
    let mut enums = IndexMap::<String, Rc<shared::ast::Enum>>::new();
    for (key, value) in map.as_object().unwrap() {
        let e = Rc::new(parse_enum(value)?);
        registry.enums.insert(key.clone(), e.clone());
        enums.insert(key.clone(), e);
    }
    return Ok(enums);
}

fn parse_input_fields(
    registry: &TypeRegistry,
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
        fields.insert(
            key.clone(),
            parse_input_field_definition(registry, key, value)?,
        );
    }
    return Ok(fields);
}

fn parse_inputs(
    registry: &mut TypeRegistry,
    map: &serde_json::Value,
) -> Result<IndexMap<String, Rc<RefCell<shared::ast::InputType>>>, String> {
    let m = map.as_object().unwrap();
    for key in m.keys() {
        registry.inputs.insert(
            key.clone(),
            Rc::new(RefCell::new(shared::ast::InputType {
                name: key.clone(),
                fields: IndexMap::new(),
            })),
        );
    }
    let mut inputs =
        IndexMap::<String, Rc<RefCell<shared::ast::InputType>>>::new();
    for (key, value) in map.as_object().unwrap() {
        let i = registry.inputs.get(key).unwrap();
        i.borrow_mut().fields = parse_input_fields(registry, &value["fields"])?;
        inputs.insert(key.clone(), i.clone());
    }
    return Ok(inputs);
}

fn parse_interfaces(
    registry: &mut TypeRegistry,
    map: &serde_json::Value,
) -> Result<IndexMap<String, Rc<RefCell<server::ast::Interface>>>, String> {
    let m = map.as_object().unwrap();
    for key in m.keys() {
        registry.interfaces.insert(
            key.clone(),
            Rc::new(RefCell::new(server::ast::Interface {
                name: key.clone(),
                fields: IndexMap::new(),
                directives: Vec::new(),
            })),
        );
    }
    let mut inputs =
        IndexMap::<String, Rc<RefCell<server::ast::Interface>>>::new();
    for (key, value) in map.as_object().unwrap() {
        let i = registry.interfaces.get(key).unwrap();
        i.borrow_mut().fields =
            parse_object_fields(registry, &value["fields"])?;
        inputs.insert(key.clone(), i.clone());
    }
    return Ok(inputs);
}

fn parse_items(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> Result<IndexMap<String, Rc<RefCell<server::ast::ObjectType>>>, String> {
    let mut items =
        IndexMap::<String, Rc<RefCell<server::ast::ObjectType>>>::new();
    for key in value.as_object().unwrap().keys() {
        items.insert(key.clone(), registry.objects.get(key).unwrap().clone());
    }
    return Ok(items);
}

fn parse_union(
    registry: &TypeRegistry,
    union: &mut server::ast::Union,
    value: &serde_json::Value,
) -> Result<(), String> {
    union.items = parse_items(registry, &value["items"])?;
    return Ok(());
}

fn parse_unions(
    registry: &TypeRegistry,
    map: &serde_json::Value,
) -> Result<IndexMap<String, Rc<RefCell<ast::Union>>>, String> {
    let mut unions = IndexMap::<String, Rc<RefCell<ast::Union>>>::new();
    for (key, value) in map.as_object().unwrap() {
        let union_rc = registry.unions.get(key).unwrap();
        parse_union(registry, &mut union_rc.borrow_mut(), value)?;
        unions.insert(key.clone(), union_rc.clone());
    }
    return Ok(unions);
}

pub fn parse_server_schema(
    registry: &mut TypeRegistry,
    value: serde_json::Value,
) -> Result<server::schema::Schema, String> {
    for key in value["objects"].as_object().unwrap().keys() {
        registry.objects.insert(
            key.clone(),
            Rc::new(RefCell::new(server::ast::ObjectType {
                name: key.clone(),
                fields: IndexMap::new(),
                directives: Vec::new(),
                implements: IndexMap::new(),
            })),
        );
    }
    for key in value["unions"].as_object().unwrap().keys() {
        registry.unions.insert(
            key.clone(),
            Rc::new(RefCell::new(server::ast::Union {
                name: key.clone(),
                items: IndexMap::new(),
            })),
        );
    }
    let new_scalars: Vec<String> = value["scalars"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap().to_string())
                .collect();
    for scalar in &new_scalars {
        registry.scalars.push(scalar.clone());
    }
    return Ok(server::schema::Schema {
        scalars: [
            shared::scalars::get_builtin_scalars(),
            new_scalars,
        ]
        .concat(),
        enums: parse_enums(registry, &value["enums"])?,
        inputs: parse_inputs(registry, &value["inputs"])?,
        interfaces: parse_interfaces(registry, &value["interfaces"])?,
        objects: parse_objects(registry, &value["objects"])?,
        unions: parse_unions(registry, &value["unions"])?,
        directives: IndexMap::new(),
    });
}
