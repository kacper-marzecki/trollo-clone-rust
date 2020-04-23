extern crate dotenv;
#[macro_use] extern crate diesel_migrations;

use dotenv::dotenv;

pub use log::{error, info, trace, warn};
mod api;
mod model;
use crate::api::board_api::board_routes;
use crate::api::card_api::card_routes;
use crate::api::lane_api::lane_routes;
use crate::api::routes;
use crate::api::user_api::user_routes;
use crate::model::{Board, Card, CardTaskItem, Lane, User};
use actix_web::web::get;
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool, Pool, Connect};
use diesel::Connection;

diesel_migrations::embed_migrations!("migrations");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_web=info");
    dotenv().ok();
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("Cannot determine database url");
    let migration_connection = diesel::pg::PgConnection::establish(&db_url)
        .expect("Cannot aquire connection for db migration");
    embedded_migrations::run(&migration_connection);
    let db_pool = PgPool::new(db_url.as_str())
        .await
        .expect("Cannot create db pool");

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .configure(routes)
    })
    .bind(format!(
        "0.0.0.0:{}",
        std::env::var("PORT").unwrap_or("8080".to_string()).as_str()
    ))?
    .run()
    .await
}
