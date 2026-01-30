use std::collections::HashMap;

use libgql::parsers::schema::type_registry::TypeRegistry;

use crate::cli::utils;

#[derive(clap::Subcommand)]
pub enum Commands {
    Execute(ParseArgs),
}

#[derive(clap::Args)]
pub struct ParseArgs {
    #[arg(
        short,
        long,
        help = "filepath to server json schema, \"-\" for stdin",
        default_value = "-"
    )]
    server_schema: std::path::PathBuf,
    #[arg(short, long, help = "path to client query to execute")]
    query_path: std::path::PathBuf,
    #[arg(short, long, help = "variables for client query")]
    variables: Option<String>,
    #[arg(short, long, help = "operation name to execute in client query")]
    operation: Option<String>,
}

fn parse_variable_from_json<S: libgql::executor::Scalar>(
    value: &serde_json::Value,
) -> Result<libgql::executor::Variable<S>, String> {
    match value {
        serde_json::Value::Null => Ok(libgql::executor::Variable::Null),
        serde_json::Value::String(s) => {
            Ok(libgql::executor::Variable::NonNullable(
                libgql::executor::NonNullableVariable::Literal(
                    libgql::executor::LiteralVariable::Scalar(S::from_string(
                        s,
                    )?),
                ),
            ))
        }
        serde_json::Value::Bool(b) => {
            Ok(libgql::executor::Variable::NonNullable(
                libgql::executor::NonNullableVariable::Literal(
                    libgql::executor::LiteralVariable::Scalar(S::from_bool(
                        *b,
                    )?),
                ),
            ))
        }
        serde_json::Value::Number(n) => {
            if let Some(u64_n) = n.as_u64() {
                Ok(libgql::executor::Variable::NonNullable(
                    libgql::executor::NonNullableVariable::Literal(
                        libgql::executor::LiteralVariable::Scalar(S::from_u64(
                            u64_n,
                        )?),
                    ),
                ))
            } else if let Some(i64_n) = n.as_i64() {
                Ok(libgql::executor::Variable::NonNullable(
                    libgql::executor::NonNullableVariable::Literal(
                        libgql::executor::LiteralVariable::Scalar(S::from_i64(
                            i64_n,
                        )?),
                    ),
                ))
            } else if let Some(f64_n) = n.as_f64() {
                Ok(libgql::executor::Variable::NonNullable(
                    libgql::executor::NonNullableVariable::Literal(
                        libgql::executor::LiteralVariable::Scalar(S::from_f64(
                            f64_n,
                        )?),
                    ),
                ))
            } else {
                unreachable!()
            }
        }
        serde_json::Value::Array(a) => {
            Ok(libgql::executor::Variable::NonNullable(
                libgql::executor::NonNullableVariable::Array(
                    a.iter()
                        .map(|element| parse_variable_from_json::<S>(element))
                        .collect::<Result<Vec<_>, String>>()?,
                ),
            ))
        }
        serde_json::Value::Object(o) => {
            let mut variables = libgql::executor::Variables::<S>::new();
            for (key, value) in o {
                variables.insert(key.clone(), parse_variable_from_json(value)?);
            }
            Ok(libgql::executor::Variable::NonNullable(
                libgql::executor::NonNullableVariable::Literal(
                    libgql::executor::LiteralVariable::Object(variables),
                ),
            ))
        }
    }
}

fn parse_variables_from_json<S: libgql::executor::Scalar>(
    value: &serde_json::Value,
) -> Result<libgql::executor::Variables<S>, String> {
    match value {
        serde_json::Value::Null => Ok(libgql::executor::Variables::new()),
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
            let mut variables = libgql::executor::Variables::<S>::new();
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

    fn get_enum_value(self: &Self) -> Option<&str> {
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

#[derive(Default)]
struct ParseRegistry {
    scalars: HashMap<
        String,
        Box<dyn Fn(&ExampleScalar) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    enum_types: HashMap<
        String,
        Box<dyn Fn(&str) -> Result<Box<dyn std::any::Any>, String>>,
    >,
    inputs: HashMap<
        String,
        Box<
            dyn Fn(
                &libgql::executor::Variables<ExampleScalar>,
            ) -> Result<Box<dyn std::any::Any>, String>,
        >,
    >,
}

impl libgql::executor::Registry<ExampleScalar> for ParseRegistry {
    fn parse_scalar(
        self: &Self,
        scalar_name: &str,
        value: &ExampleScalar,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.scalars.get(scalar_name).unwrap()(value)
    }

    fn parse_enum(
        self: &Self,
        enum_type: &libgql::parsers::schema::shared::ast::Enum,
        value: &str,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.enum_types.get(&enum_type.name).unwrap()(value)
    }

    fn parse_input(
        self: &Self,
        input_type: &libgql::parsers::schema::shared::ast::InputType,
        value: &libgql::executor::Variables<ExampleScalar>,
    ) -> Result<Box<dyn std::any::Any>, String> {
        self.inputs.get(&input_type.name).unwrap()(value)
    }
}

fn execute(args: &ParseArgs) {
    let buffer = utils::read_buffer_from_filepath(&args.server_schema);
    let mut registry = TypeRegistry::new();
    libgql::json::parsers::schema::parse_server_schema(
        &mut registry,
        serde_json::from_str::<serde_json::Value>(&buffer).unwrap(),
    )
    .unwrap();
    let mut parse_registry = ParseRegistry::default();
    parse_registry.scalars.insert(
        "String".into(),
        Box::new(|scalar| match scalar {
            ExampleScalar::String(s) => Ok(Box::new(s.clone())),
            _ => Err(format!(
                "Unexpected scalar value for String scalar: {:?}",
                { scalar }
            )),
        }),
    );
    libgql::executor::execute::<ExampleScalar, ParseRegistry>(
        &registry,
        &parse_registry,
        &utils::read_buffer_from_filepath(&args.query_path),
        &args.variables.as_ref().map_or(
            libgql::executor::Variables::new(),
            |v| {
                parse_variables_from_json(
                    &serde_json::from_str::<serde_json::Value>(&v).unwrap(),
                )
                .unwrap()
            },
        ),
        &args.operation,
    )
    .unwrap()
}

impl Commands {
    pub fn execute(self: &Self) {
        match self {
            Commands::Execute(args) => execute(args),
        }
    }
}
