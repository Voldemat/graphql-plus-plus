use super::ast::Values;
use super::scalar::Scalar;

pub type ResolvedVariable = Box<dyn std::any::Any + Send + Sync>;
pub trait ParseRegistry<S: Scalar> {
    fn parse_scalar(
        self: &Self,
        scalar_name: &str,
        value: S,
    ) -> Result<ResolvedVariable, String>;

    fn parse_scalar_array(
        self: &Self,
        scalar_name: &str,
        values: Vec<S>,
    ) -> Result<ResolvedVariable, String>;

    fn parse_scalar_optional_array(
        self: &Self,
        scalar_name: &str,
        values: Vec<Option<S>>,
    ) -> Result<ResolvedVariable, String>;

    fn parse_enum(
        self: &Self,
        enum_type_name: &str,
        value: String,
    ) -> Result<ResolvedVariable, String>;

    fn parse_enum_array(
        self: &Self,
        enum_type_name: &str,
        values: Vec<String>,
    ) -> Result<ResolvedVariable, String>;

    fn parse_enum_optional_array(
        self: &Self,
        enum_type_name: &str,
        values: Vec<Option<String>>,
    ) -> Result<ResolvedVariable, String>;

    fn parse_input(
        self: &Self,
        input_type_name: &str,
        value: Values<S>,
    ) -> Result<ResolvedVariable, String>;

    fn parse_input_array(
        self: &Self,
        input_type_name: &str,
        value: Vec<Values<S>>,
    ) -> Result<ResolvedVariable, String>;

    fn parse_input_optional_array(
        self: &Self,
        input_type_name: &str,
        value: Vec<Option<Values<S>>>,
    ) -> Result<ResolvedVariable, String>;
}
