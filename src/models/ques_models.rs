use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionPostBody {
    pub title: String,
    pub subtitle: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub uuid: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub created_at: Option<String>,
    pub answer_id: Option<String>,
    pub author_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultipleQuestions {
    pub result: String,
    pub questions: Vec<Question>,
}
