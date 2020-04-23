use actix_web::{App, error, Error, HttpRequest, HttpResponse, HttpServer, middleware, Scope, web};
use actix_web::http::StatusCode;
use actix_web::web::{get, ServiceConfig};
pub use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, Pool};

use crate::app_error::AppError;
use crate::model::{Board, Card, CardTaskItem, Lane, User};
use crate::utils::is_blank;
use crate::service::user_service;
use futures_util::FutureExt;

#[derive(Serialize)]
struct UserInfoResponse {
    id: String,
    name: String,
    email: String,
    avatar_id: String,
    roles: Vec<String>,
}

async fn get_current_user(pool: web::Data<Pool<PgConnection>>) -> Result<HttpResponse, Error> {
    info!("get_current_user");
    Ok(HttpResponse::Ok().json(UserInfoResponse {
        id: "userId".to_string(),
        name: "username".to_string(),
        email: "email@email.com".to_string(),
        avatar_id: "avatarId".to_string(),
        roles: vec!["USER".to_string()],
    }))
}


#[derive(Deserialize)]
pub struct UserRegisterRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub password_confirmation: String
}

impl UserRegisterRequest {
    pub fn validate(&self) -> Result<&Self, AppError> {
        if is_blank(&self.email)
        || is_blank(&self.password)
        || is_blank(&self.password_confirmation)
        || self.password.eq(self.password_confirmation.as_str()){
            Ok(self)
        } else {
            Err(AppError::new("validation Error".to_string(), StatusCode::from_u16(400).unwrap()))
        }
    }
}

async fn register_user(request: web::Json<UserRegisterRequest>, pool: web::Data<Pool<PgConnection>>) -> Result<HttpResponse, AppError> {
    request.validate()?;
    user_service::register_user(request.0, pool).await
        .map(|_| HttpResponse::Ok().finish())
}


async fn get_user_by_id(r: HttpRequest) -> HttpResponse {
    info!(
        "get_user_by_id {}",
        &r.match_info()["userId"].parse::<u8>().unwrap()
    );
    let id = &r.match_info()["userId"];
    HttpResponse::Ok().json(UserInfoResponse {
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
        .route("", web::post().to(register_user))
        .route("/{userId}", web::get().to(get_user_by_id))
}
