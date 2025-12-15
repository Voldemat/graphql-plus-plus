use super::config::Config;
use crate::schema;
use schema::server::object::ObjectFieldSpec;
use schema::server::object::ObjectNonCallableFieldSpec;
use schema::server::object::ObjectType;

fn generate_enum_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    gqlenum: &schema::server::gqlenum::Enum,
) {
    let local = scope
        .new_enum(&gqlenum.name)
        .derive("Debug, juniper::GraphQLEnum")
        .vis("pub");
    if let Some(scalar_type) = &config.scalar_type_override {
        local.r#macro(format!("#[graphql(scalar = {})]", scalar_type));
    }
    for value in &gqlenum.values {
        local.push_variant(codegen::Variant::new(&value));
    }
}

fn generate_union_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    union: &schema::server::union::Union,
) {
    let local = scope
        .new_enum(&union.name)
        .derive("Debug, juniper::GraphQLUnion")
        .vis("pub");
    if let Some(scalar_type) = &config.scalar_type_override {
        local.r#macro(format!("#[graphql(scalar = {})]", scalar_type));
    }
    for item in union.items.keys() {
        let mut variant = codegen::Variant::new(&item);
        variant.tuple(&item);
        local.push_variant(variant);
    }
}

fn generate_input_type(
    config: &Config,
    field_type: &schema::shared::InputType,
) -> String {
    match field_type {
        schema::shared::InputType::InputType { name } => name.clone(),
        schema::shared::InputType::Enum { name } => name.clone(),
        schema::shared::InputType::Scalar { name } => {
            println!("{}", name);
            config.scalars_mapping[name].clone()
        }
    }
}

fn generate_object_type(
    config: &Config,
    field_type: &schema::server::object::ObjectType,
) -> String {
    match field_type {
        ObjectType::ObjectType { name } => name.clone(),
        ObjectType::Enum { name } => name.clone(),
        ObjectType::Scalar { name } => {
            println!("{}", name);
            config.scalars_mapping[name].clone()
        }
        ObjectType::Union { name } => name.clone(),
        ObjectType::InterfaceType { name } => name.clone(),
    }
}

fn generate_input_field_spec_type(
    config: &Config,
    spec: &schema::shared::InputFieldSpec,
) -> String {
    match spec {
        schema::shared::InputFieldSpec::Array(a) => {
            let mut t = generate_input_type(config, &a.field_type);
            if a.nullable {
                t = format!("Option<{}>", t);
            }
            return format!("Vec<{}>", t);
        }
        schema::shared::InputFieldSpec::Literal(a) => {
            return generate_input_type(config, &a.field_type);
        }
    }
}

fn generate_field_type<T>(
    config: &Config,
    field: &schema::shared::Field<T>,
    func: impl FnOnce(&Config, &T) -> String,
) -> String {
    let t = func(config, &field.spec);
    if field.nullable {
        return format!("Option<{}>", t);
    }
    return t;
}

fn generate_input_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    input: &schema::server::input::Input,
) {
    let local = scope
        .new_struct(&input.name)
        .vis("pub")
        .derive("Debug, juniper::GraphQLInputObject");
    if let Some(scalar_type) = &config.scalar_type_override {
        local.r#macro(format!("#[graphql(scalar = {})]", scalar_type));
    }
    for (name, field) in &input.fields {
        let field_name = super::shared::format_field_name(name);
        let mut field = codegen::Field::new(
            &field_name,
            generate_field_type(config, field, generate_input_field_spec_type),
        );
        if &field_name != name {
            field.annotation = vec![format!("#[graphql(name = \"{}\")]", name)];
        }
        field.visibility = Some("pub".into());
        local.push_field(field);
    }
}

fn generate_object_noncallable_field_spec_type(
    config: &Config,
    spec: &ObjectNonCallableFieldSpec,
) -> String {
    match spec {
        ObjectNonCallableFieldSpec::Array(a) => {
            let mut t = generate_object_type(config, &a.field_type);
            if a.nullable {
                t = format!("Option<{}>", t);
            }
            return format!("Vec<{}>", t);
        }
        ObjectNonCallableFieldSpec::Literal(a) => {
            return generate_object_type(config, &a.field_type);
        }
    }
}

fn generate_object_field_spec_type(
    config: &Config,
    spec: &ObjectFieldSpec,
) -> String {
    match spec {
        ObjectFieldSpec::Array(a) => {
            let mut t = generate_object_type(config, &a.field_type);
            if a.nullable {
                t = format!("Option<{}>", t);
            }
            return format!("Vec<{}>", t);
        }
        ObjectFieldSpec::Literal(a) => {
            return generate_object_type(config, &a.field_type);
        }
        ObjectFieldSpec::Callable(a) => {
            return generate_object_noncallable_field_spec_type(
                &config,
                &a.return_type,
            );
        }
    }
}

fn generate_object_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    object: &schema::server::object::Object,
) {
    let local = scope
        .new_struct(&object.name)
        .vis("pub")
        .derive("Debug, juniper::GraphQLObject");
    if let Some(scalar_type) = &config.scalar_type_override {
        local.r#macro(format!("#[graphql(scalar = {})]", scalar_type));
    }
    for (name, field) in &object.fields {
        let field_name = super::shared::format_field_name(name);
        let mut field = codegen::Field::new(
            &field_name,
            generate_field_type(config, field, generate_object_field_spec_type),
        );
        if &field_name != name {
            field.annotation = vec![format!("#[graphql(name = \"{}\")]", name)];
        }
        field.visibility = Some("pub".into());
        local.push_field(field);
    }
}

pub fn generate_ast(config: &Config, schema: &crate::schema::Schema) -> String {
    let mut scope = codegen::Scope::new();
    for gqlenum in schema.server.enums.values() {
        generate_enum_definition(config, &mut scope, gqlenum);
    }
    for input in schema.server.inputs.values() {
        generate_input_definition(config, &mut scope, input);
    }
    for object in schema.server.objects.values() {
        generate_object_definition(config, &mut scope, object);
    }
    for union in schema.server.unions.values() {
        generate_union_definition(config, &mut scope, union);
    }
    return scope.to_string();
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_generate_enum_definition() {
        let gqlenum = schema::server::gqlenum::Enum {
            name: "Check".into(),
            values: vec![
                "FirstValue".into(),
                "SecondValue".into(),
                "ThirdValue".into(),
            ],
        };
        let mut scope = codegen::Scope::new();
        generate_enum_definition(
            &Config {
                scalars_mapping: HashMap::new(),
                scalar_type_override: None,
            },
            &mut scope,
            &gqlenum,
        );
        let output = scope.to_string();
        assert_eq!(
            output,
            "#[derive(Debug, juniper::GraphQLEnum)]
pub enum Check {
    FirstValue,
    SecondValue,
    ThirdValue,
}"
        )
    }
}
