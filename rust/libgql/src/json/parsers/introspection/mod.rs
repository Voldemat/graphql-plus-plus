use indexmap::IndexMap;

use crate::parsers::schema::{
    server::{self, type_registry::TypeRegistry},
    shared,
};

fn parse_node(registry: &mut TypeRegistry, value: &serde_json::Value) {
    let kind = value["kind"].as_str().unwrap();
    let name = value["name"].as_str().unwrap();
    match kind {
        "SCALAR" => {
            registry.scalars.insert(name.to_string());
        }
        "ENUM" => {
            registry.enums.insert(
                name.to_string(),
                shared::ast::Enum {
                    name: name.to_string(),
                    values: value["enumValues"]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|enum_value| {
                            enum_value["name"].as_str().unwrap().to_string()
                        })
                        .collect(),
                },
            );
        }
        "INPUT_OBJECT" => {
            registry.inputs.insert(
                name.to_string(),
                shared::ast::InputType {
                    name: name.to_string(),
                    fields: value["inputFields"]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|field_json| {
                            let field = parse_input_field_definition(
                                registry, field_json,
                            );
                            (field.name.clone(), field)
                        })
                        .collect(),
                },
            );
        }
        "OBJECT" => {
            registry.objects.insert(
                name.to_string(),
                server::ast::ObjectType {
                    name: name.to_string(),
                    fields: value["fields"]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|field_json| {
                            let field = parse_object_field_definition(
                                registry, field_json,
                            );
                            (field.name.clone(), field)
                        })
                        .collect(),
                    implements: value["interfaces"]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|interface_json| {
                            interface_json["name"].as_str().unwrap().to_string()
                        })
                        .collect(),
                    directives: Vec::new(),
                },
            );
        }
        "UNION" => {
            registry.unions.insert(
                name.to_string(),
                server::ast::Union {
                    name: name.to_string(),
                    items: value["possibleTypes"]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|object_json| {
                            object_json["name"].as_str().unwrap().to_string()
                        })
                        .collect(),
                },
            );
        }
        "INTERFACE" => {
            registry.interfaces.insert(
                name.to_string(),
                server::ast::Interface {
                    name: name.to_string(),
                    fields: value["fields"]
                        .as_array()
                        .unwrap()
                        .into_iter()
                        .map(|field_json| {
                            let field = parse_object_field_definition(
                                registry, field_json,
                            );
                            (field.name.clone(), field)
                        })
                        .collect(),
                    directives: Vec::new(),
                },
            );
        }
        _ => panic!("Unexpected server node kind: {}", kind),
    }
}

fn parse_input_type_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> shared::ast::InputTypeSpec {
    let kind = value["kind"].as_str().unwrap();
    match kind {
        "NON_NULL" => parse_input_type_spec(registry, &value["ofType"]),
        "SCALAR" => shared::ast::InputTypeSpec::Scalar(
            value["name"].as_str().unwrap().to_string(),
        ),
        "ENUM" => shared::ast::InputTypeSpec::Enum(
            value["name"].as_str().unwrap().to_string(),
        ),
        "INPUT_OBJECT" => shared::ast::InputTypeSpec::InputType(
            value["name"].as_str().unwrap().to_string(),
        ),
        _ => panic!("Unknown InputTypeSpec kind type: {}", kind),
    }
}

fn parse_input_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
    default_value: &serde_json::Value,
) -> shared::ast::InputFieldSpec {
    let kind = value["kind"].as_str().unwrap();
    match kind {
        "NON_NULL" => {
            parse_input_field_spec(registry, &value["ofType"], default_value)
        }
        "LIST" => {
            let t = parse_input_field_spec(
                registry,
                &value["ofType"],
                default_value,
            );
            shared::ast::ArrayFieldSpec::<shared::ast::InputTypeSpec> {
                r#type: Box::new(t),
                default_value: Some(None),
                directive_invocations: Vec::new(),
                nullable: value["ofType"]["kind"].as_str().unwrap()
                    != "NON_NULL",
            }
            .into()
        }
        _ => {
            let t = parse_input_type_spec(registry, value);
            shared::ast::LiteralFieldSpec::<shared::ast::InputTypeSpec> {
                r#type: t,
                default_value: Some(None),
                directive_invocations: IndexMap::new(),
            }
            .into()
        }
    }
}

fn parse_input_field_definition(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> shared::ast::FieldDefinition<shared::ast::InputFieldSpec> {
    return shared::ast::FieldDefinition::<shared::ast::InputFieldSpec> {
        name: value["name"].as_str().unwrap().to_string(),
        spec: parse_input_field_spec(
            registry,
            &value["type"],
            &value["defaultValue"],
        ),
        nullable: value["type"]["kind"].as_str().unwrap() != "NON_NULL",
    };
}

fn parse_object_type_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> server::ast::ObjectTypeSpec {
    let kind = value["kind"].as_str().unwrap();
    match kind {
        "NON_NULL" => parse_object_type_spec(registry, &value["ofType"]),
        "SCALAR" => server::ast::ObjectTypeSpec::Scalar(
            value["name"].as_str().unwrap().to_string(),
        ),
        "ENUM" => server::ast::ObjectTypeSpec::Enum(
            value["name"].as_str().unwrap().to_string(),
        ),
        "OBJECT" => server::ast::ObjectTypeSpec::ObjectType(
            value["name"].as_str().unwrap().to_string(),
        ),
        "UNION" => server::ast::ObjectTypeSpec::Union(
            value["name"].as_str().unwrap().to_string(),
        ),
        "INTERFACE" => server::ast::ObjectTypeSpec::Interface(
            value["name"].as_str().unwrap().to_string(),
        ),
        _ => panic!("Unknown ObjectTypeSpec kind: {}", kind),
    }
}

fn parse_non_callable_object_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> shared::ast::NonCallableFieldSpec<server::ast::ObjectTypeSpec> {
    let kind = value["kind"].as_str().unwrap();
    match kind {
        "NON_NULL" => {
            parse_non_callable_object_field_spec(registry, &value["ofType"])
        }
        "LIST" => {
            let t = parse_non_callable_object_field_spec(
                registry,
                &value["ofType"],
            );
            shared::ast::ArrayFieldSpec::<server::ast::ObjectTypeSpec> {
                r#type: Box::new(t),
                nullable: value["ofType"]["kind"].as_str().unwrap()
                    != "NON_NULL",
                default_value: None,
                directive_invocations: Vec::new(),
            }
            .into()
        }
        _ => shared::ast::LiteralFieldSpec::<server::ast::ObjectTypeSpec> {
            r#type: parse_object_type_spec(registry, value),
            default_value: None,
            directive_invocations: IndexMap::new(),
        }
        .into(),
    }
}

fn parse_object_field_spec(
    registry: &TypeRegistry,
    value: &serde_json::Value,
    args_value: &serde_json::Value,
) -> server::ast::ObjectFieldSpec {
    let kind = value["kind"].as_str().unwrap();
    let args = args_value.as_array().unwrap();
    if kind == "NON_NULL" {
        return parse_object_field_spec(registry, &value["ofType"], args_value);
    };
    if args.len() != 0 {
        return server::ast::CallableFieldSpec {
            return_type: parse_non_callable_object_field_spec(registry, value),
            arguments: args
                .iter()
                .map(|arg_json| {
                    let field =
                        parse_input_field_definition(registry, arg_json);
                    (field.name.clone(), field)
                })
                .collect(),
        }
        .into();
    };
    if kind == "LIST" {
        let t =
            parse_non_callable_object_field_spec(registry, &value["ofType"]);
        return shared::ast::ArrayFieldSpec::<server::ast::ObjectTypeSpec> {
            r#type: Box::new(t),
            nullable: value["ofType"]["kind"].as_str().unwrap() != "NON_NULL",
            default_value: None,
            directive_invocations: Vec::new(),
        }
        .into();
    };

    let t = parse_object_type_spec(registry, value);
    return shared::ast::LiteralFieldSpec::<server::ast::ObjectTypeSpec> {
        r#type: t,
        default_value: None,
        directive_invocations: IndexMap::new(),
    }
    .into();
}

fn parse_object_field_definition(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> shared::ast::FieldDefinition<server::ast::ObjectFieldSpec> {
    shared::ast::FieldDefinition::<server::ast::ObjectFieldSpec> {
        name: value["name"].as_str().unwrap().to_string(),
        spec: parse_object_field_spec(registry, &value["type"], &value["args"]),
        nullable: value["type"]["kind"].as_str().unwrap() != "NON_NULL",
    }
}
pub fn parse_server_schema(
    registry: &mut TypeRegistry,
    value: serde_json::Value,
) -> Result<(), String> {
    let types = value["data"]["__schema"]["types"]
        .as_array()
        .unwrap()
        .into_iter()
        .filter(|t| !t["name"].as_str().unwrap().starts_with("__"))
        .collect::<Vec<_>>();
    for t in &types {
        parse_node(registry, t);
    }
    Ok(())
}
