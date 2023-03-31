use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub uuid: String,
    pub content: String,
    pub author_uuid: String,
    pub question_uuid: String,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AnswerPostBody {
    pub question_uuid: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultipleAnswers {
    pub result: String,
    pub answers: Vec<Answer>,
}
