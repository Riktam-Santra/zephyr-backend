use actix_web::{
    get, post,
    web::{self, block, Data, Json, Query},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::{
    db::question_queries::{add_question, get_questions_from_db, get_single_question_from_db},
    models::{
        filter_models::question_filters::QuestionFilters,
        ok_model::ResponseOk,
        ques_models::{MultipleQuestions, Question, QuestionPostBody},
    },
    utils::auth_utils::decode_token,
    AppState,
};
#[post("/questions")]
pub async fn new_question(
    app_state: Data<AppState>,
    req: Json<QuestionPostBody>,
    auth: BearerAuth,
) -> impl Responder {
    match decode_token(auth.token().to_string(), app_state.clone()) {
        Ok(claims) => {
            match block(move || -> Option<u64> {
                match app_state.db.get() {
                    Ok(mut conn) => match add_question(
                        &mut conn,
                        Question {
                            uuid: uuid::Uuid::new_v4().to_string(),
                            title: req.title.clone(),
                            subtitle: req.subtitle.clone(),
                            created_at: None,
                            answer_id: None,
                            author_id: claims.custom.uuid,
                        },
                    ) {
                        Ok(res) => {
                            return Some(res);
                        }
                        Err(e) => {
                            log::warn!("{}", e);
                            return None;
                        }
                    },
                    Err(e) => {
                        log::warn!("{}", e);
                        None
                    }
                }
            })
            .await
            {
                Ok(rows_opt) => match rows_opt {
                    Some(_) => {
                        let res = ResponseOk {
                            result: "ok".to_string(),
                        };
                        HttpResponse::Created().body(serde_json::to_string(&res).unwrap())
                    }
                    None => {
                        return HttpResponse::InternalServerError().finish();
                    }
                },
                Err(e) => {
                    log::warn!("{}", e);
                    return HttpResponse::InternalServerError().finish();
                }
            }
        }
        Err(e) => {
            log::warn!("{}", e);
            return HttpResponse::Forbidden().finish();
        }
    }
}

#[get("/questions")]
pub async fn get_questions(
    app_state: Data<AppState>,
    filters: Query<QuestionFilters>,
) -> impl Responder {
    match web::block(move || -> Option<Vec<Question>> {
        match app_state.db.get() {
            Ok(mut conn) => match get_questions_from_db(&mut conn, filters) {
                Ok(questions) => Some(questions),
                Err(e) => {
                    log::warn!("{}", e);
                    None
                }
            },
            Err(e) => {
                log::warn!("{}", e);
                return None;
            }
        }
    })
    .await
    {
        Ok(data) => match data {
            Some(data) => {
                return HttpResponse::Ok().content_type("application/json").body(
                    serde_json::to_string(&MultipleQuestions {
                        result: String::from("ok"),
                        questions: data,
                    })
                    .unwrap(),
                );
            }
            None => {
                return HttpResponse::InternalServerError().finish();
            }
        },
        Err(e) => {
            log::warn!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

#[get("/questions/{uuid}")]
async fn get_single_question(uuid: web::Path<String>, app_state: Data<AppState>) -> impl Responder {
    match web::block(move || -> Option<Question> {
        match app_state.db.get() {
            Ok(mut conn) => match get_single_question_from_db(&mut conn, uuid.to_string()) {
                Ok(data) => Some(data),
                Err(e) => {
                    log::warn!("{}", e);
                    None
                }
            },
            Err(e) => {
                log::warn!("{}", e);
                None
            }
        }
    })
    .await
    {
        Ok(question) => match question {
            Some(question) => {
                return HttpResponse::Ok().body(serde_json::to_string(&question).unwrap())
            }
            None => return HttpResponse::NotFound().finish(),
        },
        Err(e) => {
            log::warn!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
