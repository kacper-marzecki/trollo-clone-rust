#![cfg_attr(test, feature(proc_macro_hygiene))]

#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

use actix_cors::Cors;
use actix_web::{App, error, Error, HttpRequest, HttpResponse, HttpServer, middleware, web};
use actix_web::web::get;
use deadpool_postgres::{Manager, Pool};
use diesel::Connection;
use dotenv::dotenv;
pub use log::{error, info, trace, warn};
use tokio_postgres::{Config as PgConfig, NoTls};
use std::str::FromStr;
use config::{CONFIG, Config};

use crate::api::board_api::board_routes;
use crate::api::card_api::card_routes;
use crate::api::lane_api::lane_routes;
use crate::api::routes;
use crate::api::user_api::user_routes;
use crate::model::{Board, Card, CardTaskItem, Lane, User};
use crate::repository::ConnPool;
use crate::service::authentication_service::get_identity_service;

mod api;
mod model;
mod app_error;
mod utils;
mod service;
mod repository;
mod config;
mod validation;
diesel_migrations::embed_migrations!("migrations");

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info,actix_web=info");
    env_logger::init();

    let migration_connection = diesel::pg::PgConnection::establish(&CONFIG.database_url.as_str())
        .expect("Cannot aquire connection for db migration");
    embedded_migrations::run(&migration_connection);
    // let manager = PostgresConnectionManager::new(
    //     CONFIG.database_url.as_str().parse().unwrap(),
    //     NoTls,
    // );
    // let db_pool: ConnPool = r2d2::Pool::new(manager).unwrap();
    let mut cfg = PgConfig::from_str(&CONFIG.database_url.as_str()).unwrap();
    // cfg.host("/var/run/postgresql");
    // cfg.user(env::var("USER").unwrap().as_str());
    // cfg.dbname("deadpool");
    // PgConfig::
    let mgr = Manager::new(cfg, tokio_postgres::NoTls);
    let db_pool = Pool::new(mgr, 16);
    let mut a = db_pool.get().await.unwrap();
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .wrap(get_identity_service())
            .wrap(Cors::new().supports_credentials().finish())
            .configure(routes)
    })
        .bind(CONFIG.server_address.clone())?
        .run()
        .await
}
