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
    let to_value_fn = resolver_value_impl
        .new_fn("to_value")
        .generic("'a")
        .arg("self", "&'a Self")
        .ret(format!(
            "Result<libgql::executor::ast::ResolverIntrospectionValue<'a, {}>, String>",
            config.scalar_type
        ));
    to_value_fn.line("match self {");
    for item in union.items.keys() {
        to_value_fn.line(format!("Self::{}(item) => item.to_value(),", item));
    };
    to_value_fn.line("}");
}
