pub struct APIState {
    pub graphql_registry: libgql::parsers::schema::type_registry::TypeRegistry,
    pub graphql_resolvers_map:
        libgql::executor::Resolvers<'static, super::scalar::Scalar, super::context::Context>,
    pub graphql_parse_registry: libgql::executor::HashMapRegistry<super::scalar::Scalar>,
}
