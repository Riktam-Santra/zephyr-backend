use crate::{
    db::answer_queries::{add_answer, get_answers_from_db, get_single_answer_from_db},
    models::{
        answer_models::{Answer, AnswerPostBody, MultipleAnswers},
        ok_model::ResponseOk,
    },
    utils::auth_utils::decode_token,
    AppState,
};
use actix_web::{
    get, post,
    web::{self, block, Data, Json},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;

#[get("/answers")]
pub async fn get_answers(app_state: Data<AppState>) -> impl Responder {
    match web::block(move || -> Option<MultipleAnswers> {
        match app_state.db.get() {
            Ok(mut conn) => match get_answers_from_db(&mut conn) {
                Ok(data) => Some(MultipleAnswers {
                    result: "ok".to_string(),
                    answers: data,
                }),
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
        Ok(opt_data) => match opt_data {
            Some(data) => {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&data).unwrap())
            }
            None => {
                return HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .finish()
            }
        },
        Err(e) => {
            log::warn!("{}", e);
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .finish();
        }
    }
}

#[get("/answers/{uuid}")]
pub async fn get_single_answer(
    uuid: web::Path<String>,
    app_state: Data<AppState>,
) -> impl Responder {
    match web::block(move || -> Option<Answer> {
        match app_state.db.get() {
            Ok(mut conn) => match get_single_answer_from_db(&mut conn, uuid.to_string()) {
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
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&question).unwrap())
            }
            None => {
                return HttpResponse::NotFound()
                    .content_type("application/json")
                    .finish()
            }
        },
        Err(e) => {
            log::warn!("{}", e);
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .finish();
        }
    }
}

#[post("/answers")]
pub async fn new_answer(
    app_state: Data<AppState>,
    req: Json<AnswerPostBody>,
    auth: BearerAuth,
) -> impl Responder {
    match decode_token(auth.token().to_string(), app_state.clone()) {
        Ok(claims) => {
            match block(move || -> Option<u64> {
                match app_state.db.get() {
                    Ok(mut conn) => match add_answer(
                        &mut conn,
                        Answer {
                            uuid: uuid::Uuid::new_v4().to_string(),
                            content: req.content.clone(),
                            author_uuid: claims.custom.uuid.clone(),
                            question_uuid: req.question_uuid.clone(),
                            created_at: None,
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
                        HttpResponse::Created()
                            .content_type("application/json")
                            .body(serde_json::to_string(&res).unwrap())
                    }
                    None => {
                        return HttpResponse::InternalServerError()
                            .content_type("application/json")
                            .finish();
                    }
                },
                Err(e) => {
                    log::warn!("{}", e);
                    return HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .finish();
                }
            }
        }
        Err(e) => {
            log::warn!("{}", e);
            return HttpResponse::Forbidden()
                .content_type("application/json")
                .finish();
        }
    }
}
