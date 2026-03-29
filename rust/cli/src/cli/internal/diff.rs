use indexmap::{IndexMap, IndexSet};
use libgql::parsers::schema::{
    server::ast::{
        Interface, ObjectFieldSpec, ObjectType, ObjectTypeSpec, Union,
    },
    shared::ast::{
        Enum, FieldDefinition, InputFieldSpec, InputType, InputTypeSpec,
        NonCallableFieldSpec,
    },
};

use crate::cli::utils;

#[derive(clap::Args)]
pub struct DiffArgs {
    #[arg(short, long, help = "path to server json schema")]
    pub json_schema_path: std::path::PathBuf,
    #[arg(short, long, help = "url to introspection api")]
    pub url_to_api: url::Url,
}

const INTROSPECTION_QUERY: &str = r#"{"query":"query IntrospectionQuery {  __schema {    queryType {      name    }    mutationType {      name    }    subscriptionType {      name    }    types {      ...FullType    }    directives {      name      description      locations      args {        ...InputValue      }    }  }}fragment FullType on __Type {  kind  name  description  fields(includeDeprecated: true) {    name    description    args {      ...InputValue    }    type {      ...TypeRef    }    isDeprecated    deprecationReason  }  inputFields {    ...InputValue  }  interfaces {    ...TypeRef  }  enumValues(includeDeprecated: true) {    name    description    isDeprecated    deprecationReason  }  possibleTypes {    ...TypeRef  }}fragment InputValue on __InputValue {  name  description  type {    ...TypeRef  }  defaultValue}fragment TypeRef on __Type {  kind  name  ofType {    kind    name    ofType {      kind      name      ofType {        kind        name        ofType {          kind          name          ofType {            kind            name            ofType {              kind              name              ofType {                kind                name              }            }          }        }      }    }  }}"}"#;

pub fn compare_two_object_type_specs(
    spec: &ObjectTypeSpec,
    spec2: &ObjectTypeSpec,
    _path: &str,
) -> bool {
    match (spec, spec2) {
        (ObjectTypeSpec::ObjectType(a), ObjectTypeSpec::ObjectType(b)) => {
            a == b
        }
        (ObjectTypeSpec::Interface(a), ObjectTypeSpec::Interface(b)) => a == b,
        (ObjectTypeSpec::Scalar(a), ObjectTypeSpec::Scalar(b)) => a == b,
        (ObjectTypeSpec::Union(a), ObjectTypeSpec::Union(b)) => a == b,
        (ObjectTypeSpec::Enum(a), ObjectTypeSpec::Enum(b)) => a == b,
        _ => false,
    }
}

pub fn compare_two_input_type_specs(
    spec: &InputTypeSpec,
    spec2: &InputTypeSpec,
) -> bool {
    match (spec, spec2) {
        (InputTypeSpec::Scalar(a), InputTypeSpec::Scalar(b)) => a == b,
        (InputTypeSpec::Enum(a), InputTypeSpec::Enum(b)) => a == b,
        (InputTypeSpec::InputType(a), InputTypeSpec::InputType(b)) => a == b,
        _ => false,
    }
}

pub fn compare_two_non_callable_object_field_spec(
    spec: &NonCallableFieldSpec<ObjectTypeSpec>,
    spec2: &NonCallableFieldSpec<ObjectTypeSpec>,
    path: &str,
) -> bool {
    match (spec, spec2) {
        (
            NonCallableFieldSpec::Literal(a),
            NonCallableFieldSpec::Literal(b),
        ) => compare_two_object_type_specs(&a.r#type, &b.r#type, path),

        (NonCallableFieldSpec::Array(a), NonCallableFieldSpec::Array(b)) => {
            if !a.nullable && b.nullable {
                eprintln!("[{}] Became nullable", path);
                return false;
            }
            compare_two_non_callable_object_field_spec(
                &a.r#type, &b.r#type, path,
            )
        }
        _ => {
            eprintln!("[{}] Change type", path);
            false
        }
    }
}

fn get_field_spec_name(spec: &ObjectFieldSpec) -> &str {
    match spec {
        ObjectFieldSpec::Literal(_) => "Literal",
        ObjectFieldSpec::Array(_) => "Array",
        ObjectFieldSpec::Callable(_) => "Callable",
    }
}

fn compare_two_object_field_specs(
    spec: &ObjectFieldSpec,
    spec2: &ObjectFieldSpec,
    path: &str,
) -> bool {
    match (spec, spec2) {
        (ObjectFieldSpec::Literal(a), ObjectFieldSpec::Literal(b)) => {
            compare_two_object_type_specs(&a.r#type, &b.r#type, path)
        }
        (ObjectFieldSpec::Array(a), ObjectFieldSpec::Array(b)) => {
            if !a.nullable && b.nullable {
                false
            } else {
                compare_two_non_callable_object_field_spec(
                    &a.r#type, &b.r#type, path,
                )
            }
        }
        (ObjectFieldSpec::Callable(a), ObjectFieldSpec::Callable(b)) => {
            compare_two_non_callable_object_field_spec(
                &a.return_type,
                &b.return_type,
                path,
            ) && compare_arguments(&a.arguments, &b.arguments, path)
        }
        _ => {
            let spec_name = get_field_spec_name(spec);
            let spec2_name = get_field_spec_name(spec2);
            eprintln!(
                "[{}] Change type from {} to {}",
                path, spec_name, spec2_name
            );
            false
        }
    }
}

pub fn compare_two_input_field_specs(
    spec: &InputFieldSpec,
    spec2: &InputFieldSpec,
    path: &str,
) -> bool {
    match (spec, spec2) {
        (
            NonCallableFieldSpec::Literal(a),
            NonCallableFieldSpec::Literal(b),
        ) => compare_two_input_type_specs(&a.r#type, &b.r#type),

        (NonCallableFieldSpec::Array(a), NonCallableFieldSpec::Array(b)) => {
            if !a.nullable && b.nullable {
                eprintln!("[{}] Became nullable", path);
                return false;
            }
            compare_two_input_field_specs(&a.r#type, &b.r#type, path)
        }

        _ => {
            eprintln!("[{}] Change type", path);
            false
        }
    }
}

fn compare_two_object_field_definitions(
    field: &FieldDefinition<ObjectFieldSpec>,
    field2: &FieldDefinition<ObjectFieldSpec>,
    type_name: &str,
) -> bool {
    let mut is_valid = true;
    if !field.nullable && field2.nullable {
        eprintln!("[{}.{}] Became nullable", type_name, field.name);
        is_valid = false;
    };
    if !compare_two_object_field_specs(
        &field.spec,
        &field2.spec,
        &format!("{}.{}", type_name, field.name),
    ) {
        eprintln!("[{}.{}] Changed type spec", type_name, field.name);
        is_valid = false;
    };
    return is_valid;
}

fn compare_two_input_field_definitions(
    field: &FieldDefinition<InputFieldSpec>,
    field2: &FieldDefinition<InputFieldSpec>,
    type_name: &str,
) -> bool {
    let mut is_valid = true;
    if !field.nullable && field2.nullable {
        eprintln!("[{}.{}] Became nullable", type_name, field.name);
        is_valid = false;
    };
    if !compare_two_input_field_specs(
        &field.spec,
        &field2.spec,
        &format!("{}.{}", type_name, field.name),
    ) {
        eprintln!("[{}.{}] Changed type spec", type_name, field.name);
        is_valid = false;
    };
    return is_valid;
}

pub fn compare_arguments(
    arguments: &IndexMap<String, FieldDefinition<InputFieldSpec>>,
    arguments2: &IndexMap<String, FieldDefinition<InputFieldSpec>>,
    path: &str,
) -> bool {
    let mut is_valid = true;

    for (name, field) in arguments {
        if !arguments2.contains_key(name) {
            eprintln!("[{}] Removed argument {}", path, name);
            is_valid = false;
            continue;
        }

        if !compare_two_input_field_definitions(
            field,
            arguments2.get(name).unwrap(),
            &format!("{}:args", path),
        ) {
            is_valid = false;
        }
    }

    is_valid
}

fn compare_two_objects(object: &ObjectType, object2: &ObjectType) -> bool {
    let mut is_valid = true;
    for name in &object.implements {
        if !object2.implements.contains(name) {
            eprintln!(
                "[{}] Removed interface {} from extends list",
                object.name, name
            );
            is_valid = false;
        }
    }
    for (name, field) in &object.fields {
        if !object2.fields.contains_key(name) {
            eprintln!("[{}] Deleted field {}", object.name, name);
            is_valid = false;
            continue;
        }
        if !compare_two_object_field_definitions(
            field,
            object2.fields.get(name).unwrap(),
            &object.name,
        ) {
            is_valid = false;
        }
    }
    return is_valid;
}

fn compare_objects(
    objects: &IndexMap<String, ObjectType>,
    objects2: &IndexMap<String, ObjectType>,
) -> bool {
    let mut is_valid = true;
    for (name, object) in objects {
        if !objects2.contains_key(name) {
            eprintln!("Deleted object {}", name);
            is_valid = false;
            continue;
        };
        if !compare_two_objects(&object, &objects2.get(name).unwrap()) {
            is_valid = false;
        };
    }
    return is_valid;
}

fn compare_two_unions(union: &Union, union2: &Union) -> bool {
    let mut is_valid = true;
    for item in &union.items {
        if !union2.items.contains(item) {
            eprintln!("[{}] Removed type {}", union.name, item);
            is_valid = false;
        }
    }
    return is_valid;
}

fn compare_unions(
    unions: &IndexMap<String, Union>,
    unions2: &IndexMap<String, Union>,
) -> bool {
    let mut is_valid = true;
    for (name, union) in unions {
        if !unions2.contains_key(name) {
            eprintln!("Deleted union {}", name);
            is_valid = false;
            continue;
        };
        if !compare_two_unions(&union, &unions2.get(name).unwrap()) {
            is_valid = false;
        };
    }
    return is_valid;
}

fn compare_two_inputs(input: &InputType, input2: &InputType) -> bool {
    let mut is_valid = true;
    for (name, field) in &input.fields {
        if !input2.fields.contains_key(name) {
            eprintln!("[{}] Deleted field {}", input.name, name);
            is_valid = false;
            continue;
        }
        if !compare_two_input_field_definitions(
            field,
            input2.fields.get(name).unwrap(),
            &input.name,
        ) {
            is_valid = false;
        }
    }
    return is_valid;
}

fn compare_inputs(
    inputs: &IndexMap<String, InputType>,
    inputs2: &IndexMap<String, InputType>,
) -> bool {
    let mut is_valid = true;
    for (name, input) in inputs {
        if !inputs2.contains_key(name) {
            eprintln!("Deleted input {}", name);
            is_valid = false;
            continue;
        };
        if !compare_two_inputs(&input, &inputs2.get(name).unwrap()) {
            is_valid = false;
        };
    }
    return is_valid;
}

fn compare_two_interfaces(
    interface: &Interface,
    interface2: &Interface,
) -> bool {
    let mut is_valid = true;
    for (name, field) in &interface.fields {
        if !interface2.fields.contains_key(name) {
            eprintln!("[{}] Deleted field {}", interface.name, name);
            is_valid = false;
            continue;
        }
        if !compare_two_object_field_definitions(
            field,
            interface2.fields.get(name).unwrap(),
            &interface.name,
        ) {
            is_valid = false;
        }
    }
    return is_valid;
}

fn compare_interfaces(
    interfaces: &IndexMap<String, Interface>,
    interfaces2: &IndexMap<String, Interface>,
) -> bool {
    let mut is_valid = true;
    for (name, interface) in interfaces {
        if !interfaces2.contains_key(name) {
            eprintln!("Deleted interface {}", name);
            is_valid = false;
            continue;
        };
        if !compare_two_interfaces(&interface, &interfaces2.get(name).unwrap())
        {
            is_valid = false;
        };
    }
    return is_valid;
}

fn compare_scalars(
    scalars: &IndexSet<String>,
    scalars2: &IndexSet<String>,
) -> bool {
    let mut is_valid = true;
    for name in scalars {
        if !scalars2.contains(name) {
            eprintln!("Deleted scalar {}", name);
            is_valid = false;
        };
    }
    return is_valid;
}

fn compare_two_enums(e: &Enum, e2: &Enum) -> bool {
    let mut is_valid = true;
    for value in &e.values {
        if !e2.values.contains(value) {
            eprintln!("Removed {} value from enum {}", value, e.name);
            is_valid = false;
        }
    }
    return is_valid;
}

fn compare_enums(
    enums: &IndexMap<String, Enum>,
    enums2: &IndexMap<String, Enum>,
) -> bool {
    let mut is_valid = true;
    for (name, e) in enums {
        if !enums2.contains_key(name) {
            eprintln!("Deleted enum {}", name);
            is_valid = false;
            continue;
        };
        if !compare_two_enums(e, enums2.get(name).unwrap()) {
            is_valid = false;
        }
    }
    return is_valid;
}

fn find_difference_between_schemas(
    schema: &libgql::parsers::schema::server::type_registry::HashMapTypeRegistry,
    schema2: &libgql::parsers::schema::server::type_registry::HashMapTypeRegistry,
) {
    let is_objects_valid = compare_objects(&schema.objects, &schema2.objects);
    let is_unions_valid = compare_unions(&schema.unions, &schema2.unions);
    let is_inputs_valid = compare_inputs(&schema.inputs, &schema2.inputs);
    let is_interfaces_valid =
        compare_interfaces(&schema.interfaces, &schema2.interfaces);
    let is_scalars_valid = compare_scalars(&schema.scalars, &schema2.scalars);
    let is_enums_valid = compare_enums(&schema.enums, &schema2.enums);
    let is_valid = is_objects_valid
        && is_unions_valid
        && is_inputs_valid
        && is_interfaces_valid
        && is_scalars_valid
        && is_enums_valid;
    if !is_valid {
        eprintln!("Schema is incompatible");
        std::process::exit(1);
    };
    println!("Schema is compatible");
}

pub fn command(args: DiffArgs) {
    let mut registry =
        libgql::parsers::schema::server::type_registry::HashMapTypeRegistry::new();

    libgql::json::parsers::schema::parse_server_schema(
        &mut registry,
        serde_json::from_str::<serde_json::Value>(
            &utils::read_buffer_from_filepath(&args.json_schema_path),
        )
        .unwrap(),
    )
    .unwrap();
    let mut server_schema_from_introspection =
        libgql::parsers::schema::server::type_registry::HashMapTypeRegistry::new();
    libgql::json::parsers::introspection::parse_server_schema(
        &mut server_schema_from_introspection,
        serde_json::from_str::<serde_json::Value>(
            &reqwest::blocking::Client::new()
                .post(args.url_to_api.clone())
                .body(INTROSPECTION_QUERY)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .send()
                .unwrap()
                .text()
                .unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    find_difference_between_schemas(
        &registry,
        &server_schema_from_introspection,
    );
}
