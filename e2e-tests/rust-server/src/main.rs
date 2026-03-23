use std::sync::Arc;

mod api;

const GRAPHQL_SERVER_SCHEMA: &str = include_str!("../../graphql/server-schema.json");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Arc::new(api::state::APIState {
        graphql_resolvers_map: api::generated::create_resolvers_map(),
        graphql_parse_registry: api::generated::create_parse_registry(),
        graphql_registry: {
            let mut registry = libgql::parsers::schema::type_registry::TypeRegistry::new();
            libgql::json::parsers::schema::parse_server_schema(
                &mut registry,
                serde_json_path_to_error::from_str::<serde_json_path_to_error::Value>(
                    GRAPHQL_SERVER_SCHEMA,
                )
                .unwrap(),
            )
            .unwrap();
            registry
        },
    });
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(state.clone()))
            .service(api::graphql)
    })
    .bind(("0.0.0.0", std::env::var("PORT").unwrap().parse().unwrap()))?
    .run()
    .await
}
