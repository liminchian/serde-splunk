use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Parser {
    field: Vec<Field>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Field {
    modes: Mode,
}

#[derive(Debug, Deserialize, Serialize)]
struct Mode {
    #[serde(default)]
    value: Vec<Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Value {
    text: String,
}
