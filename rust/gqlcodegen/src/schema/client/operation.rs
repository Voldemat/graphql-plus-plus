use std::collections::HashMap;

use super::fragment::FragmentSpec;
use crate::schema::shared::InputField;

#[derive(Debug, serde::Deserialize)]
pub enum OperationType {
    #[serde(rename(deserialize = "MUTATION"))]
    Mutation,
    #[serde(rename(deserialize = "QUERY"))]
    Query,
    #[serde(rename(deserialize = "SUBSCRIPTION"))]
    Subscription,
}

#[derive(Debug, serde::Deserialize)]
pub struct Operation {
    name: String,
    #[serde(rename(deserialize = "type"))]
    optype: OperationType,
    parameters: HashMap<String, InputField>,
    #[serde(rename(deserialize = "fragmentSpec"))]
    fragment_spec: FragmentSpec,
    #[serde(rename(deserialize = "sourceText"))]
    source_text: String
}
