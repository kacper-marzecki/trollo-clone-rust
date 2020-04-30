use crate::app_error::AppError;
use sqlx::{PgConnection, Pool, Executor};
use actix_web::web;
use crate::api::user_api::UserRegisterRequest;
use diesel::sql_query;
use futures_util::FutureExt;
use bcrypt::{hash, DEFAULT_COST};

// TODO no exists checks
pub async fn register_user(
    request: UserRegisterRequest,
    pool: web::Data<Pool<PgConnection>>
) -> Result<(), AppError> {
    let mut conn = pool.acquire().await?;
    sqlx::query("
        insert INTO users (username, email, password, avatar_id, created_at)
            values ($1, $2, $3, NULL, $4);
    ").bind(request.username)
        .bind(request.email)
        .bind(hash(request.password, DEFAULT_COST).unwrap())
        .bind(chrono::Local::now())
        .execute(conn)
        .await
        .map(|_|{})
        .map_err(AppError::from)
}