use crate::schema;

use super::config::Config;

pub fn generate_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    union: &schema::server::union::Union,
) {
    let local = scope.new_enum(&union.name).vis("pub");
    for item in union.items.keys() {
        let mut variant = codegen::Variant::new(&item);
        variant.tuple(&item);
        local.push_variant(variant);
    }

    let resolver_value_impl = scope.new_impl(&union.name).impl_trait(format!(
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

    let impl_block = scope.new_impl(&union.name).impl_trait(format!(
        "TryInto<(String, libgql::executor::Values<{}>)>",
        config.scalar_type
    ));
    impl_block.associate_type("Error", "String");
    let try_into_func = impl_block.new_fn("try_into").arg_self().ret(format!(
        "Result<(String, libgql::executor::Values<{}>), Self::Error>",
        config.scalar_type
    ));
    try_into_func.line("match self {");
    for item in union.items.keys() {
        try_into_func.line(format!("Self::{}(item) => TryInto::<(String, libgql::executor::Values::<{}>)>::try_into(item),", item, config.scalar_type));
    }
    try_into_func.line("}");
}
