use log::info;
use r2d2::{PooledConnection, Pool};
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

pub mod answer_queries;
pub mod auth_queries;
pub mod question_queries;
pub mod user_queries;

pub type Connection = PooledConnection<PostgresConnectionManager<NoTls>>;

pub fn check_and_create_tables(conn_pool: Pool<PostgresConnectionManager<NoTls>>) {
    let mut conn = conn_pool.get().unwrap();
    info!("Checking and creating tables if they don't exist...");

    info!("Checking for users table...");
    conn.execute("CREATE TABLE IF NOT EXISTS users( uuid VARCHAR PRIMARY KEY NOT NULL, username VARCHAR NOT NULL, loc VARCHAR, avatar VARCHAR, pass_hash VARCHAR NOT NULL, salt VARCHAR NOT NULL, is_contrib BOOLEAN NOT NULL, is_admind BOOLEAN NOT NULL, verified BOOLEAN NOT NULL );", &[]).expect("Unable to create questions table, now exiting.");
    info!("Done");

    info!("Checking for questions table...");
    conn.execute("CREATE TABLE IF NOT EXISTS questions( uuid VARCHAR PRIMARY KEY NOT NULL, title VARCHAR NOT NULL, subtitle VARCHAR, created_at TIMESTAMP NOT NULL, answer_id VARCHAR, author_id VARCHAR NOT NULL );", &[]).expect("Unable to create user table, now exiting.");
    info!("Done");

    info!("Checking for answers table...");
    conn.execute("CREATE TABLE IF NOT EXISTS answers( uuid VARCHAR PRIMARY KEY NOT NULL, content VARCHAR NOT NULL, author_uuid VARCHAR NOT NULL, question_uuid VARCHAR NOT NULL, created_at TIMESTAMP NOT NULL );", &[]).expect("Unable to create answers table, now exiting.");
    info!("Done");

    info!("ALL TABLES VERIFIED.");
}