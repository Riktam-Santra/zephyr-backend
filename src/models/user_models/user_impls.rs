use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher};
use rand_core::OsRng;

use super::{InternalUser, NewUser};

impl From<NewUser> for InternalUser {
    fn from(new_user: NewUser) -> Self {
        let password = new_user.password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let pasword_hash = argon2.hash_password(password, &salt).unwrap().to_string();
        let parsed_hash = PasswordHash::new(&pasword_hash).unwrap();
        return InternalUser {
            uuid: uuid::Uuid::new_v4().to_string(),
            username: new_user.username,
            loc: new_user.loc,
            avatar: new_user.avatar,
            hash: parsed_hash.hash.unwrap().to_string(),
            salt: parsed_hash.salt.unwrap().to_string(),
            is_contrib: false,
            is_admin: false,
            verified: false,
        };
    }
}
