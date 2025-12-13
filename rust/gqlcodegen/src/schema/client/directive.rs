use std::collections::HashMap;

use crate::schema::shared::InputField;

#[derive(Debug, serde::Deserialize)]
pub enum DirectiveLocation {
    #[serde(rename(deserialize = "QUERY"))]
    Query,
    #[serde(rename(deserialize = "MUTATION"))]
    Mutation,
    #[serde(rename(deserialize = "SUBSCRIPTION"))]
    Subscription,
    #[serde(rename(deserialize = "FIELD"))]
    Field,
    #[serde(rename(deserialize = "FRAGMENT_DEFINITION"))]
    FragmentDefinition,
    #[serde(rename(deserialize = "FRAGMENT_SPREAD"))]
    FragmentSpread,
    #[serde(rename(deserialize = "INLINE_FRAGMENT"))]
    InlineFragment,
    #[serde(rename(deserialize = "VARIABLE_DEFINITION"))]
    VariableDefinition,
}

#[derive(Debug, serde::Deserialize)]
pub struct Directive {
    name: String,
    arguments: HashMap<String, InputField>,
    locations: Vec<DirectiveLocation>
}
