use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub result: String,
    pub session: String,
    pub refresh: String,
}
