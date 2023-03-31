use std::time::SystemTime;

use crate::{models::answer_models::Answer, utils};

use super::Connection;

pub fn get_answers_from_db(
    conn: &mut Connection,
) -> Result<Vec<Answer>, r2d2_postgres::postgres::Error> {
    let query = conn.query("SELECT * FROM answers LIMIT 10", &[])?;
    return Ok(query
        .into_iter()
        .map(|x| {
            log::info!("{}", x.get::<usize, String>(4));
            let created_at: SystemTime = x.get(4);
            Answer {
                uuid: x.get(0),
                content: x.get(1),
                author_uuid: x.get(2),
                question_uuid: x.get(3),
                created_at: Some(
                    utils::time_utils::system_time_to_epoch(created_at)
                        .as_millis()
                        .to_string(),
                ),
            }
        })
        .collect::<Vec<Answer>>());
}

pub fn get_single_answer_from_db(
    conn: &mut Connection,
    uuid: String,
) -> Result<Answer, r2d2_postgres::postgres::Error> {
    let query = conn.query_one("SELECT * FROM answers WHERE uuid=$1", &[&uuid])?;
    let created_at: SystemTime = query.get(4);
    return Ok(Answer {
        uuid: query.get(0),
        content: query.get(1),
        author_uuid: query.get(2),
        question_uuid: query.get(3),
        created_at: Some(
            utils::time_utils::system_time_to_epoch(created_at)
                .as_millis()
                .to_string(),
        ),
    });
}

pub fn add_answer(
    conn: &mut Connection,
    answer: Answer,
) -> Result<u64, r2d2_postgres::postgres::Error> {
    return conn.execute(
        "INSERT INTO answers (uuid, content, question_uuid, author_uuid) VALUES ($1, $2, $3, $4)",
        &[
            &answer.uuid,
            &answer.content,
            &answer.question_uuid,
            &answer.author_uuid,
        ],
    );
}
