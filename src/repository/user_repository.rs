use sqlx::pool::PoolConnection;
use crate::app_error::AppError;
use sqlx::{PgConnection, Transaction, FromRow, QueryAs, Connect, Connection, Error};
use sqlx::prelude::PgQueryAs;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use std::convert::TryInto;
use actix_web::dev::Url;
use std::borrow::BorrowMut;
use crate::repository::{Repository, RepositoryImpl};
use std::pin::Pin;
use std::future::Future;

type Conn = PoolConnection<PgConnection>;
type Trans = Transaction<Conn>;

// #[cfg_attr(feature = "test", automock)]
#[automock]
pub trait UserRepository: Repository {
    fn create_user<'life0, 'async_trait>(
        &'life0 mut self,
        dto: CreateUserDto
    )-> Pin<Box<dyn Future<Output=Result<u64, AppError>> + Send + 'async_trait>>
        where
            'life0: 'async_trait,
            Self: 'async_trait,;
    fn exists_by_username_or_email<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 mut self,
        username: &'life1 String,
        email: &'life2 String
    )-> Pin<Box<dyn Future<Output=Result<bool, AppError>> + Send + 'async_trait>>
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            Self: 'async_trait,;
}


#[async_trait]
impl UserRepository for RepositoryImpl {
    async fn create_user(&mut self, dto: CreateUserDto) -> Result<u64, AppError> {
        sqlx::query("
        insert into users (username, email, password, avatar_id, created_at)
            values ($1, $2, $3, NULL, $4);
    ").bind(dto.username)
            .bind(dto.email)
            .bind(dto.password)
            .bind(chrono::Local::now())
            .execute(self.get())
            .await
            .map_err(|err| err.into())
    }

    async fn exists_by_username_or_email(&mut self, username: &String, email: &String) -> Result<bool, AppError> {
        let mut a = sqlx::query_as::<sqlx::Postgres, Exists>("
        select exists(select 1 from users where username = $1 or email = $2)
    ").bind(username)
            .bind(email)
            .fetch_one(self.get()).await?;
        Ok(a.exists)
    }
}

// INPUT
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

// OUTPUT
#[derive(sqlx::FromRow)]
struct Exists {
    pub exists: bool
}