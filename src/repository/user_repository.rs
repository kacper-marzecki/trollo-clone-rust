use crate::app_error::AppError;
use async_trait::async_trait;
use futures_util::future::BoxFuture;
use std::convert::TryInto;
use actix_web::dev::Url;
use std::borrow::BorrowMut;
use std::pin::Pin;
use std::future::Future;
use r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::{RunQueryDsl, sql_query};
use diesel::sql_types::{Time, Timestamp, Integer, Bool, Text};
use chrono::{DateTime, Local, TimeZone};
use diesel::pg::types::sql_types::Timestamptz;
use diesel::expression::sql_literal::sql;

struct UserRepository(PooledConnection<ConnectionManager<diesel::pg::PgConnection>>);

impl UserRepository  {
    fn create_user(&mut self, dto: CreateUserDto) -> Result<bool, AppError> {
        let created = sql_query("
        insert into users (username, email, password, avatar_id, created_at)
                                values ($1, $2, $3, NULL, $4);
          ").bind::<Text, _>(dto.username.as_str())
            .bind::<Text, _>(dto.email)
            .bind::<Text, _>(dto.password)
            .bind::<Timestamptz, _>(chrono::Utc::now())
            .get_result::<i32>(&self.0)
            .map(|it| {
                match it {
                    1 => true,
                    _ => false
                }
            })?;
        Ok(created)
    }

    fn exists_by_username_or_email(&mut self, username: &String, email: &String) -> Result<bool, AppError> {
        let mut a = diesel::sql_query("
        select exists(select 1 from users where username = $1 or email = $2)
    ").bind(username)
            .bind::<String, _>(email)
            .get_result(&self.0)
            .map(|it| {
                match it {
                    1 => true,
                    _ => false
                }
            })?;
        Ok(a)
    }
}

// INPUT
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

// OUTPUT
struct Exists {
    pub exists: bool
}