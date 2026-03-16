use std::sync::{Arc, RwLock, atomic::AtomicBool};

use chrono::prelude::*;
use libgql::{
    executor::ast::TryGetStr, parsers::schema::type_registry::TypeRegistry,
};
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

impl libgql::executor::Scalar for ExampleScalar {
    fn get_str(self: &Self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    fn from_string(str: &str) -> Result<Self, String> {
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

impl libgql::executor::GQLEnum for Direction {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "ASC" => Ok(Self::Asc),
            "DSC" => Ok(Self::Dsc),
            _ => Err(format!("Invalid value {} for enum Direction", s)),
        }
    }
}

enum EUsersTagField {
    Name,
}

impl libgql::executor::GQLEnum for EUsersTagField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "NAME" => Ok(Self::Name),
            _ => Err(format!("Invalid value {} for enum EUsersTagField", s)),
        }
    }
}

struct UsersTagSortBy {
    direction: Direction,
    field: EUsersTagField,
}

impl libgql::executor::GQLInput<ExampleScalar> for UsersTagSortBy {
    fn from_variables(
        vars: &libgql::executor::Values<ExampleScalar>,
    ) -> Result<Self, String> {
        let direction = <Direction as libgql::executor::GQLEnum>::from_str(
            vars.get("direction")
                .ok_or("Missing required field \"direction\"")?
                .try_get_str()
                .ok_or("Invalid scalar for enum EUsersTagField")?,
        )?;
        let field = <EUsersTagField as libgql::executor::GQLEnum>::from_str(
            vars.get("field")
                .ok_or("Missing required field \"field\"")?
                .try_get_str()
                .ok_or("Invalid scalar for enum EUsersTagField")?,
        )?;
        return Ok(UsersTagSortBy { direction, field });
    }
}

#[derive(Debug)]
struct UUID(String);

impl libgql::executor::GQLScalar<ExampleScalar> for UUID {
    fn from_scalar(s: &ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::String(s) => Ok(Self(s.clone())),
            _ => Err(format!("Invalid scalar for UUID {:?}", s)),
        }
    }
}

#[derive(Debug)]
struct GroupIn {
    name: String,
    tag_ids: Vec<UUID>,
    limit_of_downloads_per_day: i32,
}

impl libgql::executor::GQLInput<ExampleScalar> for GroupIn {
    fn from_variables(
        vars: &libgql::executor::Values<ExampleScalar>,
    ) -> Result<Self, String> {
        let name = vars
            .get("name")
            .ok_or("Missing required field \"name\"")?
            .try_get_str()
            .ok_or("Invalid value for String")?
            .to_string();
        let tag_ids = match vars.get("tagIds")
                    .ok_or("Missing required field \"tagIds\"")? {
            libgql::executor::Value::Null => {
                Err("Unexpected null for tagIds".to_string())
            },
            libgql::executor::Value::NonNullable(non_nullable) => match non_nullable {
                libgql::executor::NonNullableValue::Literal(_) =>
                    Err("Unexpected literal for array field tagIds".to_string()),
                libgql::executor::NonNullableValue::Array(array) =>
                    array.iter().map(|item| {
                        <UUID as libgql::executor::GQLScalar<ExampleScalar>>::from_scalar(
                        <libgql::executor::Value<ExampleScalar> as libgql::executor::ast::TryGetScalar<ExampleScalar>>::try_get_scalar(item)
                        .ok_or("Invalid value for UUID")?
                    )
                }).collect::<Result<Vec<UUID>, String>>()
            }
        }?;
        return Ok(GroupIn {
            name,
            tag_ids,
            limit_of_downloads_per_day: 0,
        });
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for i32 {
    fn from_scalar(s: &ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Int(i) => Ok(*i),
            _ => Err(format!("Invalid scalar for i32 {:?}", s)),
        }
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for f32 {
    fn from_scalar(s: &ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Float(f) => Ok(*f),
            _ => Err(format!("Invalid scalar for f32 {:?}", s)),
        }
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for String {
    fn from_scalar(s: &ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::String(s) => Ok(s.clone()),
            _ => Err(format!("Invalid scalar for String {:?}", s)),
        }
    }
}

impl libgql::executor::GQLScalar<ExampleScalar> for bool {
    fn from_scalar(s: &ExampleScalar) -> Result<Self, String> {
        match s {
            ExampleScalar::Boolean(b) => Ok(*b),
            _ => Err(format!("Invalid scalar for bool {:?}", s)),
        }
    }
}

#[derive(Debug)]
enum EGroupUsersField {
    Name,
    Email,
}

impl libgql::executor::GQLEnum for EGroupUsersField {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "NAME" => Ok(Self::Name),
            "EMAIL" => Ok(Self::Email),
            _ => Err(format!("EGroupUsersField: Unknown enum value {}", s)),
        }
    }
}

type Context = ();

fn login_resolver(
    root: &libgql::executor::ResolverRoot<ExampleScalar>,
    context: &mut Context,
    variables: &libgql::executor::ResolvedVariables,
) -> std::pin::Pin<
    Box<
        dyn Future<
            Output = Result<libgql::executor::Value<ExampleScalar>, String>,
        >,
    >,
> {
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
        Ok(libgql::executor::Value::NonNullable(
            libgql::executor::NonNullableValue::Literal(
                libgql::executor::LiteralValue::Object(
                    "ErrorInvalidCredentials".to_string(),
                    libgql::executor::Values::new(),
                ),
            ),
        ))
    })
}

fn confirm_otp_code_resolver(
    root: &libgql::executor::ResolverRoot<ExampleScalar>,
    context: &mut Context,
    variables: &libgql::executor::ResolvedVariables,
) -> std::pin::Pin<
    Box<
        dyn Future<
            Output = Result<libgql::executor::Value<ExampleScalar>, String>,
        >,
    >,
> {
    println!(
        "confirm_otp_code_resolver: {:?}, email: {}, code: {}",
        root,
        variables
            .get("email")
            .unwrap()
            .downcast_ref::<String>()
            .unwrap(),
        variables
            .get("code")
            .unwrap()
            .downcast_ref::<String>()
            .unwrap()
    );
    Box::pin(async move {
        Ok(libgql::executor::Value::NonNullable(
            libgql::executor::NonNullableValue::Literal(
                libgql::executor::LiteralValue::Object(
                    "OTPToken".to_string(),
                    libgql::executor::Values::from_iter(
                        [(
                            "token".to_string(),
                            libgql::executor::Value::NonNullable(
                                libgql::executor::NonNullableValue::Literal(
                                    libgql::executor::LiteralValue::Scalar(
                                        ExampleScalar::String(
                                            "sample-token".into(),
                                        ),
                                    ),
                                ),
                            ),
                        )]
                        .into_iter(),
                    ),
                ),
            ),
        ))
    })
}

fn create_group_resolver(
    root: &libgql::executor::ResolverRoot<ExampleScalar>,
    context: &mut Context,
    variables: &libgql::executor::ResolvedVariables,
) -> std::pin::Pin<
    Box<
        dyn Future<
            Output = Result<libgql::executor::Value<ExampleScalar>, String>,
        >,
    >,
> {
    println!(
        "create_group_resolver: {:?}, groupIn: {:?}, userIds: {:?}, field: {:?}",
        root,
        variables
            .get("groupIn")
            .unwrap()
            .downcast_ref::<GroupIn>()
            .unwrap(),
        variables
            .get("userIds")
            .unwrap()
            .downcast_ref::<Vec<Box<dyn std::any::Any>>>()
            .unwrap()
            .iter()
            .map(|v| v.downcast_ref::<Vec<UUID>>().unwrap())
            .collect::<Vec<_>>(),
        variables
            .get("field")
            .unwrap()
            .downcast_ref::<EGroupUsersField>()
            .unwrap()
    );
    Box::pin(async move {
        Ok(libgql::executor::Value::NonNullable(
            libgql::executor::NonNullableValue::Literal(
                libgql::executor::LiteralValue::Object(
                    "ErrorAlreadyExists".to_string(),
                    libgql::executor::Values::new(),
                ),
            ),
        ))
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
    root: &libgql::executor::ResolverRoot<ExampleScalar>,
    context: &mut Context,
    variables: &libgql::executor::ResolvedVariables,
) -> std::pin::Pin<
    Box<
        dyn Future<
            Output = Result<
                std::pin::Pin<
                    Box<
                        dyn futures_core::Stream<
                                Item = libgql::executor::Value<ExampleScalar>,
                            >,
                    >,
                >,
                String,
            >,
        >,
    >,
> {
    let resolver_stream = async_stream::stream! {
        loop {
            yield ExampleScalar::String(Utc::now().to_string());
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    };
    let stream = Box::pin(resolver_stream.map(
        |v| -> libgql::executor::Value<ExampleScalar> {
            libgql::executor::Value::NonNullable(
                libgql::executor::NonNullableValue::Literal(
                    libgql::executor::LiteralValue::Scalar(v),
                ),
            )
        },
    ))
        as std::pin::Pin<
            Box<
                dyn futures_core::Stream<
                        Item = libgql::executor::Value<ExampleScalar>,
                    >,
            >,
        >;
    Box::pin(async move { Ok(stream) })
}

async fn execute(args: &ParseArgs) {
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
    parse_registry.add_input::<GroupIn>("GroupIn");
    let mut sync_resolvers = libgql::executor::sync::SyncResolversMap::new();
    sync_resolvers.insert(
        ("Mutation".to_string(), "login".to_string()),
        Box::new(login_resolver),
    );
    sync_resolvers.insert(
        ("Mutation".to_string(), "confirmOTPCode".to_string()),
        Box::new(confirm_otp_code_resolver),
    );
    sync_resolvers.insert(
        ("Mutation".to_string(), "createGroup".to_string()),
        Box::new(create_group_resolver),
    );
    let mut subscription_resolvers =
        libgql::executor::subscription::SubscriptionResolversMap::new();
    subscription_resolvers
        .insert("getEvents".to_string(), Box::new(get_events_subscription));
    let operation_result = libgql::executor::execute::<
        Context,
        ExampleScalar,
        libgql::executor::HashMapRegistry<ExampleScalar>,
    >(
        &mut (),
        &registry,
        &sync_resolvers,
        &subscription_resolvers,
        &parse_registry,
        &utils::read_buffer_from_filepath(&args.query_path),
        &args
            .variables
            .as_ref()
            .map_or(libgql::executor::Values::new(), |v| {
                libgql::json::executor::ast::parse_variables_from_json(
                    &serde_json::from_str::<serde_json::Value>(&v).unwrap(),
                )
                .unwrap()
            }),
        &args.operation,
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
            let json_result =
                libgql::json::executor::ast::serialize_values_to_json(&item)
                    .unwrap();
            println!("result: {}", json_result);
        },
    }
}

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Execute(args) => tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(execute(args)),
        }
    }
}
