use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parser {
    entry: Vec<Entry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    content: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "@type")]
    content_type: String,
    dict: NestedDict,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedDict {
    #[serde(rename = "key")]
    object: Vec<Object>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Object {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$value")]
    value: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    #[serde(rename = "$text")]
    String(String),
    #[serde(rename = "dict")]
    NestedDict(NestedDict),
    #[serde(rename = "list")]
    List {
        #[serde(default)]
        item: Vec<String>,
    },
}
