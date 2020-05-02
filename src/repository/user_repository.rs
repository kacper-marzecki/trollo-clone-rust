use sqlx::pool::PoolConnection;
use crate::app_error::AppError;
use sqlx::{PgConnection, Transaction, FromRow, QueryAs, Connect, Connection, Error};
use sqlx::prelude::PgQueryAs;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use std::convert::TryInto;
use actix_web::dev::Url;
use uuid::Variant::Future;
use std::borrow::BorrowMut;
use crate::repository::{Repository, RepositoryImpl};

type Conn = PoolConnection<PgConnection>;
type Trans = Transaction<Conn>;
//
// #[async_trait]
// pub trait TransactionSource {
//     async fn commit(&self) ->  Result<(), sqlx::Error>;
//     async fn rollback(&self) -> Result<(), Error>;
//     fn get(self) -> Transaction<PoolConnection<PgConnection>>;
// }
//
// #[async_trait]
// impl TransactionSource for Transaction<PoolConnection<PgConnection>> {
//     async fn commit(&self) -> Result<(), Error> {
//         unimplemented!()
//     }
//     async fn rollback(&self) -> Result<(), Error> {
//         unimplemented!()
//     }
//
//     fn get(self) -> Transaction<PoolConnection<PgConnection>> {
//         unimplemented!()
//     }
// }

#[async_trait]
pub trait UserRepository: Repository {
    async fn create_user(&mut self, dto: CreateUserDto) -> Result<u64, AppError>;
    async fn exists_by_username_or_email(&mut self, username: &String, email: &String) -> Result<bool, AppError>;
}



#[async_trait]
impl UserRepository for RepositoryImpl {
    async fn create_user(&mut self,  dto: CreateUserDto) -> Result<u64, AppError> {
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