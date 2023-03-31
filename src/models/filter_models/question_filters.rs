use serde::{Deserialize, Serialize};

use crate::enums::order_enums::OrderEnums;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionFilters {
    pub title: Option<String>,
    pub uuid: Option<String>, // uuid of the author
    #[serde(rename = "quesUuids")]
    pub ques_uuids: Option<Vec<String>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
    #[serde(rename = "orderBy")]
    pub order_by: Option<OrderEnums>,
    pub answered: Option<bool>,
}
