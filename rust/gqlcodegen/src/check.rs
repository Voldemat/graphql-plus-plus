use std::{pin::Pin, time::Duration};

use actix_cors::Cors;
use actix_web::{
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
    http::header,
    middleware,
    web::{self, Data},
};

use super::scalar::MyScalarValue;
use juniper::{
    graphql_object, EmptyMutation, EmptySubscription, FieldError, GraphQLObject, RootNode
};
use juniper_actix::{graphiql_handler, graphql_handler};

struct QueryRoot;

#[graphql_object]
#[graphql(scalar = MyScalarValue)]
impl QueryRoot {
    fn add(a: i64, b: i64) -> i64 {
        a + b
    }
}

type Schema = RootNode<
    QueryRoot,
    EmptyMutation<()>,
    EmptySubscription<()>,
    MyScalarValue,
>;

fn schema() -> Schema {
    let query_info = ();
    let mutation_info = ();
    let subscription_info = ();
    Schema {
        query_type: QueryRoot,
        mutation_type: EmptyMutation::<()>::new(),
        subscription_type: EmptySubscription::<()>::new(),
        schema: juniper::SchemaType::<MyScalarValue>::new::<
            QueryRoot,
            EmptyMutation<()>,
            EmptySubscription<()>,
        >(&query_info, &mutation_info, &subscription_info),
        query_info,
        mutation_info,
        subscription_info,
        introspection_disabled: true
    }
}

async fn graphql(
    req: HttpRequest,
    payload: web::Payload,
    schema: Data<Schema>,
) -> Result<HttpResponse, Error> {
    graphql_handler(&schema, &(), req, payload).await
}

#[actix_web::main]
pub async fn run_graphql() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().app_data(Data::new(schema())).service(
            web::resource("/graphql")
                .route(web::post().to(graphql))
                .route(web::get().to(graphql)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
