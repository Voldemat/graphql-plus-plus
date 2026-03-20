use std::sync::{Arc, atomic::AtomicBool};

use chrono::prelude::*;
use libgql::parsers::schema::type_registry::TypeRegistry;
use tokio_stream::StreamExt;

use crate::cli::utils;

#[derive(clap::Subcommand)]
pub enum Commands {
    Execute(ParseArgs),
}

#[derive(clap::Args)]
pub struct ParseArgs {
    #[arg(short, long, help = "filepath to server json schema")]
    server_schema_path: std::path::PathBuf,
    #[arg(short, long, help = "path to client query to execute")]
    query_path: std::path::PathBuf,
    #[arg(short, long, help = "variables for client query")]
    variables: Option<String>,
    #[arg(short, long, help = "operation name to execute in client query")]
    operation: Option<String>,
}

#[derive(Debug)]
enum ExampleScalar {
    String(String),
    Int(i32),
    Float(f32),
    Boolean(bool),
}

impl libgql::executor::GQLScalar<ExampleScalar> for DateTime<Utc> {
    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::String(self.to_rfc3339()))
    }

    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::String(string) => {
                DateTime::parse_from_rfc3339(&string)
                    .map_err(|e| e.to_string())
                    .map(|v| v.with_timezone(&Utc))
            }
            _ => Err("Unexpected scalar for DateTime".to_string()),
        }
    }
}

impl libgql::executor::ast::ResolverValue<ExampleScalar> for DateTime<Utc> {
    fn create_introspection_value<'a>(
        self: &'a Self,
    ) -> libgql::executor::ast::ResolverIntrospectionValue<'a, ExampleScalar>
    {
        Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Literal(self))
    }

    fn to_value(
        self: Box<Self>,
        _: Vec<(String, libgql::executor::Value<ExampleScalar>)>,
    ) -> Result<libgql::executor::Value<ExampleScalar>, String> {
        Ok(libgql::executor::Value::NonNullable(
            libgql::executor::NonNullableValue::Literal(
                libgql::executor::LiteralValue::Scalar(
                    libgql::executor::GQLScalar::<ExampleScalar>::to_scalar(
                        *self,
                    )?,
                ),
            ),
        ))
    }
}

impl libgql::executor::Scalar for ExampleScalar {
    fn try_to_string(self: Self) -> Result<String, String> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err("Invalid scalar type for string".to_string()),
        }
    }

    fn from_str(str: &str) -> Result<Self, String> {
        Ok(Self::String(str.to_string()))
    }

    fn from_literal(
        literal: &libgql::parsers::schema::shared::ast::Literal,
    ) -> Result<ExampleScalar, String> {
        match literal {
            libgql::parsers::schema::shared::ast::Literal::Int(i) => {
                Ok(ExampleScalar::Int(
                    TryInto::<i32>::try_into(*i).map_err(|e| e.to_string())?,
                ))
            }
            libgql::parsers::schema::shared::ast::Literal::Float(f) => {
                Ok(ExampleScalar::Float(*f as f32))
            }
            libgql::parsers::schema::shared::ast::Literal::String(s) => {
                Ok(ExampleScalar::String(s.to_string()))
            }
            libgql::parsers::schema::shared::ast::Literal::Boolean(b) => {
                Ok(ExampleScalar::Boolean(*b))
            }
        }
    }
}

enum Direction {
    Asc,
    Dsc,
}

impl libgql::executor::GQLEnum<ExampleScalar> for Direction {
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
            "ASC" => Ok(Self::Asc),
            "DSC" => Ok(Self::Dsc),
            _ => Err(format!("Invalid value {} for enum Direction", s)),
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
        match self {
            Self::Asc => Ok("ASC"),
            Self::Dsc => Ok("DSC"),
        }
    }
}

enum EUsersTagField {
    Name,
}

impl libgql::executor::GQLEnum<ExampleScalar> for EUsersTagField {
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
            "NAME" => Ok(Self::Name),
            _ => Err(format!("Invalid value {} for enum EUsersTagField", s)),
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
        match self {
            Self::Name => Ok("NAME"),
        }
    }
}

struct UsersTagSortBy {
    direction: Direction,
    field: EUsersTagField,
}

impl libgql::executor::GQLInput<ExampleScalar> for UsersTagSortBy {
    fn from_variables(
        mut vars: libgql::executor::Values<ExampleScalar>,
    ) -> Result<Self, String> {
        let direction =
                vars.remove("direction").map(|v| v.to_non_nullable_option())
                    .flatten()
                    .ok_or("Missing required field \"direction\"".to_string())
                    .map(<Direction as libgql::executor::GQLEnum<ExampleScalar>>::from_non_nullable_value
                    )
                    .flatten()?;
        let field =
            vars.remove("field")
                .map(|v| v.to_non_nullable_option())
                .flatten()
                .ok_or("Missing required field \"field\"".to_string())
                .map(
                    <EUsersTagField as libgql::executor::GQLEnum<
                        ExampleScalar,
                    >>::from_non_nullable_value,
                )
                .flatten()?;
        return Ok(UsersTagSortBy { direction, field });
    }
}

#[derive(Debug)]
struct UUID(String);

impl libgql::executor::GQLScalar<ExampleScalar> for UUID {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::String(s) => Ok(Self(s)),
            _ => Err(format!("Invalid scalar for UUID {:?}", s)),
        }
    }

    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::String(self.0))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for i32 {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Int(i) => Ok(i),
            _ => Err(format!("Invalid scalar for i32 {:?}", s)),
        }
    }

    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::Int(self))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for f32 {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Float(f) => Ok(f),
            _ => Err(format!("Invalid scalar for f32 {:?}", s)),
        }
    }

    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::Float(self))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for String {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::String(s) => Ok(s),
            _ => Err(format!("Invalid scalar for String {:?}", s)),
        }
    }
    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::String(self))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for bool {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Boolean(b) => Ok(b),
            _ => Err(format!("Invalid scalar for bool {:?}", s)),
        }
    }
    fn to_scalar(self: Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::Boolean(self))
    }
}

#[derive(Debug)]
enum EGroupUsersField {
    Name,
    Email,
}

impl libgql::executor::GQLEnum<ExampleScalar> for EGroupUsersField {
    fn from_string(s: String) -> Result<Self, String> {
        match s.as_str() {
            "NAME" => Ok(Self::Name),
            "EMAIL" => Ok(Self::Email),
            _ => Err(format!("EGroupUsersField: Unknown enum value {}", s)),
        }
    }

    fn to_str(self: Self) -> Result<&'static str, String> {
        match self {
            Self::Name => Ok("NAME"),
            Self::Email => Ok("EMAIL"),
        }
    }
}

type Context = ();

struct ErrorInvalidCredentials {
    a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<ExampleScalar>
    for ErrorInvalidCredentials
{
    fn to_value(
        self: Box<Self>,
        _: Vec<(String, libgql::executor::Value<ExampleScalar>)>,
    ) -> Result<libgql::executor::Value<ExampleScalar>, String> {
        Ok(libgql::executor::Value::NonNullable(
            libgql::executor::NonNullableValue::Literal(
                libgql::executor::LiteralValue::Object(
                    "ErrorInvalidCredentials".to_string(),
                    libgql::executor::Values::new(),
                ),
            ),
        ))
    }

    fn create_introspection_value<'a>(
        self: &'a Self,
    ) -> libgql::executor::ast::ResolverIntrospectionValue<'a, ExampleScalar>
    {
        Some(
        libgql::executor::ast::NonNullableResolverIntrospectionValue::Literal(
            self
        )
        )
    }
}

fn login_resolver(
    root_any: &libgql::executor::ast::ResolverRoot<ExampleScalar>,
    _: &Context,
    variables: &libgql::executor::ResolvedVariables,
) -> libgql::executor::sync::ResolverFuture<ExampleScalar> {
    let r_any: &dyn std::any::Any = root_any.as_ref();
    let root = r_any.downcast_ref::<&()>().unwrap();
    println!(
        "login_resolver: {:?}, email: {}, password: {}",
        root,
        variables
            .get("email")
            .unwrap()
            .downcast_ref::<String>()
            .unwrap(),
        variables
            .get("password")
            .unwrap()
            .downcast_ref::<String>()
            .unwrap()
    );
    Box::pin(async move {
        Ok(Box::new(ErrorInvalidCredentials { a: None })
            as libgql::executor::ast::ResolverRoot<ExampleScalar>)
    })
}

impl libgql::json::executor::ast::JSONParsableScalar for ExampleScalar {
    fn from_json_scalar<'a>(
        json_scalar: libgql::json::executor::ast::JSONScalar<'a>,
    ) -> Result<ExampleScalar, String> {
        match json_scalar {
            libgql::json::executor::ast::JSONScalar::Bool(b) => {
                Ok(ExampleScalar::Boolean(b))
            }
            libgql::json::executor::ast::JSONScalar::String(s) => {
                Ok(ExampleScalar::String(s.to_string()))
            }
            libgql::json::executor::ast::JSONScalar::Number(n) => {
                if let Some(u64_n) = n.as_u64() {
                    Ok(ExampleScalar::Int(
                        TryInto::<i32>::try_into(u64_n)
                            .map_err(|e| e.to_string())?,
                    ))
                } else if let Some(i64_n) = n.as_i64() {
                    Ok(ExampleScalar::Int(
                        TryInto::<i32>::try_into(i64_n)
                            .map_err(|e| e.to_string())?,
                    ))
                } else if let Some(f64_n) = n.as_f64() {
                    Ok(ExampleScalar::Float(f64_n as f32))
                } else {
                    unreachable!()
                }
            }
        }
    }
}

impl libgql::json::executor::ast::JSONSerializableScalar for ExampleScalar {
    fn to_json_value(self: &Self) -> Result<serde_json::Value, String> {
        match self {
            Self::Int(i) => Ok(serde_json::Value::Number(
                serde_json::Number::from_i128(*i as i128).ok_or(
                    "Failed to convert ExampleScalar::Int to serde_json::Number",
                )?,
            )),
            Self::Float(f) => Ok(serde_json::Value::Number(
                serde_json::Number::from_f64(*f as f64).ok_or(
                    "Failed to convert ExampleScalar::Float to serde_json::Number",
                )?,
            )),
            Self::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
            Self::String(s) => Ok(serde_json::Value::String(s.clone())),
        }
    }
}

fn get_events_subscription(
    root: &libgql::executor::ast::ResolverRoot<ExampleScalar>,
    context: &Context,
    variables: &libgql::executor::ResolvedVariables,
) -> libgql::executor::subscription::SubscriptionResolverFuture<ExampleScalar> {
    let resolver_stream = async_stream::stream! {
        loop {
            yield Utc::now();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    };
    let stream = Box::pin(resolver_stream.map(
        |v| -> libgql::executor::ast::ResolverRoot<ExampleScalar> {
            Box::new(v)
        },
    ))
        as libgql::executor::subscription::SubscriptionResolverStream<
            ExampleScalar,
        >;
    Box::pin(async move { Ok(stream) })
}

async fn execute(args: ParseArgs) {
    let term = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(
        signal_hook::consts::SIGTERM,
        Arc::clone(&term),
    )
    .unwrap();
    let buffer = utils::read_buffer_from_filepath(&args.server_schema_path);
    let mut registry = TypeRegistry::new();
    libgql::json::parsers::schema::parse_server_schema(
        &mut registry,
        serde_json::from_str::<serde_json::Value>(&buffer).unwrap(),
    )
    .unwrap();
    let mut parse_registry =
        libgql::executor::HashMapRegistry::<ExampleScalar>::default();
    parse_registry.add_scalar::<String>("String");
    parse_registry.add_scalar::<bool>("Boolean");
    parse_registry.add_scalar::<i32>("Int");
    parse_registry.add_scalar::<f32>("Float");
    parse_registry.add_scalar::<UUID>("UUID");
    parse_registry.add_enum::<EGroupUsersField>("EGroupUsersField");
    parse_registry.add_input::<UsersTagSortBy>("UsersTagSortBy");
    let mut sync_resolvers = libgql::executor::sync::SyncResolversMap::new();
    sync_resolvers.insert(
        ("Mutation".to_string(), "login".to_string()),
        Box::new(login_resolver),
    );
    let mut subscription_resolvers =
        libgql::executor::subscription::SubscriptionResolversMap::new();
    subscription_resolvers
        .insert("getEvents".to_string(), Box::new(get_events_subscription));
    let operation_result = libgql::executor::execute(
        &(),
        &registry,
        &sync_resolvers,
        &subscription_resolvers,
        &parse_registry,
        &utils::read_buffer_from_filepath(&args.query_path),
        args.variables
            .as_ref()
            .map_or(libgql::executor::Values::new(), |v| {
                libgql::json::executor::ast::parse_variables_from_json(
                    &serde_json::from_str::<serde_json::Value>(&v).unwrap(),
                )
                .unwrap()
            }),
        args.operation,
    )
    .await
    .unwrap();
    match operation_result {
        libgql::executor::OperationResult::Immediate(result) => {
            let json_result =
                libgql::json::executor::ast::serialize_values_to_json(&result)
                    .unwrap();
            println!("result: {}", json_result);
        }
        libgql::executor::OperationResult::Stream(mut stream) => loop {
            if term.load(std::sync::atomic::Ordering::Relaxed) {
                println!("term signal received in main loop");
                break;
            }
            let Ok(next_item) = tokio::time::timeout(
                std::time::Duration::from_millis(100),
                stream.next(),
            )
            .await
            else {
                continue;
            };
            let Some(item) = next_item else {
                break;
            };
            let Ok(v) = item else {
                panic!("Unexpected error in subscription stream");
            };
            let json_result =
                libgql::json::executor::ast::serialize_values_to_json(&v)
                    .unwrap();
            println!("result: {}", json_result);
        },
    }
}

impl Commands {
    pub fn execute(self: Self) {
        match self {
            Commands::Execute(args) => tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(execute(args)),
        }
    }
}
