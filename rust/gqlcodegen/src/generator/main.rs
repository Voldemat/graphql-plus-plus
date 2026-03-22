use super::config::Config;

pub fn generate_ast(config: &Config, schema: &crate::schema::Schema) -> String {
    let mut scope = codegen::Scope::new();
    for gqlenum in schema.server.enums.values() {
        super::enums::generate_definition(config, &mut scope, gqlenum);
    }
    for input in schema.server.inputs.values() {
        super::input::generate_definition(config, &mut scope, input);
    }
    let mut query_resolvers_map = Vec::<(String, String)>::new();
    let mut mutation_resolvers_map = Vec::<String>::new();
    let mut subscription_resolvers_map = Vec::<String>::new();
    for object in schema.server.objects.values() {
        if object.name == "Query" {
            query_resolvers_map.append(
                &mut super::object::generate_root_object_definitions(
                    config, &mut scope, object,
                ).into_iter().map(|v| ("Query".to_string(), v)).collect(),
            )
        } else if object.name == "Mutation" {
            mutation_resolvers_map.append(
                &mut super::object::generate_root_object_definitions(
                    config, &mut scope, object,
                ),
            )
        } else if object.name == "Subscription" {
            subscription_resolvers_map.append(
                &mut super::object::generate_root_object_definitions(
                    config, &mut scope, object,
                )
            )
        } else {
            query_resolvers_map.append(&mut super::object::generate_definition(
                config, &mut scope, object,
            ))
        }
    }
    for union in schema.server.unions.values() {
        super::union::generate_definition(config, &mut scope, union);
    }
    return scope.to_string();
}
