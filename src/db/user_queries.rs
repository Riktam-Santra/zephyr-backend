use actix_web::web::Query;

use crate::{models::{user_models::{InternalUser, User}, filter_models::user_filters::UserSearchFilters}, utils::filter_utils::parse_user_search_filters};

use super::Connection;

pub fn get_users_from_db(conn: &mut Connection, filters: Query<UserSearchFilters>) -> Result<Vec<User>, r2d2::Error> {
    let query = format!("SELECT * FROM users LIMIT 10 {}", parse_user_search_filters(filters));
    let users_query= conn.query( &query, &[]).unwrap();
    

    Ok(users_query.iter().map(|row| {User {
        uuid: row.get::<usize, String>(0),
        username: row.get::<usize, String>(1),
        loc: row.get::<usize, Option<String>>(2),
        avatar: row.get::<usize, Option<String>>(3),
        is_contrib: row.get::<usize, bool>(6),
        is_admin: row.get::<usize, bool>(7),
        verified: row.get::<usize, bool>(8),
    }}).collect())
}

pub fn add_user(conn: &mut Connection, user: InternalUser) -> Result<u64, r2d2_postgres::postgres::Error> {
    match &user.loc {
        
        Some(loc) => {
            
            match &user.avatar {
                Some(avatar) => {
                    match conn.execute("INSERT INTO users (uuid, username, loc, avatar, pass_hash, salt, is_contrib, is_admin, verified) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
                    &[&user.uuid, 
                    &user.username, 
                    loc, 
                    avatar,
                    &user.hash, 
                    &user.salt,
                    &user.is_contrib, 
                    &user.is_admin, 
                    &user.verified,
                    ]){Ok(res) => return Ok(res), Err(e) => return Err(e)}
                },
                None => {
                    match conn.execute("INSERT INTO users (uuid, username, loc, pass_hash, salt, is_contrib, is_admin, verified) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                    &[&user.uuid, 
                    &user.username, 
                    loc,
                    &user.hash, 
                    &user.salt,
                    &user.is_contrib, 
                    &user.is_admin, 
                    &user.verified,
                    ]){Ok(res) => return Ok(res), Err(e) => return Err(e)}
                },
            }
        },
        None => {
            match &user.avatar {
                Some(avatar) => {
                    match conn.execute("INSERT INTO users (uuid, username, avatar, pass_hash, salt, is_contrib, is_admin, verified) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
                    &[&user.uuid, 
                    &user.username, 
                    avatar,
                    &user.hash, 
                    &user.salt,
                    &user.is_contrib, 
                    &user.is_admin, 
                    &user.verified,
                    ]){Ok(res) => return Ok(res), Err(e) => return Err(e)}
                    
                }
                None => {
                    match conn.execute("INSERT INTO users (uuid, username, pass_hash, salt, is_contrib, is_admin, verified) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                    &[&user.uuid, 
                    &user.username, 
                    &user.hash, 
                    &user.salt,
                    &user.is_contrib, 
                    &user.is_admin, 
                    &user.verified,
                    ]){Ok(res) => return Ok(res), Err(e) => return Err(e)}
                }
            }
        }
    }
    
}


pub fn user_exists(conn: &mut Connection, username: &String) -> Option<bool> {
    match conn.query("SELECT * FROM users WHERE username=$1", &[&username]) {
        Ok(query) => Some(query.len() > 0),
        Err(e) => {
            log::error!("{}", e);
            None
        }
    }
}

pub fn get_user_by_uuid_from_db(conn: &mut Connection, uuid: &String) -> Option<User>{
    match conn.query_one("SELECT * FROM users WHERE uuid=$1", &[&uuid]) {
        Ok(query) => Some(User {
            uuid: query.get::<usize, String>(0),
            username: query.get::<usize, String>(1),
            loc: query.get::<usize, Option<String>>(2),
            avatar: query.get::<usize, Option<String>>(3),
            is_contrib: query.get::<usize, bool>(6),
            is_admin: query.get::<usize, bool>(7),
            verified: query.get::<usize, bool>(8),
        }),
        Err(e) => {
            log::error!("{}", e);
            None
        },
        
    }
}
