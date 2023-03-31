use std::time::{SystemTime};

use actix_web::web::Query;

use crate::{
    models::{filter_models::question_filters::QuestionFilters, ques_models::Question},
    utils::{self, filter_utils::parse_question_search_filters},
};

use super::Connection;

pub fn add_question(
    conn: &mut Connection,
    question: Question,
) -> Result<u64, r2d2_postgres::postgres::Error> {
    match conn.execute(
        "INSERT INTO questions (uuid, title, subtitle, answer_id, author_id, created_at) VALUES ($1, $2, $3, $4, $5, $6)",
        &[
            &question.uuid,
            &question.title,
            &question.subtitle,
            &question.answer_id,
            &question.author_id,
            &SystemTime::now(),
        ],
    ) {
        Ok(rows) => Ok(rows),
        Err(e) => Err(e),
    }
}

pub fn get_questions_from_db(
    conn: &mut Connection,
    filters: Query<QuestionFilters>,
) -> Result<Vec<Question>, r2d2_postgres::postgres::Error> {
    let parsed_filters = parse_question_search_filters(filters);

    let query = conn.query(&format!("SELECT * FROM questions {}", parsed_filters), &[])?;
    return Ok(query
        .into_iter()
        .map(|x| {
            let created_at: SystemTime = x.get(3);
            Question {
                uuid: x.get::<usize, String>(0),
                title: x.get::<usize, String>(1),
                subtitle: x.get::<usize, Option<String>>(2),
                created_at: Some(
                    utils::time_utils::system_time_to_epoch(created_at)
                        .as_millis()
                        .to_string(),
                ),
                answer_id: x.get::<usize, Option<String>>(5),
                author_id: x.get::<usize, String>(5),
            }
        })
        .collect::<Vec<Question>>());
}

pub fn get_single_question_from_db(
    conn: &mut Connection,
    uuid: String,
) -> Result<Question, r2d2_postgres::postgres::Error> {
    let query = conn.query_one("SELECT * FROM questions WHERE uuid=$1", &[&uuid])?;
    let created_at: SystemTime = query.get(3);
    return Ok(Question {
        uuid: query.get::<usize, String>(0),
        title: query.get::<usize, String>(1),
        subtitle: query.get::<usize, Option<String>>(2),
        created_at: Some(
            utils::time_utils::system_time_to_epoch(created_at)
                .as_millis()
                .to_string(),
        ),
        answer_id: query.get::<usize, Option<String>>(4),
        author_id: query.get::<usize, String>(5),
    });
}
