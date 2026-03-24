use std::sync::Arc;

mod api;

const GRAPHQL_SERVER_SCHEMA: &str = include_str!("../../graphql/server-schema.json");

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let port = std::env::var("PORT").unwrap().parse().unwrap();
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
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
    loop {
        let (stream, _) = listener.accept().await?;

        let io = hyper_util::rt::TokioIo::new(stream);
        let local_state = state.clone();

        tokio::task::spawn_local(async move {
            let s = local_state;
            if let Err(err) = hyper::server::conn::http1::Builder::new()
                .serve_connection(
                    io,
                    hyper::service::service_fn(async |request| {
                        api::root_handler(&s, request).await
                    }),
                )
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
