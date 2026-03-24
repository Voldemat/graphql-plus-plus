async fn query_get_user(
    context: &super::context::Context,
    id: &uuid::Uuid,
) -> Result<User, libgql::executor::ast::ResolverError> {
    Ok(User {
        id: id.clone(),
        name: "test-name".to_string()
    })
}

fn query_get_user_wrapper<'args>(
    context: &'args super::context::Context,
    variables: &'args libgql::executor::ResolvedVariables,
) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::Scalar> {
    let id = variables
        .get("id")
        .unwrap()
        .downcast_ref::<uuid::Uuid>()
        .unwrap();
    Box::pin(async move {
        query_get_user(context, id)
            .await
            .map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::Scalar>>)
    })
}

#[derive(libgqlcodegen::macros::GQLObject)]
#[gql(scalar = super::scalar::Scalar)]
pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
}

async fn user_email(
    root: &User,
    context: &super::context::Context,
) -> Result<String, libgql::executor::ast::ResolverError> {
    Ok("test@gmail.com".to_string())
}

fn user_email_wrapper<'args>(
    root: &'args libgql::executor::ast::ResolverRoot<super::scalar::Scalar>,
    context: &'args super::context::Context,
    variables: &'args libgql::executor::ResolvedVariables,
) -> libgql::executor::ast::ResolverFuture<'args, super::scalar::Scalar> {
    Box::pin(async move {
        user_email(
            (root as &dyn std::any::Any).downcast_ref::<User>().unwrap(),
            context,
        )
        .await
        .map(|v| Box::new(v) as Box<libgql::executor::ast::ResolverRoot<super::scalar::Scalar>>)
    })
}

pub fn create_resolvers_map()
-> libgql::executor::Resolvers<'static, super::scalar::Scalar, super::context::Context> {
    libgql::executor::Resolvers {
        queries: libgql::executor::queries::QueryResolversMap::from_iter([(
            "getUser",
            &query_get_user_wrapper
                as &libgql::executor::queries::QueryResolver<
                    super::scalar::Scalar,
                    super::context::Context,
                >,
        )]),
        mutations: libgql::executor::mutations::MutationResolversMap::from_iter([]),
        subscriptions: libgql::executor::subscriptions::SubscriptionResolversMap::from_iter([]),
        object_fields: libgql::executor::object::ObjectFieldResolversMap::from_iter([(
            ("User", "email"),
            &user_email_wrapper
                as &libgql::executor::object::ObjectFieldResolver<
                    super::scalar::Scalar,
                    super::context::Context,
                >,
        )]),
    }
}

pub fn create_parse_registry() -> libgql::executor::HashMapRegistry<super::scalar::Scalar> {
    let mut registry = libgql::executor::HashMapRegistry::<super::scalar::Scalar>::default();
    registry.add_scalar::<bool>("Boolean");
    registry.add_scalar::<String>("String");
    registry.add_scalar::<i32>("Int");
    registry.add_scalar::<f32>("Float");
    registry.add_scalar::<uuid::Uuid>("UUID");
    return registry;
}
