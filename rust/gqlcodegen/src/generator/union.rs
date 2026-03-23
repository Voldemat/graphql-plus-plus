use crate::schema;

use super::config::Config;

pub fn generate_definition(
    config: &Config,
    scope: &mut codegen::Scope,
    union: &schema::server::union::Union,
) {
    let local = scope
        .new_enum(&union.name)
        .vis("pub")
        .derive("libgqlcodegen::macros::GQLUnion")
        .r#macro(format!("#[gql(scalar={})]", config.scalar_type));
    for item in union.items.keys() {
        let mut variant = codegen::Variant::new(&item);
        variant.tuple(&item);
        local.push_variant(variant);
    }
}
