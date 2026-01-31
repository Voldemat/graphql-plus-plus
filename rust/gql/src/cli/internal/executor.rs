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
        vars: &libgql::executor::Variables<ExampleScalar>,
    ) -> Result<Self, String> {
        let direction = <Direction as libgql::executor::GQLEnum>::from_str(
            vars.get("direction")
                .ok_or("Missing required field \"direction\"")?
                .get_str()
                .ok_or("Invalid scalar for enum EUsersTagField")?,
        )?;
        let field = <EUsersTagField as libgql::executor::GQLEnum>::from_str(
            vars.get("field")
                .ok_or("Missing required field \"field\"")?
                .get_str()
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

fn execute(args: &ParseArgs) {
    let buffer = utils::read_buffer_from_filepath(&args.server_schema);
    let mut registry = TypeRegistry::new();
    libgql::json::parsers::schema::parse_server_schema(
        &mut registry,
        serde_json::from_str::<serde_json::Value>(&buffer).unwrap(),
    )
    .unwrap();
    let mut parse_registry =
        libgql::executor::HashMapRegistry::<ExampleScalar>::default();
    parse_registry.scalars.insert(
        "String".into(),
        Box::new(
            <String as libgql::executor::GQLScalar<ExampleScalar>>::from_scalar_to_any,
        ),
    );
    parse_registry.scalars_array.insert(
        "String".into(),
        Box::new(
            <String as libgql::executor::GQLScalar<ExampleScalar>>::from_scalar_array_to_any,
        ),
    );
    parse_registry.scalars.insert(
        "Boolean".into(),
        Box::new(
            <bool as libgql::executor::GQLScalar<ExampleScalar>>::from_scalar_to_any,
        ),
    );
    parse_registry.scalars_array.insert(
        "Boolean".into(),
        Box::new(
            <bool as libgql::executor::GQLScalar<ExampleScalar>>::from_scalar_array_to_any,
        ),
    );
    parse_registry.scalars.insert(
        "Int".into(),
        Box::new(<i32 as libgql::executor::GQLScalar<ExampleScalar>>::from_scalar_to_any),
    );
    parse_registry.scalars_array.insert(
        "Int".into(),
        Box::new(<i32 as libgql::executor::GQLScalar<ExampleScalar>>::from_scalar_array_to_any),
    );
    parse_registry.scalars.insert(
        "Float".into(),
        Box::new(<f32 as libgql::executor::GQLScalar<ExampleScalar>>::from_scalar_to_any),
    );
    parse_registry.scalars_array.insert(
        "Float".into(),
        Box::new(<f32 as libgql::executor::GQLScalar<ExampleScalar>>::from_scalar_array_to_any),
    );
    parse_registry
        .inputs
        .insert("UsersTagSortBy".into(), Box::new(<UsersTagSortBy as libgql::executor::GQLInput<ExampleScalar>>::from_variables_to_any));
    libgql::executor::execute::<
        ExampleScalar,
        libgql::executor::HashMapRegistry<ExampleScalar>,
    >(
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
