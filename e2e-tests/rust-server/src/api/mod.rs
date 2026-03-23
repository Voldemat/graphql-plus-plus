use std::sync::Arc;

pub mod context;
pub mod generated;
pub mod scalar;
pub mod state;

#[derive(serde::Deserialize)]
struct GraphqlRequestBody {
    #[serde(rename = "operationName")]
    operation_name: Option<String>,
    query: String,
    variables: Option<serde_json_path_to_error::value::Value>,
}

#[actix_web::post("/graphql")]
pub async fn graphql(
    state: actix_web::web::Data<Arc<state::APIState>>,
    json: actix_web::web::Json<GraphqlRequestBody>,
) -> impl actix_web::Responder {
    let data = json.0;
    let operation_result = libgql::executor::execute(
        &(),
        &state.get_ref().graphql_registry,
        &state.get_ref().graphql_resolvers_map,
        &state.get_ref().graphql_parse_registry,
        data.query,
        data.variables.map_or(libgql::executor::Values::new(), |v| {
            libgql::json::executor::ast::parse_variables_from_json(&v).unwrap()
        }),
        data.operation_name,
    )
    .await
    .unwrap();
    match operation_result {
        libgql::executor::OperationResult::Immediate(result) => {
            let json_result =
                libgql::json::executor::ast::serialize_values_to_json(&result).unwrap();
            actix_web::HttpResponse::Ok().json(json_result)
        }
        libgql::executor::OperationResult::Stream(_) => {
            actix_web::HttpResponse::BadRequest().body("Unexpected stream response")
        }
    }
}
