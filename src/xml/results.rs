use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Parser {
    result: Results,
}

#[derive(Debug, Deserialize, Serialize)]
struct Results {
    field: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Field {
    #[serde(rename = "@k")]
    name: String,
    #[serde(rename = "$value")]
    object: Vec<Object>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Object {
    Value { text: String },
    V(String),
}
