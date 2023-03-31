use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerError {
    pub result: String,
    pub errors: Vec<String>,
}
