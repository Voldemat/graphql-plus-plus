use crate::{
    executor::{LiteralValue, NonNullableValue, ParseRegistry, Scalar, Value},
    parsers::schema::shared,
};

fn resolve_literal_array<S: Scalar, R: ParseRegistry<S>>(
    registry: &R,
    literal_type: &shared::ast::LiteralFieldSpec<shared::ast::InputTypeSpec>,
    nullable: bool,
    elements: Vec<Value<S>>,
) -> Result<Box<dyn std::any::Any>, String> {
    match &literal_type.r#type {
        shared::ast::InputTypeSpec::Enum(e) => {
            R::parse_enum_array(registry, &e, elements.into_iter().map(|e| {
                if let Value::NonNullable(NonNullableValue::Literal(
                    LiteralValue::Scalar(scalar),
                )) = e && let Ok(s) = scalar.try_to_string()
                {
                    Ok(s)
                } else {
                    Err(format!(
                        "Expected string scalar for enum value, received wrong type"
                    ))
                }
            }).collect::<Result<Vec<String>, String>>()?)
        }
        shared::ast::InputTypeSpec::Scalar(scalar_name) => {
            match nullable {
            false => R::parse_scalar_array(registry, &scalar_name, elements.into_iter().map(|e| {
                if let Value::NonNullable(NonNullableValue::Literal(
                    LiteralValue::Scalar(scalar),
                )) = e
                {
                    Ok(scalar)
                } else {
                    Err(format!(
                        "Expected scalar, received: {:?}",
                        e
                    ))
                }
            }).collect::<Result<Vec<_>, String>>()?),
            true => R::parse_scalar_optional_array(registry, &scalar_name, elements.into_iter().map(|e| {
                match e {
                Value::Null => Ok(None),
                Value::NonNullable(NonNullableValue::Literal(
                    LiteralValue::Scalar(scalar)
                )) => Ok(Some(scalar)),
                _ => Err(format!(
                        "Expected scalar, received: {:?}",
                        e
                    ))
                }
            }).collect::<Result<Vec<_>, String>>()?)
            }
        }
        shared::ast::InputTypeSpec::InputType(input_type) => {
            R::parse_input_array(registry, &input_type, elements.into_iter().map(|e| {
                if let Value::NonNullable(NonNullableValue::Literal(
                    LiteralValue::Object(object),
                )) = e
                {
                    Ok(object)
                } else {
                    Err(format!(
                        "Expected object, received: {:?}",
                        e
                    ))
                }
            }).collect::<Result<Vec<_>, String>>()?)
        }
    }
}

pub fn resolve_array<S: Scalar, R: ParseRegistry<S>>(
    registry: &R,
    array_type: &shared::ast::ArrayFieldSpec<shared::ast::InputTypeSpec>,
    elements: Vec<Value<S>>,
) -> Result<Box<dyn std::any::Any>, String> {
    match array_type.r#type.as_ref() {
        shared::ast::NonCallableFieldSpec::Literal(literal) => {
            resolve_literal_array(
                registry,
                literal,
                array_type.nullable,
                elements,
            )
        }
        shared::ast::NonCallableFieldSpec::Array(array) => {
            if !array_type.nullable {
                let mut a = Vec::new();
                for element in elements {
                    let Value::NonNullable(NonNullableValue::Array(
                        nested_elements,
                    )) = element
                    else {
                        return Err("Unexpected value for nested array".into());
                    };
                    a.push(resolve_array(registry, array, nested_elements)?);
                }
                return Ok(Box::new(a));
            } else {
                let mut a = Vec::new();
                for element in elements {
                    match element {
                        Value::Null => a.push(None),
                        Value::NonNullable(NonNullableValue::Array(
                            nested_elements,
                        )) => a.push(Some(resolve_array(
                            registry,
                            array,
                            nested_elements,
                        )?)),
                        _ => {
                            return Err(
                                "Unexpected value for nested array".into()
                            );
                        }
                    };
                }
                Ok(Box::new(a))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;

    use crate::executor::{GQLScalar, HashMapRegistry};

    use super::*;

    #[derive(Debug)]
    enum TestScalar {
        Empty(()),
    }

    impl Scalar for TestScalar {
        fn try_to_string(self: Self) -> Result<String, String> {
            todo!()
        }

        fn from_str(_: &str) -> Result<Self, String> {
            todo!()
        }

        fn from_literal(_: &shared::ast::Literal) -> Result<Self, String> {
            todo!()
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    struct EmptyScalar(());

    impl GQLScalar<TestScalar> for EmptyScalar {
        fn from_scalar(s: TestScalar) -> Result<Self, String> {
            match s {
                TestScalar::Empty(_) => Ok(Self(())),
            }
        }

        fn to_scalar(self: &Self) -> Result<TestScalar, String> {
            Ok(TestScalar::Empty(()))
        }
    }

    #[test]
    fn test_resolve_array() {
        let mut registry = HashMapRegistry::<TestScalar>::default();
        registry.add_scalar::<EmptyScalar>("Empty");
        let result = resolve_array::<TestScalar, HashMapRegistry<TestScalar>>(
            &registry,
            &shared::ast::ArrayFieldSpec::<shared::ast::InputTypeSpec> {
                nullable: true,
                default_value: Some(None),
                r#type: Box::new(shared::ast::NonCallableFieldSpec::Literal(
                    shared::ast::LiteralFieldSpec {
                        default_value: Some(None),
                        r#type: shared::ast::InputTypeSpec::Scalar(
                            "Empty".to_string(),
                        ),
                        directive_invocations: IndexMap::new(),
                    },
                )),
                directive_invocations: Vec::new(),
            },
            vec![
                Value::NonNullable(NonNullableValue::Literal(
                    LiteralValue::Scalar(TestScalar::Empty(())),
                )),
                Value::Null,
            ],
        )
        .unwrap();
        let array = result.downcast_ref::<Vec<Option<EmptyScalar>>>().unwrap();
        assert_eq!(array[0], Some(EmptyScalar(())));
        assert_eq!(array[1], None);
    }

    #[test]
    fn test_resolve_nested_array() {
        let mut registry = HashMapRegistry::<TestScalar>::default();
        registry.add_scalar::<EmptyScalar>("Empty");
        let result = resolve_array::<TestScalar, HashMapRegistry<TestScalar>>(
            &registry,
            &shared::ast::ArrayFieldSpec {
                r#type: Box::new(shared::ast::NonCallableFieldSpec::Array(
                    shared::ast::ArrayFieldSpec::<shared::ast::InputTypeSpec> {
                        nullable: true,
                        default_value: Some(None),
                        r#type: Box::new(
                            shared::ast::NonCallableFieldSpec::Literal(
                                shared::ast::LiteralFieldSpec {
                                    default_value: Some(None),
                                    r#type: shared::ast::InputTypeSpec::Scalar(
                                        "Empty".to_string(),
                                    ),
                                    directive_invocations: IndexMap::new(),
                                },
                            ),
                        ),
                        directive_invocations: Vec::new(),
                    },
                )),
                default_value: Some(None),
                nullable: true,
                directive_invocations: Vec::new(),
            },
            vec![
                Value::Null,
                Value::NonNullable(NonNullableValue::Array(vec![
                    Value::NonNullable(NonNullableValue::Literal(
                        LiteralValue::Scalar(TestScalar::Empty(())),
                    )),
                    Value::Null,
                ])),
            ],
        )
        .unwrap();
        let array = result
            .downcast_ref::<Vec<Option<Box<dyn std::any::Any>>>>()
            .unwrap();
        assert_eq!(array[0].is_none(), true);
        let nested_array = array[1]
            .as_ref()
            .unwrap()
            .downcast_ref::<Vec<Option<EmptyScalar>>>()
            .unwrap();
        assert_eq!(nested_array[0], Some(EmptyScalar(())));
    }
}
