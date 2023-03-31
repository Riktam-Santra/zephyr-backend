use actix_extensible_rate_limit::{
    backend::{memory::InMemoryBackend, SimpleInputFunctionBuilder},
    RateLimiter,
};
use actix_web::{web::{Data}, App, HttpServer};

use db::check_and_create_tables;
use jwt_simple::prelude::*;
use log::info;
use r2d2::Pool;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};

mod db;
mod enums;
mod models;
mod services;
mod utils;

use actix_web::dev::Service;
use futures_util::future::FutureExt;

pub type DbPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: DbPool,
    pub key: HS256Key,
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    
    console_subscriber::init();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let manager = r2d2_postgres::PostgresConnectionManager::new(
        match std::env::var("DB_CONFIG").expect("DB_CONFIG is not set.").parse() {
            Ok(config) => config,
            Err(_) => panic!("Couldn't parse server config, make sure your DB_CONFIG environment variable is set to 'host=[HOSTNAME] dbname=[DATABASE_NAME] user=[USERNAME] password=PASSWORD'."),
        },
        NoTls,
    );
    
    match r2d2::Pool::builder().max_size(25).build(manager) {
        Ok(pool) => {
            
            let key = HS256Key::generate();
            // let claims = Claims::create(Duration::from_hours(1));
            let state = AppState { db: pool, key };
            let temp_pool = state.db.clone();
            actix_web::web::block(||{
                check_and_create_tables(temp_pool);
            }).await.expect("");
            HttpServer::new(move || {
                let input =
                    SimpleInputFunctionBuilder::new(std::time::Duration::from_secs(60), 100)
                        .real_ip_key()
                        .build();
                let ratelimiter_backend = InMemoryBackend::builder().build();
                let ratelimit_middleware = RateLimiter::builder(ratelimiter_backend, input)
                    .add_headers()
                    .build();
                let cors = actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header();
                info!("API Ready!");
                App::new()
                    .app_data(Data::new(state.clone()))
                    .service(services::user_services::add_user)
                    .service(services::user_services::get_users)
                    .service(services::user_services::auth_services::authentiate_users)
                    .service(services::question_services::new_question)
                    .service(services::question_services::get_questions)
                    .service(services::question_services::get_single_question)
                    .service(services::answer_services::new_answer)
                    .service(services::answer_services::get_answers)
                    .service(services::answer_services::get_single_answer)
                    .wrap(ratelimit_middleware)
                    .wrap(cors)
                    .wrap_fn(|req, srv| {
                        log::info!(
                            "{} {} [{}] {:#?}",
                            req.connection_info().host(),
                            req.method().to_string(),
                            req.path(),
                            req.version()
                        );
                        return srv.call(req).map(|res| res);
                    })
            })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
        }
        Err(e) => {
            panic!(
                "Unable to connect to database, make sure there is a LOCAL postgres database running!\n UNABLE TO CREATE POOL!!!\n Error was: {:#?}\n Exiting...",
                e
            );
        }
    }
}
