//! Lexicon schema parser.

use std::collections::BTreeMap;

use serde::{de::Unexpected, Deserialize};

/// The top level of a Lexicon schema file.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Lexicon {
    /// The Lexicon language version used by this schema.
    pub lexicon: i64,
    /// The NSID of this Lexicon schema.
    pub id: std::string::String,
    /// The version of this Lexicon schema, if changes have occurred.
    pub revision: Option<i64>,
    /// Optional description of this Lexicon schema.
    pub description: Option<std::string::String>,
    pub defs: BTreeMap<std::string::String, Schema>,
}

/// A typed schema item.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Schema {
    Array(Array),
    Blob(Blob),
    Boolean(Boolean),
    Bytes(Bytes),
    CidLink,
    Integer(Integer),
    Null,
    Object(Object),
    Params(Object),
    Procedure(Procedure),
    Query(Query),
    Record(Record),
    Ref(Ref),
    String(String),
    Subscription(Subscription),
    Token(Token),
    Union(Union),
    Unknown,
}

/// A typed schema item.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum FieldSchema {
    Array(Array),
    Blob(Blob),
    Boolean(Boolean),
    Bytes(Bytes),
    CidLink,
    Integer(Integer),
    Ref(Ref),
    String(String),
    Union(Union),
    Unknown,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Array {
    pub description: Option<std::string::String>,
    pub items: Box<FieldSchema>,
    pub min_length: Option<i64>,
    pub max_length: Option<i64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Blob {
    pub description: Option<std::string::String>,
    pub accept: Option<Vec<std::string::String>>,
    pub max_size: Option<i64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Boolean {
    pub description: Option<std::string::String>,
    pub default: Option<bool>,
    #[serde(rename = "const")]
    #[doc(alias = "const")]
    pub is_const: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Bytes {
    pub description: Option<std::string::String>,
    pub min_length: Option<i64>,
    pub max_length: Option<i64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Integer {
    pub description: Option<std::string::String>,
    pub minimum: Option<i64>,
    pub maximum: Option<i64>,
    #[serde(rename = "enum")]
    #[doc(alias = "enum")]
    pub enum_values: Option<Vec<i64>>,
    pub default: Option<i64>,
    #[serde(rename = "const")]
    #[doc(alias = "const")]
    pub is_const: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Object {
    pub description: Option<std::string::String>,
    pub properties: BTreeMap<std::string::String, FieldSchema>,
    #[serde(default)]
    pub required: Vec<std::string::String>,
    #[serde(default)]
    pub nullable: Vec<std::string::String>,
}

/// The input of a procedure or query.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub description: Option<std::string::String>,
    pub encoding: std::string::String,
    pub schema: Option<IoSchema>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum IoSchema {
    Object(Object),
    Ref(Ref),
    Union(Union),
}

/// The output of a procedure or query.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Output {
    pub description: Option<std::string::String>,
    pub encoding: std::string::String,
    pub schema: Option<IoSchema>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Error {
    pub name: std::string::String,
    pub description: Option<std::string::String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Procedure {
    pub description: Option<std::string::String>,
    #[serde(default, deserialize_with = "deserialize_params_schema")]
    pub parameters: Option<Object>,
    pub output: Option<Output>,
    pub input: Option<Input>,
    pub errors: Option<Vec<Error>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Query {
    pub description: Option<std::string::String>,
    #[serde(default, deserialize_with = "deserialize_params_schema")]
    pub parameters: Option<Object>,
    pub output: Option<Output>,
    pub errors: Option<Vec<Error>>,
}

fn deserialize_params_schema<'de, D>(des: D) -> Result<Option<Object>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;

    let schema = match Option::<Schema>::deserialize(des)? {
        Some(s) => s,
        None => return Ok(None),
    };

    match schema {
        Schema::Params(object) => Ok(Some(object)),
        _ => Err(D::Error::invalid_value(
            Unexpected::Enum,
            &"a schema of type `params`",
        )),
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub description: Option<std::string::String>,
    pub key: std::string::String,
    #[serde(deserialize_with = "Record::deserialize_record_field")]
    pub record: Object,
}

impl Record {
    fn deserialize_record_field<'de, D>(des: D) -> Result<Object, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let schema = Schema::deserialize(des)?;
        match schema {
            Schema::Object(object) => Ok(object),
            _ => Err(D::Error::invalid_value(
                Unexpected::Enum,
                &"a schema of type `object`",
            )),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ref {
    pub description: Option<std::string::String>,
    #[serde(rename = "ref")]
    #[doc(alias = "ref")]
    pub ref_: std::string::String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct String {
    pub description: Option<std::string::String>,
    pub format: Option<StringFormat>,
    pub max_length: Option<i64>,
    pub min_length: Option<i64>,
    pub max_graphemes: Option<i64>,
    pub min_graphemes: Option<i64>,
    #[serde(default)]
    pub known_values: Vec<std::string::String>,
    #[serde(rename = "enum")]
    #[doc(alias = "enum")]
    pub enum_values: Option<Vec<std::string::String>>,
    pub default: Option<std::string::String>,
    #[serde(rename = "const")]
    #[doc(alias = "const")]
    pub const_value: Option<std::string::String>,
}

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub enum StringFormat {
    AtIdentifier,
    AtUri,
    Cid,
    Datetime,
    Did,
    Handle,
    Language,
    Nsid,
    RecordKey,
    Tid,
    Uri,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Subscription {
    pub description: Option<std::string::String>,
    pub parameters: Option<Box<Schema>>,
    pub message: Option<Message>,
    pub errors: Option<Vec<Error>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Message {
    pub description: Option<std::string::String>,
    pub schema: Box<Schema>,
    pub errors: Option<Vec<Error>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Token {
    pub description: Option<std::string::String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Union {
    pub description: Option<std::string::String>,
    pub refs: Vec<std::string::String>,
    #[serde(default)]
    pub closed: bool,
}
