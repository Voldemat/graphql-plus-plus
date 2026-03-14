use std::collections::HashMap;

use libgql::{
    executor::ast::TryGetStr, parsers::schema::type_registry::TypeRegistry,
};

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

fn parse_variable_from_json<S: libgql::executor::Scalar>(
    value: &serde_json::Value,
) -> Result<libgql::executor::Value<S>, String> {
    match value {
        serde_json::Value::Null => Ok(libgql::executor::Value::Null),
        serde_json::Value::String(s) => {
            Ok(libgql::executor::Value::NonNullable(
                libgql::executor::NonNullableValue::Literal(
                    libgql::executor::LiteralValue::Scalar(S::from_string(s)?),
                ),
            ))
        }
        serde_json::Value::Bool(b) => Ok(libgql::executor::Value::NonNullable(
            libgql::executor::NonNullableValue::Literal(
                libgql::executor::LiteralValue::Scalar(S::from_bool(*b)?),
            ),
        )),
        serde_json::Value::Number(n) => {
            if let Some(u64_n) = n.as_u64() {
                Ok(libgql::executor::Value::NonNullable(
                    libgql::executor::NonNullableValue::Literal(
                        libgql::executor::LiteralValue::Scalar(S::from_u64(
                            u64_n,
                        )?),
                    ),
                ))
            } else if let Some(i64_n) = n.as_i64() {
                Ok(libgql::executor::Value::NonNullable(
                    libgql::executor::NonNullableValue::Literal(
                        libgql::executor::LiteralValue::Scalar(S::from_i64(
                            i64_n,
                        )?),
                    ),
                ))
            } else if let Some(f64_n) = n.as_f64() {
                Ok(libgql::executor::Value::NonNullable(
                    libgql::executor::NonNullableValue::Literal(
                        libgql::executor::LiteralValue::Scalar(S::from_f64(
                            f64_n,
                        )?),
                    ),
                ))
            } else {
                unreachable!()
            }
        }
        serde_json::Value::Array(a) => {
            Ok(libgql::executor::Value::NonNullable(
                libgql::executor::NonNullableValue::Array(
                    a.iter()
                        .map(|element| parse_variable_from_json::<S>(element))
                        .collect::<Result<Vec<_>, String>>()?,
                ),
            ))
        }
        serde_json::Value::Object(o) => {
            let mut variables = libgql::executor::Values::<S>::new();
            for (key, value) in o {
                variables.insert(key.clone(), parse_variable_from_json(value)?);
            }
            Ok(libgql::executor::Value::NonNullable(
                libgql::executor::NonNullableValue::Literal(
                    libgql::executor::LiteralValue::Object(
                        "".to_string(),
                        variables,
                    ),
                ),
            ))
        }
    }
}

fn parse_variables_from_json<S: libgql::executor::Scalar>(
    value: &serde_json::Value,
) -> Result<libgql::executor::Values<S>, String> {
    match value {
        serde_json::Value::Null => Ok(libgql::executor::Values::new()),
        serde_json::Value::String(_) => {
            Err("Variables must be json object, received a string".into())
        }
        serde_json::Value::Number(_) => {
            Err("Variables must be json object, received a number".into())
        }
        serde_json::Value::Bool(_) => {
            Err("Variables must be json object, received a bool".into())
        }
        serde_json::Value::Array(_) => {
            Err("Variables must be json object, received an array".into())
        }
        serde_json::Value::Object(o) => {
            let mut variables = libgql::executor::Values::<S>::new();
            for (key, value) in o {
                variables.insert(key.clone(), parse_variable_from_json(value)?);
            }
            Ok(variables)
        }
    }
}

#[derive(Debug)]
enum ExampleScalar {
    String(String),
    Int(i32),
    Float(f32),
    Boolean(bool),
}

impl libgql::executor::Scalar for ExampleScalar {
    fn from_string(s: &str) -> Result<Self, String> {
        return Ok(Self::String(s.to_string()));
    }

    fn get_str(self: &Self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    fn from_u64(n: u64) -> Result<Self, String> {
        return Ok(Self::Int(
            TryInto::<i32>::try_into(n).map_err(|e| e.to_string())?,
        ));
    }

    fn from_i64(n: i64) -> Result<Self, String> {
        return Ok(Self::Int(
            TryInto::<i32>::try_into(n).map_err(|e| e.to_string())?,
        ));
    }

    fn from_f64(n: f64) -> Result<Self, String> {
        return Ok(Self::Float(n as f32));
    }

    fn from_bool(b: bool) -> Result<Self, String> {
        return Ok(Self::Boolean(b));
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

type Context = ();

fn login_resolver(
    root: &libgql::executor::ResolverRoot<ExampleScalar>,
    context: &mut Context,
    variables: &libgql::executor::ResolvedVariables,
) -> Result<libgql::executor::Value<ExampleScalar>, String> {
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
    Ok(libgql::executor::Value::NonNullable(
        libgql::executor::NonNullableValue::Literal(
            libgql::executor::LiteralValue::Object(
                "ErrorInvalidCredentials".to_string(),
                libgql::executor::Values::new(),
            ),
        ),
    ))
}

fn confirm_otp_code_resolver(
    root: &libgql::executor::ResolverRoot<ExampleScalar>,
    context: &mut Context,
    variables: &libgql::executor::ResolvedVariables,
) -> Result<libgql::executor::Value<ExampleScalar>, String> {
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
    Ok(libgql::executor::Value::NonNullable(
        libgql::executor::NonNullableValue::Literal(
            libgql::executor::LiteralValue::Object(
                "OTPToken".to_string(),
                libgql::executor::Values::from_iter([
                    ("token".to_string(), libgql::executor::Value::NonNullable(libgql::executor::NonNullableValue::Literal(libgql::executor::LiteralValue::Scalar(<ExampleScalar as libgql::executor::Scalar>::from_string("sample-token")?)))),
                ].into_iter()),
            ),
        ),
    ))
}

fn execute(args: &ParseArgs) {
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
    parse_registry.add_input::<UsersTagSortBy>("UsersTagSortBy");
    let mut resolvers = libgql::executor::ResolversMap::new();
    resolvers.insert(
        ("Mutation".to_string(), "login".to_string()),
        Box::new(login_resolver),
    );
    resolvers.insert(
        ("Mutation".to_string(), "confirmOTPCode".to_string()),
        Box::new(confirm_otp_code_resolver),
    );
    let result = libgql::executor::execute::<
        Context,
        ExampleScalar,
        libgql::executor::HashMapRegistry<ExampleScalar>,
    >(
        &mut (),
        &registry,
        &resolvers,
        &parse_registry,
        &utils::read_buffer_from_filepath(&args.query_path),
        &args
            .variables
            .as_ref()
            .map_or(libgql::executor::Values::new(), |v| {
                parse_variables_from_json(
                    &serde_json::from_str::<serde_json::Value>(&v).unwrap(),
                )
                .unwrap()
            }),
        &args.operation,
    )
    .unwrap();
    println!("result: {:?}", result);
}

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Execute(args) => execute(args),
        }
    }
}
