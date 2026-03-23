use std::{cell::RefCell, sync::Arc};

use indexmap::IndexMap;

use crate::parsers::schema::{server, shared, type_registry::TypeRegistry};

fn parse_node_first_pass(
    value: &serde_json::Value,
) -> server::ast::ServerSchemaNode {
    let kind = value["kind"].as_str().unwrap();
    let name = value["name"].as_str().unwrap();
    match kind {
        "SCALAR" => server::ast::ServerSchemaNode::Scalar(name.to_string()),
        "ENUM" => Arc::new(shared::ast::Enum {
            name: name.to_string(),
            values: value["enumValues"]
                .as_array()
                .unwrap()
                .into_iter()
                .map(|enum_value| {
                    enum_value["name"].as_str().unwrap().to_string()
                })
                .collect(),
        })
        .into(),
        "INPUT_OBJECT" => Arc::new(RefCell::new(shared::ast::InputType {
            name: name.to_string(),
            fields: IndexMap::new(),
        }))
        .into(),
        "OBJECT" => Arc::new(RefCell::new(server::ast::ObjectType {
            name: name.to_string(),
            fields: IndexMap::new(),
            implements: IndexMap::new(),
            directives: Vec::new(),
        }))
        .into(),
        "UNION" => Arc::new(RefCell::new(server::ast::Union {
            name: name.to_string(),
            items: IndexMap::new(),
        }))
        .into(),
        "INTERFACE" => Arc::new(RefCell::new(server::ast::Interface {
            name: name.to_string(),
            fields: IndexMap::new(),
            directives: Vec::new(),
        }))
        .into(),
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
        "ENUM" => registry
            .enums
            .get(value["name"].as_str().unwrap())
            .unwrap()
            .clone()
            .into(),
        "INPUT_OBJECT" => registry
            .inputs
            .get(value["name"].as_str().unwrap())
            .unwrap()
            .clone()
            .into(),
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
        "SCALAR" => server::ast::ObjectTypeSpec::Scalar {
            name: value["name"].as_str().unwrap().into(),
        },
        "ENUM" => registry
            .enums
            .get(value["name"].as_str().unwrap())
            .unwrap()
            .clone()
            .into(),
        "OBJECT" => registry
            .objects
            .get(value["name"].as_str().unwrap())
            .unwrap()
            .clone()
            .into(),
        "UNION" => registry
            .unions
            .get(value["name"].as_str().unwrap())
            .unwrap()
            .clone()
            .into(),
        "INTERFACE" => registry
            .interfaces
            .get(value["name"].as_str().unwrap())
            .unwrap()
            .clone()
            .into(),
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

fn parse_node_second_pass(
    registry: &TypeRegistry,
    value: &serde_json::Value,
) -> server::ast::ServerSchemaNode {
    let kind = value["kind"].as_str().unwrap();
    let name = value["name"].as_str().unwrap();

    match kind {
        "SCALAR" => name.to_string().into(),
        "ENUM" => registry.enums.get(name).unwrap().clone().into(),
        "INPUT_OBJECT" => {
            let input = registry.inputs.get(name).unwrap().clone();
            input.borrow_mut().fields = value["inputFields"]
                .as_array()
                .unwrap()
                .into_iter()
                .map(|field_json| {
                    let field =
                        parse_input_field_definition(registry, field_json);
                    (field.name.clone(), field)
                })
                .collect();
            return input.into();
        }
        "OBJECT" => {
            let object = registry.objects.get(name).unwrap().clone();
            object.borrow_mut().implements = value["interfaces"]
                .as_array()
                .unwrap()
                .into_iter()
                .map(|interface_json| {
                    let interface = registry
                        .interfaces
                        .get(interface_json["name"].as_str().unwrap())
                        .unwrap();
                    (interface.borrow().name.clone(), interface.clone())
                })
                .collect();
            object.borrow_mut().fields = value["fields"]
                .as_array()
                .unwrap()
                .into_iter()
                .map(|field_json| {
                    let field =
                        parse_object_field_definition(registry, field_json);
                    (field.name.clone(), Arc::new(field))
                })
                .collect();
            return object.into();
        }
        "UNION" => {
            let union = registry.unions.get(name).unwrap().clone();
            union.borrow_mut().items = value["possibleTypes"]
                .as_array()
                .unwrap()
                .into_iter()
                .map(|object_json| {
                    let object = registry
                        .objects
                        .get(object_json["name"].as_str().unwrap())
                        .unwrap();
                    (object.borrow().name.clone(), object.clone())
                })
                .collect();
            return union.into();
        }
        "INTERFACE" => {
            let interface = registry.interfaces.get(name).unwrap().clone();
            interface.borrow_mut().fields = value["fields"]
                .as_array()
                .unwrap()
                .into_iter()
                .map(|field_json| {
                    let field =
                        parse_object_field_definition(registry, field_json);
                    (field.name.clone(), Arc::new(field))
                })
                .collect();
            return interface.into();
        }
        _ => panic!("Unexpected server node kind: {}", kind),
    }
}

pub fn parse_server_schema(
    registry: &mut TypeRegistry,
    value: serde_json::Value,
) -> Result<server::schema::Schema, String> {
    let types = value["data"]["__schema"]["types"]
        .as_array()
        .unwrap()
        .into_iter()
        .filter(|t| !t["name"].as_str().unwrap().starts_with("__"))
        .collect::<Vec<_>>();
    for t in &types {
        registry.add_server_node(parse_node_first_pass(t));
    }
    return Ok(server::schema::Schema::from_nodes(
        &types
            .into_iter()
            .map(|t| parse_node_second_pass(registry, t))
            .collect::<Vec<_>>(),
    ));
}
