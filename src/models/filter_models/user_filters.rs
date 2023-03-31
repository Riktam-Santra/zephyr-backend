use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSearchFilters {
    pub uuids: Option<Vec<String>>,
}
