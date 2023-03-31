use actix_web::{
    get, post,
    web::{self, block, Data, Json, Query},
    HttpResponse, Responder,
};

use crate::{
    db::{
        self,
        user_queries::{get_user_by_uuid_from_db, get_users_from_db, user_exists},
    },
    models::{
        error_models::ServerError,
        filter_models::user_filters::UserSearchFilters,
        ok_model::ResponseOk,
        user_models::{MultipleUsers, NewUser, User},
    },
    AppState,
};

pub mod auth_services;

#[get("/users")]
async fn get_users(query: Query<UserSearchFilters>, app_state: Data<AppState>) -> impl Responder {
    match web::block(move || -> Result<Vec<User>, r2d2::Error> {
        match app_state.db.get() {
            Ok(mut conn) => get_users_from_db(&mut conn, query),
            Err(e) => Err(e),
        }
    })
    .await
    {
        Ok(users_query) => {
            match users_query {
                Ok(users) => {
                    return HttpResponse::Ok()
                        .content_type("application/json")
                        .body(serde_json::to_string(&MultipleUsers { users }).unwrap());
                }
                Err(e) => {
                    print!("{}", e);
                    return HttpResponse::InternalServerError()
                        .content_type("application/json")
                        .finish();
                }
            };
        }
        Err(e) => {
            log::warn!("{}", e);
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .finish();
        }
    };
}

#[post("/users")]
async fn add_user(req: Json<NewUser>, app_state: Data<AppState>) -> impl Responder {
    match block(move || -> Option<u64> {
        match app_state.db.get() {
            Ok(mut conn) => match user_exists(&mut conn, &req.username) {
                Some(exists) => {
                    if exists {
                        return Some(0);
                    } else {
                        let query = db::user_queries::add_user(&mut conn, req.0.into());
                        match query {
                            Ok(no_of_rows) => return Some(no_of_rows),
                            Err(e) => {
                                log::warn!("unable to get data from connection! {:#?}", e);
                                return None;
                            }
                        };
                    }
                }
                None => None,
            },
            Err(e) => {
                log::warn!("Unable to fetch connection from connection pool! {:#?}", e);
                return None;
            }
        }
    })
    .await
    {
        Ok(query) => match query {
            Some(data) => {
                if data == 0 {
                    let err = ServerError {
                        result: "error".to_string(),
                        errors: vec!["User already exists".to_string()],
                    };
                    HttpResponse::Forbidden()
                        .content_type("application/json")
                        .body(serde_json::to_string(&err).unwrap())
                } else {
                    let res = ResponseOk {
                        result: "ok".to_string(),
                    };
                    HttpResponse::Created()
                        .content_type("application/json")
                        .body(serde_json::to_string(&res).unwrap())
                }
            }
            None => HttpResponse::InternalServerError()
                .content_type("application/json")
                .finish(),
        },
        Err(e) => {
            log::error!("{}", e);
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .finish();
        }
    }
}

#[get("/user/{id}")]
async fn get_user_by_uuid(
    uuid: web::Path<String>,
    app_state: web::Data<AppState>,
) -> impl Responder {
    match block(move || -> Result<Option<User>, r2d2::Error> {
        match app_state.db.get() {
            Ok(mut conn) => Ok(get_user_by_uuid_from_db(&mut conn, &uuid)),
            Err(e) => Err(e),
        }
    })
    .await
    {
        Ok(data) => match data {
            Ok(data) => match data {
                Some(data_assure) => {
                    return HttpResponse::Ok().body(serde_json::to_string(&data_assure).unwrap());
                }
                None => {
                    let error = ServerError {
                        result: String::from("error"),
                        errors: [String::from("user not found")].into(),
                    };
                    return HttpResponse::NotFound().body(serde_json::to_string(&error).unwrap());
                }
            },
            Err(e) => {
                log::error!("{}", e);
                let err = ServerError {
                    result: "error".to_string(),
                    errors: vec!["User already exists".to_string()],
                };
                HttpResponse::Forbidden()
                    .content_type("application/json")
                    .body(serde_json::to_string(&err).unwrap())
            }
        },
        Err(e) => {
            log::error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
