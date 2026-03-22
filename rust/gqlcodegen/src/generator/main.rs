use std::collections::HashMap;

use super::config::Config;

pub fn generate_create_resolvers_map(
    config: &Config,
    scope: &mut codegen::Scope,
    query_resolvers_map: HashMap<(String, String), String>,
    mutation_resolvers_map: HashMap<String, String>,
    subscription_resolvers_map: HashMap<String, String>,
) {
    let f = scope.new_fn("create_resolvers_map").vis("pub").ret(format!(
        "libgql::executor::Resolvers<{}, {}>",
        config.scalar_type, config.resolvers.context_type
    ));
    let query_resolvers_str = query_resolvers_map.into_iter().map(|((object_name, field_name), wrapper_fn)| {
        format!(
            "((\"{}\", \"{}\"), Box::new({}) as libgql::executor::queries::QueryResolver<{}, {}>)",
            object_name, field_name, wrapper_fn, config.scalar_type,
            config.resolvers.context_type
        )
    }).map(|s| format!("        {}", s)).collect::<Vec<_>>().join(",\n");
    let mutation_resolvers_str = mutation_resolvers_map.into_iter().map(|(field_name, wrapper_fn)| {
        format!(
            "(\"{}\", Box::new({}) as libgql::executor::mutations::MutationResolver<{}, {}>)",
            field_name, wrapper_fn, config.scalar_type,
            config.resolvers.context_type
        )
    }).map(|s| format!("        {}", s)).collect::<Vec<_>>().join(",\n");
    let subscription_resolvers_str = subscription_resolvers_map.into_iter().map(|(field_name, wrapper_fn)| {
        format!(
            "(\"{}\", Box::new({}) as libgql::executor::subscriptions::SubscriptionResolver<{}, {}>)",
            field_name, wrapper_fn, config.scalar_type,
            config.resolvers.context_type
        )
    }).map(|s| format!("        {}", s)).collect::<Vec<_>>().join(",\n");
    f.line("libgql::executor::Resolvers {");
    f.line(format!("   queries: libgql::executor::queries::QueryResolversMap::from_iter([\n{}\n]),", query_resolvers_str));
    f.line(format!("   mutations: libgql::executor::mutations::MutationResolversMap::from_iter([\n{}\n]),", mutation_resolvers_str));
    f.line(format!("   subscriptions: libgql::executor::subscriptions::SubscriptionResolversMap::from_iter([\n{}\n])", subscription_resolvers_str));
    f.line("}");
}

fn generate_create_parse_registry_function(
    config: &Config,
    schema: &crate::schema::Schema,
    scope: &mut codegen::Scope,
) {
    let f = scope.new_fn("create_parse_registry").ret(format!(
        "libgql::executor::HashMapRegistry<{}>",
        config.scalar_type
    )).vis("pub").line(format!("let mut registry = libgql::executor::HashMapRegistry::<{}>::default();", config.scalar_type));
    for input_name in schema.server.inputs.keys() {
        f.line(format!(
            "registry.add_input::<{}>(\"{}\");",
            input_name, input_name
        ));
    }
    for enum_name in schema.server.enums.keys() {
        f.line(format!(
            "registry.add_enum::<{}>(\"{}\");",
            enum_name, enum_name
        ));
    }
    for (scalar_name, rust_name) in &config.scalars_mapping {
        f.line(format!(
            "registry.add_scalar::<{}>(\"{}\");",
            rust_name, scalar_name
        ));
    }
    f.line("return registry;");
}

pub fn generate_ast(config: &Config, schema: &crate::schema::Schema) -> String {
    let mut scope = codegen::Scope::new();
    for gqlenum in schema.server.enums.values() {
        super::enums::generate_definition(config, &mut scope, gqlenum);
    }
    for input in schema.server.inputs.values() {
        super::input::generate_definition(config, &mut scope, input);
    }
    let mut query_resolvers_map = HashMap::<(String, String), String>::new();
    let mut mutation_resolvers_map = HashMap::<String, String>::new();
    let mut subscription_resolvers_map = HashMap::<String, String>::new();
    for object in schema.server.objects.values() {
        if object.name == "Query" {
            query_resolvers_map.extend(
                super::object::generate_root_object_definitions(
                    config, &mut scope, object,
                )
                .into_iter()
                .map(|(field, wrapper_fn)| {
                    (("Query".to_string(), field), wrapper_fn)
                })
                .collect::<HashMap<(String, String), String>>(),
            )
        } else if object.name == "Mutation" {
            mutation_resolvers_map.extend(
                super::object::generate_root_object_definitions(
                    config, &mut scope, object,
                )
                .into_iter()
                .collect::<HashMap<String, String>>(),
            )
        } else if object.name == "Subscription" {
            subscription_resolvers_map.extend(
                super::object::generate_root_object_definitions(
                    config, &mut scope, object,
                )
                .into_iter()
                .collect::<HashMap<String, String>>(),
            )
        } else {
            query_resolvers_map.extend(super::object::generate_definition(
                config, &mut scope, object,
            ))
        }
    }
    for union in schema.server.unions.values() {
        super::union::generate_definition(config, &mut scope, union);
    }
    generate_create_resolvers_map(
        config,
        &mut scope,
        query_resolvers_map,
        mutation_resolvers_map,
        subscription_resolvers_map,
    );
    generate_create_parse_registry_function(config, schema, &mut scope);
    return scope.to_string();
}
