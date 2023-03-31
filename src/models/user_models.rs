mod user_impls;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalUser {
    // Internal User struct to pass it around
    // This struct should NEVER be send out of the server as is.
    pub uuid: String,
    pub username: String,
    pub loc: Option<String>,
    pub avatar: Option<String>,
    pub hash: String,
    pub salt: String,
    pub is_contrib: bool,
    pub is_admin: bool,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    // Info required to add new users
    pub username: String,
    pub password: String,
    pub loc: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    //The user to return when user data is requested
    pub uuid: String,
    pub username: String,
    pub loc: Option<String>,
    pub avatar: Option<String>,
    pub is_contrib: bool,
    pub is_admin: bool,
    pub verified: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MultipleUsers {
    // A simple wrapper for multiple users to send it as an array of users inside
    // of a single object rather than sending the whole array directly
    pub users: Vec<User>,
}
