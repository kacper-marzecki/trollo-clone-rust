use crate::model::{Board, Card, CardTaskItem, Lane, User};
use actix_web::web::{get, ServiceConfig};
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Scope};
pub use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, Pool};

async fn get_current_user(pool: web::Data<Pool<PgConnection>>) -> HttpResponse {
    info!("get_current_user");
    HttpResponse::Ok().json(User {
        id: "userId".to_string(),
        name: "username".to_string(),
        email: "email@email.com".to_string(),
        avatar_id: "avatarId".to_string(),
        roles: vec!["USER".to_string()],
    })
}

async fn get_user_by_id(r: HttpRequest) -> HttpResponse {
    info!(
        "get_user_by_id {}",
        &r.match_info()["userId"].parse::<u8>().unwrap()
    );
    let id = &r.match_info()["userId"];
    HttpResponse::Ok().json(User {
        id: id.to_string(),
        name: "username".to_string(),
        email: "email@email.com".to_string(),
        avatar_id: "avatarId".to_string(),
        roles: vec!["USER".to_string()],
    })
}

pub fn user_routes() -> Scope {
    web::scope("/user")
        .route("", web::get().to(get_current_user))
        .route("/{userId}", web::get().to(get_user_by_id))
}
