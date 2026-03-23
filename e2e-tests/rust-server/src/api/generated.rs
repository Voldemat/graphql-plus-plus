#[derive(libgqlcodegen::macros::GQLObject)]
#[gql(scalar = super::scalar::Scalar)]
pub struct Check {
    pub a: i32,
}

async fn query_get_check(context: &super::context::Context) -> Result<Check, String> {
    todo!()
}

fn query_get_check_wrapper<'args>(context: &'args super::context::Context, variables: &'args libgql::executor::ResolvedVariables) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::Scalar> {
    Box::pin(async move {
        query_get_check(context).await.map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::Scalar>>)
    })
}

pub fn create_resolvers_map() -> libgql::executor::Resolvers<'static, super::scalar::Scalar, super::context::Context> {
    libgql::executor::Resolvers {
       queries: libgql::executor::queries::QueryResolversMap::from_iter([
            ("getCheck", &query_get_check_wrapper as &libgql::executor::queries::QueryResolver<super::scalar::Scalar, super::context::Context>)
    ]),
       mutations: libgql::executor::mutations::MutationResolversMap::from_iter([

    ]),
       subscriptions: libgql::executor::subscriptions::SubscriptionResolversMap::from_iter([

    ]),
       object_fields: libgql::executor::object::ObjectFieldResolversMap::from_iter([

    ])
    }
}

pub fn create_parse_registry() -> libgql::executor::HashMapRegistry<super::scalar::Scalar> {
    let mut registry = libgql::executor::HashMapRegistry::<super::scalar::Scalar>::default();
    registry.add_scalar::<f32>("Float");
    registry.add_scalar::<bool>("Boolean");
    registry.add_scalar::<i32>("Int");
    registry.add_scalar::<String>("String");
    return registry;
}