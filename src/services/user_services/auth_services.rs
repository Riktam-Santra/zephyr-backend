use actix_web::{
    post,
    web::{block, Data, Json},
    HttpResponse, Responder,
};
use jwt_simple::prelude::{Claims, Duration, MACLike};

use crate::{
    db::auth_queries::get_auth_user_data,
    models::{
        auth_models::{token_models::Token, PostLoginBody},
        error_models::ServerError,
    },
    AppState,
};

#[post("/users/auth/login")]
async fn authentiate_users(app_state: Data<AppState>, req: Json<PostLoginBody>) -> impl Responder {
    match block(move || -> Option<Token> {
        match app_state.db.get() {
            Ok(mut conn) => match get_auth_user_data(&mut conn, req) {
                Ok(opt_user) => match opt_user {
                    Some(user) => {
                        let claims_mins =
                            Claims::with_custom_claims(user.clone(), Duration::from_hours(1));
                        let claims_days =
                            Claims::with_custom_claims(user.clone(), Duration::from_days(30));

                        match app_state.key.authenticate(claims_days) {
                            Ok(token_days) => match app_state.key.authenticate(claims_mins) {
                                Ok(token_mins) => {
                                    return Some(Token {
                                        result: "ok".to_string(),
                                        session: token_mins,
                                        refresh: token_days,
                                    });
                                }
                                Err(e) => {
                                    log::error!("{}", e);
                                    return None;
                                }
                            },
                            Err(e) => {
                                log::error!("{}", e);
                                return None;
                            }
                        }
                    }
                    None => {
                        return None;
                    }
                },
                Err(e) => {
                    log::error!("{}", e);
                    return None;
                }
            },
            Err(e) => {
                log::error!("{}", e);
                return None;
            }
        }
    })
    .await
    {
        Ok(opt_token) => match opt_token {
            Some(token) => {
                return HttpResponse::Ok()
                    .content_type("application/json")
                    .body(serde_json::to_string(&token).unwrap());
            }
            None => {
                let err = ServerError {
                    result: "error".to_string(),
                    errors: vec![String::from("username/password incorrect")],
                };
                return HttpResponse::NotFound()
                    .content_type("application/json")
                    .body(serde_json::to_string(&err).unwrap());
            }
        },
        Err(e) => {
            log::error!("{}", e);
            return HttpResponse::InternalServerError()
                .content_type("application/json")
                .finish();
        }
    }
}
