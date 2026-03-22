use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, atomic::AtomicBool},
};

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
    fn to_scalar(self: &Self) -> Result<ExampleScalar, String> {
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
    fn to_value<'a>(
        self: &'a Self,
    ) -> Result<
        libgql::executor::ast::ResolverIntrospectionValue<'a, ExampleScalar>,
        String,
    > {
        Ok(Some(
            libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(
                    libgql::executor::GQLScalar::<ExampleScalar>::to_scalar(
                        self
                    )?
        )))
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

    fn to_str(self: &Self) -> Result<&'static str, String> {
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

    fn to_str(self: &Self) -> Result<&'static str, String> {
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

    fn to_scalar(self: &Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::String(self.0.clone()))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for i32 {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Int(i) => Ok(i),
            _ => Err(format!("Invalid scalar for i32 {:?}", s)),
        }
    }

    fn to_scalar(self: &Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::Int(*self))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for f32 {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Float(f) => Ok(f),
            _ => Err(format!("Invalid scalar for f32 {:?}", s)),
        }
    }

    fn to_scalar(self: &Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::Float(*self))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for String {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::String(s) => Ok(s),
            _ => Err(format!("Invalid scalar for String {:?}", s)),
        }
    }
    fn to_scalar(self: &Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::String(self.clone()))
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for bool {
    fn from_scalar(s: ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Boolean(b) => Ok(b),
            _ => Err(format!("Invalid scalar for bool {:?}", s)),
        }
    }
    fn to_scalar(self: &Self) -> Result<ExampleScalar, String> {
        Ok(ExampleScalar::Boolean(*self))
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

    fn to_str(self: &Self) -> Result<&'static str, String> {
        match self {
            Self::Name => Ok("NAME"),
            Self::Email => Ok("EMAIL"),
        }
    }
}

impl libgql::executor::ast::ResolverValue<ExampleScalar> for bool {
    fn to_value<'a>(
        self: &'a Self,
    ) -> Result<
        libgql::executor::ast::ResolverIntrospectionValue<'a, ExampleScalar>,
        String,
    > {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Scalar(libgql::executor::GQLScalar::<ExampleScalar>::to_scalar(self)?)))
    }
}

type Context = ();

struct ErrorInvalidCredentials {
    a: Option<bool>,
}

impl libgql::executor::ast::ResolverValue<ExampleScalar>
    for ErrorInvalidCredentials
{
    fn to_value<'a>(
        self: &'a Self,
    ) -> Result<
        libgql::executor::ast::ResolverIntrospectionValue<'a, ExampleScalar>,
        String,
    > {
        Ok(Some(libgql::executor::ast::NonNullableResolverIntrospectionValue::Object(
                    self,
                    "ErrorInvalidCredentials",
                    HashMap::from_iter([
                        ("a", &self.a as &libgql::executor::ast::ResolverRoot<ExampleScalar>)
                    ])
        )))
    }
}

fn login_resolver<'a>(
    _: &'a Context,
    variables: &'a libgql::executor::ResolvedVariables,
) -> libgql::executor::ast::ResolverFuture<'a, ExampleScalar> {
    println!(
        "login_resolver: email: {}, password: {}",
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
            as Box<libgql::executor::ast::ResolverRoot<ExampleScalar>>)
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

fn get_events_subscription<'a>(
    context: &'a Context,
    variables: &'a libgql::executor::ResolvedVariables,
) -> libgql::executor::subscriptions::SubscriptionResolverFuture<
    'a,
    ExampleScalar,
> {
    let resolver_stream = async_stream::stream! {
        loop {
            yield Utc::now();
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    };
    let stream = Box::pin(resolver_stream.map(
        |v| -> Box<libgql::executor::ast::ResolverRoot<ExampleScalar>> {
            Box::new(v)
        },
    ))
        as libgql::executor::subscriptions::SubscriptionResolverStream<
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
    let mut mutation_resolvers =
        libgql::executor::mutations::MutationResolversMap::new();
    mutation_resolvers.insert("login", Box::new(login_resolver));
    let mut subscription_resolvers =
        libgql::executor::subscriptions::SubscriptionResolversMap::new();
    subscription_resolvers
        .insert("getEvents", Box::new(get_events_subscription));
    let resolvers = libgql::executor::Resolvers {
        queries: libgql::executor::queries::QueryResolversMap::new(),
        mutations: mutation_resolvers,
        subscriptions: subscription_resolvers,
    };
    let operation_result = libgql::executor::execute(
        &(),
        &registry,
        &resolvers,
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
