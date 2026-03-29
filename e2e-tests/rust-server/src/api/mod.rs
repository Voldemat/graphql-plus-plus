pub mod context;
pub mod generated;
pub mod scalar;
pub mod state;

#[derive(serde::Deserialize)]
struct GraphqlRequestBody {
    #[serde(rename = "operationName")]
    operation_name: Option<String>,
    query: String,
    variables: Option<serde_json::value::Value>,
}

#[derive(serde::Serialize)]
struct GraphqlError {
    message: String,
    path: Vec<String>,
}

#[derive(serde::Serialize)]
struct GraphqlResponseBody {
    data: serde_json::value::Value,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    errors: Vec<GraphqlError>,
}
type BoxBody<'state> = std::pin::Pin<
    Box<
        dyn hyper::body::Body<
                Data = hyper::body::Bytes,
                Error = std::convert::Infallible,
            > + Send
            + 'state,
    >,
>;
pub type Response<'state> = hyper::Response<BoxBody<'state>>;

pub struct Full(Option<hyper::body::Bytes>);

impl hyper::body::Body for Full {
    type Data = hyper::body::Bytes;
    type Error = std::convert::Infallible;

    fn poll_frame(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<
        Option<Result<hyper::body::Frame<hyper::body::Bytes>, Self::Error>>,
    > {
        std::task::Poll::Ready(
            self.0.take().map(|v| Ok(hyper::body::Frame::data(v))),
        )
    }

    fn is_end_stream(&self) -> bool {
        self.0.is_none()
    }

    fn size_hint(&self) -> hyper::body::SizeHint {
        self.0
            .as_ref()
            .map(|data| {
                hyper::body::SizeHint::with_exact(
                    u64::try_from(data.len()).unwrap(),
                )
            })
            .unwrap_or_else(|| hyper::body::SizeHint::with_exact(0))
    }
}

pub struct StreamBody<'state>(
    std::pin::Pin<
        Box<
            dyn futures_util::Stream<
                    Item = Result<
                        hyper::body::Frame<hyper::body::Bytes>,
                        std::convert::Infallible,
                    >,
                > + Send
                + 'state,
        >,
    >,
);

impl<'state> hyper::body::Body for StreamBody<'state> {
    type Data = hyper::body::Bytes;
    type Error = std::convert::Infallible;

    fn poll_frame(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<
        Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>,
    > {
        match self.0.as_mut().poll_next(cx) {
            std::task::Poll::Ready(Some(result)) => {
                std::task::Poll::Ready(Some(result))
            }
            std::task::Poll::Ready(None) => std::task::Poll::Ready(None),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}

pub async fn root_handler<'state>(
    state: &'state state::APIState,
    request: hyper::Request<hyper::body::Incoming>,
) -> Result<Response<'state>, std::convert::Infallible> {
    match (request.method(), request.uri().path()) {
        (&hyper::Method::POST, "/graphql") => {
            graphql_handler(state, request).await
        }
        _ => {
            let mut not_found: Response<'state> = hyper::Response::new(
                Box::pin(Full(Some(hyper::body::Bytes::new()))),
            );
            *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

async fn read_body(
    request: hyper::Request<hyper::body::Incoming>,
) -> Result<Vec<u8>, Response<'static>> {
    let upper =
        (request.body()
            as &dyn hyper::body::Body<
                Data = hyper::body::Bytes,
                Error = hyper::Error,
            >)
            .size_hint()
            .upper()
            .unwrap_or(u64::MAX);
    if upper > 1024 * 64 {
        return Err(build_response(
            "Body too big".to_string(),
            hyper::StatusCode::PAYLOAD_TOO_LARGE,
        ));
    }

    let whole_body = http_body_util::BodyExt::collect(request)
        .await
        .map_err(|e| {
            build_response(e.to_string(), hyper::StatusCode::BAD_REQUEST)
        })?
        .to_bytes();
    Ok(whole_body.to_vec())
}

async fn graphql_handler<'state>(
    state: &'state state::APIState,
    request: hyper::Request<hyper::body::Incoming>,
) -> Result<Response<'state>, std::convert::Infallible> {
    let body = match read_body(request).await {
        Ok(body) => body,
        Err(response) => {
            return Ok(response);
        }
    };
    let data = match serde_json::from_slice::<GraphqlRequestBody>(&body) {
        Ok(data) => data,
        Err(error) => {
            return Ok(build_response(
                error.to_string(),
                hyper::StatusCode::BAD_REQUEST,
            ));
        }
    };
    let operation_result = match libgql::executor::execute(
        &(),
        &state.graphql_registry,
        &state.graphql_resolvers_map,
        &state.graphql_parse_registry,
        data.query,
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
                let body = match serde_json::to_string(&GraphqlResponseBody {
                    data: serde_json::Value::Null,
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
                libgql::json::executor::ast::serialize_values_to_json(&result)
                    .unwrap();
            let body = match serde_json::to_string(&GraphqlResponseBody {
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
        libgql::executor::OperationResult::Stream(stream) => {
            let wrapped = wrap_stream(stream);

            let body: BoxBody<'state> = Box::pin(StreamBody::<'state>(wrapped));
            let response: Response<'state> = hyper::Response::builder()
                .status(hyper::StatusCode::OK)
                .header("Content-Type", "text/event-stream")
                .body(body)
                .unwrap();
            Ok(response)
        }
    }
}

fn wrap_stream<'a>(
    stream: libgql::executor::subscriptions::SubscriptionOperationStream<
        'a,
        scalar::Scalar,
    >,
) -> std::pin::Pin<
    Box<
        dyn futures_util::Stream<
                Item = Result<
                    hyper::body::Frame<hyper::body::Bytes>,
                    std::convert::Infallible,
                >,
            > + Send
            + 'a,
    >,
> {
    Box::pin(futures_util::StreamExt::map(
        stream,
        |result| -> Result<
            hyper::body::Frame<hyper::body::Bytes>,
            std::convert::Infallible,
        > {
            let r = result.unwrap();
            let json_result =
                libgql::json::executor::ast::serialize_values_to_json(&r)
                    .unwrap();
            let body = match serde_json::to_string(&GraphqlResponseBody {
                data: json_result,
                errors: Vec::new(),
            }) {
                Ok(body) => format!("data: {}\n", body),
                Err(error) => {
                    panic!("{}", error.to_string());
                }
            };
            Ok(hyper::body::Frame::data(hyper::body::Bytes::from(body)))
        },
    ))
}

fn build_response(body: String, status_code: hyper::StatusCode) -> Response<'static> {
    let mut response: Response = hyper::Response::new(
        Box::pin(Full(Some(hyper::body::Bytes::from(body)))),
    );
    *response.status_mut() = status_code;
    response
}
