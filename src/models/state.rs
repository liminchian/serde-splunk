use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum DispatchState {
    Parsing,
    Running,
    Pause,
    Finalizing,
    Failed,
    Done,

    #[serde(other)]
    Queue,
}
