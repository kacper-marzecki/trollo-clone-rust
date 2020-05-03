use std::borrow::BorrowMut;
use std::sync::Arc;

use actix_web::{App, error, Error, HttpRequest, HttpResponse, HttpServer, middleware, Scope, web};
use actix_web::http::StatusCode;
use actix_web::web::{get, ServiceConfig};
use diesel::pg::PgConnection;
use futures_util::FutureExt;
pub use log::{error, info, trace, warn};
use serde::{Deserialize, Serialize};
use validator::Validate;
use diesel::connection::Connection;
use crate::app_error::AppError;
use crate::model::{Board, Card, CardTaskItem, Lane, User};
use crate::service::user_service;
use crate::utils::{is_blank, respond_ok};
use crate::validation::validate;
use crate::repository::{ConnPool};
use crate::repository::user_repository::{UserRepository, UserRepositoryImpl};
use std::rc::Rc;

#[derive(Serialize)]
struct UserInfoResponse {
    id: String,
    name: String,
    email: String,
    avatar_id: String,
    roles: Vec<String>,
}

async fn get_current_user(pool: web::Data<ConnPool>) -> Result<HttpResponse, Error> {
    info!("get_current_user");
    Ok(HttpResponse::Ok().json(UserInfoResponse {
        id: "userId".to_string(),
        name: "username".to_string(),
        email: "email@email.com".to_string(),
        avatar_id: "avatarId".to_string(),
        roles: vec!["USER".to_string()],
    }))
}


#[derive(Clone, Debug, Deserialize, Validate)]
pub struct UserRegisterRequest {
    #[validate(length(
    min = 3,
    message = "username is required and must be at least 3 characters"
    ))]
    pub username: String,

    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
    min = 6,
    message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}
// impl UserRegisterRequest {
//     pub fn validate(&self) -> Result<&Self, AppError> {
//         if is_blank(&self.email)
//         || is_blank(&self.password)
//         || is_blank(&self.password_confirmation)
//         || self.password.eq(self.password_confirmation.as_str()){
//             Ok(self)
//         } else {
//             Err(AppError::ValidationError(vec!["validation Error".to_string()]))
//         }
//     }
// }

async fn register_user(request: web::Json<UserRegisterRequest>,
                       pool: web::Data<ConnPool>) -> Result<HttpResponse, AppError> {
    validate(&request)?;
    let mut conn = pool.get().await?;
    let mut transaction = conn.transaction().await?;
    let mut repository = UserRepositoryImpl{conn: Some(&mut transaction)};
    let result = user_service::register_user(&mut repository, request.0)
        .await
        .map(move |result|{
            transaction.commit();
            result
        })?;
    respond_ok()
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String
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

