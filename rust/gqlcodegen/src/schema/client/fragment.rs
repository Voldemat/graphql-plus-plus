use std::collections::HashMap;

use crate::schema::client::argument::Argument;

#[derive(Debug, serde::Deserialize)]
pub struct FieldSelection {
    pub name: String,
    pub alias: String,
    pub arguments: HashMap<String, Argument>,
    pub selection: Option<FragmentSpec>
}

#[derive(Debug, serde::Deserialize)]
pub struct TypenameSelection {
    pub alias: Option<String>
}

#[derive(Debug, serde::Deserialize)]
pub struct SpreadSelection {
    pub fragment: String
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "_type")]
pub enum ObjectSelection {
    #[serde(rename(deserialize = "FieldSelection"))]
    Field(FieldSelection),
    #[serde(rename(deserialize = "TypenameField"))]
    Typename(TypenameSelection),
    #[serde(rename(deserialize = "SpreadSelection"))]
    Spread(SpreadSelection)
}



#[derive(Debug, serde::Deserialize)]
#[serde(tag = "_type")]
pub enum UnionSelection {
    #[serde(rename(deserialize = "ObjectConditionalSpreadSelection"))]
    ObjectConditionalSpread {
        object: String,
        spec: ObjectFragmentSpec
    },
    #[serde(rename(deserialize = "UnionConditionalSpreadSelection"))]
    UnionConditionalSpread {
        union: String,
        selections: Vec<UnionSelection>
    },
    #[serde(rename(deserialize = "TypenameField"))]
    Typename(TypenameSelection),
    #[serde(rename(deserialize = "SpreadSelection"))]
    Spread(SpreadSelection)
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "_type")]
pub struct ObjectFragmentSpec {
    pub name: String,
    pub selections: Vec<ObjectSelection>
}

#[derive(Debug, serde::Deserialize)]
#[serde(tag = "_type")]
pub enum FragmentSpec {
    #[serde(rename(deserialize = "ObjectFragmentSpec"))]
    Object(ObjectFragmentSpec),
    #[serde(rename(deserialize = "UnionFragmentSpec"))]
    Union{
        name: String,
        selections: Vec<UnionSelection>
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Fragment {
    #[serde(rename(deserialize = "sourceText"))]
    source_text: String,
    spec: FragmentSpec
}


