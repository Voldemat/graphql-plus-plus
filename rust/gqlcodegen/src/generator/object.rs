use indexmap::IndexMap;

use crate::schema::{
    self,
    server::object::{
        ObjectField, ObjectFieldSpec, ObjectNonCallableFieldSpec, ObjectType,
    },
};

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

pub fn generate_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    existing_resolvers_code: &IndexMap<String, String>,
    object: &schema::server::object::Object,
) -> indexmap::IndexMap<(String, String), String> {
    let local = scope
        .new_struct(&object.name)
        .vis("pub")
        .derive("libgqlcodegen::macros::GQLObject")
        .r#macro(format!("#[gql(scalar = {})]", config.scalar_type));
    let mut resolver_fields = Vec::<(&String, &ObjectField)>::new();
    let mut local_fields = Vec::new();
    for (name, field) in &object.fields {
        if let ObjectFieldSpec::Callable(_) = field.spec {
            resolver_fields.push((name, field));
            continue;
        } else if config
            .field_to_resolver
            .contains(&(object.name.clone(), name.clone()))
        {
            resolver_fields.push((name, field));
            continue;
        }
        local_fields.push(name);
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
        if &field_name != name {
            field.annotation(format!("#[gql(name = \"{}\")]", name));
        }
        field.visibility = Some("pub".into());
        local.push_field(field);
    }
    return resolver_fields
        .iter()
        .map(|(name, field)| {
            (
                (object.name.to_string(), name.to_string()),
                generate_resolver_nodes(
                    config,
                    scope,
                    &object.name,
                    name,
                    field,
                    true,
                    existing_resolvers_code,
                ),
            )
        })
        .collect();
}

pub fn generate_resolver_nodes(
    config: &Config,
    scope: &mut codegen::Scope,
    object_name: &str,
    field_name: &str,
    field: &ObjectField,
    has_root: bool,
    existing_resolvers_code: &IndexMap<String, String>,
) -> String {
    let rust_name = super::shared::format_field_name(field_name);
    let object_rust_name = super::shared::format_field_name(&object_name);
    let resolver_fn_name = format!("{}_{}", object_rust_name, rust_name);
    let existing_code = existing_resolvers_code.get(&resolver_fn_name);
    let return_type = super::shared::generate_field_type(
        config,
        field,
        generate_object_field_spec_type,
        false,
    );
    let resolver_fn = scope
        .new_fn(&resolver_fn_name)
        .ret(format!(
            "Result<{}, libgql::executor::ast::ResolverError>",
            if object_name == "Subscription" {
                format!("impl futures_util::Stream<Item={}>", return_type)
            } else {
                return_type
            }
        ))
        .line(
            existing_code.unwrap_or(
                &"Err(\"Resolver is not implemented yet\".to_string().into())"
                    .to_string(),
            ),
        )
        .set_async(true);
    let mut call_arguments = Vec::new();
    if has_root {
        resolver_fn.arg("root", format!("&{}", object_name));
        call_arguments.push(format!(
            "(root as &dyn std::any::Any).downcast_ref::<{}>().unwrap()",
            object_name
        ));
    };
    resolver_fn.arg("context", format!("&{}", config.resolvers.context_type));
    call_arguments.push("context".to_string());
    let arguments_option = field.spec.get_arguments();
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

    let wrapper_fn_name = format!("{}_{}_wrapper", object_rust_name, rust_name);
    let wrapper_fn = scope.new_fn(&wrapper_fn_name).generic("'args");
    if has_root {
        wrapper_fn.arg(
            "root",
            format!(
                "&'args libgql::executor::ast::ResolverRoot<{}>",
                config.scalar_type
            ),
        );
    };
    wrapper_fn
        .arg(
            "context",
            format!("&'args {}", config.resolvers.context_type),
        )
        .arg("variables", "&'args libgql::executor::ResolvedVariables")
        .ret(format!(
            "{}<'args, {}>",
            if object_name == "Subscription" {
                "libgql::executor::subscriptions::SubscriptionResolverFuture"
            } else {
                "libgql::executor::ast::ResolverFuture"
            },
            config.scalar_type
        ));
    for line in arg_lines {
        wrapper_fn.line(line);
    }
    wrapper_fn.line("Box::pin(async move {");
    if object_name == "Subscription" {
        wrapper_fn.line(format!(
            "    {}({}).await.map(|s| Box::pin(s.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<{}>>)) as libgql::executor::subscriptions::SubscriptionResolverStream<{}>)",
            resolver_fn_name, call_arguments_str, config.scalar_type, config.scalar_type
        ));
    } else {
        wrapper_fn.line(format!(
            "    {}({}).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<{}>>)",
            resolver_fn_name, call_arguments_str, config.scalar_type
        ));
    }
    wrapper_fn.line("})");
    wrapper_fn_name
}

pub fn generate_root_object_definitions(
    config: &Config,
    scope: &mut codegen::Scope,
    existing_resolvers_code: &IndexMap<String, String>,
    object: &schema::server::object::Object,
) -> Vec<(String, String)> {
    object
        .fields
        .iter()
        .map(|(field_name, field)| {
            (
                field_name.clone(),
                generate_resolver_nodes(
                    config,
                    scope,
                    &object.name,
                    field_name,
                    field,
                    false,
                    existing_resolvers_code,
                ),
            )
        })
        .collect()
}
