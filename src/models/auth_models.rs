use serde::{Deserialize, Serialize};
pub mod token_models;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostLoginBody {
    pub username: String,
    pub password: String,
}
