use actix_web::web::Data;
use argon2::{Argon2, PasswordHasher};
use jwt_simple::prelude::{JWTClaims, MACLike};
use r2d2_postgres::postgres::Row;

use crate::{models::user_models::User, AppState};

pub fn verify_passhash(password: String, row: &Vec<Row>) -> bool {
    let pass_hash = row[0].get::<usize, String>(4);
    let salt = row[0].get::<usize, String>(5);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .hash
        .unwrap()
        .to_string();
    if password_hash == pass_hash {
        return true;
    } else {
        return false;
    }
}

pub fn decode_token(
    token: String,
    state: Data<AppState>,
) -> Result<JWTClaims<User>, jwt_simple::Error> {
    state.key.verify_token::<User>(token.as_str(), None)
}
