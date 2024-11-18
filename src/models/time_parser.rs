use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Parser {
    dict: Dict,
}

#[derive(Debug, Deserialize, Serialize)]
struct Dict {
    #[serde(rename = "key")]
    object: Vec<Object>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Object {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$value")]
    value: String,
}
