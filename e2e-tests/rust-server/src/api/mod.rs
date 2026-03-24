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

#[derive(serde::Serialize)]
struct GraphqlError {
    message: String,
    path: Vec<String>,
}

#[derive(serde::Serialize)]
struct GraphqlResponseBody {
    data: serde_json_path_to_error::value::Value,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<GraphqlError>,
}

pub async fn root_handler(
    state: &state::APIState,
    request: hyper::Request<hyper::body::Incoming>,
) -> Result<hyper::Response<String>, std::convert::Infallible> {
    match (request.method(), request.uri().path()) {
        (&hyper::Method::POST, "/graphql") => graphql_handler(state, request).await,
        _ => {
            let mut not_found = hyper::Response::new("".to_string());
            *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

async fn read_body(
    request: hyper::Request<hyper::body::Incoming>,
) -> Result<Vec<u8>, hyper::Response<String>> {
    let upper = (request.body()
        as &dyn hyper::body::Body<Data = hyper::body::Bytes, Error = hyper::Error>)
        .size_hint()
        .upper()
        .unwrap_or(u64::MAX);
    if upper > 1024 * 64 {
        let mut resp = hyper::Response::new("Body too big".to_string());
        *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
        return Err(resp);
    }

    let whole_body = http_body_util::BodyExt::collect(request)
        .await
        .map_err(|e| {
            let mut response = hyper::Response::new(e.to_string());
            *response.status_mut() = hyper::StatusCode::BAD_REQUEST;
            response
        })?
        .to_bytes();
    Ok(whole_body.to_vec())
}

async fn graphql_handler(
    state: &state::APIState,
    request: hyper::Request<hyper::body::Incoming>,
) -> Result<hyper::Response<String>, std::convert::Infallible> {
    let body = match read_body(request).await {
        Ok(body) => body,
        Err(response) => {
            return Ok(response);
        }
    };
    let data = match serde_json_path_to_error::from_slice::<GraphqlRequestBody>(&body) {
        Ok(data) => data,
        Err(error) => {
            let mut response = hyper::Response::new(error.to_string());
            *response.status_mut() = hyper::StatusCode::BAD_REQUEST;
            return Ok(response);
        }
    };
    let query = data.query;
    let operation_result = match libgql::executor::execute(
        &(),
        &state.graphql_registry,
        &state.graphql_resolvers_map,
        &state.graphql_parse_registry,
        &query,
        data.variables.map_or(libgql::executor::Values::new(), |v| {
            libgql::json::executor::ast::parse_variables_from_json(&v).unwrap()
        }),
        data.operation_name,
    )
    .await
    {
        Ok(result) => result,
        Err(error) => match error {
            libgql::executor::Error::ExecutionErrors(errors) => {
                let body = match serde_json_path_to_error::to_string(&GraphqlResponseBody {
                    data: serde_json_path_to_error::Value::Null,
                    errors: errors
                        .into_iter()
                        .map(|gql_error| GraphqlError {
                            message: gql_error.message.to_string(),
                            path: gql_error.path,
                        })
                        .collect(),
                }) {
                    Ok(body) => body,
                    Err(error) => {
                        return Ok(build_response(
                            error.to_string(),
                            hyper::StatusCode::BAD_REQUEST,
                        ));
                    }
                };
                return Ok(build_response(body, hyper::StatusCode::OK));
            }
            _ => {
                return Ok(build_response(
                    format!("{:?}", error),
                    hyper::StatusCode::BAD_REQUEST,
                ));
            }
        },
    };
    match operation_result {
        libgql::executor::OperationResult::Immediate(result) => {
            let json_result =
                libgql::json::executor::ast::serialize_values_to_json(&result).unwrap();
            let body = match serde_json_path_to_error::to_string(&GraphqlResponseBody {
                data: json_result,
                errors: Vec::new(),
            }) {
                Ok(body) => body,
                Err(error) => {
                    return Ok(build_response(
                        error.to_string(),
                        hyper::StatusCode::BAD_REQUEST,
                    ));
                }
            };
            Ok(build_response(body, hyper::StatusCode::OK))
        }
        libgql::executor::OperationResult::Stream(_) => Ok(build_response(
            "Unexpected stream response".to_string(),
            hyper::StatusCode::BAD_REQUEST,
        )),
    }
}

fn build_response(body: String, status_code: hyper::StatusCode) -> hyper::Response<String> {
    let mut response = hyper::Response::new(body);
    *response.status_mut() = status_code;
    response
}
