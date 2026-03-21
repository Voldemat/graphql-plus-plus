use crate::schema::{self, server::object::{ObjectFieldSpec, ObjectNonCallableFieldSpec, ObjectType}};

use super::config::Config;


fn generate_object_type(
    config: &Config,
    field_type: &schema::server::object::ObjectType,
) -> String {
    match field_type {
        ObjectType::ObjectType { name } => name.clone(),
        ObjectType::Enum { name } => name.clone(),
        ObjectType::Scalar { name } => config.scalars_mapping[name].clone(),
        ObjectType::Union { name } => name.clone(),
        ObjectType::InterfaceType { name } => name.clone(),
    }
}

fn generate_object_noncallable_field_spec_type(
    config: &Config,
    spec: &ObjectNonCallableFieldSpec,
) -> String {
    match spec {
        ObjectNonCallableFieldSpec::Array(a) => {
            let mut t = generate_object_noncallable_field_spec_type(
                config,
                &a.field_type,
            );
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
            let mut t = generate_object_noncallable_field_spec_type(
                config,
                &a.field_type,
            );
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

fn generate_object_type_value(
    config: &Config,
    input_expression: &str,
    type_spec: &schema::server::object::ObjectType,
) -> String {
    match type_spec {
        schema::server::object::ObjectType::ObjectType { name: _ } => {
            format!(
                "TryInto::<(String, libgql::executor::Values::<{}>)>::try_into({})?.into()",
                config.scalar_type, input_expression
            )
        }
        schema::server::object::ObjectType::Union { name: _ } => {
            format!(
                "TryInto::<(String, libgql::executor::Values::<{}>)>::try_into({})?.into()",
                config.scalar_type, input_expression
            )
        }
        schema::server::object::ObjectType::InterfaceType { name: _ } => {
            format!(
                "TryInto::<(String, libgql::executor::Values::<{}>)>::try_into({})?.into()",
                config.scalar_type, input_expression
            )
        }
        schema::server::object::ObjectType::Enum { name } => {
            format!(
                "<{} as libgql::executor::GQLEnum<{}>>::to_literal_value(&{})?",
                name, config.scalar_type, input_expression
            )
        }
        schema::server::object::ObjectType::Scalar { name } => {
            format!(
                "<{} as libgql::executor::GQLScalar<{}>>::to_literal_value(&{})?",
                config.scalars_mapping[name],
                config.scalar_type,
                input_expression
            )
        }
    }
}

fn generate_object_non_callable_field_spec_value(
    config: &Config,
    input_expression: &str,
    spec: &schema::server::object::ObjectNonCallableFieldSpec,
) -> String {
    match spec {
        schema::server::object::ObjectNonCallableFieldSpec::Literal(
            literal,
        ) => {
            format!(
                "libgql::executor::NonNullableValue::Literal({})",
                generate_object_type_value(
                    config,
                    input_expression,
                    &literal.field_type
                )
            )
        }
        schema::server::object::ObjectNonCallableFieldSpec::Array(array) => {
            format!(
                "libgql::executor::NonNullableValue::Array({}.into_iter().map(|element| Ok({})).collect::<Result<Vec<_>, String>>()?)",
                input_expression,
                generate_object_field_value(
                    config,
                    "element",
                    array.nullable,
                    array.field_type.as_ref()
                )
            )
        }
    }
}

fn generate_object_field_value(
    config: &Config,
    input_expression: &str,
    nullable: bool,
    spec: &schema::server::object::ObjectNonCallableFieldSpec,
) -> String {
    if !nullable {
        format!(
            "libgql::executor::Value::NonNullable({})",
            generate_object_non_callable_field_spec_value(
                config,
                input_expression,
                spec
            )
        )
    } else {
        format!(
            "{}.map(|v| -> Result<libgql::executor::Value<{}>, String> {{Ok(libgql::executor::Value::NonNullable({}))}}).transpose()?.unwrap_or(libgql::executor::Value::Null)",
            input_expression,
            config.scalar_type,
            generate_object_non_callable_field_spec_value(config, "v", spec)
        )
    }
}

fn generate_object_fields_value(
    config: &Config,
    fields: &indexmap::IndexMap<String, schema::server::object::ObjectField>,
) -> String {
    let mut s = "[".to_string();
    for (field_name, nullable, spec) in
        fields
            .iter()
            .filter_map(|(field_name, field)| match &field.spec {
                schema::server::object::ObjectFieldSpec::Literal(l) => Some((
                    field_name,
                    field.nullable,
                    schema::server::object::ObjectNonCallableFieldSpec::Literal(
                        l.clone(),
                    ),
                )),
                schema::server::object::ObjectFieldSpec::Array(a) => Some((
                    field_name,
                    field.nullable,
                    schema::server::object::ObjectNonCallableFieldSpec::Array(
                        a.clone(),
                    ),
                )),
                schema::server::object::ObjectFieldSpec::Callable(_) => None,
            })
    {
        s += &format!(
            "(\"{}\".to_string(), {}),\n",
            field_name,
            generate_object_field_value(
                config,
                &format!(
                    "self.{}",
                    super::shared::format_field_name(field_name)
                ),
                nullable,
                &spec
            )
        )
    }
    s += "]";
    return s;
}

pub fn generate_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    object: &schema::server::object::Object,
) -> Vec<(String, String)> {
    let local = scope.new_struct(&object.name).vis("pub");
    for (name, field) in &object.fields {
        let field_name = super::shared::format_field_name(name);
        let mut field = codegen::Field::new(
            &field_name,
            super::shared::generate_field_type(
                config,
                field,
                generate_object_field_spec_type,
                false,
            ),
        );
        field.visibility = Some("pub".into());
        local.push_field(field);
    }
    let resolver_value_impl = scope.new_impl(&object.name).impl_trait(format!(
        "libgql::executor::ast::ResolverValue<{}>",
        config.scalar_type
    ));
    resolver_value_impl
        .new_fn("create_introspection_value")
        .generic("'a")
        .arg("self", "&'a Self")
        .ret(format!(
            "libgql::executor::ast::ResolverIntrospectionValue<'a, {}>",
            config.scalar_type
        ))
        .line("todo!()");

    resolver_value_impl
        .new_fn("get_existing_fields")
        .arg("self", "&Self")
        .ret("std::collections::HashSet<String>")
        .line("todo!()");
    resolver_value_impl
        .new_fn("to_value")
        .arg_ref_self()
        .arg(
            "_callable_fields",
            format!(
                "Vec<(String, libgql::executor::ast::Value<{}>)>",
                config.scalar_type
            ),
        )
        .ret(format!(
            "Result<libgql::executor::ast::Value<{}>, String>",
            config.scalar_type
        ))
        .line("todo!()");

    let impl_block = scope.new_impl(&object.name).impl_trait(format!(
        "TryInto<(String, libgql::executor::Values<{}>)>",
        config.scalar_type
    ));
    impl_block.associate_type("Error", "String");
    impl_block
        .new_fn("try_into")
        .arg_self()
        .ret(format!(
            "Result<(String, libgql::executor::Values<{}>), Self::Error>",
            config.scalar_type
        ))
        .line(format!(
            "Ok((\"{}\".to_string(), libgql::executor::Values::from_iter({})))",
            object.name,
            generate_object_fields_value(config, &object.fields)
        ));
    return Vec::new();
}

pub fn generate_root_object_definitions(
    config: &Config,
    scope: &mut codegen::Scope,
    object: &schema::server::object::Object,
) -> Vec<(String, String)> {
    object.fields.iter().map(|(field_name, field)| {
        let rust_name = super::shared::format_field_name(field_name);
        let object_rust_name = super::shared::format_field_name(&object.name);
        let resolver_fn_name = format!("{}_{}", object_rust_name, rust_name);
        let resolver_fn = scope
            .new_fn(&resolver_fn_name)
            .ret(format!(
                "Result<{}, String>",
                super::shared::generate_field_type(
                    config,
                    field,
                    generate_object_field_spec_type,
                    false
                )
            ))
            .line("todo!()")
            .set_async(true);
        resolver_fn
            .arg("context", format!("&{}", config.resolvers.context_type));
        let arguments_option = field.spec.get_arguments();
        let mut call_arguments = vec!["context".to_string()];
        let mut arg_lines = Vec::new();
        if let Some(arguments) = arguments_option {
            for (arg_name, arg) in arguments {
                let arg_rust_name = super::shared::format_field_name(arg_name);
                resolver_fn.arg(
                    &arg_rust_name,
                    super::shared::generate_field_type(
                        config,
                        arg,
                        super::input::generate_input_field_spec_type,
                        true,
                    ),
                );
                arg_lines.push(format!(
                    "let {} = {};",
                    arg_rust_name,
                    super::input::generate_extract_arg_expression(
                        config,
                        "variables",
                        arg_name,
                        arg
                    )
                ));
                call_arguments.push(arg_rust_name);
            }
        }
        let call_arguments_str = call_arguments.join(", ");

        let wrapper_fn = scope
            .new_fn(format!("{}_{}_wrapper", object_rust_name, rust_name))
            .generic("'args");
        if object.name == "Query" {
            wrapper_fn.arg(
                "root_any_ref",
                format!(
                    "&'args libgql::executor::ast::ResolverRoot<{}>",
                    config.scalar_type
                ),
            );
        };
        wrapper_fn.arg(
            "context",
            format!("&'args {}", config.resolvers.context_type),
        )
        .arg("variables", "&'args libgql::executor::ResolvedVariables")
        .ret(format!(
            "libgql::executor::ast::ResolverFuture<'args, {}>",
            config.scalar_type
        ));
        for line in arg_lines {
            wrapper_fn.line(line);
        }
        wrapper_fn.line("Box::pin(async move {");
        wrapper_fn.line(format!(
            "    {}({}).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<{}>>)",
            resolver_fn_name, call_arguments_str, config.scalar_type
        ));
        wrapper_fn.line("})");
        (object.name.clone(), field_name.clone())
    }).collect()
}
