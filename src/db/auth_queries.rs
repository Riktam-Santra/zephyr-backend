use crate::{
    models::{auth_models::PostLoginBody, user_models::User},
    utils::auth_utils::verify_passhash,
};
use actix_web::web::Json;
use r2d2_postgres::postgres::Error;

use super::Connection;

pub fn get_auth_user_data(
    conn: &mut Connection,
    req: Json<PostLoginBody>,
) -> Result<Option<User>, Error> {
    match conn.query(
        "SELECT * FROM users WHERE username=$1 LIMIT 1",
        &[&req.username],
    ) {
        Ok(users_query) => {
            if verify_passhash(req.password.clone(), &users_query) {
                return Ok(Some(User {
                    uuid: users_query[0].get::<usize, String>(0),
                    username: users_query[0].get::<usize, String>(1),
                    loc: users_query[0].get::<usize, Option<String>>(2),
                    avatar: users_query[0].get::<usize, Option<String>>(3),
                    is_contrib: users_query[0].get::<usize, bool>(6),
                    is_admin: users_query[0].get::<usize, bool>(7),
                    verified: users_query[0].get::<usize, bool>(8),
                }));
            } else {
                return Ok(None);
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}
